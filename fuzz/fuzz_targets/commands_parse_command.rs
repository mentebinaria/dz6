#![no_main]
use libfuzzer_sys::fuzz_target;

use dz6_fuzz::commands::parse_command;
use dz6_fuzz::app::App;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let mut app = App::new();
        let _ = parse_command(&mut app, s);
    }
});
