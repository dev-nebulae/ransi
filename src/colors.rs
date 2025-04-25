/// Function that returns an ANSI escape code for a given color name
pub fn get_ansi_from_name(short_name: &str) -> Option<String> {
    match short_name {
        "red" => Some("\x1B[31m".to_string()),
        "green" => Some("\x1B[32m".to_string()),
        "yellow" => Some("\x1B[33m".to_string()),
        "blue" => Some("\x1B[34m".to_string()),
        "magenta" => Some("\x1B[35m".to_string()),
        "cyan" => Some("\x1B[36m".to_string()),
        "white" => Some("\x1B[36m".to_string()),
        _ => None,
    }
}

pub fn get_ansi_bg_from_name(short_name: &str) -> Option<String> {
    match short_name {
        "red" => Some("\x1B[41m".to_string()),
        "green" => Some("\x1B[42m".to_string()),
        "yellow" => Some("\x1B[43m".to_string()),
        "blue" => Some("\x1B[44m".to_string()),
        "magenta" => Some("\x1B[45m".to_string()),
        "cyan" => Some("\x1B[46m".to_string()),
        "white" => Some("\x1B[47m".to_string()),
        _ => None,
    }
}

/// Function that returns an ANSI escape code for a given RGB color
pub fn get_ansi_from_rgb(r: u8, g: u8, b: u8) -> String {
    // NOTE: no need to check for out of range values, as this is u8 (max 255)
    format!("\x1B[38;2;{};{};{}m", r, g, b)
}

/// Trait that supports either the RGB color or the color name
pub trait ColorIn {
    fn to_ansi(&self) -> Option<String>;
    fn to_ansi_bg(&self) -> Option<String>;
}

/// Implement the ColorIn trait for when a string is passed
impl ColorIn for &str {
    fn to_ansi(&self) -> Option<String> {
        get_ansi_from_name(self)
    }
    fn to_ansi_bg(&self) -> Option<String> {
        get_ansi_bg_from_name(self)
    }
}

/// Implement the ColorIn trait for when a tuple of u8 is passed
impl ColorIn for (u8, u8, u8) {
    fn to_ansi(&self) -> Option<String> {
        let (r, g, b) = *self;
        Some(format!("\x1B[38;2;{};{};{}m", r, g, b))
    }
	fn to_ansi_bg(&self) -> Option<String> {
		let (r, g, b) = *self;
		Some(format!("\x1B[48;2;{};{};{}", r, g, b))
	}
}

/// Implement the ColorIn trait for when a single u8 is passed (256 colors)
impl ColorIn for u8 {
    fn to_ansi(&self) -> Option<String> {
        Some(format!("\x1B[38;5;{}m", self))
    }
	fn to_ansi_bg(&self) -> Option<String> {
		Some(format!("\x1B[48;5;{}m", self))
	}
}

/// reset() - Manually reset the color
///
/// Reset manually clears all attributes
pub fn reset() {
    println!("\x1B[0m");
}

/// set_fg(&str color OR (u8, u8, u8) color) - Prints and returns the ANSI 
/// escape code for the color
/// given on color.
///
/// # Examples
/// ```
/// use ransi::colors::{set_fg, reset};
///
/// set_fg("red");
/// reset();
/// println!("This text is red");
/// ```
pub fn set_fg<C: ColorIn>(color: C) -> Option<String> {
    let ret = color.to_ansi();
    
    if let Some(ansi_code) = ret {
        println!("{}", ansi_code); // Change the color
        return Some(ansi_code);
    }

    None
}

/// set_bg(&str color OR (u8, u8, u8) color) - Prints and returns the ANSI 
/// escape code for the color
/// given on color.
///
/// # Examples
/// ```
/// use ransi::colors::{set_bg, reset};
///
/// set_bg("red");
/// reset();
/// println!("This text is on red");
/// ```
pub fn set_bg<C: ColorIn>(color: C) -> Option<String> {
    let ret = color.to_ansi_bg();

    if let Some(ansi_code) = ret {
        println!("{}", ansi_code);
        return Some(ansi_code);
    }

    None
}

