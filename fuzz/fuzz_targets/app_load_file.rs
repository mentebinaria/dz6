#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Write;
use dz6_fuzz::app::App;

fuzz_target!(|data: &[u8]| {
    let Some((&flags, body)) = data.split_first() else {
        return;
    };

    // app.load_file opens and eventually mmap's the input filepath
    let path = std::env::temp_dir().join(format!("dz6-fuzz-{}", rand::random::<u32>()));
    let Ok(mut f) = std::fs::File::create(&path) else {
        return
    };

    if f.write_all(body).is_err() {
        return;
    }

    let Some(path) = path.to_str() else {
        return;
    };

    let mut app = App::new();
    let _ = app.load_file(path, 0, flags & 1 == 0);
});
