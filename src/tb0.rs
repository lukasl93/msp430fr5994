#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    pub tb0ctl: TB0CTL,
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl0: TB0CCTL0,
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl1: TB0CCTL1,
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl2: TB0CCTL2,
    #[doc = "0x08 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl3: TB0CCTL3,
    #[doc = "0x0a - Timer_B Capture/Compare Control Register"]
    pub tb0cctl4: TB0CCTL4,
    #[doc = "0x0c - Timer_B Capture/Compare Control Register"]
    pub tb0cctl5: TB0CCTL5,
    #[doc = "0x0e - Timer_B Capture/Compare Control Register"]
    pub tb0cctl6: TB0CCTL6,
    #[doc = "0x10 - Timer_B count register"]
    pub tb0r: TB0R,
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    pub tb0ccr0: TB0CCR0,
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    pub tb0ccr1: TB0CCR1,
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb0ccr2: TB0CCR2,
    #[doc = "0x18 - Timer_B Capture/Compare Register"]
    pub tb0ccr3: TB0CCR3,
    #[doc = "0x1a - Timer_B Capture/Compare Register"]
    pub tb0ccr4: TB0CCR4,
    #[doc = "0x1c - Timer_B Capture/Compare Register"]
    pub tb0ccr5: TB0CCR5,
    #[doc = "0x1e - Timer_B Capture/Compare Register"]
    pub tb0ccr6: TB0CCR6,
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    pub tb0ex0: TB0EX0,
    _reserved17: [u8; 12usize],
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    pub tb0iv: TB0IV,
}
#[doc = "Timer_B Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ctl](tb0ctl) module"]
pub type TB0CTL = crate::Reg<u16, _TB0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CTL;
#[doc = "`read()` method returns [tb0ctl::R](tb0ctl::R) reader structure"]
impl crate::Readable for TB0CTL {}
#[doc = "`write(|w| ..)` method takes [tb0ctl::W](tb0ctl::W) writer structure"]
impl crate::Writable for TB0CTL {}
#[doc = "Timer_B Control Register"]
pub mod tb0ctl;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl0](tb0cctl0) module"]
pub type TB0CCTL0 = crate::Reg<u16, _TB0CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL0;
#[doc = "`read()` method returns [tb0cctl0::R](tb0cctl0::R) reader structure"]
impl crate::Readable for TB0CCTL0 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl0::W](tb0cctl0::W) writer structure"]
impl crate::Writable for TB0CCTL0 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl0;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl1](tb0cctl1) module"]
pub type TB0CCTL1 = crate::Reg<u16, _TB0CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL1;
#[doc = "`read()` method returns [tb0cctl1::R](tb0cctl1::R) reader structure"]
impl crate::Readable for TB0CCTL1 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl1::W](tb0cctl1::W) writer structure"]
impl crate::Writable for TB0CCTL1 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl1;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl2](tb0cctl2) module"]
pub type TB0CCTL2 = crate::Reg<u16, _TB0CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL2;
#[doc = "`read()` method returns [tb0cctl2::R](tb0cctl2::R) reader structure"]
impl crate::Readable for TB0CCTL2 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl2::W](tb0cctl2::W) writer structure"]
impl crate::Writable for TB0CCTL2 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl2;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl3](tb0cctl3) module"]
pub type TB0CCTL3 = crate::Reg<u16, _TB0CCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL3;
#[doc = "`read()` method returns [tb0cctl3::R](tb0cctl3::R) reader structure"]
impl crate::Readable for TB0CCTL3 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl3::W](tb0cctl3::W) writer structure"]
impl crate::Writable for TB0CCTL3 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl3;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl4](tb0cctl4) module"]
pub type TB0CCTL4 = crate::Reg<u16, _TB0CCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL4;
#[doc = "`read()` method returns [tb0cctl4::R](tb0cctl4::R) reader structure"]
impl crate::Readable for TB0CCTL4 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl4::W](tb0cctl4::W) writer structure"]
impl crate::Writable for TB0CCTL4 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl4;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl5](tb0cctl5) module"]
pub type TB0CCTL5 = crate::Reg<u16, _TB0CCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL5;
#[doc = "`read()` method returns [tb0cctl5::R](tb0cctl5::R) reader structure"]
impl crate::Readable for TB0CCTL5 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl5::W](tb0cctl5::W) writer structure"]
impl crate::Writable for TB0CCTL5 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl5;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl6](tb0cctl6) module"]
pub type TB0CCTL6 = crate::Reg<u16, _TB0CCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL6;
#[doc = "`read()` method returns [tb0cctl6::R](tb0cctl6::R) reader structure"]
impl crate::Readable for TB0CCTL6 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl6::W](tb0cctl6::W) writer structure"]
impl crate::Writable for TB0CCTL6 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl6;
#[doc = "Timer_B count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0r](tb0r) module"]
pub type TB0R = crate::Reg<u16, _TB0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0R;
#[doc = "`read()` method returns [tb0r::R](tb0r::R) reader structure"]
impl crate::Readable for TB0R {}
#[doc = "`write(|w| ..)` method takes [tb0r::W](tb0r::W) writer structure"]
impl crate::Writable for TB0R {}
#[doc = "Timer_B count register"]
pub mod tb0r;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr0](tb0ccr0) module"]
pub type TB0CCR0 = crate::Reg<u16, _TB0CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR0;
#[doc = "`read()` method returns [tb0ccr0::R](tb0ccr0::R) reader structure"]
impl crate::Readable for TB0CCR0 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr0::W](tb0ccr0::W) writer structure"]
impl crate::Writable for TB0CCR0 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr0;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr1](tb0ccr1) module"]
pub type TB0CCR1 = crate::Reg<u16, _TB0CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR1;
#[doc = "`read()` method returns [tb0ccr1::R](tb0ccr1::R) reader structure"]
impl crate::Readable for TB0CCR1 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr1::W](tb0ccr1::W) writer structure"]
impl crate::Writable for TB0CCR1 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr1;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr2](tb0ccr2) module"]
pub type TB0CCR2 = crate::Reg<u16, _TB0CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR2;
#[doc = "`read()` method returns [tb0ccr2::R](tb0ccr2::R) reader structure"]
impl crate::Readable for TB0CCR2 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr2::W](tb0ccr2::W) writer structure"]
impl crate::Writable for TB0CCR2 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr2;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr3](tb0ccr3) module"]
pub type TB0CCR3 = crate::Reg<u16, _TB0CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR3;
#[doc = "`read()` method returns [tb0ccr3::R](tb0ccr3::R) reader structure"]
impl crate::Readable for TB0CCR3 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr3::W](tb0ccr3::W) writer structure"]
impl crate::Writable for TB0CCR3 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr3;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr4](tb0ccr4) module"]
pub type TB0CCR4 = crate::Reg<u16, _TB0CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR4;
#[doc = "`read()` method returns [tb0ccr4::R](tb0ccr4::R) reader structure"]
impl crate::Readable for TB0CCR4 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr4::W](tb0ccr4::W) writer structure"]
impl crate::Writable for TB0CCR4 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr4;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr5](tb0ccr5) module"]
pub type TB0CCR5 = crate::Reg<u16, _TB0CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR5;
#[doc = "`read()` method returns [tb0ccr5::R](tb0ccr5::R) reader structure"]
impl crate::Readable for TB0CCR5 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr5::W](tb0ccr5::W) writer structure"]
impl crate::Writable for TB0CCR5 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr5;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr6](tb0ccr6) module"]
pub type TB0CCR6 = crate::Reg<u16, _TB0CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR6;
#[doc = "`read()` method returns [tb0ccr6::R](tb0ccr6::R) reader structure"]
impl crate::Readable for TB0CCR6 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr6::W](tb0ccr6::W) writer structure"]
impl crate::Writable for TB0CCR6 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr6;
#[doc = "Timer_Bx Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ex0](tb0ex0) module"]
pub type TB0EX0 = crate::Reg<u16, _TB0EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0EX0;
#[doc = "`read()` method returns [tb0ex0::R](tb0ex0::R) reader structure"]
impl crate::Readable for TB0EX0 {}
#[doc = "`write(|w| ..)` method takes [tb0ex0::W](tb0ex0::W) writer structure"]
impl crate::Writable for TB0EX0 {}
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb0ex0;
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0iv](tb0iv) module"]
pub type TB0IV = crate::Reg<u16, _TB0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0IV;
#[doc = "`read()` method returns [tb0iv::R](tb0iv::R) reader structure"]
impl crate::Readable for TB0IV {}
#[doc = "`write(|w| ..)` method takes [tb0iv::W](tb0iv::W) writer structure"]
impl crate::Writable for TB0IV {}
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb0iv;
