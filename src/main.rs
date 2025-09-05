use std::thread;
use std::time::Duration;

use embedded_graphics::{
    mono_font::{ascii::FONT_8X13_BOLD, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, PrimitiveStyle},
    text::{Baseline, Text},
};

use sh1107g_rs::Sh1107gBuilder;

use esp_idf_hal::{i2c::{I2cConfig, I2cDriver}, peripherals::Peripherals};
// use esp_idf_sys as _; // 必須: esp-idf std 環境で初期化パッチをロード

fn main() -> anyhow::Result<()> {
    // ESP-IDF std 環境用初期化
    esp_idf_sys::link_patches();

    // --- I2C 初期化 ---
    let peripherals = Peripherals::take().unwrap();
    let config = I2cConfig::new().baudrate(esp_idf_hal::prelude::Hertz(100000));
    let mut i2c = I2cDriver::new(peripherals.i2c0, peripherals.pins.gpio26, peripherals.pins.gpio32, &config)?;

//     for addr in 0x00..=0x7F {
//     if i2c.write(addr, &[0], 1000).is_ok() {
//         println!("Found device @ 0x{:02X}", addr);
//     }
// }

    // --- SH1107G 初期化 ---
    let mut display = Sh1107gBuilder::new(&mut i2c)
        // .with_address(0x3C)
        .clear_on_init(true)
        .build();

    display.init().unwrap();
    display.clear_buffer();

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

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
