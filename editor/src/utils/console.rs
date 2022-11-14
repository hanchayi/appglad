#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

#[macro_export]
macro_rules! console_warn {
    ($($t:tt)*) => (web_sys::console::warn_1(&format_args!($($t)*).to_string().into()))
}

#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => (web_sys::console::error_1(&format_args!($($t)*).to_string().into()))
}
