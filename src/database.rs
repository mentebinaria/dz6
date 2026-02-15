use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::app::App;

impl App {
    pub fn save_database(&self) -> Result<(), Box<dyn Error>> {
        let toml_string = toml::to_string_pretty(&self.hex_view)?;
        let target_dir: &Path = Path::new(&self.file_info.path)
            .parent()
            .unwrap_or(Path::new("."));
        let cwd_db = format!("{}.dz6", self.file_info.name);
        let target_db: PathBuf = target_dir.join(&cwd_db);

        // if there's nothing to be saved, delete any existing db files and return
        if self.hex_view.bookmarks.is_empty() && self.hex_view.comment_name_list.is_empty() {
            let _ = fs::remove_file(target_db);
            let _ = fs::remove_file(cwd_db);
            return Ok(());
        }

        // try target's path or else current directory
        fs::write(&target_db, &toml_string).or_else(|_| fs::write(&cwd_db, &toml_string))?;

        Ok(())
    }
    pub fn load_database(&mut self) -> Result<(), Box<dyn Error>> {
        let target_dir: &Path = Path::new(&self.file_info.path)
            .parent()
            .unwrap_or(Path::new("."));
        let cwd_db = format!("{}.dz6", self.file_info.name);
        let target_db: PathBuf = target_dir.join(&cwd_db);
        let data = fs::read_to_string(&cwd_db).or_else(|_| fs::read_to_string(&target_db))?;

        // TODO: Although we're only interested in comments and bookmarks, other fields might
        // be loaded if they are defined in the TOML file. How to prevent that?
        self.hex_view = toml::from_str(&data)?;
        self.hex_view.editing_hex = true; // otherwise it defaults to false if a .dz6 file exists for the target
        Ok(())
    }
}
