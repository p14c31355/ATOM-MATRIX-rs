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

use sh1107g_rs::Sh1107gBuilder;

use esp_hal::i2c::master::I2c;
use esp_hal::i2c::master::Config;
use esp_hal::clock::CpuClock; // CpuClockをインポート
use xtensa_lx_rt::entry;

#[entry]
fn main() -> ! {
    // --- ペリフェラル取得 ---
    // --- クロック設定 ---
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // --- I2C 初期化 ---
    let mut i2c = I2c::new(
        peripherals.I2C0,
        Config::default(), // Config::new() を Config::default() に変更
    ).unwrap();

    // --- SH1107G 初期化 ---
    let mut display = Sh1107gBuilder::new(&mut i2c)
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
