#![no_std]
#![no_main]

use cortex_m_rt::entry;
use pybadge::PyBadge;
use pybadge_high as pybadge;
use pybadge_high::prelude::*;

#[entry]
fn main() -> ! {
	let pybadge = PyBadge::take().unwrap();
	let mut speaker = pybadge.speaker;
	speaker.set_freq();
	speaker.enable();

	loop {}
}
