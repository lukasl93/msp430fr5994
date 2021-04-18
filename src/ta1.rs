#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    pub ta1ctl: TA1CTL,
    #[doc = "0x02 - Timer_A Capture/Compare Control Register"]
    pub ta1cctl0: TA1CCTL0,
    #[doc = "0x04 - Timer_A Capture/Compare Control Register"]
    pub ta1cctl1: TA1CCTL1,
    #[doc = "0x06 - Timer_A Capture/Compare Control Register"]
    pub ta1cctl2: TA1CCTL2,
    _reserved4: [u8; 8usize],
    #[doc = "0x10 - TimerA register"]
    pub ta1r: TA1R,
    #[doc = "0x12 - Timer_A Capture/Compare Register"]
    pub ta1ccr0: TA1CCR0,
    #[doc = "0x14 - Timer_A Capture/Compare Register"]
    pub ta1ccr1: TA1CCR1,
    #[doc = "0x16 - Timer_A Capture/Compare Register"]
    pub ta1ccr2: TA1CCR2,
    _reserved8: [u8; 8usize],
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    pub ta1ex0: TA1EX0,
    _reserved9: [u8; 12usize],
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    pub ta1iv: TA1IV,
}
#[doc = "TimerAx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1ctl](ta1ctl) module"]
pub type TA1CTL = crate::Reg<u16, _TA1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CTL;
#[doc = "`read()` method returns [ta1ctl::R](ta1ctl::R) reader structure"]
impl crate::Readable for TA1CTL {}
#[doc = "`write(|w| ..)` method takes [ta1ctl::W](ta1ctl::W) writer structure"]
impl crate::Writable for TA1CTL {}
#[doc = "TimerAx Control Register"]
pub mod ta1ctl;
#[doc = "Timer_A Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1cctl0](ta1cctl0) module"]
pub type TA1CCTL0 = crate::Reg<u16, _TA1CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCTL0;
#[doc = "`read()` method returns [ta1cctl0::R](ta1cctl0::R) reader structure"]
impl crate::Readable for TA1CCTL0 {}
#[doc = "`write(|w| ..)` method takes [ta1cctl0::W](ta1cctl0::W) writer structure"]
impl crate::Writable for TA1CCTL0 {}
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta1cctl0;
#[doc = "Timer_A Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1cctl1](ta1cctl1) module"]
pub type TA1CCTL1 = crate::Reg<u16, _TA1CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCTL1;
#[doc = "`read()` method returns [ta1cctl1::R](ta1cctl1::R) reader structure"]
impl crate::Readable for TA1CCTL1 {}
#[doc = "`write(|w| ..)` method takes [ta1cctl1::W](ta1cctl1::W) writer structure"]
impl crate::Writable for TA1CCTL1 {}
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta1cctl1;
#[doc = "Timer_A Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1cctl2](ta1cctl2) module"]
pub type TA1CCTL2 = crate::Reg<u16, _TA1CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCTL2;
#[doc = "`read()` method returns [ta1cctl2::R](ta1cctl2::R) reader structure"]
impl crate::Readable for TA1CCTL2 {}
#[doc = "`write(|w| ..)` method takes [ta1cctl2::W](ta1cctl2::W) writer structure"]
impl crate::Writable for TA1CCTL2 {}
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod ta1cctl2;
#[doc = "TimerA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1r](ta1r) module"]
pub type TA1R = crate::Reg<u16, _TA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1R;
#[doc = "`read()` method returns [ta1r::R](ta1r::R) reader structure"]
impl crate::Readable for TA1R {}
#[doc = "`write(|w| ..)` method takes [ta1r::W](ta1r::W) writer structure"]
impl crate::Writable for TA1R {}
#[doc = "TimerA register"]
pub mod ta1r;
#[doc = "Timer_A Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1ccr0](ta1ccr0) module"]
pub type TA1CCR0 = crate::Reg<u16, _TA1CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCR0;
#[doc = "`read()` method returns [ta1ccr0::R](ta1ccr0::R) reader structure"]
impl crate::Readable for TA1CCR0 {}
#[doc = "`write(|w| ..)` method takes [ta1ccr0::W](ta1ccr0::W) writer structure"]
impl crate::Writable for TA1CCR0 {}
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta1ccr0;
#[doc = "Timer_A Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1ccr1](ta1ccr1) module"]
pub type TA1CCR1 = crate::Reg<u16, _TA1CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCR1;
#[doc = "`read()` method returns [ta1ccr1::R](ta1ccr1::R) reader structure"]
impl crate::Readable for TA1CCR1 {}
#[doc = "`write(|w| ..)` method takes [ta1ccr1::W](ta1ccr1::W) writer structure"]
impl crate::Writable for TA1CCR1 {}
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta1ccr1;
#[doc = "Timer_A Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1ccr2](ta1ccr2) module"]
pub type TA1CCR2 = crate::Reg<u16, _TA1CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1CCR2;
#[doc = "`read()` method returns [ta1ccr2::R](ta1ccr2::R) reader structure"]
impl crate::Readable for TA1CCR2 {}
#[doc = "`write(|w| ..)` method takes [ta1ccr2::W](ta1ccr2::W) writer structure"]
impl crate::Writable for TA1CCR2 {}
#[doc = "Timer_A Capture/Compare Register"]
pub mod ta1ccr2;
#[doc = "TimerAx Expansion 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1ex0](ta1ex0) module"]
pub type TA1EX0 = crate::Reg<u16, _TA1EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1EX0;
#[doc = "`read()` method returns [ta1ex0::R](ta1ex0::R) reader structure"]
impl crate::Readable for TA1EX0 {}
#[doc = "`write(|w| ..)` method takes [ta1ex0::W](ta1ex0::W) writer structure"]
impl crate::Writable for TA1EX0 {}
#[doc = "TimerAx Expansion 0 Register"]
pub mod ta1ex0;
#[doc = "TimerAx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1iv](ta1iv) module"]
pub type TA1IV = crate::Reg<u16, _TA1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA1IV;
#[doc = "`read()` method returns [ta1iv::R](ta1iv::R) reader structure"]
impl crate::Readable for TA1IV {}
#[doc = "`write(|w| ..)` method takes [ta1iv::W](ta1iv::W) writer structure"]
impl crate::Writable for TA1IV {}
#[doc = "TimerAx Interrupt Vector Register"]
pub mod ta1iv;
