/// clear() - Clear the entire screen
///
/// Clears the entire screen and moves the cursor to the top-left corner.
///
/// # Examples
///
/// ```
/// use ransi::screen::clear;
/// use std::{time, thread};
///
/// let one_sec = time::Duration::from_millis(1000);
///
/// // Print some data
/// for _ in 0..10 {
///     println!("Hello, world!");
/// }
///
/// thread::sleep(one_sec);
///
/// println!("{}", clear());
/// ```
pub fn clear() -> String {
    "\x1B[2J\x1B[H".to_string()
}

/// clear_from_cursor() - Clear screen from cursor to end
///
/// Clears from the cursor to the end of the screen.
///
/// # Examples
///
/// ```
/// use ransi::screen::clear_from_cursor;
///
/// println!("{}", clear_from_cursor());
/// ```
pub fn clear_from_cursor() -> String {
    "\x1B[0J".to_string()
}

/// clear_to_cursor() - Clear screen from start to cursor
///
/// Clears from the beginning of the screen to the cursor.
///
/// # Examples
///
/// ```
/// use ransi::screen::clear_to_cursor;
///
/// println!("{}", clear_to_cursor());
/// ```
pub fn clear_to_cursor() -> String {
    "\x1B[1J".to_string()
}

/// clear_all() - Clear entire screen and scrollback buffer
///
/// Clears the entire screen and the terminal scrollback buffer.
///
/// # Examples
///
/// ```
/// use ransi::screen::clear_all;
///
/// println!("{}", clear_all());
/// ```
pub fn clear_all() -> String {
    "\x1B[3J".to_string()
}

/// clear_line() - Clear the entire current line
///
/// Clears the entire current line where the cursor is.
///
/// # Examples
///
/// ```
/// use ransi::screen::clear_line;
///
/// println!("{}", clear_line());
/// ```
pub fn clear_line() -> String {
    "\x1B[2K".to_string()
}

/// clear_line_from_cursor() - Clear from cursor to end of line
///
/// Clears the current line from the cursor to the end.
///
/// # Examples
///
/// ```
/// use ransi::screen::clear_line_from_cursor;
///
/// println!("{}", clear_line_from_cursor());
/// ```
pub fn clear_line_from_cursor() -> String {
    "\x1B[0K".to_string()
}

/// clear_line_to_cursor() - Clear from beginning of line to cursor
///
/// Clears the current line from the start to the cursor.
///
/// # Examples
///
/// ```
/// use ransi::screen::clear_line_to_cursor;
///
/// println!("{}", clear_line_to_cursor());
/// ```
pub fn clear_line_to_cursor() -> String {
    "\x1B[1K".to_string()
}

