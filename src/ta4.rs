#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    pub ta4ctl: TA4CTL,
    #[doc = "0x02 - Timer_A Capture/Compare Control Register"]
    pub ta4cctl0: TA4CCTL0,
    #[doc = "0x04 - Timer_A Capture/Compare Control Register"]
    pub ta4cctl1: TA4CCTL1,
    #[doc = "0x06 - Timer_A Capture/Compare Control Register"]
    pub ta4cctl2: TA4CCTL2,
    _reserved4: [u8; 8usize],
    #[doc = "0x10 - TimerA register"]
    pub ta4r: TA4R,
    #[doc = "0x12 - Timer_A Capture/Compare Register"]
    pub ta4ccr0: TA4CCR0,
    #[doc = "0x14 - Timer_A Capture/Compare Register"]
    pub ta4ccr1: TA4CCR1,
    #[doc = "0x16 - Timer_A Capture/Compare Register"]
    pub ta4ccr2: TA4CCR2,
    _reserved8: [u8; 8usize],
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    pub ta4ex0: TA4EX0,
    _reserved9: [u8; 12usize],
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    pub ta4iv: TA4IV,
}
#[doc = "TimerAx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4ctl](ta4ctl) module"]
pub type TA4CTL = crate::Reg<u16, _TA4CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4CTL;
#[doc = "`read()` method returns [ta4ctl::R](ta4ctl::R) reader structure"]
impl crate::Readable for TA4CTL {}
#[doc = "`write(|w| ..)` method takes [ta4ctl::W](ta4ctl::W) writer structure"]
impl crate::Writable for TA4CTL {}
#[doc = "TimerAx Control Register"]
pub mod ta4ctl;
#[doc = "Timer_A Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4cctl0](ta4cctl0) module"]
pub type TA4CCTL0 = crate::Reg<u16, _TA4CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4CCTL0;
#[doc = "`read()` method returns [ta4cctl0::R](ta4cctl0::R) reader structure"]
impl crate::Readable for TA4CCTL0 {}
#[doc = "`write(|w| ..)` method takes [ta4cctl0::W](ta4cctl0::W) writer structure"]
impl crate::Writable for TA4CCTL0 {}
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta4cctl0;
#[doc = "Timer_A Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4cctl1](ta4cctl1) module"]
pub type TA4CCTL1 = crate::Reg<u16, _TA4CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4CCTL1;
#[doc = "`read()` method returns [ta4cctl1::R](ta4cctl1::R) reader structure"]
impl crate::Readable for TA4CCTL1 {}
#[doc = "`write(|w| ..)` method takes [ta4cctl1::W](ta4cctl1::W) writer structure"]
impl crate::Writable for TA4CCTL1 {}
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta4cctl1;
#[doc = "Timer_A Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4cctl2](ta4cctl2) module"]
pub type TA4CCTL2 = crate::Reg<u16, _TA4CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4CCTL2;
#[doc = "`read()` method returns [ta4cctl2::R](ta4cctl2::R) reader structure"]
impl crate::Readable for TA4CCTL2 {}
#[doc = "`write(|w| ..)` method takes [ta4cctl2::W](ta4cctl2::W) writer structure"]
impl crate::Writable for TA4CCTL2 {}
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta4cctl2;
#[doc = "TimerA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4r](ta4r) module"]
pub type TA4R = crate::Reg<u16, _TA4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4R;
#[doc = "`read()` method returns [ta4r::R](ta4r::R) reader structure"]
impl crate::Readable for TA4R {}
#[doc = "`write(|w| ..)` method takes [ta4r::W](ta4r::W) writer structure"]
impl crate::Writable for TA4R {}
#[doc = "TimerA register"]
pub mod ta4r;
#[doc = "Timer_A Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4ccr0](ta4ccr0) module"]
pub type TA4CCR0 = crate::Reg<u16, _TA4CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4CCR0;
#[doc = "`read()` method returns [ta4ccr0::R](ta4ccr0::R) reader structure"]
impl crate::Readable for TA4CCR0 {}
#[doc = "`write(|w| ..)` method takes [ta4ccr0::W](ta4ccr0::W) writer structure"]
impl crate::Writable for TA4CCR0 {}
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta4ccr0;
#[doc = "Timer_A Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4ccr1](ta4ccr1) module"]
pub type TA4CCR1 = crate::Reg<u16, _TA4CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4CCR1;
#[doc = "`read()` method returns [ta4ccr1::R](ta4ccr1::R) reader structure"]
impl crate::Readable for TA4CCR1 {}
#[doc = "`write(|w| ..)` method takes [ta4ccr1::W](ta4ccr1::W) writer structure"]
impl crate::Writable for TA4CCR1 {}
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta4ccr1;
#[doc = "Timer_A Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4ccr2](ta4ccr2) module"]
pub type TA4CCR2 = crate::Reg<u16, _TA4CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4CCR2;
#[doc = "`read()` method returns [ta4ccr2::R](ta4ccr2::R) reader structure"]
impl crate::Readable for TA4CCR2 {}
#[doc = "`write(|w| ..)` method takes [ta4ccr2::W](ta4ccr2::W) writer structure"]
impl crate::Writable for TA4CCR2 {}
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta4ccr2;
#[doc = "TimerAx Expansion 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4ex0](ta4ex0) module"]
pub type TA4EX0 = crate::Reg<u16, _TA4EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4EX0;
#[doc = "`read()` method returns [ta4ex0::R](ta4ex0::R) reader structure"]
impl crate::Readable for TA4EX0 {}
#[doc = "`write(|w| ..)` method takes [ta4ex0::W](ta4ex0::W) writer structure"]
impl crate::Writable for TA4EX0 {}
#[doc = "TimerAx Expansion 0 Register"]
pub mod ta4ex0;
#[doc = "TimerAx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta4iv](ta4iv) module"]
pub type TA4IV = crate::Reg<u16, _TA4IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA4IV;
#[doc = "`read()` method returns [ta4iv::R](ta4iv::R) reader structure"]
impl crate::Readable for TA4IV {}
#[doc = "`write(|w| ..)` method takes [ta4iv::W](ta4iv::W) writer structure"]
impl crate::Writable for TA4IV {}
#[doc = "TimerAx Interrupt Vector Register"]
pub mod ta4iv;
