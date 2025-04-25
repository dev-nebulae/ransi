/// bell(text: &str) -> String - Makes a bell sound when text is printed
///
/// bell returns an string, that when printing, makes a bell sound (if enabled on the
/// terminal)
///
/// # Example
///
/// ```
/// use ransi::term::bell;
/// use std::{thread, time};
///
/// fn main() {
///     println!("Starting long process");
///     println!("We'll notify you when it finishes");
///
///     let ten_secs = time::Duration::from_millis(10000);
///
///     thread::sleep(ten_secs);
///
///     let mesg: &str = "Finished in 10 seconds!";
///
///     println!("{}", bell(mesg));
///
/// }
/// ```
pub fn bell(text: &str) -> String{
    format!("\007{}", text)
}

/// set_title() -> String - Changes the terminal title
///
/// set_title() sets the terminal title to what is set on title.
///
/// This works in all modern terminals (xTerm, GNOME terminal, Windows Terminal, iTerm2, Kitty,
/// Alacritty, and more!)
///
/// # Example
///
/// ```
/// use ransi::term::set_title;
///
/// println!("{}", set_title("My program is running!"));
///
/// ```
pub fn set_title(title: &str) -> String {
    format!("\x1B]0;{}\x07", title)
}

