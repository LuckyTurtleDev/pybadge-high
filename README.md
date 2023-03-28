# pybadge-high ![License: none](https://img.shields.io/badge/license-none-blue) [![pybadge-high on crates.io](https://img.shields.io/crates/v/pybadge-high)](https://crates.io/crates/pybadge-high) [![pybadge-high on docs.rs](https://docs.rs/pybadge-high/badge.svg)](https://docs.rs/pybadge-high) [![Source Code Repository](https://img.shields.io/badge/Code-On%20none-blue)](none) ![Rust Version: none](https://img.shields.io/badge/rustc--orange.svg)

Goal of this crate is to provide **high level hardware abstraction** layer for the pybade and the edgebadge. It should allow people with no/less knowledge of rust and embedded hardware, to program the boards mention before. If you try to do anything hardware-near or usinig additonal expensions, you should probably use the more hardware-near the [edgebadge][__link0] or [atsamd_hal][__link1] crate instead.


## Setup


##### Installation

 - install rustup: I recommand you to use the [package manger][__link2] of your operation system. Alternative you can install it from https://www.rust-lang.org/tools/install
 - install the rust thumbv7em-none-eabihf (the architecture of the micronctroller)


```bash
rustup target install thumbv7em-none-eabihf
```


##### Create your Project

 - create a new rust project


```bash
cargo new my-app
```

 - add a `.carge/config.tom` with the following content, to define target architecture and flasher


```toml
TODO
```

 - add this crate as dependency


```bash
cargo add pybadge-high
```


 [__link0]: https://crates.io/crates/edgebadge
 [__link1]: https://docs.rs/atsamd-hal/latest/atsamd_hal/
 [__link2]: https://repology.org/project/rustup/versions
