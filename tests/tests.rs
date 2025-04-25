
#[cfg(test)]
mod tests {

    use ransi::colors::*;
    #[test]
    fn get_ansi_from_rgb_test() {
        let r: u8 = 100;
        let g: u8 = 150;
        let b: u8 = 200;

        assert_eq!(get_ansi_from_rgb(r, g, b), "\033[38;2;100;150;200m");
    }

    fn get_ansi_from_name_test() {
        let color_name = "red";

        assert_eq!(get_ansi_from_name(color_name), Some("\033[31m".to_string()));
    }
}
