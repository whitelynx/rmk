use bitfield_struct::bitfield;
use defmt::info;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Sender};
use embedded_hal::digital::InputPin;

use crate::keyboard::KeyboardReportMessage;

/// Rotary encoder input device
/// The push key in rotary encoder is in the key matrix, so it's not included in this struct
pub struct RotaryEncoder<'a, P: InputPin> {
    sender: &'a Sender<'a, CriticalSectionRawMutex, KeyboardReportMessage, 8>,
    state: EncoderState,
    dir: Direction,
    pin_a: P,
    pin_b: P,
}

#[bitfield(u8)]
#[derive(Eq, PartialEq)]
struct EncoderState {
    #[bits(1)]
    pin_a_prev: bool,
    #[bits(1)]
    pin_b_prev: bool,
    #[bits(1)]
    pin_a: bool,
    #[bits(1)]
    pin_b: bool,
    #[bits(4)]
    _unused: u8,
}

impl EncoderState {
    pub(crate) fn update(&mut self) {
        self.set_pin_a_prev(self.pin_a());
        self.set_pin_b_prev(self.pin_b());
        self.set_pin_a(false);
        self.set_pin_b(false);
    }
}

/// Direction of Rotary Encoder rotation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// No Direction is specified,
    None,
    /// Clockwise direction
    Clockwise,
    /// Counter-clockwise direction
    CounterClockwise,
}

impl From<EncoderState> for Direction {
    fn from(s: EncoderState) -> Self {
        match s.0 {
            0b0001 | 0b0111 | 0b1000 | 0b1110 => Direction::Clockwise,
            0b0010 | 0b0100 | 0b1011 | 0b1101 => Direction::CounterClockwise,
            _ => Direction::None,
        }
    }
}

// Impl RotaryEncoder
impl<'a, P: InputPin> RotaryEncoder<'a, P> {
    pub fn new(
        sender: &'a Sender<'a, CriticalSectionRawMutex, KeyboardReportMessage, 8>,
        pin_a: P,
        pin_b: P,
    ) -> Self {
        Self {
            sender,
            state: EncoderState(0),
            dir: Direction::None,
            pin_a,
            pin_b,
        }
    }

    async fn process(&mut self) {
        self.state
            .set_pin_a(self.pin_a.is_high().unwrap_or_default());
        self.state
            .set_pin_b(self.pin_b.is_high().unwrap_or_default());
        // Update direction first
        self.dir = Direction::from(self.state);
        // Then update rotary encoder state
        self.state.update();
    }

    async fn send_report(&self) {
        // TODO: Remap the rotary encoder action to keymap, how?
        match self.dir {
            Direction::None => (),
            Direction::Clockwise => {
                info!("Rotary Encoder Clockwise");
                // self.sender.send().await
            }
            Direction::CounterClockwise => {
                info!("Rotary Encoder CounterClockwise");
            }
        }
    }
}
