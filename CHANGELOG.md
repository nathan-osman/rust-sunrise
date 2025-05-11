# Changelog

## 2.1.0

- Use functions and constants from `core` instead of vendoring them
  in "no-std" mode.
- Implement `Display` without "std" feature enabled.

## 2.0.0

- To make features additive the "no-std" feature has been replaced by
  the "std" feature (default). To build for "no-std" now, set 
  "default-features" to false, and enable the "libm" feature.

## 1.2.1

- Fix _SolarEvent::Dawn_ & _SolarEvent::Dusk_ being inverted.

## 1.2.0

- Add new feature "no-std"

## 1.1.0

- Add the `SolarDay` struct that allows to performs calculations on a
  particular date and place with validated inputs.
- Mark `sunrise_sunset` as deprecated.
- Do not expose internal implementation functions anymore
- Bump to edition 2024
