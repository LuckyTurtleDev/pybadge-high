use cortex_m::peripheral::NVIC;
use edgebadge::{
	gpio::{
		v2::{PA02, PA27},
		*
	},
	pac,
	pac::TC4 as TC,
	prelude::*,
	thumbv7em::timer::TimerCounter,
	time::{Hertz, Milliseconds}
};
use pac::interrupt;

static mut SPAKER_PIN: Option<Pin<PA02, Output<PushPull>>> = None;

pub struct PwmSound {
	enable: Pin<PA27, Output<PushPull>>,
	counter: TimerCounter<TC>
}

impl PwmSound {
	pub(crate) fn init(
		enable_pin: Pin<PA27, Output<PushPull>>,
		speaker_pin: Pin<PA02, Output<PushPull>>,
		counter: TimerCounter<TC>
	) -> Self {
		let mut enable_pin = enable_pin;
		enable_pin.set_low().unwrap();
		unsafe { SPAKER_PIN = Some(speaker_pin) };
		PwmSound {
			enable: enable_pin,
			counter
		}
	}

	pub fn set_freq(&mut self) {
		self.counter.start(Milliseconds(6));
	}

	pub fn enable(&mut self) {
		self.enable.set_high().unwrap();
		self.counter.enable_interrupt();
		unsafe {
			NVIC::unmask(interrupt::TC4);
		}
	}

	pub fn disable(&mut self) {
		self.enable.set_low().unwrap();
		self.counter.disable_interrupt();
		NVIC::mask(interrupt::TC4);
	}
}

#[interrupt]
fn TC4() {
	unsafe {
		SPAKER_PIN.as_mut().unwrap().toggle();
	}
}
