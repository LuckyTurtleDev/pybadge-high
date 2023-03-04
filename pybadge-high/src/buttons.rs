use super::hal;

use cortex_m::asm::delay as cycle_delay;
use gpio::v2::{Floating, Input, Output, PushPull};
use hal::{
	gpio::{self, *},
	prelude::*
};

const B: u8 = 1 << 7;
const A: u8 = 1 << 6;
const START: u8 = 1 <<5;
const SELECT: u8 = 1 << 4;
const RIGHT: u8 = 1 << 3;
const DOWN: u8 = 1 << 2;
const UP: u8 = 1 << 1;
const LEFT: u8 = 1;


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
	/// Check if any key is pressed
	pub fn any_pressed(&self) -> bool {
		self.current_state == 0
	}

	/// Check if none key is pressed
	pub fn none_pressed(&self) -> bool {
		self.current_state != 0
	}
	
	pub fn a_pressed(&self) -> bool {
		self.current_state & A != 0
	}
	
	pub fn b_pressed(&self) -> bool {
		self.current_state & B != 0
	}

	pub fn start_pressed(&self) -> bool {
		self.current_state & START != 0
	}
	
	pub fn select_pressed(&self) -> bool {
		self.current_state & SELECT != 0
	}
		
	pub fn right_pressed(&self) -> bool {
		self.current_state & RIGHT != 0
	}
	
	pub fn down_pressed(&self) -> bool {
		self.current_state & DOWN != 0
	}
	
	pub fn up_pressed(&self) -> bool {
		self.current_state & UP != 0
	}
	
	pub fn left_pressed(&self) -> bool {
		self.current_state & LEFT != 0
	}
			
	/// Returns a ButtonIter of button changes as Keys enums
	pub fn event(&self) {}

	/// Update self the current button state.
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
