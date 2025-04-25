/// Function that returns an ANSI escape code for a given color name
pub fn get_ansi_from_name(short_name: &str) -> Option<String> {
    match short_name {
        "red" => Some("\033[31m".to_string()),
        "green" => Some("\033[32m".to_string()),
        "yellow" => Some("\033[33m".to_string()),
        "blue" => Some("\033[34m".to_string()),
        "magenta" => Some("\033[35m".to_string()),
        "cyan" => Some("\033[36m".to_string()),
        _ => None,

    }
}

/// Function that returns an ANSI escape code for a given RGB color
pub fn get_ansi_from_rgb(r: u8, g: u8, b: u8) -> String {
    // NOTE: no need to check for out of range values, as this is u8 (max 255)
    format!("\033[38;2;{};{};{}m", r, g, b)
}

/// Trait that supports or the rgb color or the color name
pub trait ColorIn {
    fn to_ansi(&self) -> Option<String>;
}

/// Implement the ColorIn trait for when a string is passed
impl ColorIn for &str {
    fn to_ansi(&self) -> Option<String> {
        get_ansi_from_name(self)
    }
}

/// Implement the ColorIn trait for when a tuple of u8 is passed
impl ColorIn for (u8, u8, u8) {
    fn to_ansi(&self) -> Option<String> {
        let (r, g, b) = *self;
        format!("\033[38;2;{};{};{}m", r, g, b).into()
    }
}

/// set_color(&str color OR (u8, u8, u8) color) - Prints and returns the ANSI 
/// escape code for the color
/// given on color.
///
/// # Examples
/// ```
/// use ransi::colors::{set_color};
///
/// set_color("red");
/// reset();
/// println!("This text is red");
/// ```
pub fn set_color<C: ColorIn>(color: C) -> Option<String> {

    let ret = color.to_ansi();
    
    if None != ret {
        println!("{}", ret.clone().unwrap()); // Change the color
        return ret;
    }

    None
    
}
