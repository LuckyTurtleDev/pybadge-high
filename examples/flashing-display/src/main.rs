#![no_std]
#![no_main]

use embedded_graphics::{
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle}
};
use pybadge::{prelude::*, Color, PyBadge};
use pybadge_high as pybadge;

#[entry]
fn main() -> ! {
	let mut pybadge = PyBadge::take().unwrap();
	// fill the full display with black, red and green after each other, using diffrent functions.
	loop {
		Rectangle::with_corners(Point::new(0, 0), Point::new(160, 128))
			.into_styled(
				PrimitiveStyleBuilder::new()
					.fill_color(Color::BLACK)
					.build()
			)
			.draw(&mut pybadge.display)
			.unwrap();
		Rectangle::with_corners(Point::new(0, 0), Point::new(160, 128))
			.into_styled(PrimitiveStyleBuilder::new().fill_color(Color::RED).build())
			.draw(&mut pybadge.display)
			.unwrap();
		pybadge.display.clear(Color::GREEN).unwrap();
	}
}
