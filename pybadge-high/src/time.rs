//! Time Units and current time

#[cfg(feature = "time")]
use cortex_m::peripheral::NVIC;
pub use edgebadge::time::*;
#[cfg(feature = "time")]
use edgebadge::{pac, pac::TC5 as TC, prelude::*, thumbv7em::timer::TimerCounter};
#[cfg(feature = "time")]
use pac::interrupt;

//time since start of microntroller
#[cfg(feature = "time")]
static mut COUNT: Milliseconds = Milliseconds(0);

///return time since [`PyBadge::take()`](crate::PyBadge::take) was called.
///Can be used for time measurements.
///
///Does overflow after 50 days uptime at release mode and panic after 50 days at debug mode!
#[cfg(feature = "time")]
pub fn uptime() -> Milliseconds {
	unsafe { COUNT }
}

#[cfg(feature = "time")]
pub(crate) fn init_counter(mut counter: TimerCounter<TC>) {
	let freq = Hertz(1000);
	counter.start(freq);
	counter.enable_interrupt();
	unsafe {
		NVIC::unmask(interrupt::TC5);
	}
}

#[cfg(feature = "time")]
#[interrupt]
fn TC5() {
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
		COUNT.0 += 1;
	}
}
