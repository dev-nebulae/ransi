use ransi::colors::*;
use ransi::layout::*;
use ransi::term::*;

fn main() {
	set_bg(171);
	println!("{}", italic("Hello!"));
	reset();
}
