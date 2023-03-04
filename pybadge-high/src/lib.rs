#![no_std]

//! ```bash
//! rustup target install thumbv7em-none-eabihf
//! ```

use edgebadge::{
	clock::ClockId,
	entry, gpio,
	gpio::{v2::PA23, *},
	hal, pac, pins,
	prelude::*,
	Pins
};
use hal::{clock::GenericClockController, pwm::Pwm2, sercom::SPIMaster4};
use pac::{CorePeripherals, Peripherals};
use st7735_lcd::ST7735;

mod buttons;
use buttons::Buttons;

pub mod prelude {
	pub use edgebadge::prelude::_embedded_hal_blocking_delay_DelayMs;
}

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
	pub delay: Delay
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

		Ok(PyBadge {
			backlight,
			display,
			buttons,
			red_led,
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
