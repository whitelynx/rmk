# Use `keyboard.toml`

The generated `main.rs` should be like:

```rust
use rmk::macros::rmk_keyboard;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

#[rmk_keyboard]
mod keyboard {}
```

There's a macro `rmk_keyboard` that does the magic for you. This macro will automatically read the `keyboard.toml` in your project root and generate all boilerplate code for you.

There're steps you have to do to customize your own firmware:

### Edit `keyboard.toml`

The generated `keyboard.toml` should have some fields configured from `cargo generate`. But there are still some fields that you want to fill, such as the pin matrix, default keymap, led config, etc.

The [Keyboard Configuration](../keyboard_configuration.md) section has full instructions of how to write your own `keyboard.toml`. Follow the doc and report any issues/questions at <https://github.com/HaoboGu/rmk/issues>. We appreciate your feedback!

### Update `memory.x`

`memory.x` is the linker script of Rust embedded project, it's used to define the memory layout of the microcontroller. RMK enables `memory-x` feature for `embassy-stm32`, so if you're using stm32, you can just ignore this step.

For other ARM Cortex-M microcontrollers, you only need to update the `LENGTH` of FLASH and RAM to your microcontroller.

If you're using **nRF52840**, generally you have to change start address in `memory.x` to 0x27000 or 0x26000, according to your softdevice version. For example, softdevice v6.1.x should use 0x00026000 and v7.3.0 should be 0x00027000

You can either checkout your microcontroller's datasheet or existing Rust project of your microcontroller for it.

### Add your own layout

> The layout should be consistent with the default keymap set in `keyboard.toml`

The next step is to add your own keymap layout for your firmware. RMK supports [vial app](https://get.vial.today/), an
open-source cross-platform(windows/macos/linux/web) keyboard configurator. So the vial like keymap definition has to be
imported to the firmware project.

Fortunately, RMK does most of the heavy things for you, all you need to do is to create your own keymap definition and
convert it to `vial.json` following **[vial's doc here](https://get.vial.today/docs/porting-to-via.html)**, and place it
at the root of the firmware project, replacing the default one. RMK would do all the rest things for you.
