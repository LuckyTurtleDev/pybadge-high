#![no_std]
#![no_main]

use pybadge::PyBadge;
use pybadge_high as pybadge;
use pybadge_high::prelude::*;

#[entry]
fn main() -> ! {
	let mut pybadge = PyBadge::take().unwrap();
	loop {
		pybadge.red_led.on().unwrap();
		pybadge.delay.delay_ms(1000_u16);
		pybadge.red_led.off().unwrap();
		pybadge.delay.delay_ms(1000_u16);
	}
}
