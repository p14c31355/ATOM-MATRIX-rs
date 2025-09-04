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

// esp-hal 1.0
use esp_hal::{
    clock::ClockControl,
    i2c::I2c,
    peripherals::Peripherals,
    system::SystemExt,
    gpio::IO,
};

use fugit::RateExtU32; // trait をスコープに入れる

#[entry]
fn main() -> ! {
    // --- ペリフェラル取得 ---
    let peripherals = unsafe { Peripherals::steal() };

    // --- クロック設定 ---
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // --- GPIO/ピン設定 ---
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // SDA / SCL ピン設定
    let sda = io.pins.gpio21;
    let scl = io.pins.gpio22;

    // --- I2C 初期化 ---
    let i2c = I2c::new(
        peripherals.I2C0,
        sda,
        scl,
        100_000u32.Hz(), // 100kHz
        &clocks,
    );

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
