#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use cortex_m::asm;
use stm32f3::stm32f303;

#[entry]
fn main() -> ! {
  let device = stm32f303::Peripherals::take().unwrap();
  let rcc = device.RCC;
  rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());
  let gpioe = &device.GPIOE;
  gpioe.moder.modify(|_, w| w.moder9().output());
  
  loop {
    gpioe.odr.modify(|_, w| w.odr9().set_bit());
    delay();
    gpioe.odr.modify(|_, w| w.odr9().clear_bit());
    delay();
  }
}

fn delay() {
  for _ in 0..10000 {
    asm::nop();
  };
}