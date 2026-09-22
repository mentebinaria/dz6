// Every log message in dz6 goes through tracing, so RUST_LOG controls all of
// them, as usual in Rust programs:
//
//     RUST_LOG=debug dz6 file.bin
//     RUST_LOG=dz6::hex::search=trace dz6 file.bin
//
// Messages go to the log window (Alt+l) and to stderr. The TUI owns the
// terminal, so writing to stderr while dz6 is running would mess up the
// screen. That's why we only do it when stderr is redirected somewhere else
// (dz6 file.bin 2> dz6.log). Otherwise the messages stay in the ring buffer
// and flush() prints them after the terminal is restored.

use std::{
    collections::VecDeque,
    env,
    fmt::{self, Write as _},
    io::{self, IsTerminal},
    sync::{LazyLock, Mutex, MutexGuard, OnceLock},
};

use chrono::{DateTime, Local};
use tracing::{
    Event, Level, Subscriber,
    field::{Field, Visit},
    warn,
};
use tracing_subscriber::{
    EnvFilter,
    fmt::{format::Writer, time::FormatTime},
    layer::{Context, Layer, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
};

/// Filter used when RUST_LOG is not set: dz6 at info, other crates at warn
pub const DEFAULT_FILTER: &str = concat!("warn,", env!("CARGO_CRATE_NAME"), "=info");

/// How many messages we keep for the log window. The oldest ones go first
const CAPACITY: usize = 2048;

/// Time format used in the log window
const TIME_FORMAT: &str = "%H:%M:%S%.3f";

/// Same thing for stderr, but with the date, as it usually goes to a file
const STREAM_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

/// Local time, so the window and stderr show the same clock
struct LocalTime;

impl FormatTime for LocalTime {
    fn format_time(&self, writer: &mut Writer<'_>) -> fmt::Result {
        write!(writer, "{}", Local::now().format(STREAM_TIME_FORMAT))
    }
}

/// A single log message
pub struct Record {
    pub time: DateTime<Local>,
    pub level: Level,
    /// module that logged it, like dz6::hex::search
    pub target: &'static str,
    /// innermost span, if the message was logged inside one
    pub span: Option<&'static str>,
    /// the message itself followed by its fields, like "saved bytes=12"
    pub message: String,
}

impl Record {
    /// Writes something like "12:34:56.789 WARN dz6::database{save}: message"
    fn write_to(&self, out: &mut impl io::Write) -> io::Result<()> {
        write!(
            out,
            "{} {:>5} {}",
            self.time.format(TIME_FORMAT),
            self.level.as_str(),
            self.target
        )?;

        if let Some(span) = self.span {
            write!(out, "{{{span}}}")?;
        }

        writeln!(out, ": {}", self.message)
    }

    /// Time as shown in the log window
    pub fn time(&self) -> impl fmt::Display {
        self.time.format(TIME_FORMAT)
    }

    /// Target without our own crate name, so lines in the window are shorter
    pub fn module(&self) -> &'static str {
        self.target
            .strip_prefix(concat!(env!("CARGO_CRATE_NAME"), "::"))
            .unwrap_or(self.target)
    }
}

/// Ring buffer with the most recent messages
pub struct Buffer {
    records: VecDeque<Record>,
    dropped: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            records: VecDeque::with_capacity(256),
            dropped: 0,
        }
    }
}

impl Buffer {
    /// Messages we still have, oldest first
    pub fn records(&self) -> &VecDeque<Record> {
        &self.records
    }

    /// How many messages we had to throw away because the buffer was full
    pub fn dropped(&self) -> usize {
        self.dropped
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.dropped = 0;
    }

    fn push(&mut self, record: Record) {
        if self.records.len() == CAPACITY {
            self.records.pop_front();
            self.dropped += 1;
        }

        self.records.push_back(record);
    }
}

pub struct Handle(Mutex<Buffer>);

impl Handle {
    /// A poisoned lock only means some thread panicked while logging, and the
    /// buffer is still fine, so we take it anyway
    pub fn lock(&self) -> MutexGuard<'_, Buffer> {
        self.0.lock().unwrap_or_else(|poison| poison.into_inner())
    }
}

static BUFFER: LazyLock<Handle> = LazyLock::new(|| Handle(Mutex::new(Buffer::default())));

/// Messages logged so far. Works before init() too (in tests, for example),
/// where the buffer is just empty
pub fn buffer() -> &'static Handle {
    &BUFFER
}

static FILTER: OnceLock<String> = OnceLock::new();

/// Filter in use, shown at the bottom of the log window
pub fn filter() -> &'static str {
    FILTER.get().map_or(DEFAULT_FILTER, String::as_str)
}

/// Puts every message in the buffer the log window reads from
struct RingLayer;

impl<S> Layer<S> for RingLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let metadata = event.metadata();
        let mut visitor = MessageVisitor::default();
        event.record(&mut visitor);

        buffer().lock().push(Record {
            time: Local::now(),
            level: *metadata.level(),
            target: metadata.target(),
            span: ctx
                .event_scope(event)
                .and_then(|mut scope| scope.next())
                .map(|span| span.name()),
            message: visitor.finish(),
        });
    }
}

/// Turns an event into "message key=value ..."
#[derive(Default)]
struct MessageVisitor {
    message: String,
    fields: String,
}

impl MessageVisitor {
    fn finish(mut self) -> String {
        self.message.push_str(&self.fields);
        self.message
    }
}

impl Visit for MessageVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message.push_str(value);
        } else {
            self.record_debug(field, &value);
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        // fields can come before the message, so we keep them apart and
        // join everything in finish(). writing to a String never fails
        let _ = if field.name() == "message" {
            write!(self.message, "{value:?}")
        } else {
            write!(self.fields, " {}={value:?}", field.name())
        };
    }
}

/// What to do with the messages in the buffer when dz6 exits
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Flush {
    /// stderr is redirected, so they were already written there
    Streamed,
    /// stderr is the terminal and the user asked for logs, so print them all
    All,
    /// stderr is the terminal and nobody asked for anything: only problems
    Problems,
}

#[derive(Clone, Copy)]
pub struct Logging {
    flush: Flush,
}

impl Logging {
    /// Prints the messages the log window couldn't show. Call it only when the
    /// TUI is not on the screen (that is, after ratatui::restore())
    pub fn flush(&self) {
        let lowest = match self.flush {
            Flush::Streamed => return,
            Flush::All => Level::TRACE,
            Flush::Problems => Level::WARN,
        };

        let buffer = buffer().lock();
        let mut records = buffer
            .records()
            .iter()
            .filter(|record| record.level <= lowest)
            .peekable();

        if records.peek().is_none() {
            return;
        }

        let mut stderr = io::stderr().lock();
        for record in records {
            // if stderr is broken there's nowhere to complain about it
            let _ = record.write_to(&mut stderr);
        }
    }
}

/// Sets up tracing. Call it once, before anything logs
pub fn init() -> Logging {
    let requested = env::var(EnvFilter::DEFAULT_ENV).ok();
    let (filter, rejected) = parse_filter(requested.as_deref());

    // show what the user asked for, not the directives EnvFilter ends up with
    let _ = FILTER.set(match requested.as_deref() {
        Some(value) if rejected.is_none() => value.to_owned(),
        _ => DEFAULT_FILTER.to_owned(),
    });

    // writing to stderr would show up over the TUI, so we only do it when
    // stderr goes somewhere else
    let streamed = !io::stderr().is_terminal();
    let stderr_layer = streamed.then(|| {
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_timer(LocalTime)
            .with_writer(io::stderr)
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(RingLayer)
        .with(stderr_layer)
        .init();

    if let Some(rejected) = rejected {
        warn!(
            value = %rejected,
            "invalid {}, using {DEFAULT_FILTER:?}",
            EnvFilter::DEFAULT_ENV
        );
    }

    let flush = if streamed {
        Flush::Streamed
    } else if requested.is_some() {
        Flush::All
    } else {
        Flush::Problems
    };

    Logging { flush }
}

/// Builds the filter from RUST_LOG, or uses DEFAULT_FILTER if it's not set or
/// doesn't parse. A bad value is returned so we can warn about it after the
/// subscriber exists
fn parse_filter(requested: Option<&str>) -> (EnvFilter, Option<String>) {
    match requested {
        None => (EnvFilter::new(DEFAULT_FILTER), None),
        Some(value) => match EnvFilter::builder().parse(value) {
            Ok(filter) => (filter, None),
            Err(_) => (EnvFilter::new(DEFAULT_FILTER), Some(value.to_owned())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn record(message: &str) -> Record {
        Record {
            time: Local::now(),
            level: Level::INFO,
            target: "dz6::tests",
            span: None,
            message: message.to_string(),
        }
    }

    #[test]
    fn test_buffer_drops_oldest_messages() {
        let mut buffer = Buffer::default();

        for i in 0..CAPACITY + 3 {
            buffer.push(record(&i.to_string()));
        }

        assert_eq!(buffer.records().len(), CAPACITY);
        assert_eq!(buffer.dropped(), 3);
        assert_eq!(buffer.records().front().unwrap().message, "3");
        assert_eq!(
            buffer.records().back().unwrap().message,
            (CAPACITY + 2).to_string()
        );
    }

    #[test]
    fn test_invalid_rust_log_falls_back() {
        // EnvFilter reorders directives, so we compare with a parsed default
        let default = EnvFilter::new(DEFAULT_FILTER).to_string();

        let (filter, rejected) = parse_filter(Some("dz6=definitely_not_a_level"));

        assert_eq!(filter.to_string(), default);
        assert_eq!(rejected.as_deref(), Some("dz6=definitely_not_a_level"));

        let (filter, rejected) = parse_filter(Some("dz6::hex=trace"));

        assert_eq!(filter.to_string(), "dz6::hex=trace");
        assert_eq!(rejected, None);
    }

    #[test]
    fn test_event_becomes_a_record() {
        let subscriber = tracing_subscriber::registry().with(RingLayer);

        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::info_span!("scope");
            let _entered = span.enter();
            tracing::info!(offset = 0x40, "jumped");
        });

        let buffer = buffer().lock();
        let record = buffer.records().back().expect("event was recorded");

        assert_eq!(record.message, "jumped offset=64");
        assert_eq!(record.level, Level::INFO);
        assert_eq!(record.span, Some("scope"));
        assert_eq!(record.target, "dz6::logging::tests");
    }
}
