#![no_std]
#![no_main]

use panic_halt as _;

use embedded_graphics::{
    mono_font::{ascii::FONT_8X13_BOLD, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, PrimitiveStyle},
    text::{Baseline, Text},
};

use sh1107g_rs::{Sh1107g, Sh1107gBuilder};

// esp-hal 1.0.0-rc.0用
use esp_hal::peripherals::Peripherals;
use esp_hal::clock::ClockControl;
use esp_hal::i2c::master::{I2c, self};
use esp_hal::gpio::{GpioPin, Input, Output, PushPull};
use fugit::RateExtU32;
use core::convert::Infallible;

#[no_mangle]
pub extern "C" fn main() -> ! {
    // --- ペリフェラル取得 ---
    let peripherals = Peripherals::take();

    // --- クロック設定 ---
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // --- GPIO / ピン設定 ---
    let io = esp_hal::gpio::io::Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let sda: GpioPin<Input<PullDown>, 21> = io.pins.gpio21;
    let scl: GpioPin<Input<PullDown>, 22> = io.pins.gpio22;

    // --- I2C 初期化 ---
    let i2c = I2c::new(
        peripherals.I2C0,
        i2c::Config::new().baudrate(100.kHz().into()),
        sda.into_open_drain_output(),
        scl.into_open_drain_output(),
        &clocks,
    ).unwrap();

    // --- SH1107G 初期化 ---
    let mut display = Sh1107gBuilder::new(i2c)
        .with_address(0x3C)
        .clear_on_init(true)
        .build();

    display.init().unwrap();

    // --- 描画 ---
    Rectangle::new(Point::new(10, 10), Size::new(100, 50))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    Circle::new(Point::new(64, 90), 20)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    let style = MonoTextStyle::new(&FONT_8X13_BOLD, BinaryColor::On);
    Text::with_baseline("Hello, ESP32!", Point::new(15, 30), style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}