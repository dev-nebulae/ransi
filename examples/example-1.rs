use ransi::colors::*;
use ransi::layout::*;

fn main() {
	println!("{}", set_fg("red", &blink("This text blinks")));
}

