#![no_std]
#![allow(clippy::tabs_in_doc_comments)]
#![warn(unreachable_pub)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]
//#![allow(deprecated)]

//! Goal of this crate is to provide **high level hardware abstraction** layer for the pybade and the edgebadge.
//! It should allow people with no/less knowledge of rust and embedded hardware, to program the boards mention before.
//! If you try to do anything hardware-near or usinig additonal expensions,
//! you should probably use the more hardware-near the [edgebadge](https://crates.io/crates/edgebadge) or [atsamd_hal](https://docs.rs/atsamd-hal/latest/atsamd_hal/) crate instead.
//!
//! # Setup
//! #### Installation
//! * Install rustup.
//! I recommand to use the [package manger](https://repology.org/project/rustup/versions) of your operation system.
//! Alternative you can install it from https://www.rust-lang.org/tools/install
//! * install the rust thumbv7em-none-eabihf target. (the architecture of the micronctroller)
//! ```bash
//! rustup target install thumbv7em-none-eabihf
//! ```
//! * install the [hf2-cli](https://crates.io/crates/hf2-cli) flasher
//!
//! #### Create your Project
//! * Create a new rust project.
//! ```bash
//! cargo new my-app
//! ```
//! * Add a `.carge/config.toml` with the following content, to define target architecture and flasher
//! ```toml
//! TODO
//! ```
//! * Add this crate as dependency
//! ```bash
//! cargo add pybadge-high
//! ```
//!
//! #### Flashing:
//! To flash you program, put your device in bootloader mode by hitting the reset button twice.
//! After this excute
//! ```bash
//! cargo run --release
//! ```

#[cfg(feature = "neopixel")]
use edgebadge::gpio::v2::PA15;
use edgebadge::{
	gpio,
	gpio::{v2::PA23, *},
	hal, pac,
	prelude::*,
	Pins
};
#[cfg(feature = "neopixel")]
use embedded_hal::digital::v1_compat::OldOutputPin;
#[cfg(feature = "neopixel")]
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, pwm::Pwm2, sercom::SPIMaster4};
use pac::{CorePeripherals, Peripherals};
#[cfg(feature = "neopixel")]
use smart_leds_trait::SmartLedsWrite;
use st7735_lcd::ST7735;
#[cfg(feature = "neopixel")]
use ws2812::Ws2812;
#[cfg(feature = "neopixel")]
use ws2812_timer_delay as ws2812;

mod buttons;
pub use buttons::Buttons;

pub mod prelude {
	pub use cortex_m_rt::entry;
	pub use edgebadge::prelude::_embedded_hal_blocking_delay_DelayMs;
	#[cfg(feature = "neopixel")]
	pub use smart_leds_trait::SmartLedsWrite;
}

#[cfg(feature = "usb")]
mod usb;
#[cfg(feature = "usb")]
pub use usb::Usb;
#[cfg(feature = "usb")]
pub use usb::UsbError;

///Display Color type
pub type Color = embedded_graphics::pixelcolor::Rgb565;
pub type Backlight = Pwm2<gpio::v2::PA01>;
pub type Display = ST7735<
	SPIMaster4<
		hal::sercom::Sercom4Pad2<Pb14<PfC>>,
		hal::sercom::Sercom4Pad3<Pb15<PfC>>,
		hal::sercom::Sercom4Pad1<Pb13<PfC>>
	>,
	Pb5<Output<PushPull>>,
	Pa0<Output<PushPull>>
>;
pub type Delay = edgebadge::delay::Delay;
#[cfg(feature = "neopixel")]
///The RGB NeoPixel leds below the display.
pub type NeoPixel = Ws2812<
	SpinTimer,
	OldOutputPin<edgebadge::gpio::Pin<PA15, gpio::v2::Output<gpio::v2::PushPull>>>
>;
#[cfg(feature = "neopixel")]
///Color type of the NeoPixel leds.
pub type NeoPixelColor = <NeoPixel as SmartLedsWrite>::Color;

///The red led at the back of the board.
pub struct Led {
	pin: Pin<PA23, Output<PushPull>>
}

impl Led {
	pub fn off(&mut self) -> Result<(), ()> {
		self.pin.set_low()
	}

	pub fn on(&mut self) -> Result<(), ()> {
		self.pin.set_high()
	}
}

///Allow acces to the peripherals, like display, buttons, flash etc.
pub struct PyBadge {
	pub backlight: Backlight,
	pub display: Display,
	pub buttons: Buttons,
	pub red_led: Led,
	pub delay: Delay,
	#[cfg(feature = "neopixel")]
	pub neopixel: NeoPixel,
	#[cfg(feature = "usb")]
	pub usb: Usb
}

impl PyBadge {
	pub fn take() -> Result<PyBadge, ()> {
		let mut peripherals = Peripherals::take().ok_or(())?;
		let core = CorePeripherals::take().ok_or(())?;
		let mut clocks = GenericClockController::with_internal_32kosc(
			peripherals.GCLK,
			&mut peripherals.MCLK,
			&mut peripherals.OSC32KCTRL,
			&mut peripherals.OSCCTRL,
			&mut peripherals.NVMCTRL
		);
		let mut pins = Pins::new(peripherals.PORT).split();
		let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

		//display
		let (display, backlight) = pins.display.init(
			&mut clocks,
			peripherals.SERCOM4,
			&mut peripherals.MCLK,
			peripherals.TC2,
			&mut delay,
			&mut pins.port
		)?;

		//buttons
		let buttons = {
			let latch = pins.buttons.latch.into_push_pull_output(&mut pins.port);
			let data_in = pins.buttons.data_in.into_floating_input(&mut pins.port);
			let clock = pins.buttons.clock.into_push_pull_output(&mut pins.port);
			Buttons {
				current_state: 0,
				laste_state: 0,
				latch,
				data_in,
				clock
			}
		};

		//red led
		let red_led = {
			let mut led = Led {
				pin: pins.led_pin.into_push_pull_output(&mut pins.port)
			};
			led.off()?;
			led
		};

		//neopixel
		#[cfg(feature = "neopixel")]
		let neopixel = {
			let timer = SpinTimer::new(4);
			pins.neopixel.init(timer, &mut pins.port)
		};

		//usb
		#[cfg(feature = "usb")]
		let usb = Usb::init(
			pins.usb
				.init(peripherals.USB, &mut clocks, &mut peripherals.MCLK)
		);

		Ok(PyBadge {
			backlight,
			display,
			buttons,
			red_led,
			#[cfg(feature = "neopixel")]
			neopixel,
			#[cfg(feature = "usb")]
			usb,
			delay
		})
	}
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	//simple turn red led on
	let mut peripherals = unsafe { crate::pac::Peripherals::steal() };
	let mut pins = Pins::new(peripherals.PORT);
	let mut led = pins.d13.into_push_pull_output(&mut pins.port);
	led.set_high().ok();

	//enable blinking for led
	let core = unsafe { CorePeripherals::steal() };
	let mut clocks = GenericClockController::with_internal_32kosc(
		peripherals.GCLK,
		&mut peripherals.MCLK,
		&mut peripherals.OSC32KCTRL,
		&mut peripherals.OSCCTRL,
		&mut peripherals.NVMCTRL
	);
	let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
	loop {
		led.set_high().ok();
		delay.delay_ms(200_u8);
		led.set_low().ok();
		delay.delay_ms(200_u8);
	}
}
