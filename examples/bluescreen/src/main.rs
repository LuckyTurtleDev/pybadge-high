#![no_std]
#![no_main]

use pybadge_high::prelude::*;

#[entry]
fn main() -> ! {
	//when the bluescreen feature is enabled
	//a bluescreen is show
	panic!("something went wrong");
}
