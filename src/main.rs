#![no_std]
#![no_main]

extern crate panic_halt;
extern crate riscv_rt;

#[riscv_rt::entry]
fn main() -> ! {
    // do something here
    loop {}
}
