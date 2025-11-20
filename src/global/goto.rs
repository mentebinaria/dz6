use crate::config::APP_PAGE_SIZE;
use crate::{app::App, config::*};

impl App {
    /// The goto() function checks if the received offset is cached, otherwise
    /// it calls .read_chunk_from_file to fill the right cache block.
    pub fn goto(&mut self, offset: usize) {
        if offset >= self.file_info.size {
            return;
        }

        // If offset is not cached, read and cache the
        // block containing the offset
        if offset > self.reader.cache_end {
            let nblock = offset / APP_CACHE_SIZE;
            self.read_chunk_from_file(nblock).unwrap();
            self.reader.cache_start += nblock * APP_CACHE_SIZE;
            self.reader.cache_end += nblock * APP_CACHE_SIZE;
        } else if offset < self.reader.cache_start {
            self.read_chunk_from_file(offset / APP_CACHE_SIZE).unwrap();
            self.reader.cache_start -= APP_CACHE_SIZE;
            self.reader.cache_end -= APP_CACHE_SIZE;
        }

        // If offset is zero, go to it (it should be cached anyway)
        if offset == 0 {
            self.reader.cache_start = 0;
            self.reader.cache_end = APP_CACHE_SIZE - 1;
            self.reader.page_start = 0;
            self.reader.page_end = APP_PAGE_SIZE - 1;
        } else {
            // Offset is not zero, but is cached. Just go there.
            self.reader.page_start = APP_PAGE_SIZE * (offset / APP_PAGE_SIZE);
            self.reader.page_end = APP_PAGE_SIZE - 1;
        }

        // Update the cursor
        self.hex_view.cursor.y =
            (offset - self.reader.page_start) / self.config.hex_mode_bytes_per_line;
        self.hex_view.cursor.x =
            (offset - self.reader.page_start) % self.config.hex_mode_bytes_per_line;

        // Save current offset (user can press backspace to restore it)
        self.hex_view.last_visited_offset = self.hex_view.offset;
        // Update offset
        self.hex_view.offset = offset;

        // Update offset location in cache. (offset % APP_CACHE_SIZE) / APP_PAGE_SIZE)
        // give the page number within the cache block, then I multiply it by
        // the page size to know how much I have to advance in cache to render
        self.reader.offset_location_in_cache =
            ((offset % APP_CACHE_SIZE) / APP_PAGE_SIZE) * APP_PAGE_SIZE;

        self.reader.page_current = offset / APP_PAGE_SIZE;

        let page_is_aligned = self.file_info.size.is_multiple_of(APP_PAGE_SIZE);

        self.reader.page_current_size =
            if self.reader.page_current == self.reader.page_last && !page_is_aligned {
                self.file_info.size % APP_PAGE_SIZE
            } else {
                APP_PAGE_SIZE
            };
        App::log(self, format!("goto: {:x}", offset));
    }
}
