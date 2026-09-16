use std::{fs, io};

use directories_next::UserDirs;
use tracing::{debug, info, instrument, warn};

use crate::{app::App, commands::parse_command};

impl App {
    /// Runs the commands in `~/.dz6init`. It's fine if the file is not there
    #[instrument(name = "read", level = "debug", skip(self))]
    pub fn read_initfile(&mut self) {
        let Some(dirs) = UserDirs::new() else {
            warn!("no home directory, skipping the init file");
            return;
        };

        let path = dirs.home_dir().join(".dz6init");

        let data = match fs::read_to_string(&path) {
            Ok(data) => data,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                debug!(path = %path.display(), "no init file");
                return;
            }
            Err(error) => {
                warn!(path = %path.display(), %error, "cannot read the init file");
                return;
            }
        };

        let mut commands = 0;

        for cmdline in data
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
        {
            parse_command(self, cmdline);
            commands += 1;
        }

        info!(path = %path.display(), commands, "init file loaded");
    }
}
