/// Write text to `stderr`, so it doesn't interfere with the JSON output that gets written.
#[macro_export]
macro_rules! println_err(
    ($($arg:tt)*) => { {
        use std::io::Write;
        writeln!(&mut ::std::io::stderr(), $($arg)*).expect("failed printing to stderr");
    } }
);
