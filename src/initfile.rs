use std::{error::Error, fs};

use directories_next::UserDirs;

use crate::{app::App, commands::parse_command};

impl App {
    pub fn read_initfile(&mut self) -> Result<(), Box<dyn Error>> {
        let home = UserDirs::new();

        if let Some(home) = home {
            let home = home.home_dir().to_owned();
            let path = home.join(".dz6init");
            let data = fs::read_to_string(path)?;

            for cmdline in data.lines() {
                parse_command(self, cmdline);
            }
        }

        Ok(())
    }
}
