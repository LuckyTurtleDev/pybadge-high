# pybadge-high ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![pybadge-high on crates.io](https://img.shields.io/crates/v/pybadge-high)](https://crates.io/crates/pybadge-high) [![pybadge-high on docs.rs](https://docs.rs/pybadge-high/badge.svg)](https://docs.rs/pybadge-high) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/LuckyTurtleDev/more-wallpapers) ![Rust Version: none](https://img.shields.io/badge/rustc--orange.svg)

Goal of this crate is to provide **high level hardware abstraction** layer for the pybade and the edgebadge. It should allow people with no/less knowledge of rust and embedded hardware, to program the boards mention before. If you try to do anything hardware-near or usinig additonal expensions, you should probably use the more hardware-near the [edgebadge][__link0] or [atsamd_hal][__link1] crate instead.


## Setup


##### Installation

 - Install rustup. I recommand to use the [package manger][__link2] of your operation system. Alternative you can install it from <https://www.rust-lang.org/tools/install>
 - install the rust thumbv7em-none-eabihf target. (the architecture of the micronctroller)


```bash
rustup target install thumbv7em-none-eabihf
```

 - optional: install nightly toolchain for better doc (only relevant if you build the doc by yourself).


```rust
rustup toolchain install nightly --target thumbv7em-none-eabihf
```

 - install the [hf2-cli][__link4] flasher


##### Create your Project

 - Create a new rust project.


```bash
cargo new my-app
```

 - Add a `.cargo/config.toml` with the following content, to define target architecture and flasher


```toml
[target.thumbv7em-none-eabihf]
runner = "hf2 elf"
#runner = 'probe-run --chip ATSAMD51J19A'

[build]
target = "thumbv7em-none-eabihf"
rustflags = [

  # This is needed if your flash or ram addresses are not aligned to 0x10000 in memory.x
  # See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
  "-C", "link-arg=--nmagic",

  "-C", "link-arg=-Tlink.x",
]
```

 - Add this crate as dependency


```bash
cargo add pybadge-high
```

 - optional: add this to your `cargo.toml` for better optimizations


```toml
[profile.release]
codegen-units = 1 # better optimizations
debug = true # symbols are nice and they don't increase the size on Flash
lto = true # better optimizations
```

 - Addjust your `main.rs`

You need to do some changes at your `main.rs`. First you must disable the rust standart libary by adding `#![no_std]`, because it is not supported by the pybadge. This does also mean you can not use the default main function any more and must disable it with `#![no_main]`. But because we still need a main function to enter the code we need to define our own main with `#[entry]`. This main function does never return (`!`). Otherwise the pybadge would do random stuff after the program has finish. So we need a endless loop. To get access to the peripherals of the pybadge, like display, buttons, leds etc you call [`PyBadge::take()`][__link5]. This function can only called once at runtime otherwise it will return an Error.


```rust
#![no_std]
#![no_main]

use pybadge_high::{prelude::*, PyBadge};

#[entry]
fn main() -> ! {
	let mut pybadge = PyBadge::take().unwrap();
	loop {}
}
```

When a program does panic, the red led at the back of the board starts flashing. If the `bluescreen`(default) feature is enable, the display does show the postion of the error. When the `beep_panic` feature is enable, the pybadge also beep for 3 seconds.


##### Flashing:

To flash you program, put your device in bootloader mode by hitting the reset button twice. After this excute


```bash
cargo run --release
```

The display does not work until you have press the reset button of the pybadge after flashing.


## Feature-flags

This crate has spilt functionallity in multiple feature flags. See the [rust book][__link6] for more information about features. Enabling only the feauters, which are needed, helps to keep the binary size small and reduce the number of needed dependencies.

The following features are aviable:

 - **`beep_panic`** —  beep for 3 seconds, when rust pancics
	
	
 - **`bluescreen`** *(enabled by default)* —  show a bluescreen with error postion, when rust pancics
	
	
 - **`neopixel`** —  support for the Neopixel below the screen
	
	
 - **`usb`** —  support for serial communication over usb
	
	
 - **`pwm_sound`** —  support for single frequenc sound
	
	
 - **`time`** *(enabled by default)* —  support for time measurement
	
	


 [__link0]: https://crates.io/crates/edgebadge
 [__link1]: https://docs.rs/atsamd-hal/latest/atsamd_hal/
 [__link2]: https://repology.org/project/rustup/versions
 [__link4]: https://crates.io/crates/hf2-cli
 [__link5]: `PyBadge::take()`
 [__link6]: https://doc.rust-lang.org/cargo/reference/features.html
