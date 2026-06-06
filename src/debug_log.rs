use std::{
    fs::OpenOptions,
    sync::{LazyLock, Mutex},
};

pub static LOG_FILE: LazyLock<Mutex<std::fs::File>> = LazyLock::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/bitpill-debug.log")
        .expect("failed to open debug log");
    Mutex::new(file)
});

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let now = chrono::Local::now().format("%H:%M:%S%.3f");
        let msg = format!("[{}] {}\n", now, format!($($arg)*));
        if let Ok(mut f) = $crate::debug_log::LOG_FILE.lock() {
            let _ = f.write_all(msg.as_bytes());
            let _ = f.flush();
        }
    }};
}
