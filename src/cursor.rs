// move_to(row, col) - Move cursor to specific position
///
/// Moves the cursor to the given (row, column) position (1-indexed).
///
/// # Examples
///
/// ```
/// use ransi::cursor::move_to;
///
/// println!("{}", move_to(5, 10));
/// println!("This text appears at (5, 10)");
/// ```
pub fn move_to(row: u16, col: u16) -> String {
    format!("\x1B[{};{}H", row, col)
}

/// move_up(n) - Move cursor up by n rows
///
/// Moves the cursor up by the specified number of rows.
///
/// # Examples
///
/// ```
/// use ransi::cursor::move_up;
///
/// println!("Line 1");
/// println!("Line 2");
/// print!("{}", move_up(1));
/// println!("Overwritten!");
/// ```
pub fn move_up(n: u16) -> String {
    format!("\x1B[{}A", n)
}

/// move_down(n) - Move cursor down by n rows
///
/// Moves the cursor down by the specified number of rows.
///
/// # Examples
///
/// ```
/// use ransi::cursor::move_down;
///
/// print!("Top line");
/// print!("{}", move_down(2));
/// println!("This appears two lines below.");
/// ```
pub fn move_down(n: u16) -> String {
    format!("\x1B[{}B", n)
}

/// move_right(n) - Move cursor right by n columns
///
/// Moves the cursor right by the specified number of columns.
///
/// # Examples
///
/// ```
/// use ransi::cursor::move_right;
///
/// print!("Start");
/// print!("{}", move_right(10));
/// println!("Indented text");
/// ```
pub fn move_right(n: u16) -> String {
    format!("\x1B[{}C", n)
}

/// move_left(n) - Move cursor left by n columns
///
/// Moves the cursor left by the specified number of columns.
///
/// # Examples
///
/// ```
/// use ransi::cursor::move_left;
///
/// print!("1234567890");
/// print!("{}", move_left(5));
/// println!("X"); // Overwrites the '6'
/// ```
pub fn move_left(n: u16) -> String {
    format!("\x1B[{}D", n)
}

/// home() - Move cursor to top-left corner (1,1)
///
/// Moves the cursor to the very top-left of the screen.
///
/// # Examples
///
/// ```
/// use ransi::cursor::home;
///
/// println!("{}", home());
/// println!("Screen starts fresh here.");
/// ```
pub fn home() -> String {
    format!("\x1B[H")
}

/// save_position() - Save the current cursor position
///
/// Saves the current cursor position so it can be restored later with `restore_position()`.
///
/// # Examples
///
/// ```
/// use ransi::cursor::{save_position, restore_position};
///
/// print!("{}", save_position());
/// println!("Moved away from here...");
/// print!("{}", restore_position());
/// ```
pub fn save_position() -> String {
    format!("\x1B[s")
}

/// restore_position() - Restore the previously saved cursor position
///
/// Restores the cursor to the position saved by `save_position()`.
///
/// # Examples
///
/// ```
/// use ransi::cursor::{save_position, restore_position};
///
/// print!("{}", save_position());
/// println!("Moved away from here...");
/// print!("{}", restore_position());
/// println!("Back to saved spot!");
/// ```
pub fn restore_position() -> String {
    format!("\x1B[u")
}

