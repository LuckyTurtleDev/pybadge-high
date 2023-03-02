#![no_std]

use pygamer::{clock::ClockId, entry, hal, pac, Pins};

use hal::{clock::GenericClockController, pwm::Pwm2, sercom::SPIMaster4};
use pac::{CorePeripherals, Peripherals};
use pygamer::{gpio, gpio::*};
use st7735_lcd::ST7735;

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

pub struct PyBadge {
	pub backlight: Backlight,
	pub display: Display
}

impl PyBadge {
	pub fn take() -> Option<PyBadge> {
		let mut peripherals = Peripherals::take().unwrap();
		let core = CorePeripherals::take().unwrap();
		let mut clocks = GenericClockController::with_internal_32kosc(
			peripherals.GCLK,
			&mut peripherals.MCLK,
			&mut peripherals.OSC32KCTRL,
			&mut peripherals.OSCCTRL,
			&mut peripherals.NVMCTRL
		);
		let mut pins = Pins::new(peripherals.PORT).split();
		let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

		let (mut display, backlight) = pins
			.display
			.init(
				&mut clocks,
				peripherals.SERCOM4,
				&mut peripherals.MCLK,
				peripherals.TC2,
				&mut delay,
				&mut pins.port
			)
			.unwrap();
		Some(PyBadge { backlight, display })
	}
}
