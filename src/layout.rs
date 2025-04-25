/// bold(text: &str) -> String - Bold an string
///
/// This function takes a string slice as an input and returns a new string that,
/// when printed, will appear in bold.
///
/// # Examples
///
/// ```
/// use ransi::layout::bold;
///
/// let important_text: &str = "This is really important!";
/// println!("{}", bold(important_text));
/// ```
pub fn bold(text: &str) -> String {
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
/// use ransi::layout::blink;
///
/// let text = "This text will blink";
///
/// let blink_text = blink(text);
///
/// println!("{}", blink_text);
/// println!("This text is not blinking.");
/// ```
pub fn blink(text: &str) -> String {
    format!("\033[5m{}\033[25m", text) // NOTE: use 25m to reset the blink
}

/// italic(text: &str) -> String - Makes the text italic
///
/// This function takes a string slice as an input and returns a new string that,
/// when printed, is italic.
///
/// # Examples
///
/// ```
/// use ransi::layout::italic;
///
/// let text1 = "Violets are blue, Roses are red, This is not italic text";
/// let text2 = "Violets are blue, Roses are red, This is italic text";
/// let italic_text = italic(text1);
///
/// println!("{}", text1);
/// println!("And now italic...");
/// println!("{}", text2);
/// ```
pub fn italic(text: &str) -> String {
    format!("\033[3m{}\033[23m", text)
}

