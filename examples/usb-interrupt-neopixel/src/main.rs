#![no_std]
#![no_main]

use pybadge::{prelude::*, usb::Usb, NeoPixelColor, PyBadge};
use pybadge_high as pybadge;

const NUM_LEDS: u8 = 5;

static mut USB: Option<Usb> = None;
static mut COLOR: NeoPixelColor = NeoPixelColor { r: 1, g: 1, b: 1 };

#[entry]
fn main() -> ! {
	let pybadge = PyBadge::take().unwrap();
	let mut neopixel = pybadge.neopixel;
	let mut delay = pybadge.delay;

	//set usb + interrupt
	let mut usb = pybadge.usb_builder.product("LED-Controller").build();
	usb.set_interrupt(interrupt); //set the interupt() as interrupt handler
	unsafe {
		USB = Some(usb);
		// set USB firt to Some first,
		// otherwise a interrup called in the next cycle will not be able to unwrap USB.
		USB.as_mut().unwrap().enable_interrupt();
	};

	delay.delay_ms(100u8); //neopixel needs some delay at the start
	let mut i = 0;
	loop {
		neopixel
			.write((0..NUM_LEDS).map(|ii| {
				if i == ii {
					unsafe { COLOR }
				} else {
					NeoPixelColor::default()
				}
			}))
			.ok();
		delay.delay_ms(250u8);
		delay.delay_ms(250u8);
		if i >= NUM_LEDS {
			i = 0
		} else {
			i += 1;
		}
	}
}

fn interrupt() {
	let usb = unsafe { USB.as_mut().unwrap() };
	let mut buf = [0u8; 64];
	let count = usb.read(&mut buf).ok();
	return;
	if let Some(count) = count {
		let char = buf[count - 1] as char; //get the lasted sended element
		let color = match char {
			'r' | 'R' => NeoPixelColor { r: 2, g: 0, b: 0 },
			'g' | 'G' => NeoPixelColor { r: 0, g: 2, b: 0 },
			'b' | 'B' => NeoPixelColor { r: 0, g: 0, b: 2 },
			_ => NeoPixelColor::default()
		};
		unsafe { COLOR = color };
	}
}
