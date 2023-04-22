#![no_std]
#![allow(clippy::tabs_in_doc_comments)]
#![warn(unreachable_pub)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]
#![allow(deprecated)]

//! Goal of this crate is to provide **high level hardware abstraction** layer for the pybade and the edgebadge.
//! It should allow people with no/less knowledge of rust and embedded hardware, to program the boards mention before.
//! If you try to do anything hardware-near or usinig additonal expensions,
//! you should probably use the more hardware-near the [edgebadge](https://crates.io/crates/edgebadge) or [atsamd_hal](https://docs.rs/atsamd-hal/latest/atsamd_hal/) crate instead.
//!
//! # Setup
//! #### Installation
//! * Install rustup.
//! I recommand to use the [package manger](https://repology.org/project/rustup/versions) of your operation system.
//! Alternative you can install it from <https://www.rust-lang.org/tools/install>
//! * install the rust thumbv7em-none-eabihf target. (the architecture of the micronctroller)
//! ```bash
//! rustup target install thumbv7em-none-eabihf
//! ```
//! * optional: install nightly toolchain for better doc.
//! ```
//! rustup toolchain install nightly --target thumbv7em-none-eabihf
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
//! [target.thumbv7em-none-eabihf]
//! runner = "hf2 elf"
//! #runner = 'probe-run --chip ATSAMD51J19A'
//!
//! [build]
//! target = "thumbv7em-none-eabihf"
//! rustflags = [
//!
//!   # This is needed if your flash or ram addresses are not aligned to 0x10000 in memory.x
//!   # See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
//!   "-C", "link-arg=--nmagic",
//!
//!   "-C", "link-arg=-Tlink.x",
//! ]
//!
//! [profile.release]
//! codegen-units = 1 # better optimizations
//! debug = true # symbols are nice and they don't increase the size on Flash
//! lto = true # better optimizations
//! ```
//! * Add this crate as dependency
//! ```bash
//! cargo add pybadge-high
//! ```
//!
//! * Addjust your `main.rs`
//! You need to do some changes at your `main.rs`.
//! First you must disable the rust standart libary by adding `#![no_std]`, because it does not supported the pybadge.
//! This does also mean you can not use the default main function and must disable it with `#![no_main]`.
//! But because we still need a main function we need to define our own with `#[entry]`.
//! This main function does never return (`!`).
//! Otherwise the pybadge would do random stuff after the program has finish.
//! So we need a endless loop.
//! To get access to the peripherals of the pybadge, like display, buttons, leds etc you call [`PyBadge::take()`];
//! This function can only called once at runtime otherwise it will return an Error.
//! ```
//! #![no_std]
//! #![no_main]
//!
//! use pybadge_high::{prelude::*, PyBadge};
//!
//! #[entry]
//! fn main() -> ! {
//! 	let mut pybadge = PyBadge::take().unwrap();
//! 	loop {}
//! }
//! ```
//!
//! #### Flashing:
//! To flash you program, put your device in bootloader mode by hitting the reset button twice.
//! After this excute
//! ```bash
//! cargo run --release
//! ```
//! The display does not work until you have press the reset button of the pybadge after flashing.

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

pub mod time;

mod buttons;
pub use buttons::Buttons;

pub mod prelude {
	pub use cortex_m_rt::entry;
	pub use edgebadge::prelude::{
		_embedded_hal_blocking_delay_DelayMs, _embedded_hal_blocking_delay_DelayUs
	};
	#[cfg(feature = "neopixel")]
	pub use smart_leds_trait::SmartLedsWrite;
}

#[cfg(feature = "usb")]
pub mod usb;
#[cfg(feature = "usb")]
use usb::UsbBuilder;

#[cfg(feature = "flash")]
mod flash;
#[doc(hidden)] //feature temporary disable
#[cfg(feature = "flash")]
pub use flash::Flash;

#[cfg(feature = "pwm_sound")]
mod sound;
#[cfg(feature = "pwm_sound")]
use sound::PwmSound;

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
///
///Can only called once at runtime otherwise it will return an Error.
#[non_exhaustive]
pub struct PyBadge {
	pub backlight: Backlight,
	pub display: Display,
	pub buttons: Buttons,
	pub red_led: Led,
	pub delay: Delay,
	#[cfg(feature = "neopixel")]
	pub neopixel: NeoPixel,
	#[doc(hidden)] //feature temporary disable
	#[cfg(feature = "flash")]
	pub flash: Flash,
	#[cfg(feature = "pwm_sound")]
	pub speaker: PwmSound,
	#[cfg(feature = "usb")]
	pub usb_builder: UsbBuilder
}

impl PyBadge {
	/// Returns all the supported peripherals.
	/// This function can only called once,
	/// otherwise it does return Err.
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
		//move TC2
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

		//flash
		#[cfg(feature = "flash")]
		let flash = flash::Flash::init(
			pins.flash,
			&mut peripherals.MCLK,
			peripherals.QSPI,
			&mut delay
		);

		//32kHz clock to be used for sound and time at TC4 and TC5
		//move tc4_tc5
		#[cfg(any(feature = "pwm_sound", feature = "time"))]
		let tc4_tc5 = {
			let gclk = clocks.gclk1();
			clocks.tc4_tc5(&gclk).unwrap()
		};

		//speaker
		//move Tc4
		#[cfg(feature = "pwm_sound")]
		let speaker = {
			let enable_pin = pins.speaker.enable.into_push_pull_output(&mut pins.port);
			let speaker_pin = pins.speaker.speaker.into_push_pull_output(&mut pins.port);
			let counter = edgebadge::thumbv7em::timer::TimerCounter::tc4_(
				&tc4_tc5,
				peripherals.TC4,
				&mut peripherals.MCLK
			);
			sound::PwmSound::init(enable_pin, speaker_pin, counter)
		};

		//time
		//move TC5
		#[cfg(feature = "time")]
		{
			let counter = edgebadge::thumbv7em::timer::TimerCounter::tc5_(
				&tc4_tc5,
				peripherals.TC5,
				&mut peripherals.MCLK
			);
			time::init_counter(counter);
		};

		//usb
		#[cfg(feature = "usb")]
		let usb_builder = UsbBuilder {
			usb_vid: 0x16c0,
			usb_pid: 0x27dd,
			manufacturer: "Fake company",
			product: "Serial port",
			serial_number: "Test",
			pins: pins.usb,
			peripherals: peripherals.USB,
			clocks,
			mclk: peripherals.MCLK
		};

		Ok(PyBadge {
			backlight,
			display,
			buttons,
			red_led,
			#[cfg(feature = "neopixel")]
			neopixel,
			#[cfg(feature = "flash")]
			flash,
			#[cfg(feature = "pwm_sound")]
			speaker,
			#[cfg(feature = "usb")]
			usb_builder,
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
	let mut speaker = pins.speaker.into_push_pull_output(&mut pins.port);
	let mut speaker_enable = pins.speaker_enable.into_push_pull_output(&mut pins.port);
	let mut i = 0_u8;
	loop {
		led.toggle();
		//stop sound after 3 seconds (it is annoying)
		if i <= 8 {
			speaker_enable.toggle();
			i += 1
		} else {
			speaker_enable.set_low().ok();
		}
		for _ in 0..100 {
			delay.delay_ms(2_u8);
			speaker.toggle();
		}
	}
}
