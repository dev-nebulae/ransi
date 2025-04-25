/// bold(text: &str) -> String - Bold an string
///
/// This function takes a string slice as an input and returns a new string that,
/// when printed, will appear in bold.
fn bold(text: &str) -> String {
    format!("\033[22m")
}

/// blink(text: &str) -> String - Makes text blink
///
/// This function takes a string slice as an input and returns a new string that,
/// when printed, will blink.
///
/// # Examples
///
/// ```
/// // Cool Hello World
/// use ransi::layout::blink;
/// let text = "This text will blink";
///
/// let blink_text = blink(text);
///
/// println!("{}", blink_text);
/// println!("This text is not blinking.");
/// ```
fn blink(text: &str) -> String {
    format!("\033[5m{}\033[25m", text) // NOTE: use 25m to reset the blink
}
