#![no_std]
#![no_main]


use rmk::macros::rmk_keyboard;

// Vial config is automatically generated by `build.rs`, according to `vial.json`
// Please put `vial.json` at your project's root
#[rmk_keyboard]
mod keyboard {}
