#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate panic_halt;
extern crate nrf91;

use core;
use cty;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{hprintln};

mod bl_cc310;

static mut JUMP: Option<extern "C" fn()> = None;

//pub fn verify_image(address: u32){
//}

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

pub fn cc310_enable(){
    let mut peripherals = nrf91::Peripherals::take().unwrap();
    peripherals.CRYPTOCELL_S.enable.write(|w| w.enable().enabled());
}

pub fn cc310_disable(){
    let mut peripherals = nrf91::Peripherals::take().unwrap();
    peripherals.CRYPTOCELL_S.enable.write(|w| w.enable().disabled());
}

/* Should be called only once(find a way to enforce this) */
pub fn cc310_init() -> bl_cc310::CRYSError_t {
    cc310_enable();
    let mut ret = 0;
    unsafe{
        ret = bl_cc310::nrf_cc310_bl_init();
    }
    cc310_disable();
    return ret;
}

pub fn cc310_ecdsa_validate(public_key : u8, signature: u8, hash : u8, hash_len : u32) -> bl_cc310::CRYSError_t
{
    cc310_enable();
    let mut ret = 0;
    let mut ctx = bl_cc310::nrf_cc310_bl_ecdsa_verify_context_secp256r1_t { init_val: 0, context_buffer: [0; 160usize]};
    let ctx_pointer = &mut ctx as *mut bl_cc310::nrf_cc310_bl_ecdsa_verify_context_secp256r1_t;
    /*
    unsafe{
        ret = bl_cc310::nrf_cc310_bl_ecdsa_verify_secp256r1(ctx_pointer, public_key, signature, hash, hash_len);
    }
    */
    cc310_disable();
    return ret;

}

#[entry]
fn main() -> ! {
    let ret = cc310_init();
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
