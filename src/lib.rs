#![feature(abi_msp430_interrupt)]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for MSP430FR5994 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate msp430;
#[cfg(feature = "rt")]
extern crate msp430_rt;
#[cfg(feature = "rt")]
pub use msp430_rt::default_handler;
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 5;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "ADC12_B"]
pub struct ADC12_B {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC12_B {}
impl ADC12_B {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc12_b::RegisterBlock {
        0x0800 as *const _
    }
}
impl Deref for ADC12_B {
    type Target = adc12_b::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC12_B::ptr() }
    }
}
#[doc = "ADC12_B"]
pub mod adc12_b;
#[doc = "AES256"]
pub struct AES256 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES256 {}
impl AES256 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes256::RegisterBlock {
        0x09c0 as *const _
    }
}
impl Deref for AES256 {
    type Target = aes256::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES256::ptr() }
    }
}
#[doc = "AES256"]
pub mod aes256;
#[doc = "CAPTIO0"]
pub struct CAPTIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAPTIO0 {}
impl CAPTIO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const captio0::RegisterBlock {
        0x0430 as *const _
    }
}
impl Deref for CAPTIO0 {
    type Target = captio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAPTIO0::ptr() }
    }
}
#[doc = "CAPTIO0"]
pub mod captio0;
#[doc = "CAPTIO1"]
pub struct CAPTIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAPTIO1 {}
impl CAPTIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const captio1::RegisterBlock {
        0x0470 as *const _
    }
}
impl Deref for CAPTIO1 {
    type Target = captio1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAPTIO1::ptr() }
    }
}
#[doc = "CAPTIO1"]
pub mod captio1;
#[doc = "COMP_E"]
pub struct COMP_E {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP_E {}
impl COMP_E {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp_e::RegisterBlock {
        0x08c0 as *const _
    }
}
impl Deref for COMP_E {
    type Target = comp_e::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP_E::ptr() }
    }
}
#[doc = "COMP_E"]
pub mod comp_e;
#[doc = "CRC"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x0150 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "CRC"]
pub mod crc;
#[doc = "CRC32"]
pub struct CRC32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC32 {}
impl CRC32 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc32::RegisterBlock {
        0x0980 as *const _
    }
}
impl Deref for CRC32 {
    type Target = crc32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC32::ptr() }
    }
}
#[doc = "CRC32"]
pub mod crc32;
#[doc = "CS"]
pub struct CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CS {}
impl CS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cs::RegisterBlock {
        0x0160 as *const _
    }
}
impl Deref for CS {
    type Target = cs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CS::ptr() }
    }
}
#[doc = "CS"]
pub mod cs;
#[doc = "PA"]
pub struct PA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PA {}
impl PA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pa::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for PA {
    type Target = pa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PA::ptr() }
    }
}
#[doc = "PA"]
pub mod pa;
#[doc = "P1"]
pub struct P1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P1 {}
impl P1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p1::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P1 {
    type Target = p1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P1::ptr() }
    }
}
#[doc = "P1"]
pub mod p1;
#[doc = "P2"]
pub struct P2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P2 {}
impl P2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p2::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P2 {
    type Target = p2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P2::ptr() }
    }
}
#[doc = "P2"]
pub mod p2;
#[doc = "PB"]
pub struct PB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PB {}
impl PB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pb::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for PB {
    type Target = pb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PB::ptr() }
    }
}
#[doc = "PB"]
pub mod pb;
#[doc = "P3"]
pub struct P3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P3 {}
impl P3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p3::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P3 {
    type Target = p3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P3::ptr() }
    }
}
#[doc = "P3"]
pub mod p3;
#[doc = "P4"]
pub struct P4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P4 {}
impl P4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p4::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P4 {
    type Target = p4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P4::ptr() }
    }
}
#[doc = "P4"]
pub mod p4;
#[doc = "PC"]
pub struct PC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PC {}
impl PC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pc::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for PC {
    type Target = pc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PC::ptr() }
    }
}
#[doc = "PC"]
pub mod pc;
#[doc = "P5"]
pub struct P5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P5 {}
impl P5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p5::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P5 {
    type Target = p5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P5::ptr() }
    }
}
#[doc = "P5"]
pub mod p5;
#[doc = "P6"]
pub struct P6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P6 {}
impl P6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p6::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P6 {
    type Target = p6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P6::ptr() }
    }
}
#[doc = "P6"]
pub mod p6;
#[doc = "PD"]
pub struct PD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PD {}
impl PD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pd::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for PD {
    type Target = pd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PD::ptr() }
    }
}
#[doc = "PD"]
pub mod pd;
#[doc = "P7"]
pub struct P7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P7 {}
impl P7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p7::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P7 {
    type Target = p7::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P7::ptr() }
    }
}
#[doc = "P7"]
pub mod p7;
#[doc = "P8"]
pub struct P8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P8 {}
impl P8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p8::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for P8 {
    type Target = p8::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P8::ptr() }
    }
}
#[doc = "P8"]
pub mod p8;
#[doc = "PJ"]
pub struct PJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PJ {}
impl PJ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pj::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for PJ {
    type Target = pj::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PJ::ptr() }
    }
}
#[doc = "PJ"]
pub mod pj;
#[doc = "DMA"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x0500 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA"]
pub mod dma;
#[doc = "FRCTL_A"]
pub struct FRCTL_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FRCTL_A {}
impl FRCTL_A {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const frctl_a::RegisterBlock {
        0x0140 as *const _
    }
}
impl Deref for FRCTL_A {
    type Target = frctl_a::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FRCTL_A::ptr() }
    }
}
#[doc = "FRCTL_A"]
pub mod frctl_a;
#[doc = "LEA"]
pub struct LEA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEA {}
impl LEA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lea::RegisterBlock {
        0x0a80 as *const _
    }
}
impl Deref for LEA {
    type Target = lea::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LEA::ptr() }
    }
}
#[doc = "LEA"]
pub mod lea;
#[doc = "MPU"]
pub struct MPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPU {}
impl MPU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpu::RegisterBlock {
        0x05a0 as *const _
    }
}
impl Deref for MPU {
    type Target = mpu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MPU::ptr() }
    }
}
#[doc = "MPU"]
pub mod mpu;
#[doc = "MPY32"]
pub struct MPY32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY32 {}
impl MPY32 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpy32::RegisterBlock {
        0x04c0 as *const _
    }
}
impl Deref for MPY32 {
    type Target = mpy32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MPY32::ptr() }
    }
}
#[doc = "MPY32"]
pub mod mpy32;
#[doc = "PMM"]
pub struct PMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMM {}
impl PMM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmm::RegisterBlock {
        0x0120 as *const _
    }
}
impl Deref for PMM {
    type Target = pmm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMM::ptr() }
    }
}
#[doc = "PMM"]
pub mod pmm;
#[doc = "RAMCTL"]
pub struct RAMCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RAMCTL {}
impl RAMCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ramctl::RegisterBlock {
        0x0158 as *const _
    }
}
impl Deref for RAMCTL {
    type Target = ramctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RAMCTL::ptr() }
    }
}
#[doc = "RAMCTL"]
pub mod ramctl;
#[doc = "REF_A"]
pub struct REF_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REF_A {}
impl REF_A {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ref_a::RegisterBlock {
        0x01b0 as *const _
    }
}
impl Deref for REF_A {
    type Target = ref_a::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*REF_A::ptr() }
    }
}
#[doc = "REF_A"]
pub mod ref_a;
#[doc = "RTC_C"]
pub struct RTC_C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_C {}
impl RTC_C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_c::RegisterBlock {
        0x04a0 as *const _
    }
}
impl Deref for RTC_C {
    type Target = rtc_c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC_C::ptr() }
    }
}
#[doc = "RTC_C"]
pub mod rtc_c;
#[doc = "SFR"]
pub struct SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SFR {}
impl SFR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sfr::RegisterBlock {
        0x0100 as *const _
    }
}
impl Deref for SFR {
    type Target = sfr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SFR::ptr() }
    }
}
#[doc = "SFR"]
pub mod sfr;
#[doc = "SYS"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys::RegisterBlock {
        0x0180 as *const _
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYS::ptr() }
    }
}
#[doc = "SYS"]
pub mod sys;
#[doc = "TA0"]
pub struct TA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TA0 {}
impl TA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ta0::RegisterBlock {
        0x0340 as *const _
    }
}
impl Deref for TA0 {
    type Target = ta0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TA0::ptr() }
    }
}
#[doc = "TA0"]
pub mod ta0;
#[doc = "TA1"]
pub struct TA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TA1 {}
impl TA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ta1::RegisterBlock {
        0x0380 as *const _
    }
}
impl Deref for TA1 {
    type Target = ta1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TA1::ptr() }
    }
}
#[doc = "TA1"]
pub mod ta1;
#[doc = "TA2"]
pub struct TA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TA2 {}
impl TA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ta2::RegisterBlock {
        0x0400 as *const _
    }
}
impl Deref for TA2 {
    type Target = ta2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TA2::ptr() }
    }
}
#[doc = "TA2"]
pub mod ta2;
#[doc = "TA3"]
pub struct TA3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TA3 {}
impl TA3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ta3::RegisterBlock {
        0x0440 as *const _
    }
}
impl Deref for TA3 {
    type Target = ta3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TA3::ptr() }
    }
}
#[doc = "TA3"]
pub mod ta3;
#[doc = "TA4"]
pub struct TA4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TA4 {}
impl TA4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ta4::RegisterBlock {
        0x07c0 as *const _
    }
}
impl Deref for TA4 {
    type Target = ta4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TA4::ptr() }
    }
}
#[doc = "TA4"]
pub mod ta4;
#[doc = "TB0"]
pub struct TB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TB0 {}
impl TB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tb0::RegisterBlock {
        0x03c0 as *const _
    }
}
impl Deref for TB0 {
    type Target = tb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TB0::ptr() }
    }
}
#[doc = "TB0"]
pub mod tb0;
#[doc = "WDT_A"]
pub struct WDT_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT_A {}
impl WDT_A {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt_a::RegisterBlock {
        0x015c as *const _
    }
}
impl Deref for WDT_A {
    type Target = wdt_a::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT_A::ptr() }
    }
}
#[doc = "WDT_A"]
pub mod wdt_a;
#[doc = "eUSCI_A0"]
pub struct EUSCI_A0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_A0 {}
impl EUSCI_A0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_a0::RegisterBlock {
        0x05c0 as *const _
    }
}
impl Deref for EUSCI_A0 {
    type Target = e_usci_a0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_A0::ptr() }
    }
}
#[doc = "eUSCI_A0"]
pub mod e_usci_a0;
#[doc = "eUSCI_A1"]
pub struct EUSCI_A1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_A1 {}
impl EUSCI_A1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_a1::RegisterBlock {
        0x05e0 as *const _
    }
}
impl Deref for EUSCI_A1 {
    type Target = e_usci_a1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_A1::ptr() }
    }
}
#[doc = "eUSCI_A1"]
pub mod e_usci_a1;
#[doc = "eUSCI_A2"]
pub struct EUSCI_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_A2 {}
impl EUSCI_A2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_a2::RegisterBlock {
        0x0600 as *const _
    }
}
impl Deref for EUSCI_A2 {
    type Target = e_usci_a2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_A2::ptr() }
    }
}
#[doc = "eUSCI_A2"]
pub mod e_usci_a2;
#[doc = "eUSCI_A3"]
pub struct EUSCI_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_A3 {}
impl EUSCI_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_a3::RegisterBlock {
        0x0620 as *const _
    }
}
impl Deref for EUSCI_A3 {
    type Target = e_usci_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_A3::ptr() }
    }
}
#[doc = "eUSCI_A3"]
pub mod e_usci_a3;
#[doc = "eUSCI_B0"]
pub struct EUSCI_B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_B0 {}
impl EUSCI_B0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_b0::RegisterBlock {
        0x0640 as *const _
    }
}
impl Deref for EUSCI_B0 {
    type Target = e_usci_b0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_B0::ptr() }
    }
}
#[doc = "eUSCI_B0"]
pub mod e_usci_b0;
#[doc = "eUSCI_B1"]
pub struct EUSCI_B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_B1 {}
impl EUSCI_B1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_b1::RegisterBlock {
        0x0680 as *const _
    }
}
impl Deref for EUSCI_B1 {
    type Target = e_usci_b1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_B1::ptr() }
    }
}
#[doc = "eUSCI_B1"]
pub mod e_usci_b1;
#[doc = "eUSCI_B2"]
pub struct EUSCI_B2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_B2 {}
impl EUSCI_B2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_b2::RegisterBlock {
        0x06c0 as *const _
    }
}
impl Deref for EUSCI_B2 {
    type Target = e_usci_b2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_B2::ptr() }
    }
}
#[doc = "eUSCI_B2"]
pub mod e_usci_b2;
#[doc = "eUSCI_B3"]
pub struct EUSCI_B3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EUSCI_B3 {}
impl EUSCI_B3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const e_usci_b3::RegisterBlock {
        0x0700 as *const _
    }
}
impl Deref for EUSCI_B3 {
    type Target = e_usci_b3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EUSCI_B3::ptr() }
    }
}
#[doc = "eUSCI_B3"]
pub mod e_usci_b3;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC12_B"]
    pub ADC12_B: ADC12_B,
    #[doc = "AES256"]
    pub AES256: AES256,
    #[doc = "CAPTIO0"]
    pub CAPTIO0: CAPTIO0,
    #[doc = "CAPTIO1"]
    pub CAPTIO1: CAPTIO1,
    #[doc = "COMP_E"]
    pub COMP_E: COMP_E,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "CRC32"]
    pub CRC32: CRC32,
    #[doc = "CS"]
    pub CS: CS,
    #[doc = "PA"]
    pub PA: PA,
    #[doc = "P1"]
    pub P1: P1,
    #[doc = "P2"]
    pub P2: P2,
    #[doc = "PB"]
    pub PB: PB,
    #[doc = "P3"]
    pub P3: P3,
    #[doc = "P4"]
    pub P4: P4,
    #[doc = "PC"]
    pub PC: PC,
    #[doc = "P5"]
    pub P5: P5,
    #[doc = "P6"]
    pub P6: P6,
    #[doc = "PD"]
    pub PD: PD,
    #[doc = "P7"]
    pub P7: P7,
    #[doc = "P8"]
    pub P8: P8,
    #[doc = "PJ"]
    pub PJ: PJ,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "FRCTL_A"]
    pub FRCTL_A: FRCTL_A,
    #[doc = "LEA"]
    pub LEA: LEA,
    #[doc = "MPU"]
    pub MPU: MPU,
    #[doc = "MPY32"]
    pub MPY32: MPY32,
    #[doc = "PMM"]
    pub PMM: PMM,
    #[doc = "RAMCTL"]
    pub RAMCTL: RAMCTL,
    #[doc = "REF_A"]
    pub REF_A: REF_A,
    #[doc = "RTC_C"]
    pub RTC_C: RTC_C,
    #[doc = "SFR"]
    pub SFR: SFR,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "TA0"]
    pub TA0: TA0,
    #[doc = "TA1"]
    pub TA1: TA1,
    #[doc = "TA2"]
    pub TA2: TA2,
    #[doc = "TA3"]
    pub TA3: TA3,
    #[doc = "TA4"]
    pub TA4: TA4,
    #[doc = "TB0"]
    pub TB0: TB0,
    #[doc = "WDT_A"]
    pub WDT_A: WDT_A,
    #[doc = "EUSCI_A0"]
    pub EUSCI_A0: EUSCI_A0,
    #[doc = "EUSCI_A1"]
    pub EUSCI_A1: EUSCI_A1,
    #[doc = "EUSCI_A2"]
    pub EUSCI_A2: EUSCI_A2,
    #[doc = "EUSCI_A3"]
    pub EUSCI_A3: EUSCI_A3,
    #[doc = "EUSCI_B0"]
    pub EUSCI_B0: EUSCI_B0,
    #[doc = "EUSCI_B1"]
    pub EUSCI_B1: EUSCI_B1,
    #[doc = "EUSCI_B2"]
    pub EUSCI_B2: EUSCI_B2,
    #[doc = "EUSCI_B3"]
    pub EUSCI_B3: EUSCI_B3,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        msp430::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADC12_B: ADC12_B {
                _marker: PhantomData,
            },
            AES256: AES256 {
                _marker: PhantomData,
            },
            CAPTIO0: CAPTIO0 {
                _marker: PhantomData,
            },
            CAPTIO1: CAPTIO1 {
                _marker: PhantomData,
            },
            COMP_E: COMP_E {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            CRC32: CRC32 {
                _marker: PhantomData,
            },
            CS: CS {
                _marker: PhantomData,
            },
            PA: PA {
                _marker: PhantomData,
            },
            P1: P1 {
                _marker: PhantomData,
            },
            P2: P2 {
                _marker: PhantomData,
            },
            PB: PB {
                _marker: PhantomData,
            },
            P3: P3 {
                _marker: PhantomData,
            },
            P4: P4 {
                _marker: PhantomData,
            },
            PC: PC {
                _marker: PhantomData,
            },
            P5: P5 {
                _marker: PhantomData,
            },
            P6: P6 {
                _marker: PhantomData,
            },
            PD: PD {
                _marker: PhantomData,
            },
            P7: P7 {
                _marker: PhantomData,
            },
            P8: P8 {
                _marker: PhantomData,
            },
            PJ: PJ {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            FRCTL_A: FRCTL_A {
                _marker: PhantomData,
            },
            LEA: LEA {
                _marker: PhantomData,
            },
            MPU: MPU {
                _marker: PhantomData,
            },
            MPY32: MPY32 {
                _marker: PhantomData,
            },
            PMM: PMM {
                _marker: PhantomData,
            },
            RAMCTL: RAMCTL {
                _marker: PhantomData,
            },
            REF_A: REF_A {
                _marker: PhantomData,
            },
            RTC_C: RTC_C {
                _marker: PhantomData,
            },
            SFR: SFR {
                _marker: PhantomData,
            },
            SYS: SYS {
                _marker: PhantomData,
            },
            TA0: TA0 {
                _marker: PhantomData,
            },
            TA1: TA1 {
                _marker: PhantomData,
            },
            TA2: TA2 {
                _marker: PhantomData,
            },
            TA3: TA3 {
                _marker: PhantomData,
            },
            TA4: TA4 {
                _marker: PhantomData,
            },
            TB0: TB0 {
                _marker: PhantomData,
            },
            WDT_A: WDT_A {
                _marker: PhantomData,
            },
            EUSCI_A0: EUSCI_A0 {
                _marker: PhantomData,
            },
            EUSCI_A1: EUSCI_A1 {
                _marker: PhantomData,
            },
            EUSCI_A2: EUSCI_A2 {
                _marker: PhantomData,
            },
            EUSCI_A3: EUSCI_A3 {
                _marker: PhantomData,
            },
            EUSCI_B0: EUSCI_B0 {
                _marker: PhantomData,
            },
            EUSCI_B1: EUSCI_B1 {
                _marker: PhantomData,
            },
            EUSCI_B2: EUSCI_B2 {
                _marker: PhantomData,
            },
            EUSCI_B3: EUSCI_B3 {
                _marker: PhantomData,
            },
        }
    }
}
