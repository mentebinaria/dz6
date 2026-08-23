use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::app::App;
use crate::hex::{blocks::ColoredBlock, comment::Comment, hex_view::HexView};

impl App {
    pub fn save_database(&self) -> Result<(), Box<dyn Error>> {
        let target_dir: &Path = Path::new(&self.file_info.path)
            .parent()
            .unwrap_or(Path::new("."));
        let cwd_db = format!("{}.dz6", self.file_info.name);
        let target_db: PathBuf = target_dir.join(&cwd_db);

        // if there's nothing to be saved, delete any existing db files and return
        if self.hex_view.bookmarks.is_empty()
            && self.hex_view.comment_name_list.is_empty()
            && self.hex_view.blocks.is_empty()
        {
            let _ = fs::remove_file(target_db);
            let _ = fs::remove_file(cwd_db);
            return Ok(());
        }

        // serialize after checking for empty
        let db = hex_view_to_db(&self.hex_view);
        let toml_string = toml::to_string_pretty(&db)?;

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

        let db = toml::from_str::<Database>(&data)?;
        self.hex_view = hex_view_from_db(db);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Database {
    // blocks are ByteBlock structs -- ranges with different colors
    pub blocks: Vec<DbColoredBlock>,
    pub bookmarks: Vec<usize>,

    /// offset -> comment map
    /// Used to populate the comment_name_list as well
    pub comments: BTreeMap<usize, String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DbColoredBlock {
    pub start: usize,
    pub end: usize,
    pub bg_color: u32,
    pub fg_color: u32,
}

fn hex_view_from_db(db: Database) -> HexView {
    HexView {
        blocks: db.blocks.into_iter().map(colored_block_from_db).collect(),
        bookmarks: db.bookmarks,
        comment_name_list: db
            .comments
            .iter()
            .map(|(&k, v)| Comment::new(k, v))
            .collect(),
        comments: db.comments.into_iter().collect(),
        editing_hex: true, // otherwise it defaults to false if a .dz6 file exists for the target
        ..Default::default()
    }
}
fn colored_block_from_db(db_block: DbColoredBlock) -> ColoredBlock {
    ColoredBlock {
        start: db_block.start,
        end: db_block.end,
        bg_color: db_block.bg_color,
        fg_color: db_block.fg_color,
    }
}

/// HexView to Database conversion,
/// takes a reference because we don't want to consume the live HexView that is
/// being used in the app.
fn hex_view_to_db(hex_view: &HexView) -> Database {
    Database {
        blocks: hex_view.blocks.iter().map(colored_block_to_db).collect(),
        bookmarks: hex_view.bookmarks.clone(),
        comments: hex_view
            .comments
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect(),
    }
}
fn colored_block_to_db(block: &ColoredBlock) -> DbColoredBlock {
    DbColoredBlock {
        start: block.start,
        end: block.end,
        bg_color: block.bg_color,
        fg_color: block.fg_color,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_database_round_trip() {
        // db file generated with the hexview serialization
        let db_file = "test_data/db.toml";
        let db_str = fs::read_to_string(db_file).expect("Failed to read test database file");

        let db: Database = toml::from_str(&db_str).expect("Failed to deserialize database");

        // tests that we don't drop any data from the original file and don't add any
        let serialized = toml::to_string_pretty(&db).expect("Failed to serialize database");
        assert_eq!(db_str, serialized);

        // tests that we can deserialize the serialized string and get the same data back
        let re_deserialized: Database =
            toml::from_str(&serialized).expect("Failed to deserialize database");
        assert_eq!(db, re_deserialized);
    }

    #[test]
    fn test_old_database_still_parses() {
        // db file generated with the hexview serialization
        let db_file = "test_data/old_db.toml";
        let db_str = fs::read_to_string(db_file).expect("Failed to read test database file");

        let db: Database = toml::from_str(&db_str).expect("Failed to deserialize database");
        let serialized = toml::to_string_pretty(&db).expect("Failed to serialize database");

        // tests that we can deserialize the serialized string and get the same data back
        let re_deserialized: Database =
            toml::from_str(&serialized).expect("Failed to deserialize database");
        assert_eq!(db, re_deserialized);
    }

    #[test]
    fn test_comment_name_list_gets_populated() {
        let list = vec![
            Comment::new(21, " one comment"),
            Comment::new(101, "comment"),
        ];

        let db_file = "test_data/db.toml";
        let db_str = fs::read_to_string(db_file).expect("Failed to read test database file");
        let db: Database = toml::from_str(&db_str).expect("Failed to deserialize databse");
        let hex_view = hex_view_from_db(db);

        assert_eq!(hex_view.comment_name_list, list);
    }
}
