#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate panic_halt;
extern crate nrf91;

use core;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprintln};

static mut JUMP: Option<extern "C" fn()> = None;

/* SCB is the System Control block in an Cortex-M4 device */
pub fn boot_from(scb: &mut cortex_m::peripheral::SCB, address: u32){
    hprintln!("Trying to boot from 0x{:x}",address).unwrap();
    unsafe {
        let stack_pointer = *(address as * const u32);
        /* 4 byte offset from the stackpointer is the application vector table */
        let vector_table  = *((address + 4) as * const u32);

        cortex_m::asm::dsb();
        cortex_m::asm::isb();
        JUMP = Some(core::mem::transmute(vector_table));
        scb.vtor.write(address);
        /* Write the address of the new stack_pointer to the main stack pointer(msp) */
        cortex_m::register::msp::write(stack_pointer);
        (JUMP.unwrap())();
    }
}

#[entry]
fn main() -> ! {
    let mut core_periphials = nrf91::CorePeripherals::take().unwrap() ;
    boot_from(&mut core_periphials.SCB, 0x4000);
    loop
    {
    }
}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
