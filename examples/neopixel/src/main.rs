#![no_std]
#![no_main]

use pybadge::{prelude::*, NeoPixelColor, PyBadge};
use pybadge_high as pybadge;

const NUM_LEDS: u8 = 5;

#[entry]
fn main() -> ! {
	let mut pybadge = PyBadge::take().unwrap();
	pybadge
		.neopixel
		.write((0..NUM_LEDS).map(|_i| NeoPixelColor {
			r: 255,
			g: 0,
			b: 255
		}))
		.ok();
	loop {}
}
