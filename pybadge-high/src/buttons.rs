use super::hal;

use cortex_m::asm::delay as cycle_delay;
use gpio::v2::{Floating, Input, Output, PushPull};
use hal::{
	gpio::{self, *},
	prelude::*
};
use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
/// There are 8 buttons on the front: A, B, Select, Start and four arranged in a d-pad.
///
/// ![🖼️](https://cdn-learn.adafruit.com/assets/assets/000/075/106/original/adafruit_products_PyBadge_Top_Buttons.jpg)
pub enum Button {
	B = 1 << 7,
	A = 1 << 6,
	Start = 1 << 5,
	Sesect = 1 << 4,
	Right = 1 << 3,
	Down = 1 << 2,
	Up = 1 << 1,
	Left = 1
}

/// Button status changes.

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
	Pressed(Button),
	Released(Button)
}

pub struct EventIter<'a> {
	postion: u8,
	buttons: &'a Buttons,
	update_postions: u8
}

impl<'a> Iterator for EventIter<'a> {
	type Item = Event;
	fn next(&mut self) -> Option<Self::Item> {
		for i in self.postion..8 {
			let mask = 1 << i;
			//check if state was changed
			if mask & self.update_postions != 0 {
				//mask is always an valid Button value
				let button = Button::try_from(mask).unwrap();
				if self.buttons.button_pressed(button) {
					return Some(Event::Pressed(button));
				} else {
					return Some(Event::Released(button));
				}
			}
		}
		None
	}
}

/// Store the state of the Buttons.
///
/// The [`Button`]s do not connect to GPIO pins directly.
/// Instead they connect to an 8-channel shift register to save pins.
/// This result thate they must be manual updated by calling [`.update()`](Self::update)
/// and can not be acess directly.
pub struct Buttons {
	pub(crate) current_state: u8,
	pub(crate) laste_state: u8,
	pub(crate) latch: Pb0<Output<PushPull>>,
	/// Button Out
	pub(crate) data_in: Pb30<Input<Floating>>,
	/// Button Clock
	pub(crate) clock: Pb31<Output<PushPull>>
}

impl Buttons {
	/// Check if some key is pressed.
	pub fn some_pressed(&self) -> bool {
		self.current_state != 0
	}

	/// Check if none key is pressed
	pub fn none_pressed(&self) -> bool {
		self.current_state == 0
	}

	/// Check if a button is pressed
	pub fn button_pressed(&self, button: Button) -> bool {
		self.current_state & button as u8 != 0
	}
	
	pub fn a_pressed(&self) -> bool {
		self.button_pressed(Button::A)
	}

	pub fn b_pressed(&self) -> bool {
		self.button_pressed(Button::B)
	}

	pub fn start_pressed(&self) -> bool {
		self.button_pressed(Button::Start)
	}

	pub fn select_pressed(&self) -> bool {
		self.button_pressed(Button::Sesect)
	}

	pub fn right_pressed(&self) -> bool {
		self.button_pressed(Button::Right)
	}

	pub fn down_pressed(&self) -> bool {
		self.button_pressed(Button::Down)
	}

	pub fn up_pressed(&self) -> bool {
		self.button_pressed(Button::Up)
	}

	pub fn left_pressed(&self) -> bool {
		self.button_pressed(Button::Left)
	}

	/// Iterator over alle [`Event`]s (Button status changes) occured between the last and penultimate update.
	///
	/// This does only include the changes of Buttons!
	/// For example if a button was pressed at the penultimate update und is still pressed at the last update,
	/// the iterator does skip the button.
	pub fn events(&self) -> EventIter {
		EventIter {
			postion: 0,
			buttons: self,
			update_postions: self.current_state ^ self.laste_state
		}
	}

	//Returns a ButtonIter of button changes as Keys enums
	//pub fn event(&self) {}

	/// Update the state of the buttons.
	/// 400ns total blocking read.
	//
	//based on https://github.com/atsamd-rs/atsamd/blob/master/boards/pygamer/src/buttons.rs
	//120mhz, 1 cycle = 0.000000008333333 = 8.333333ns
	//https://www.onsemi.com/pub/Collateral/MC74HC165A-D.PDF
	//3v <=125c
	//tsu min setup time 55ns = 7 cycles
	//th min hold time 5ns = 1 cycles
	//tw min pulse width 36ns = 5 cycles
	//trec min recovery time 55ns, how long before you should attempt to read
	// again?
	pub fn update(&mut self) {
		// 48*8.333ns total blocking read
		self.latch.set_low().ok();
		cycle_delay(7); //tsu?
		self.latch.set_high().ok();
		cycle_delay(1); //th?
		let mut current: u8 = 0;

		// they only use the top 8 bits
		for _ in 0..8 {
			current <<= 1;

			self.clock.set_low().ok();
			cycle_delay(5); //tw

			if self.data_in.is_high().unwrap() {
				current |= 1;
			}
			self.clock.set_high().ok();
		}

		self.laste_state = self.current_state;
		self.current_state = current;
	}
}
