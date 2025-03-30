//! For text output

use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            crate::sbi::console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// Print! to the host console using the format string and arguments.
/// 
/// - `#[macro_export]` exports the macro so it can be used in other modules or crates.
/// Without it, the macro would only be available inside the module where it's defined.
/// - `$crate` ensure correct absolute paths even if the macro is used in another crate.
/// It expands to the absolute path of the crate where the macro was defined, avoiding conflicts.
/// - `format_args!` is a built-in marcro that formats the input, 
/// returning a `core::fmt::Arguments` structure.
/// 
/// ## Pattern Mathcing for Arguments:
/// 
/// Notice that Rust macro patterns feel like RegEx (regular expression)!
/// 
/// - `$fmt: literal` -> captures a single argumnet which is a string literal.
/// - `$(, $($arg: tt)+)?`
///     - `$()?` -> an optional group.
///     - `,` -> start with a comma.
///     - `$()+` -> a repetition group with one or more tokens.
///     - `$arg: tt` -> any token type.
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

/// Println! to the host console using the format string and arguments.
/// 
/// Same as `print!` except it adds a new line symbol `\n` at the end of the format.
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}
