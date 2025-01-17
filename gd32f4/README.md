# gd32f4
This crate provides an autogenerated API for access to GD32F4 peripherals.
The API is generated using [svd2rust] with patched svd files containing
extensive type-safe support. For more information please see the [main repo].

Refer to the [documentation] for full details.

[svd2rust]: https://github.com/japaric/svd2rust
[main repo]: https://github.com/gd32-rust/gd32-rs
[documentation]: https://docs.rs/gd32f4/latest/gd32f4/

## Usage
Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.gd32f4]
version = "0.9.1"
features = ["gd32f425", "rt", "critical-section"]
```

The `rt` feature is optional and brings in support for `cortex-m-rt`.

In your code:

```rust
use gd32f4::gd32f425;

let mut peripherals = gd32f425::Peripherals::take().unwrap();
let gpioa = &peripherals.GPIOA;
gpioa.odr.modify(|_, w| w.odr0().set_bit());
```

For full details on the autogenerated API, please see:
https://docs.rs/svd2rust/0.32.0/svd2rust/#peripheral-api

## Supported Devices

| Module | Devices | Links |
|:------:|:-------:|:-----:|
| gd32f425 | GD32F425 | [GD32F425](https://www.gigadevice.com/datasheet/gd32f425xxxx-datasheet/), [gigadevice.com](https://www.gigadevice.com/product/mcu/high-performance-mcus/gd32f4xx-series/gd32f425) |
