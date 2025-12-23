/// Reader is the class that keep tracks on current page size
/// and its location in the memory mapped file.

#[derive(Default, Debug)]
pub struct Reader {
    pub page_current_size: usize,
    pub page_start: usize,
    pub page_end: usize,
}

impl Reader {
    pub fn new() -> Self {
        Reader {
            // We just initialize this to a non zero value to avoid a division by zero
            // on application startup
            page_current_size: 1,
            page_end: 1,
            ..Default::default()
        }
    }
}
