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
	time::Nanoseconds
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

	pub fn set_freq<T>(&mut self, freq: T)
	where
		T: Into<Nanoseconds>
	{
		let mut time: Nanoseconds = freq.into();
		time.0 = time.0 / 2;
		self.counter.start(time);
		self.counter.enable_interrupt();
	}

	pub fn enable(&mut self) {
		self.enable.set_high().unwrap();
		unsafe {
			NVIC::unmask(interrupt::TC4);
		}
	}

	pub fn disable(&mut self) {
		self.enable.set_low().unwrap();
		NVIC::mask(interrupt::TC4);
	}
}

#[interrupt]
fn TC4() {
	//clear intfalg, oterwise interrup is fired again at the next cycle
	unsafe {
		TC::ptr()
			.as_ref()
			.unwrap()
			.count16()
			.intflag
			.modify(|_, w| w.ovf().set_bit());
	}
	unsafe {
		SPAKER_PIN.as_mut().unwrap().toggle();
	}
}
