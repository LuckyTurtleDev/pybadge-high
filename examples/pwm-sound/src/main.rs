#![no_std]
#![no_main]
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_graphics::{
	mono_font::{ascii::FONT_6X10, MonoTextStyle},
	prelude::*,
	text::Text
};
use heapless::String;
use pybadge::{prelude::*, Display, PyBadge};
use pybadge_high as pybadge;
use pybadge_high::{time::Hertz, Color};

fn draw(display: &mut Display, freq: Hertz) {
	let mut string = String::<32>::new();
	let style = MonoTextStyle::new(&FONT_6X10, Color::WHITE);
	string.clear();
	write!(string, "{} Herz", freq.0).unwrap();
	display.clear(Color::BLACK).unwrap();
	Text::new(&string, Point::new(20, 30), style)
		.draw(display)
		.ok();
}

#[entry]
fn main() -> ! {
	let pybadge = PyBadge::take().unwrap();
	let mut speaker = pybadge.speaker;
	let mut display = pybadge.display;
	let mut buttons = pybadge.buttons;
	let mut delay = pybadge.delay;
	let mut freq = Hertz(300);
	speaker.set_freq(freq);
	speaker.enable();
	draw(&mut display, freq);

	loop {
		buttons.update();
		if buttons.some_pressed() {
			if buttons.up_pressed() {
				freq.0 += 50;
			}
			if buttons.down_pressed() {
				freq.0 -= 50;
			}
			speaker.set_freq(freq);
			draw(&mut display, freq);
			delay.delay_ms(200_u8);
		}
	}
}
