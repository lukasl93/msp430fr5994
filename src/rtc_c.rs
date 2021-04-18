#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTCCTL0 Register"]
    pub rtcctl0: RTCCTL0,
    #[doc = "0x02 - RTCCTL13 Register"]
    pub rtcctl13: RTCCTL13,
    #[doc = "0x04 - RTCOCAL Register"]
    pub rtcocal: RTCOCAL,
    #[doc = "0x06 - RTCTCMP Register"]
    pub rtctcmp: RTCTCMP,
    #[doc = "0x08 - Real-Time Clock Prescale Timer 0 Control Register"]
    pub rtcps0ctl: RTCPS0CTL,
    #[doc = "0x0a - Real-Time Clock Prescale Timer 1 Control Register"]
    pub rtcps1ctl: RTCPS1CTL,
    _reserved_6_rt0ps: [u8; 2usize],
    #[doc = "0x0e - Real-Time Clock Interrupt Vector Register"]
    pub rtciv: RTCIV,
    _reserved_8_rtccnt1: [u8; 2usize],
    _reserved_9_rtccnt3: [u8; 2usize],
    _reserved_10_rtcdate: [u8; 2usize],
    _reserved_11_rtcyear: [u8; 2usize],
    _reserved_12_rtcaminhr: [u8; 2usize],
    _reserved_13_rtcadowday: [u8; 2usize],
    #[doc = "0x1c - Binary-to-BCD Conversion Register"]
    pub bin2bcd: BIN2BCD,
    #[doc = "0x1e - BCD-to-Binary Conversion Register"]
    pub bcd2bin: BCD2BIN,
}
impl RegisterBlock {
    #[doc = "0x0c - Prescale timer 0 counter value"]
    #[inline(always)]
    pub fn rt0ps(&self) -> &RT0PS {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const RT0PS) }
    }
    #[doc = "0x0c - Prescale timer 0 counter value"]
    #[inline(always)]
    pub fn rt0ps_mut(&self) -> &mut RT0PS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut RT0PS) }
    }
    #[doc = "0x0c - Real-Time Clock Prescale Timer Counter Register"]
    #[inline(always)]
    pub fn rtcps(&self) -> &RTCPS {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const RTCPS) }
    }
    #[doc = "0x0c - Real-Time Clock Prescale Timer Counter Register"]
    #[inline(always)]
    pub fn rtcps_mut(&self) -> &mut RTCPS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut RTCPS) }
    }
    #[doc = "0x0d - Prescale timer 1 counter value"]
    #[inline(always)]
    pub fn rt1ps(&self) -> &RT1PS {
        unsafe { &*(((self as *const Self) as *const u8).add(13usize) as *const RT1PS) }
    }
    #[doc = "0x0d - Prescale timer 1 counter value"]
    #[inline(always)]
    pub fn rt1ps_mut(&self) -> &mut RT1PS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(13usize) as *mut RT1PS) }
    }
    #[doc = "0x10 - The RTCCNT1 register is the count of RTCCNT1"]
    #[inline(always)]
    pub fn rtccnt1(&self) -> &RTCCNT1 {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const RTCCNT1) }
    }
    #[doc = "0x10 - The RTCCNT1 register is the count of RTCCNT1"]
    #[inline(always)]
    pub fn rtccnt1_mut(&self) -> &mut RTCCNT1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut RTCCNT1) }
    }
    #[doc = "0x10 - Real-Time Clock Counter 1 and 2 Register Counter Mode"]
    #[inline(always)]
    pub fn rtccnt12(&self) -> &RTCCNT12 {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const RTCCNT12) }
    }
    #[doc = "0x10 - Real-Time Clock Counter 1 and 2 Register Counter Mode"]
    #[inline(always)]
    pub fn rtccnt12_mut(&self) -> &mut RTCCNT12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut RTCCNT12) }
    }
    #[doc = "0x10 - Real-Time Clock Seconds, Minutes Register - BCD Format"]
    #[inline(always)]
    pub fn rtctim0_bcd(&self) -> &RTCTIM0_BCD {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const RTCTIM0_BCD) }
    }
    #[doc = "0x10 - Real-Time Clock Seconds, Minutes Register - BCD Format"]
    #[inline(always)]
    pub fn rtctim0_bcd_mut(&self) -> &mut RTCTIM0_BCD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut RTCTIM0_BCD) }
    }
    #[doc = "0x10 - RTCTIM0 Register Hexadecimal Format"]
    #[inline(always)]
    pub fn rtctim0(&self) -> &RTCTIM0 {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const RTCTIM0) }
    }
    #[doc = "0x10 - RTCTIM0 Register Hexadecimal Format"]
    #[inline(always)]
    pub fn rtctim0_mut(&self) -> &mut RTCTIM0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut RTCTIM0) }
    }
    #[doc = "0x11 - The RTCCNT2 register is the count of RTCCNT2"]
    #[inline(always)]
    pub fn rtccnt2(&self) -> &RTCCNT2 {
        unsafe { &*(((self as *const Self) as *const u8).add(17usize) as *const RTCCNT2) }
    }
    #[doc = "0x11 - The RTCCNT2 register is the count of RTCCNT2"]
    #[inline(always)]
    pub fn rtccnt2_mut(&self) -> &mut RTCCNT2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(17usize) as *mut RTCCNT2) }
    }
    #[doc = "0x12 - The RTCCNT3 register is the count of RTCCNT3"]
    #[inline(always)]
    pub fn rtccnt3(&self) -> &RTCCNT3 {
        unsafe { &*(((self as *const Self) as *const u8).add(18usize) as *const RTCCNT3) }
    }
    #[doc = "0x12 - The RTCCNT3 register is the count of RTCCNT3"]
    #[inline(always)]
    pub fn rtccnt3_mut(&self) -> &mut RTCCNT3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(18usize) as *mut RTCCNT3) }
    }
    #[doc = "0x12 - Real-Time Clock Counter 3 and 4 Register Counter Mode"]
    #[inline(always)]
    pub fn rtccnt34(&self) -> &RTCCNT34 {
        unsafe { &*(((self as *const Self) as *const u8).add(18usize) as *const RTCCNT34) }
    }
    #[doc = "0x12 - Real-Time Clock Counter 3 and 4 Register Counter Mode"]
    #[inline(always)]
    pub fn rtccnt34_mut(&self) -> &mut RTCCNT34 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(18usize) as *mut RTCCNT34) }
    }
    #[doc = "0x12 - Real-Time Clock Hour, Day of Week - BCD Format"]
    #[inline(always)]
    pub fn rtctim1_bcd(&self) -> &RTCTIM1_BCD {
        unsafe { &*(((self as *const Self) as *const u8).add(18usize) as *const RTCTIM1_BCD) }
    }
    #[doc = "0x12 - Real-Time Clock Hour, Day of Week - BCD Format"]
    #[inline(always)]
    pub fn rtctim1_bcd_mut(&self) -> &mut RTCTIM1_BCD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(18usize) as *mut RTCTIM1_BCD) }
    }
    #[doc = "0x12 - Real-Time Clock Hour, Day of Week"]
    #[inline(always)]
    pub fn rtctim1(&self) -> &RTCTIM1 {
        unsafe { &*(((self as *const Self) as *const u8).add(18usize) as *const RTCTIM1) }
    }
    #[doc = "0x12 - Real-Time Clock Hour, Day of Week"]
    #[inline(always)]
    pub fn rtctim1_mut(&self) -> &mut RTCTIM1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(18usize) as *mut RTCTIM1) }
    }
    #[doc = "0x13 - The RTCCNT4 register is the count of RTCCNT4"]
    #[inline(always)]
    pub fn rtccnt4(&self) -> &RTCCNT4 {
        unsafe { &*(((self as *const Self) as *const u8).add(19usize) as *const RTCCNT4) }
    }
    #[doc = "0x13 - The RTCCNT4 register is the count of RTCCNT4"]
    #[inline(always)]
    pub fn rtccnt4_mut(&self) -> &mut RTCCNT4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(19usize) as *mut RTCCNT4) }
    }
    #[doc = "0x14 - Real-Time Clock Date - BCD Format"]
    #[inline(always)]
    pub fn rtcdate_bcd(&self) -> &RTCDATE_BCD {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const RTCDATE_BCD) }
    }
    #[doc = "0x14 - Real-Time Clock Date - BCD Format"]
    #[inline(always)]
    pub fn rtcdate_bcd_mut(&self) -> &mut RTCDATE_BCD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut RTCDATE_BCD) }
    }
    #[doc = "0x14 - RTCDATE - Hexadecimal Format"]
    #[inline(always)]
    pub fn rtcdate(&self) -> &RTCDATE {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const RTCDATE) }
    }
    #[doc = "0x14 - RTCDATE - Hexadecimal Format"]
    #[inline(always)]
    pub fn rtcdate_mut(&self) -> &mut RTCDATE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut RTCDATE) }
    }
    #[doc = "0x16 - Real-Time Clock Year Register - BCD Format"]
    #[inline(always)]
    pub fn rtcyear_bcd(&self) -> &RTCYEAR_BCD {
        unsafe { &*(((self as *const Self) as *const u8).add(22usize) as *const RTCYEAR_BCD) }
    }
    #[doc = "0x16 - Real-Time Clock Year Register - BCD Format"]
    #[inline(always)]
    pub fn rtcyear_bcd_mut(&self) -> &mut RTCYEAR_BCD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(22usize) as *mut RTCYEAR_BCD) }
    }
    #[doc = "0x16 - RTCYEAR Register Hexadecimal Format"]
    #[inline(always)]
    pub fn rtcyear(&self) -> &RTCYEAR {
        unsafe { &*(((self as *const Self) as *const u8).add(22usize) as *const RTCYEAR) }
    }
    #[doc = "0x16 - RTCYEAR Register Hexadecimal Format"]
    #[inline(always)]
    pub fn rtcyear_mut(&self) -> &mut RTCYEAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(22usize) as *mut RTCYEAR) }
    }
    #[doc = "0x18 - Real-Time Clock Minutes, Hour Alarm - BCD Format"]
    #[inline(always)]
    pub fn rtcaminhr_bcd(&self) -> &RTCAMINHR_BCD {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const RTCAMINHR_BCD) }
    }
    #[doc = "0x18 - Real-Time Clock Minutes, Hour Alarm - BCD Format"]
    #[inline(always)]
    pub fn rtcaminhr_bcd_mut(&self) -> &mut RTCAMINHR_BCD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut RTCAMINHR_BCD) }
    }
    #[doc = "0x18 - RTCMINHR - Hexadecimal Format"]
    #[inline(always)]
    pub fn rtcaminhr(&self) -> &RTCAMINHR {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const RTCAMINHR) }
    }
    #[doc = "0x18 - RTCMINHR - Hexadecimal Format"]
    #[inline(always)]
    pub fn rtcaminhr_mut(&self) -> &mut RTCAMINHR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut RTCAMINHR) }
    }
    #[doc = "0x1a - Real-Time Clock Day of Week, Day of Month Alarm - BCD Format"]
    #[inline(always)]
    pub fn rtcadowday_bcd(&self) -> &RTCADOWDAY_BCD {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const RTCADOWDAY_BCD) }
    }
    #[doc = "0x1a - Real-Time Clock Day of Week, Day of Month Alarm - BCD Format"]
    #[inline(always)]
    pub fn rtcadowday_bcd_mut(&self) -> &mut RTCADOWDAY_BCD {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut RTCADOWDAY_BCD) }
    }
    #[doc = "0x1a - RTCADOWDAY - Hexadecimal Format"]
    #[inline(always)]
    pub fn rtcadowday(&self) -> &RTCADOWDAY {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const RTCADOWDAY) }
    }
    #[doc = "0x1a - RTCADOWDAY - Hexadecimal Format"]
    #[inline(always)]
    pub fn rtcadowday_mut(&self) -> &mut RTCADOWDAY {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut RTCADOWDAY) }
    }
}
#[doc = "RTCCTL0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl0](rtcctl0) module"]
pub type RTCCTL0 = crate::Reg<u16, _RTCCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCTL0;
#[doc = "`read()` method returns [rtcctl0::R](rtcctl0::R) reader structure"]
impl crate::Readable for RTCCTL0 {}
#[doc = "`write(|w| ..)` method takes [rtcctl0::W](rtcctl0::W) writer structure"]
impl crate::Writable for RTCCTL0 {}
#[doc = "RTCCTL0 Register"]
pub mod rtcctl0;
#[doc = "RTCCTL13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl13](rtcctl13) module"]
pub type RTCCTL13 = crate::Reg<u16, _RTCCTL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCTL13;
#[doc = "`read()` method returns [rtcctl13::R](rtcctl13::R) reader structure"]
impl crate::Readable for RTCCTL13 {}
#[doc = "`write(|w| ..)` method takes [rtcctl13::W](rtcctl13::W) writer structure"]
impl crate::Writable for RTCCTL13 {}
#[doc = "RTCCTL13 Register"]
pub mod rtcctl13;
#[doc = "RTCOCAL Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcocal](rtcocal) module"]
pub type RTCOCAL = crate::Reg<u16, _RTCOCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCOCAL;
#[doc = "`read()` method returns [rtcocal::R](rtcocal::R) reader structure"]
impl crate::Readable for RTCOCAL {}
#[doc = "`write(|w| ..)` method takes [rtcocal::W](rtcocal::W) writer structure"]
impl crate::Writable for RTCOCAL {}
#[doc = "RTCOCAL Register"]
pub mod rtcocal;
#[doc = "RTCTCMP Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctcmp](rtctcmp) module"]
pub type RTCTCMP = crate::Reg<u16, _RTCTCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTCMP;
#[doc = "`read()` method returns [rtctcmp::R](rtctcmp::R) reader structure"]
impl crate::Readable for RTCTCMP {}
#[doc = "`write(|w| ..)` method takes [rtctcmp::W](rtctcmp::W) writer structure"]
impl crate::Writable for RTCTCMP {}
#[doc = "RTCTCMP Register"]
pub mod rtctcmp;
#[doc = "Real-Time Clock Prescale Timer 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps0ctl](rtcps0ctl) module"]
pub type RTCPS0CTL = crate::Reg<u16, _RTCPS0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPS0CTL;
#[doc = "`read()` method returns [rtcps0ctl::R](rtcps0ctl::R) reader structure"]
impl crate::Readable for RTCPS0CTL {}
#[doc = "`write(|w| ..)` method takes [rtcps0ctl::W](rtcps0ctl::W) writer structure"]
impl crate::Writable for RTCPS0CTL {}
#[doc = "Real-Time Clock Prescale Timer 0 Control Register"]
pub mod rtcps0ctl;
#[doc = "Real-Time Clock Prescale Timer 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps1ctl](rtcps1ctl) module"]
pub type RTCPS1CTL = crate::Reg<u16, _RTCPS1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPS1CTL;
#[doc = "`read()` method returns [rtcps1ctl::R](rtcps1ctl::R) reader structure"]
impl crate::Readable for RTCPS1CTL {}
#[doc = "`write(|w| ..)` method takes [rtcps1ctl::W](rtcps1ctl::W) writer structure"]
impl crate::Writable for RTCPS1CTL {}
#[doc = "Real-Time Clock Prescale Timer 1 Control Register"]
pub mod rtcps1ctl;
#[doc = "Real-Time Clock Prescale Timer Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps](rtcps) module"]
pub type RTCPS = crate::Reg<u16, _RTCPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCPS;
#[doc = "`read()` method returns [rtcps::R](rtcps::R) reader structure"]
impl crate::Readable for RTCPS {}
#[doc = "`write(|w| ..)` method takes [rtcps::W](rtcps::W) writer structure"]
impl crate::Writable for RTCPS {}
#[doc = "Real-Time Clock Prescale Timer Counter Register"]
pub mod rtcps;
#[doc = "Real-Time Clock Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtciv](rtciv) module"]
pub type RTCIV = crate::Reg<u16, _RTCIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCIV;
#[doc = "`read()` method returns [rtciv::R](rtciv::R) reader structure"]
impl crate::Readable for RTCIV {}
#[doc = "`write(|w| ..)` method takes [rtciv::W](rtciv::W) writer structure"]
impl crate::Writable for RTCIV {}
#[doc = "Real-Time Clock Interrupt Vector Register"]
pub mod rtciv;
#[doc = "RTCTIM0 Register Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctim0](rtctim0) module"]
pub type RTCTIM0 = crate::Reg<u16, _RTCTIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIM0;
#[doc = "`read()` method returns [rtctim0::R](rtctim0::R) reader structure"]
impl crate::Readable for RTCTIM0 {}
#[doc = "`write(|w| ..)` method takes [rtctim0::W](rtctim0::W) writer structure"]
impl crate::Writable for RTCTIM0 {}
#[doc = "RTCTIM0 Register Hexadecimal Format"]
pub mod rtctim0;
#[doc = "Real-Time Clock Seconds, Minutes Register - BCD Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctim0_bcd](rtctim0_bcd) module"]
pub type RTCTIM0_BCD = crate::Reg<u16, _RTCTIM0_BCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIM0_BCD;
#[doc = "`read()` method returns [rtctim0_bcd::R](rtctim0_bcd::R) reader structure"]
impl crate::Readable for RTCTIM0_BCD {}
#[doc = "`write(|w| ..)` method takes [rtctim0_bcd::W](rtctim0_bcd::W) writer structure"]
impl crate::Writable for RTCTIM0_BCD {}
#[doc = "Real-Time Clock Seconds, Minutes Register - BCD Format"]
pub mod rtctim0_bcd;
#[doc = "Real-Time Clock Counter 1 and 2 Register Counter Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccnt12](rtccnt12) module"]
pub type RTCCNT12 = crate::Reg<u16, _RTCCNT12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCNT12;
#[doc = "`read()` method returns [rtccnt12::R](rtccnt12::R) reader structure"]
impl crate::Readable for RTCCNT12 {}
#[doc = "`write(|w| ..)` method takes [rtccnt12::W](rtccnt12::W) writer structure"]
impl crate::Writable for RTCCNT12 {}
#[doc = "Real-Time Clock Counter 1 and 2 Register Counter Mode"]
pub mod rtccnt12;
#[doc = "Real-Time Clock Hour, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctim1](rtctim1) module"]
pub type RTCTIM1 = crate::Reg<u16, _RTCTIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIM1;
#[doc = "`read()` method returns [rtctim1::R](rtctim1::R) reader structure"]
impl crate::Readable for RTCTIM1 {}
#[doc = "`write(|w| ..)` method takes [rtctim1::W](rtctim1::W) writer structure"]
impl crate::Writable for RTCTIM1 {}
#[doc = "Real-Time Clock Hour, Day of Week"]
pub mod rtctim1;
#[doc = "Real-Time Clock Hour, Day of Week - BCD Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctim1_bcd](rtctim1_bcd) module"]
pub type RTCTIM1_BCD = crate::Reg<u16, _RTCTIM1_BCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIM1_BCD;
#[doc = "`read()` method returns [rtctim1_bcd::R](rtctim1_bcd::R) reader structure"]
impl crate::Readable for RTCTIM1_BCD {}
#[doc = "`write(|w| ..)` method takes [rtctim1_bcd::W](rtctim1_bcd::W) writer structure"]
impl crate::Writable for RTCTIM1_BCD {}
#[doc = "Real-Time Clock Hour, Day of Week - BCD Format"]
pub mod rtctim1_bcd;
#[doc = "Real-Time Clock Counter 3 and 4 Register Counter Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccnt34](rtccnt34) module"]
pub type RTCCNT34 = crate::Reg<u16, _RTCCNT34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCNT34;
#[doc = "`read()` method returns [rtccnt34::R](rtccnt34::R) reader structure"]
impl crate::Readable for RTCCNT34 {}
#[doc = "`write(|w| ..)` method takes [rtccnt34::W](rtccnt34::W) writer structure"]
impl crate::Writable for RTCCNT34 {}
#[doc = "Real-Time Clock Counter 3 and 4 Register Counter Mode"]
pub mod rtccnt34;
#[doc = "RTCDATE - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdate](rtcdate) module"]
pub type RTCDATE = crate::Reg<u16, _RTCDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCDATE;
#[doc = "`read()` method returns [rtcdate::R](rtcdate::R) reader structure"]
impl crate::Readable for RTCDATE {}
#[doc = "`write(|w| ..)` method takes [rtcdate::W](rtcdate::W) writer structure"]
impl crate::Writable for RTCDATE {}
#[doc = "RTCDATE - Hexadecimal Format"]
pub mod rtcdate;
#[doc = "Real-Time Clock Date - BCD Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdate_bcd](rtcdate_bcd) module"]
pub type RTCDATE_BCD = crate::Reg<u16, _RTCDATE_BCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCDATE_BCD;
#[doc = "`read()` method returns [rtcdate_bcd::R](rtcdate_bcd::R) reader structure"]
impl crate::Readable for RTCDATE_BCD {}
#[doc = "`write(|w| ..)` method takes [rtcdate_bcd::W](rtcdate_bcd::W) writer structure"]
impl crate::Writable for RTCDATE_BCD {}
#[doc = "Real-Time Clock Date - BCD Format"]
pub mod rtcdate_bcd;
#[doc = "RTCYEAR Register Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcyear](rtcyear) module"]
pub type RTCYEAR = crate::Reg<u16, _RTCYEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCYEAR;
#[doc = "`read()` method returns [rtcyear::R](rtcyear::R) reader structure"]
impl crate::Readable for RTCYEAR {}
#[doc = "`write(|w| ..)` method takes [rtcyear::W](rtcyear::W) writer structure"]
impl crate::Writable for RTCYEAR {}
#[doc = "RTCYEAR Register Hexadecimal Format"]
pub mod rtcyear;
#[doc = "Real-Time Clock Year Register - BCD Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcyear_bcd](rtcyear_bcd) module"]
pub type RTCYEAR_BCD = crate::Reg<u16, _RTCYEAR_BCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCYEAR_BCD;
#[doc = "`read()` method returns [rtcyear_bcd::R](rtcyear_bcd::R) reader structure"]
impl crate::Readable for RTCYEAR_BCD {}
#[doc = "`write(|w| ..)` method takes [rtcyear_bcd::W](rtcyear_bcd::W) writer structure"]
impl crate::Writable for RTCYEAR_BCD {}
#[doc = "Real-Time Clock Year Register - BCD Format"]
pub mod rtcyear_bcd;
#[doc = "RTCMINHR - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcaminhr](rtcaminhr) module"]
pub type RTCAMINHR = crate::Reg<u16, _RTCAMINHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCAMINHR;
#[doc = "`read()` method returns [rtcaminhr::R](rtcaminhr::R) reader structure"]
impl crate::Readable for RTCAMINHR {}
#[doc = "`write(|w| ..)` method takes [rtcaminhr::W](rtcaminhr::W) writer structure"]
impl crate::Writable for RTCAMINHR {}
#[doc = "RTCMINHR - Hexadecimal Format"]
pub mod rtcaminhr;
#[doc = "Real-Time Clock Minutes, Hour Alarm - BCD Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcaminhr_bcd](rtcaminhr_bcd) module"]
pub type RTCAMINHR_BCD = crate::Reg<u16, _RTCAMINHR_BCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCAMINHR_BCD;
#[doc = "`read()` method returns [rtcaminhr_bcd::R](rtcaminhr_bcd::R) reader structure"]
impl crate::Readable for RTCAMINHR_BCD {}
#[doc = "`write(|w| ..)` method takes [rtcaminhr_bcd::W](rtcaminhr_bcd::W) writer structure"]
impl crate::Writable for RTCAMINHR_BCD {}
#[doc = "Real-Time Clock Minutes, Hour Alarm - BCD Format"]
pub mod rtcaminhr_bcd;
#[doc = "RTCADOWDAY - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcadowday](rtcadowday) module"]
pub type RTCADOWDAY = crate::Reg<u16, _RTCADOWDAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCADOWDAY;
#[doc = "`read()` method returns [rtcadowday::R](rtcadowday::R) reader structure"]
impl crate::Readable for RTCADOWDAY {}
#[doc = "`write(|w| ..)` method takes [rtcadowday::W](rtcadowday::W) writer structure"]
impl crate::Writable for RTCADOWDAY {}
#[doc = "RTCADOWDAY - Hexadecimal Format"]
pub mod rtcadowday;
#[doc = "Real-Time Clock Day of Week, Day of Month Alarm - BCD Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcadowday_bcd](rtcadowday_bcd) module"]
pub type RTCADOWDAY_BCD = crate::Reg<u16, _RTCADOWDAY_BCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCADOWDAY_BCD;
#[doc = "`read()` method returns [rtcadowday_bcd::R](rtcadowday_bcd::R) reader structure"]
impl crate::Readable for RTCADOWDAY_BCD {}
#[doc = "`write(|w| ..)` method takes [rtcadowday_bcd::W](rtcadowday_bcd::W) writer structure"]
impl crate::Writable for RTCADOWDAY_BCD {}
#[doc = "Real-Time Clock Day of Week, Day of Month Alarm - BCD Format"]
pub mod rtcadowday_bcd;
#[doc = "Binary-to-BCD Conversion Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bin2bcd](bin2bcd) module"]
pub type BIN2BCD = crate::Reg<u16, _BIN2BCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIN2BCD;
#[doc = "`read()` method returns [bin2bcd::R](bin2bcd::R) reader structure"]
impl crate::Readable for BIN2BCD {}
#[doc = "`write(|w| ..)` method takes [bin2bcd::W](bin2bcd::W) writer structure"]
impl crate::Writable for BIN2BCD {}
#[doc = "Binary-to-BCD Conversion Register"]
pub mod bin2bcd;
#[doc = "BCD-to-Binary Conversion Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcd2bin](bcd2bin) module"]
pub type BCD2BIN = crate::Reg<u16, _BCD2BIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCD2BIN;
#[doc = "`read()` method returns [bcd2bin::R](bcd2bin::R) reader structure"]
impl crate::Readable for BCD2BIN {}
#[doc = "`write(|w| ..)` method takes [bcd2bin::W](bcd2bin::W) writer structure"]
impl crate::Writable for BCD2BIN {}
#[doc = "BCD-to-Binary Conversion Register"]
pub mod bcd2bin;
#[doc = "Prescale timer 0 counter value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rt0ps](rt0ps) module"]
pub type RT0PS = crate::Reg<u8, _RT0PS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RT0PS;
#[doc = "`read()` method returns [rt0ps::R](rt0ps::R) reader structure"]
impl crate::Readable for RT0PS {}
#[doc = "`write(|w| ..)` method takes [rt0ps::W](rt0ps::W) writer structure"]
impl crate::Writable for RT0PS {}
#[doc = "Prescale timer 0 counter value"]
pub mod rt0ps;
#[doc = "Prescale timer 1 counter value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rt1ps](rt1ps) module"]
pub type RT1PS = crate::Reg<u8, _RT1PS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RT1PS;
#[doc = "`read()` method returns [rt1ps::R](rt1ps::R) reader structure"]
impl crate::Readable for RT1PS {}
#[doc = "`write(|w| ..)` method takes [rt1ps::W](rt1ps::W) writer structure"]
impl crate::Writable for RT1PS {}
#[doc = "Prescale timer 1 counter value"]
pub mod rt1ps;
#[doc = "The RTCCNT1 register is the count of RTCCNT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccnt1](rtccnt1) module"]
pub type RTCCNT1 = crate::Reg<u8, _RTCCNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCNT1;
#[doc = "`read()` method returns [rtccnt1::R](rtccnt1::R) reader structure"]
impl crate::Readable for RTCCNT1 {}
#[doc = "`write(|w| ..)` method takes [rtccnt1::W](rtccnt1::W) writer structure"]
impl crate::Writable for RTCCNT1 {}
#[doc = "The RTCCNT1 register is the count of RTCCNT1"]
pub mod rtccnt1;
#[doc = "The RTCCNT2 register is the count of RTCCNT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccnt2](rtccnt2) module"]
pub type RTCCNT2 = crate::Reg<u8, _RTCCNT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCNT2;
#[doc = "`read()` method returns [rtccnt2::R](rtccnt2::R) reader structure"]
impl crate::Readable for RTCCNT2 {}
#[doc = "`write(|w| ..)` method takes [rtccnt2::W](rtccnt2::W) writer structure"]
impl crate::Writable for RTCCNT2 {}
#[doc = "The RTCCNT2 register is the count of RTCCNT2"]
pub mod rtccnt2;
#[doc = "The RTCCNT3 register is the count of RTCCNT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccnt3](rtccnt3) module"]
pub type RTCCNT3 = crate::Reg<u8, _RTCCNT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCNT3;
#[doc = "`read()` method returns [rtccnt3::R](rtccnt3::R) reader structure"]
impl crate::Readable for RTCCNT3 {}
#[doc = "`write(|w| ..)` method takes [rtccnt3::W](rtccnt3::W) writer structure"]
impl crate::Writable for RTCCNT3 {}
#[doc = "The RTCCNT3 register is the count of RTCCNT3"]
pub mod rtccnt3;
#[doc = "The RTCCNT4 register is the count of RTCCNT4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccnt4](rtccnt4) module"]
pub type RTCCNT4 = crate::Reg<u8, _RTCCNT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCNT4;
#[doc = "`read()` method returns [rtccnt4::R](rtccnt4::R) reader structure"]
impl crate::Readable for RTCCNT4 {}
#[doc = "`write(|w| ..)` method takes [rtccnt4::W](rtccnt4::W) writer structure"]
impl crate::Writable for RTCCNT4 {}
#[doc = "The RTCCNT4 register is the count of RTCCNT4"]
pub mod rtccnt4;
