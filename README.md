## sunrise

[![](https://img.shields.io/crates/l/sunrise)][license]
[![](https://img.shields.io/crates/v/sunrise)][crate]
[![](https://img.shields.io/docsrs/sunrise)][docs]

This crate provides a function for calculating sunrise and sunset times using [this method](https://en.wikipedia.org/wiki/Sunrise_equation#Complete_calculation_on_Earth).

To work in a *no-std* environment disable the default features and enable the `libm` feature.

### Usage

In order to perform the calculation, you'll need to provide the following:

- the date for which you wish to calculate the times
- the latitude and longitude of the location

Begin by adding this crate to `Cargo.toml`:

```toml
[dependencies]
sunrise = "1.2"
```

You can use the `SolarDay` struct to perform computation of an event at a
particular place and time:

```rust
use chrono::NaiveDate;
use sunrise::{Coordinates, SolarDay, SolarEvent, DawnType};

// January 1, 2016 in Toronto
let date = NaiveDate::from_ymd_opt(2016, 1, 1).unwrap();
let coord = Coordinates::new(43.6532, -79.3832).unwrap();

let dawn = SolarDay::new(coord, date)
    .with_altitude(54.)
    .event_time(SolarEvent::Dawn(DawnType::Civil));
```

[crate]: https://crates.io/crates/sunrise "crates.io"
[docs]: https://docs.rs/sunrise "Documentation"
[license]: http://opensource.org/licenses/MIT "MIT License"
