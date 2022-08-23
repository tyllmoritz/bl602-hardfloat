#![no_std]
#![no_main]
use riscv;
use riscv_rt;

use core::fmt::Write;
use embedded_time::rate::*;

use panic_write::PanicHandler;

use bl602_hal as hal;
use hal::{
    clock::{Clocks, Strict, SysclkFreq, UART_PLL_FREQ},
    gpio::*,
    pac,
    serial::*,
};

#[riscv_rt::entry]
fn main() -> ! {
    // get the peripherals
    let dp = pac::Peripherals::take().unwrap();

    // split out the parts
    let mut gpio = dp.GLB.split();

    // Set up all the clocks we need
    let clocks = init_clocks(&mut gpio.clk_cfg);

    // Set up uart output for debug printing. Since this microcontroller has a pin matrix,
    // we need to set up both the pins and the muxes
    let serial = init_usb_serial(
        dp.UART,
        clocks,
        2_000_000.Bd(),
        gpio.pin16,
        gpio.pin7,
        gpio.uart_mux0,
        gpio.uart_mux7,
    );

    // writes panic messages to serial to see where things went wrong
    let mut serial = PanicHandler::new(serial);

    writeln!(serial, "Debug Serial Initialized...\r").ok();

    let mut last_time = riscv::register::mcycle::read64();
    loop {
        let current = riscv::register::mcycle::read64();
        if  current - last_time > 160_000_001 * 2 {
            last_time = current;
            let x = (riscv::register::mcycle::read() % 255) as f32;
            let y = (riscv::register::mcycle::read() % 255) as f32;
            let result = test(x, y);
            writeln!(serial, "{:?}", result).ok();
        }
    }
}

pub fn test(x: f32, y: f32) -> ([f32; 3], usize) {
    let start = riscv::register::mcycle::read();
    let x = if x > y { x - 1. } else { x };
    let z = (x * y) / (x + y);
    // let z = unsafe {core::intrinsics::powf32(x, y)};
    let end = riscv::register::mcycle::read();
    ([x, y, z], end - start)
}

fn init_clocks(config: &mut ClkCfg) -> Clocks {
    Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(SysclkFreq::Pll160Mhz)
        .uart_clk(UART_PLL_FREQ.Hz())
        .freeze(config)
}

fn init_usb_serial<MODE>(
    uart: pac::UART,
    clocks: Clocks,
    baud_rate: Baud,
    tx_pin: Pin16<MODE>,
    rx_pin: Pin7<MODE>,
    tx_mux: UartMux0<Uart0Cts>,
    rx_mux: UartMux7<Uart0Cts>,
) -> impl Write {
    let tx = tx_pin.into_uart_sig0();
    let rx = rx_pin.into_uart_sig7();
    let tx_mux = tx_mux.into_uart0_tx();
    let rx_mux = rx_mux.into_uart0_rx();

    Serial::uart0(
        uart,
        Config::default().baudrate(baud_rate),
        ((tx, tx_mux), (rx, rx_mux)),
        clocks,
    )
}
