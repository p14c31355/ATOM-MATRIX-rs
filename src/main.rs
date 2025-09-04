#![no_std]
#![no_main]
use fugit::rate::ExtU32;

use esp_hal::{
    clock::ClockControl,
    gpio::Io,
    i2c::I2c,
    prelude::*,
    system::SystemControl,
};
use panic_halt as _;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use esp_backtrace as _; 

use embedded_graphics::{
    mono_font::{ascii::FONT_8X13_BOLD, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, PrimitiveStyle},
    text::{Baseline, Text},
};

use sh1107g_rs::{Sh1107g, Sh1107gBuilder};

#[unsefe(no_mangle)]
fn main() -> ! {
    // 初期化（クロック & ペリフェラル取得）
    let peripherals = pac::Peripherals::take().unwrap();
let system = peripherals.system.split(); // HAL ラッパーを使う
let io = peripherals.iomux.split();     // GPIO/IO_MUX を安全に取得


    // I2C 初期化 (SDA=21, SCL=22 → ATOM Matrix のデフォルト)
    let i2c = I2C::new(
    peripherals.I2C0,
    pins.sda,
    pins.scl,
    100000_u32.Hz(), // 型明示
);


    // SH1107G ドライバ初期化
    let mut display = Sh1107gBuilder::new(i2c)
        .with_address(0x3C)
        .clear_on_init(true)
        .build();

    display.init().unwrap();

    // 描画例
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

    // 一度だけ flush
    display.flush().unwrap();

    loop {}
}
