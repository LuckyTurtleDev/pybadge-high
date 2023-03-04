#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::{
	prelude::*,
	primitives::{Circle, PrimitiveStyleBuilder}
};
use pybadge::{Color, Display, PyBadge};
use pybadge_high as pybadge;

#[entry]
fn main() -> ! {
	let mut pybadge = PyBadge::take().unwrap();
	pybadge.display.clear(Color::BLACK).unwrap();

	loop {
		pybadge.buttons.update();
		draw_button(
			30,
			15,
			pybadge.buttons.select_pressed(),
			&mut pybadge.display
		); //selet
		draw_button(
			130,
			15,
			pybadge.buttons.start_pressed(),
			&mut pybadge.display
		); //start
		draw_button(15, 80, pybadge.buttons.left_pressed(), &mut pybadge.display); //left
		draw_button(
			39,
			80,
			pybadge.buttons.right_pressed(),
			&mut pybadge.display
		); //right
		draw_button(27, 68, pybadge.buttons.up_pressed(), &mut pybadge.display); //up
		draw_button(27, 92, pybadge.buttons.down_pressed(), &mut pybadge.display); //down
		draw_button(145, 64, pybadge.buttons.a_pressed(), &mut pybadge.display); //A
		draw_button(130, 74, pybadge.buttons.b_pressed(), &mut pybadge.display); //B
	}
}

fn draw_button(x: i32, y: i32, pressed: bool, display: &mut Display) {
	let color = if pressed { Color::GREEN } else { Color::RED };
	Circle::with_center(Point::new(x, y), 15)
		.into_styled(PrimitiveStyleBuilder::new().fill_color(color).build())
		.draw(display)
		.unwrap();
}
