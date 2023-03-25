#![no_std]
#![allow(clippy::tabs_in_doc_comments)]
#![warn(unreachable_pub)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]

//! ```bash
//! rustup target install thumbv7em-none-eabihf
//! ```

#[cfg(feature = "neopixel")]
use edgebadge::gpio::v2::PA15;
use edgebadge::{
	clock::ClockId,
	entry, gpio,
	gpio::{v2, v2::PA23, *},
	hal, pac, pins,
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
use buttons::Buttons;

pub mod prelude {
	pub use cortex_m_rt::entry;
	pub use edgebadge::prelude::_embedded_hal_blocking_delay_DelayMs;
	#[cfg(feature = "neopixel")]
	pub use smart_leds_trait::SmartLedsWrite;
}

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
pub type NeoPixel = Ws2812<
	SpinTimer,
	OldOutputPin<edgebadge::gpio::Pin<PA15, gpio::v2::Output<gpio::v2::PushPull>>>
>;
#[cfg(feature = "neopixel")]
pub type NeoPixelColor = <NeoPixel as SmartLedsWrite>::Color;

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

pub struct PyBadge {
	pub backlight: Backlight,
	pub display: Display,
	pub buttons: Buttons,
	pub red_led: Led,
	pub delay: Delay,
	#[cfg(feature = "neopixel")]
	pub neopixel: NeoPixel
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
		let (mut display, backlight) = pins.display.init(
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

		Ok(PyBadge {
			backlight,
			display,
			buttons,
			red_led,
			#[cfg(feature = "neopixel")]
			neopixel,
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
