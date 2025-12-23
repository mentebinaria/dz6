use crate::app::App;

impl App {
    /// The goto() function handles moving page position and smooth transition between pages
    pub fn goto(&mut self, offset: usize) {
        if offset >= self.file_info.size {
            return;
        }

        if offset == 0 {
            // If offset is zero, go to it
            self.reader.page_start = 0;
        } else if offset >= self.reader.page_start && offset <= self.reader.page_end {
            // No need to change page position
        } else if offset > self.hex_view.offset {
            // Scrolling down
            if offset > self.reader.page_end
                && self.reader.page_end + self.config.hex_mode_bytes_per_line > offset
            {
                self.reader.page_start += self.config.hex_mode_bytes_per_line;
            } else if offset - self.hex_view.offset == self.reader.page_current_size {
                self.reader.page_start += self.reader.page_current_size;
            } else {
                self.reader.page_start =
                    offset / self.reader.page_current_size * self.reader.page_current_size;
            }
        } else {
            if offset < self.reader.page_start
                && offset + self.config.hex_mode_bytes_per_line >= self.reader.page_start
            {
                self.reader.page_start = offset / self.config.hex_mode_bytes_per_line
                    * self.config.hex_mode_bytes_per_line;
            } else if self.hex_view.offset - offset == self.reader.page_current_size {
                if self.reader.page_start > self.reader.page_current_size {
                    self.reader.page_start -= self.reader.page_current_size;
                } else {
                    self.reader.page_start = 0;
                }
            } else {
                self.reader.page_start =
                    offset / self.reader.page_current_size * self.reader.page_current_size;
            }
        }

        self.reader.page_end = self.reader.page_start + self.reader.page_current_size - 1;

        if self.reader.page_end > self.file_info.size {
            self.reader.page_start = (self.file_info.size - self.reader.page_current_size
                + self.config.hex_mode_bytes_per_line)
                / self.config.hex_mode_bytes_per_line
                * self.config.hex_mode_bytes_per_line;
            self.reader.page_end = self.reader.page_start + self.reader.page_current_size - 1;
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

        App::log(self, format!("goto: {:x}", offset));
    }
}
