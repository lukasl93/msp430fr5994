#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC12_B Control 0"]
    pub adc12ctl0: ADC12CTL0,
    #[doc = "0x02 - ADC12_B Control 1"]
    pub adc12ctl1: ADC12CTL1,
    #[doc = "0x04 - ADC12_B Control 2"]
    pub adc12ctl2: ADC12CTL2,
    #[doc = "0x06 - ADC12_B Control 3"]
    pub adc12ctl3: ADC12CTL3,
    #[doc = "0x08 - ADC12_B Window Comparator Low Threshold Register"]
    pub adc12lo: ADC12LO,
    #[doc = "0x0a - ADC12_B Window Comparator High Threshold Register"]
    pub adc12hi: ADC12HI,
    #[doc = "0x0c - ADC12_B Interrupt Flag 0"]
    pub adc12ifgr0: ADC12IFGR0,
    #[doc = "0x0e - ADC12_B Interrupt Flag 1"]
    pub adc12ifgr1: ADC12IFGR1,
    #[doc = "0x10 - ADC12_B Interrupt Flag 2"]
    pub adc12ifgr2: ADC12IFGR2,
    #[doc = "0x12 - ADC12_B Interrupt Enable 0"]
    pub adc12ier0: ADC12IER0,
    #[doc = "0x14 - ADC12_B Interrupt Enable 1"]
    pub adc12ier1: ADC12IER1,
    #[doc = "0x16 - ADC12_B Interrupt Enable 2"]
    pub adc12ier2: ADC12IER2,
    #[doc = "0x18 - ADC12_B Interrupt Vector"]
    pub adc12iv: ADC12IV,
    _reserved13: [u8; 6usize],
    #[doc = "0x20 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl0: ADC12MCTL0,
    #[doc = "0x22 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl1: ADC12MCTL1,
    #[doc = "0x24 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl2: ADC12MCTL2,
    #[doc = "0x26 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl3: ADC12MCTL3,
    #[doc = "0x28 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl4: ADC12MCTL4,
    #[doc = "0x2a - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl5: ADC12MCTL5,
    #[doc = "0x2c - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl6: ADC12MCTL6,
    #[doc = "0x2e - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl7: ADC12MCTL7,
    #[doc = "0x30 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl8: ADC12MCTL8,
    #[doc = "0x32 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl9: ADC12MCTL9,
    #[doc = "0x34 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl10: ADC12MCTL10,
    #[doc = "0x36 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl11: ADC12MCTL11,
    #[doc = "0x38 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl12: ADC12MCTL12,
    #[doc = "0x3a - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl13: ADC12MCTL13,
    #[doc = "0x3c - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl14: ADC12MCTL14,
    #[doc = "0x3e - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl15: ADC12MCTL15,
    #[doc = "0x40 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl16: ADC12MCTL16,
    #[doc = "0x42 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl17: ADC12MCTL17,
    #[doc = "0x44 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl18: ADC12MCTL18,
    #[doc = "0x46 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl19: ADC12MCTL19,
    #[doc = "0x48 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl20: ADC12MCTL20,
    #[doc = "0x4a - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl21: ADC12MCTL21,
    #[doc = "0x4c - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl22: ADC12MCTL22,
    #[doc = "0x4e - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl23: ADC12MCTL23,
    #[doc = "0x50 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl24: ADC12MCTL24,
    #[doc = "0x52 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl25: ADC12MCTL25,
    #[doc = "0x54 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl26: ADC12MCTL26,
    #[doc = "0x56 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl27: ADC12MCTL27,
    #[doc = "0x58 - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl28: ADC12MCTL28,
    #[doc = "0x5a - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl29: ADC12MCTL29,
    #[doc = "0x5c - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl30: ADC12MCTL30,
    #[doc = "0x5e - ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
    pub adc12mctl31: ADC12MCTL31,
    #[doc = "0x60 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem0: ADC12MEM0,
    #[doc = "0x62 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem1: ADC12MEM1,
    #[doc = "0x64 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem2: ADC12MEM2,
    #[doc = "0x66 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem3: ADC12MEM3,
    #[doc = "0x68 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem4: ADC12MEM4,
    #[doc = "0x6a - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem5: ADC12MEM5,
    #[doc = "0x6c - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem6: ADC12MEM6,
    #[doc = "0x6e - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem7: ADC12MEM7,
    #[doc = "0x70 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem8: ADC12MEM8,
    #[doc = "0x72 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem9: ADC12MEM9,
    #[doc = "0x74 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem10: ADC12MEM10,
    #[doc = "0x76 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem11: ADC12MEM11,
    #[doc = "0x78 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem12: ADC12MEM12,
    #[doc = "0x7a - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem13: ADC12MEM13,
    #[doc = "0x7c - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem14: ADC12MEM14,
    #[doc = "0x7e - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem15: ADC12MEM15,
    #[doc = "0x80 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem16: ADC12MEM16,
    #[doc = "0x82 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem17: ADC12MEM17,
    #[doc = "0x84 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem18: ADC12MEM18,
    #[doc = "0x86 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem19: ADC12MEM19,
    #[doc = "0x88 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem20: ADC12MEM20,
    #[doc = "0x8a - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem21: ADC12MEM21,
    #[doc = "0x8c - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem22: ADC12MEM22,
    #[doc = "0x8e - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem23: ADC12MEM23,
    #[doc = "0x90 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem24: ADC12MEM24,
    #[doc = "0x92 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem25: ADC12MEM25,
    #[doc = "0x94 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem26: ADC12MEM26,
    #[doc = "0x96 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem27: ADC12MEM27,
    #[doc = "0x98 - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem28: ADC12MEM28,
    #[doc = "0x9a - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem29: ADC12MEM29,
    #[doc = "0x9c - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem30: ADC12MEM30,
    #[doc = "0x9e - ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
    pub adc12mem31: ADC12MEM31,
}
#[doc = "ADC12_B Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl0](adc12ctl0) module"]
pub type ADC12CTL0 = crate::Reg<u16, _ADC12CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12CTL0;
#[doc = "`read()` method returns [adc12ctl0::R](adc12ctl0::R) reader structure"]
impl crate::Readable for ADC12CTL0 {}
#[doc = "`write(|w| ..)` method takes [adc12ctl0::W](adc12ctl0::W) writer structure"]
impl crate::Writable for ADC12CTL0 {}
#[doc = "ADC12_B Control 0"]
pub mod adc12ctl0;
#[doc = "ADC12_B Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl1](adc12ctl1) module"]
pub type ADC12CTL1 = crate::Reg<u16, _ADC12CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12CTL1;
#[doc = "`read()` method returns [adc12ctl1::R](adc12ctl1::R) reader structure"]
impl crate::Readable for ADC12CTL1 {}
#[doc = "`write(|w| ..)` method takes [adc12ctl1::W](adc12ctl1::W) writer structure"]
impl crate::Writable for ADC12CTL1 {}
#[doc = "ADC12_B Control 1"]
pub mod adc12ctl1;
#[doc = "ADC12_B Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl2](adc12ctl2) module"]
pub type ADC12CTL2 = crate::Reg<u16, _ADC12CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12CTL2;
#[doc = "`read()` method returns [adc12ctl2::R](adc12ctl2::R) reader structure"]
impl crate::Readable for ADC12CTL2 {}
#[doc = "`write(|w| ..)` method takes [adc12ctl2::W](adc12ctl2::W) writer structure"]
impl crate::Writable for ADC12CTL2 {}
#[doc = "ADC12_B Control 2"]
pub mod adc12ctl2;
#[doc = "ADC12_B Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ctl3](adc12ctl3) module"]
pub type ADC12CTL3 = crate::Reg<u16, _ADC12CTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12CTL3;
#[doc = "`read()` method returns [adc12ctl3::R](adc12ctl3::R) reader structure"]
impl crate::Readable for ADC12CTL3 {}
#[doc = "`write(|w| ..)` method takes [adc12ctl3::W](adc12ctl3::W) writer structure"]
impl crate::Writable for ADC12CTL3 {}
#[doc = "ADC12_B Control 3"]
pub mod adc12ctl3;
#[doc = "ADC12_B Window Comparator Low Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12lo](adc12lo) module"]
pub type ADC12LO = crate::Reg<u16, _ADC12LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12LO;
#[doc = "`read()` method returns [adc12lo::R](adc12lo::R) reader structure"]
impl crate::Readable for ADC12LO {}
#[doc = "`write(|w| ..)` method takes [adc12lo::W](adc12lo::W) writer structure"]
impl crate::Writable for ADC12LO {}
#[doc = "ADC12_B Window Comparator Low Threshold Register"]
pub mod adc12lo;
#[doc = "ADC12_B Window Comparator High Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12hi](adc12hi) module"]
pub type ADC12HI = crate::Reg<u16, _ADC12HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12HI;
#[doc = "`read()` method returns [adc12hi::R](adc12hi::R) reader structure"]
impl crate::Readable for ADC12HI {}
#[doc = "`write(|w| ..)` method takes [adc12hi::W](adc12hi::W) writer structure"]
impl crate::Writable for ADC12HI {}
#[doc = "ADC12_B Window Comparator High Threshold Register"]
pub mod adc12hi;
#[doc = "ADC12_B Interrupt Flag 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ifgr0](adc12ifgr0) module"]
pub type ADC12IFGR0 = crate::Reg<u16, _ADC12IFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IFGR0;
#[doc = "`read()` method returns [adc12ifgr0::R](adc12ifgr0::R) reader structure"]
impl crate::Readable for ADC12IFGR0 {}
#[doc = "`write(|w| ..)` method takes [adc12ifgr0::W](adc12ifgr0::W) writer structure"]
impl crate::Writable for ADC12IFGR0 {}
#[doc = "ADC12_B Interrupt Flag 0"]
pub mod adc12ifgr0;
#[doc = "ADC12_B Interrupt Flag 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ifgr1](adc12ifgr1) module"]
pub type ADC12IFGR1 = crate::Reg<u16, _ADC12IFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IFGR1;
#[doc = "`read()` method returns [adc12ifgr1::R](adc12ifgr1::R) reader structure"]
impl crate::Readable for ADC12IFGR1 {}
#[doc = "`write(|w| ..)` method takes [adc12ifgr1::W](adc12ifgr1::W) writer structure"]
impl crate::Writable for ADC12IFGR1 {}
#[doc = "ADC12_B Interrupt Flag 1"]
pub mod adc12ifgr1;
#[doc = "ADC12_B Interrupt Flag 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ifgr2](adc12ifgr2) module"]
pub type ADC12IFGR2 = crate::Reg<u16, _ADC12IFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IFGR2;
#[doc = "`read()` method returns [adc12ifgr2::R](adc12ifgr2::R) reader structure"]
impl crate::Readable for ADC12IFGR2 {}
#[doc = "`write(|w| ..)` method takes [adc12ifgr2::W](adc12ifgr2::W) writer structure"]
impl crate::Writable for ADC12IFGR2 {}
#[doc = "ADC12_B Interrupt Flag 2"]
pub mod adc12ifgr2;
#[doc = "ADC12_B Interrupt Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ier0](adc12ier0) module"]
pub type ADC12IER0 = crate::Reg<u16, _ADC12IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IER0;
#[doc = "`read()` method returns [adc12ier0::R](adc12ier0::R) reader structure"]
impl crate::Readable for ADC12IER0 {}
#[doc = "`write(|w| ..)` method takes [adc12ier0::W](adc12ier0::W) writer structure"]
impl crate::Writable for ADC12IER0 {}
#[doc = "ADC12_B Interrupt Enable 0"]
pub mod adc12ier0;
#[doc = "ADC12_B Interrupt Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ier1](adc12ier1) module"]
pub type ADC12IER1 = crate::Reg<u16, _ADC12IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IER1;
#[doc = "`read()` method returns [adc12ier1::R](adc12ier1::R) reader structure"]
impl crate::Readable for ADC12IER1 {}
#[doc = "`write(|w| ..)` method takes [adc12ier1::W](adc12ier1::W) writer structure"]
impl crate::Writable for ADC12IER1 {}
#[doc = "ADC12_B Interrupt Enable 1"]
pub mod adc12ier1;
#[doc = "ADC12_B Interrupt Enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ier2](adc12ier2) module"]
pub type ADC12IER2 = crate::Reg<u16, _ADC12IER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IER2;
#[doc = "`read()` method returns [adc12ier2::R](adc12ier2::R) reader structure"]
impl crate::Readable for ADC12IER2 {}
#[doc = "`write(|w| ..)` method takes [adc12ier2::W](adc12ier2::W) writer structure"]
impl crate::Writable for ADC12IER2 {}
#[doc = "ADC12_B Interrupt Enable 2"]
pub mod adc12ier2;
#[doc = "ADC12_B Interrupt Vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12iv](adc12iv) module"]
pub type ADC12IV = crate::Reg<u16, _ADC12IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12IV;
#[doc = "`read()` method returns [adc12iv::R](adc12iv::R) reader structure"]
impl crate::Readable for ADC12IV {}
#[doc = "`write(|w| ..)` method takes [adc12iv::W](adc12iv::W) writer structure"]
impl crate::Writable for ADC12IV {}
#[doc = "ADC12_B Interrupt Vector"]
pub mod adc12iv;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl0](adc12mctl0) module"]
pub type ADC12MCTL0 = crate::Reg<u16, _ADC12MCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL0;
#[doc = "`read()` method returns [adc12mctl0::R](adc12mctl0::R) reader structure"]
impl crate::Readable for ADC12MCTL0 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl0::W](adc12mctl0::W) writer structure"]
impl crate::Writable for ADC12MCTL0 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl0;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl1](adc12mctl1) module"]
pub type ADC12MCTL1 = crate::Reg<u16, _ADC12MCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL1;
#[doc = "`read()` method returns [adc12mctl1::R](adc12mctl1::R) reader structure"]
impl crate::Readable for ADC12MCTL1 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl1::W](adc12mctl1::W) writer structure"]
impl crate::Writable for ADC12MCTL1 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl1;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl2](adc12mctl2) module"]
pub type ADC12MCTL2 = crate::Reg<u16, _ADC12MCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL2;
#[doc = "`read()` method returns [adc12mctl2::R](adc12mctl2::R) reader structure"]
impl crate::Readable for ADC12MCTL2 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl2::W](adc12mctl2::W) writer structure"]
impl crate::Writable for ADC12MCTL2 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl2;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl3](adc12mctl3) module"]
pub type ADC12MCTL3 = crate::Reg<u16, _ADC12MCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL3;
#[doc = "`read()` method returns [adc12mctl3::R](adc12mctl3::R) reader structure"]
impl crate::Readable for ADC12MCTL3 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl3::W](adc12mctl3::W) writer structure"]
impl crate::Writable for ADC12MCTL3 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl3;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl4](adc12mctl4) module"]
pub type ADC12MCTL4 = crate::Reg<u16, _ADC12MCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL4;
#[doc = "`read()` method returns [adc12mctl4::R](adc12mctl4::R) reader structure"]
impl crate::Readable for ADC12MCTL4 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl4::W](adc12mctl4::W) writer structure"]
impl crate::Writable for ADC12MCTL4 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl4;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl5](adc12mctl5) module"]
pub type ADC12MCTL5 = crate::Reg<u16, _ADC12MCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL5;
#[doc = "`read()` method returns [adc12mctl5::R](adc12mctl5::R) reader structure"]
impl crate::Readable for ADC12MCTL5 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl5::W](adc12mctl5::W) writer structure"]
impl crate::Writable for ADC12MCTL5 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl5;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl6](adc12mctl6) module"]
pub type ADC12MCTL6 = crate::Reg<u16, _ADC12MCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL6;
#[doc = "`read()` method returns [adc12mctl6::R](adc12mctl6::R) reader structure"]
impl crate::Readable for ADC12MCTL6 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl6::W](adc12mctl6::W) writer structure"]
impl crate::Writable for ADC12MCTL6 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl6;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl7](adc12mctl7) module"]
pub type ADC12MCTL7 = crate::Reg<u16, _ADC12MCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL7;
#[doc = "`read()` method returns [adc12mctl7::R](adc12mctl7::R) reader structure"]
impl crate::Readable for ADC12MCTL7 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl7::W](adc12mctl7::W) writer structure"]
impl crate::Writable for ADC12MCTL7 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl7;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl8](adc12mctl8) module"]
pub type ADC12MCTL8 = crate::Reg<u16, _ADC12MCTL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL8;
#[doc = "`read()` method returns [adc12mctl8::R](adc12mctl8::R) reader structure"]
impl crate::Readable for ADC12MCTL8 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl8::W](adc12mctl8::W) writer structure"]
impl crate::Writable for ADC12MCTL8 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl8;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl9](adc12mctl9) module"]
pub type ADC12MCTL9 = crate::Reg<u16, _ADC12MCTL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL9;
#[doc = "`read()` method returns [adc12mctl9::R](adc12mctl9::R) reader structure"]
impl crate::Readable for ADC12MCTL9 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl9::W](adc12mctl9::W) writer structure"]
impl crate::Writable for ADC12MCTL9 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl9;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl10](adc12mctl10) module"]
pub type ADC12MCTL10 = crate::Reg<u16, _ADC12MCTL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL10;
#[doc = "`read()` method returns [adc12mctl10::R](adc12mctl10::R) reader structure"]
impl crate::Readable for ADC12MCTL10 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl10::W](adc12mctl10::W) writer structure"]
impl crate::Writable for ADC12MCTL10 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl10;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl11](adc12mctl11) module"]
pub type ADC12MCTL11 = crate::Reg<u16, _ADC12MCTL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL11;
#[doc = "`read()` method returns [adc12mctl11::R](adc12mctl11::R) reader structure"]
impl crate::Readable for ADC12MCTL11 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl11::W](adc12mctl11::W) writer structure"]
impl crate::Writable for ADC12MCTL11 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl11;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl12](adc12mctl12) module"]
pub type ADC12MCTL12 = crate::Reg<u16, _ADC12MCTL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL12;
#[doc = "`read()` method returns [adc12mctl12::R](adc12mctl12::R) reader structure"]
impl crate::Readable for ADC12MCTL12 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl12::W](adc12mctl12::W) writer structure"]
impl crate::Writable for ADC12MCTL12 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl12;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl13](adc12mctl13) module"]
pub type ADC12MCTL13 = crate::Reg<u16, _ADC12MCTL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL13;
#[doc = "`read()` method returns [adc12mctl13::R](adc12mctl13::R) reader structure"]
impl crate::Readable for ADC12MCTL13 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl13::W](adc12mctl13::W) writer structure"]
impl crate::Writable for ADC12MCTL13 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl13;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl14](adc12mctl14) module"]
pub type ADC12MCTL14 = crate::Reg<u16, _ADC12MCTL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL14;
#[doc = "`read()` method returns [adc12mctl14::R](adc12mctl14::R) reader structure"]
impl crate::Readable for ADC12MCTL14 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl14::W](adc12mctl14::W) writer structure"]
impl crate::Writable for ADC12MCTL14 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl14;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl15](adc12mctl15) module"]
pub type ADC12MCTL15 = crate::Reg<u16, _ADC12MCTL15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL15;
#[doc = "`read()` method returns [adc12mctl15::R](adc12mctl15::R) reader structure"]
impl crate::Readable for ADC12MCTL15 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl15::W](adc12mctl15::W) writer structure"]
impl crate::Writable for ADC12MCTL15 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl15;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl16](adc12mctl16) module"]
pub type ADC12MCTL16 = crate::Reg<u16, _ADC12MCTL16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL16;
#[doc = "`read()` method returns [adc12mctl16::R](adc12mctl16::R) reader structure"]
impl crate::Readable for ADC12MCTL16 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl16::W](adc12mctl16::W) writer structure"]
impl crate::Writable for ADC12MCTL16 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl16;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl17](adc12mctl17) module"]
pub type ADC12MCTL17 = crate::Reg<u16, _ADC12MCTL17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL17;
#[doc = "`read()` method returns [adc12mctl17::R](adc12mctl17::R) reader structure"]
impl crate::Readable for ADC12MCTL17 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl17::W](adc12mctl17::W) writer structure"]
impl crate::Writable for ADC12MCTL17 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl17;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl18](adc12mctl18) module"]
pub type ADC12MCTL18 = crate::Reg<u16, _ADC12MCTL18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL18;
#[doc = "`read()` method returns [adc12mctl18::R](adc12mctl18::R) reader structure"]
impl crate::Readable for ADC12MCTL18 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl18::W](adc12mctl18::W) writer structure"]
impl crate::Writable for ADC12MCTL18 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl18;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl19](adc12mctl19) module"]
pub type ADC12MCTL19 = crate::Reg<u16, _ADC12MCTL19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL19;
#[doc = "`read()` method returns [adc12mctl19::R](adc12mctl19::R) reader structure"]
impl crate::Readable for ADC12MCTL19 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl19::W](adc12mctl19::W) writer structure"]
impl crate::Writable for ADC12MCTL19 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl19;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl20](adc12mctl20) module"]
pub type ADC12MCTL20 = crate::Reg<u16, _ADC12MCTL20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL20;
#[doc = "`read()` method returns [adc12mctl20::R](adc12mctl20::R) reader structure"]
impl crate::Readable for ADC12MCTL20 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl20::W](adc12mctl20::W) writer structure"]
impl crate::Writable for ADC12MCTL20 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl20;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl21](adc12mctl21) module"]
pub type ADC12MCTL21 = crate::Reg<u16, _ADC12MCTL21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL21;
#[doc = "`read()` method returns [adc12mctl21::R](adc12mctl21::R) reader structure"]
impl crate::Readable for ADC12MCTL21 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl21::W](adc12mctl21::W) writer structure"]
impl crate::Writable for ADC12MCTL21 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl21;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl22](adc12mctl22) module"]
pub type ADC12MCTL22 = crate::Reg<u16, _ADC12MCTL22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL22;
#[doc = "`read()` method returns [adc12mctl22::R](adc12mctl22::R) reader structure"]
impl crate::Readable for ADC12MCTL22 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl22::W](adc12mctl22::W) writer structure"]
impl crate::Writable for ADC12MCTL22 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl22;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl23](adc12mctl23) module"]
pub type ADC12MCTL23 = crate::Reg<u16, _ADC12MCTL23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL23;
#[doc = "`read()` method returns [adc12mctl23::R](adc12mctl23::R) reader structure"]
impl crate::Readable for ADC12MCTL23 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl23::W](adc12mctl23::W) writer structure"]
impl crate::Writable for ADC12MCTL23 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl23;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl24](adc12mctl24) module"]
pub type ADC12MCTL24 = crate::Reg<u16, _ADC12MCTL24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL24;
#[doc = "`read()` method returns [adc12mctl24::R](adc12mctl24::R) reader structure"]
impl crate::Readable for ADC12MCTL24 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl24::W](adc12mctl24::W) writer structure"]
impl crate::Writable for ADC12MCTL24 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl24;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl25](adc12mctl25) module"]
pub type ADC12MCTL25 = crate::Reg<u16, _ADC12MCTL25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL25;
#[doc = "`read()` method returns [adc12mctl25::R](adc12mctl25::R) reader structure"]
impl crate::Readable for ADC12MCTL25 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl25::W](adc12mctl25::W) writer structure"]
impl crate::Writable for ADC12MCTL25 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl25;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl26](adc12mctl26) module"]
pub type ADC12MCTL26 = crate::Reg<u16, _ADC12MCTL26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL26;
#[doc = "`read()` method returns [adc12mctl26::R](adc12mctl26::R) reader structure"]
impl crate::Readable for ADC12MCTL26 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl26::W](adc12mctl26::W) writer structure"]
impl crate::Writable for ADC12MCTL26 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl26;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl27](adc12mctl27) module"]
pub type ADC12MCTL27 = crate::Reg<u16, _ADC12MCTL27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL27;
#[doc = "`read()` method returns [adc12mctl27::R](adc12mctl27::R) reader structure"]
impl crate::Readable for ADC12MCTL27 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl27::W](adc12mctl27::W) writer structure"]
impl crate::Writable for ADC12MCTL27 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl27;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl28](adc12mctl28) module"]
pub type ADC12MCTL28 = crate::Reg<u16, _ADC12MCTL28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL28;
#[doc = "`read()` method returns [adc12mctl28::R](adc12mctl28::R) reader structure"]
impl crate::Readable for ADC12MCTL28 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl28::W](adc12mctl28::W) writer structure"]
impl crate::Writable for ADC12MCTL28 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl28;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl29](adc12mctl29) module"]
pub type ADC12MCTL29 = crate::Reg<u16, _ADC12MCTL29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL29;
#[doc = "`read()` method returns [adc12mctl29::R](adc12mctl29::R) reader structure"]
impl crate::Readable for ADC12MCTL29 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl29::W](adc12mctl29::W) writer structure"]
impl crate::Writable for ADC12MCTL29 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl29;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl30](adc12mctl30) module"]
pub type ADC12MCTL30 = crate::Reg<u16, _ADC12MCTL30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL30;
#[doc = "`read()` method returns [adc12mctl30::R](adc12mctl30::R) reader structure"]
impl crate::Readable for ADC12MCTL30 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl30::W](adc12mctl30::W) writer structure"]
impl crate::Writable for ADC12MCTL30 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl30;
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mctl31](adc12mctl31) module"]
pub type ADC12MCTL31 = crate::Reg<u16, _ADC12MCTL31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MCTL31;
#[doc = "`read()` method returns [adc12mctl31::R](adc12mctl31::R) reader structure"]
impl crate::Readable for ADC12MCTL31 {}
#[doc = "`write(|w| ..)` method takes [adc12mctl31::W](adc12mctl31::W) writer structure"]
impl crate::Writable for ADC12MCTL31 {}
#[doc = "ADC12_B Memory Control 0 Register to ADC12_B Memory Control 31 Register"]
pub mod adc12mctl31;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem0](adc12mem0) module"]
pub type ADC12MEM0 = crate::Reg<u16, _ADC12MEM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM0;
#[doc = "`read()` method returns [adc12mem0::R](adc12mem0::R) reader structure"]
impl crate::Readable for ADC12MEM0 {}
#[doc = "`write(|w| ..)` method takes [adc12mem0::W](adc12mem0::W) writer structure"]
impl crate::Writable for ADC12MEM0 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem0;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem1](adc12mem1) module"]
pub type ADC12MEM1 = crate::Reg<u16, _ADC12MEM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM1;
#[doc = "`read()` method returns [adc12mem1::R](adc12mem1::R) reader structure"]
impl crate::Readable for ADC12MEM1 {}
#[doc = "`write(|w| ..)` method takes [adc12mem1::W](adc12mem1::W) writer structure"]
impl crate::Writable for ADC12MEM1 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem1;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem2](adc12mem2) module"]
pub type ADC12MEM2 = crate::Reg<u16, _ADC12MEM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM2;
#[doc = "`read()` method returns [adc12mem2::R](adc12mem2::R) reader structure"]
impl crate::Readable for ADC12MEM2 {}
#[doc = "`write(|w| ..)` method takes [adc12mem2::W](adc12mem2::W) writer structure"]
impl crate::Writable for ADC12MEM2 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem2;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem3](adc12mem3) module"]
pub type ADC12MEM3 = crate::Reg<u16, _ADC12MEM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM3;
#[doc = "`read()` method returns [adc12mem3::R](adc12mem3::R) reader structure"]
impl crate::Readable for ADC12MEM3 {}
#[doc = "`write(|w| ..)` method takes [adc12mem3::W](adc12mem3::W) writer structure"]
impl crate::Writable for ADC12MEM3 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem3;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem4](adc12mem4) module"]
pub type ADC12MEM4 = crate::Reg<u16, _ADC12MEM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM4;
#[doc = "`read()` method returns [adc12mem4::R](adc12mem4::R) reader structure"]
impl crate::Readable for ADC12MEM4 {}
#[doc = "`write(|w| ..)` method takes [adc12mem4::W](adc12mem4::W) writer structure"]
impl crate::Writable for ADC12MEM4 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem4;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem5](adc12mem5) module"]
pub type ADC12MEM5 = crate::Reg<u16, _ADC12MEM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM5;
#[doc = "`read()` method returns [adc12mem5::R](adc12mem5::R) reader structure"]
impl crate::Readable for ADC12MEM5 {}
#[doc = "`write(|w| ..)` method takes [adc12mem5::W](adc12mem5::W) writer structure"]
impl crate::Writable for ADC12MEM5 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem5;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem6](adc12mem6) module"]
pub type ADC12MEM6 = crate::Reg<u16, _ADC12MEM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM6;
#[doc = "`read()` method returns [adc12mem6::R](adc12mem6::R) reader structure"]
impl crate::Readable for ADC12MEM6 {}
#[doc = "`write(|w| ..)` method takes [adc12mem6::W](adc12mem6::W) writer structure"]
impl crate::Writable for ADC12MEM6 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem6;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem7](adc12mem7) module"]
pub type ADC12MEM7 = crate::Reg<u16, _ADC12MEM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM7;
#[doc = "`read()` method returns [adc12mem7::R](adc12mem7::R) reader structure"]
impl crate::Readable for ADC12MEM7 {}
#[doc = "`write(|w| ..)` method takes [adc12mem7::W](adc12mem7::W) writer structure"]
impl crate::Writable for ADC12MEM7 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem7;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem8](adc12mem8) module"]
pub type ADC12MEM8 = crate::Reg<u16, _ADC12MEM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM8;
#[doc = "`read()` method returns [adc12mem8::R](adc12mem8::R) reader structure"]
impl crate::Readable for ADC12MEM8 {}
#[doc = "`write(|w| ..)` method takes [adc12mem8::W](adc12mem8::W) writer structure"]
impl crate::Writable for ADC12MEM8 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem8;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem9](adc12mem9) module"]
pub type ADC12MEM9 = crate::Reg<u16, _ADC12MEM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM9;
#[doc = "`read()` method returns [adc12mem9::R](adc12mem9::R) reader structure"]
impl crate::Readable for ADC12MEM9 {}
#[doc = "`write(|w| ..)` method takes [adc12mem9::W](adc12mem9::W) writer structure"]
impl crate::Writable for ADC12MEM9 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem9;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem10](adc12mem10) module"]
pub type ADC12MEM10 = crate::Reg<u16, _ADC12MEM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM10;
#[doc = "`read()` method returns [adc12mem10::R](adc12mem10::R) reader structure"]
impl crate::Readable for ADC12MEM10 {}
#[doc = "`write(|w| ..)` method takes [adc12mem10::W](adc12mem10::W) writer structure"]
impl crate::Writable for ADC12MEM10 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem10;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem11](adc12mem11) module"]
pub type ADC12MEM11 = crate::Reg<u16, _ADC12MEM11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM11;
#[doc = "`read()` method returns [adc12mem11::R](adc12mem11::R) reader structure"]
impl crate::Readable for ADC12MEM11 {}
#[doc = "`write(|w| ..)` method takes [adc12mem11::W](adc12mem11::W) writer structure"]
impl crate::Writable for ADC12MEM11 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem11;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem12](adc12mem12) module"]
pub type ADC12MEM12 = crate::Reg<u16, _ADC12MEM12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM12;
#[doc = "`read()` method returns [adc12mem12::R](adc12mem12::R) reader structure"]
impl crate::Readable for ADC12MEM12 {}
#[doc = "`write(|w| ..)` method takes [adc12mem12::W](adc12mem12::W) writer structure"]
impl crate::Writable for ADC12MEM12 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem12;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem13](adc12mem13) module"]
pub type ADC12MEM13 = crate::Reg<u16, _ADC12MEM13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM13;
#[doc = "`read()` method returns [adc12mem13::R](adc12mem13::R) reader structure"]
impl crate::Readable for ADC12MEM13 {}
#[doc = "`write(|w| ..)` method takes [adc12mem13::W](adc12mem13::W) writer structure"]
impl crate::Writable for ADC12MEM13 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem13;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem14](adc12mem14) module"]
pub type ADC12MEM14 = crate::Reg<u16, _ADC12MEM14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM14;
#[doc = "`read()` method returns [adc12mem14::R](adc12mem14::R) reader structure"]
impl crate::Readable for ADC12MEM14 {}
#[doc = "`write(|w| ..)` method takes [adc12mem14::W](adc12mem14::W) writer structure"]
impl crate::Writable for ADC12MEM14 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem14;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem15](adc12mem15) module"]
pub type ADC12MEM15 = crate::Reg<u16, _ADC12MEM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM15;
#[doc = "`read()` method returns [adc12mem15::R](adc12mem15::R) reader structure"]
impl crate::Readable for ADC12MEM15 {}
#[doc = "`write(|w| ..)` method takes [adc12mem15::W](adc12mem15::W) writer structure"]
impl crate::Writable for ADC12MEM15 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem15;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem16](adc12mem16) module"]
pub type ADC12MEM16 = crate::Reg<u16, _ADC12MEM16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM16;
#[doc = "`read()` method returns [adc12mem16::R](adc12mem16::R) reader structure"]
impl crate::Readable for ADC12MEM16 {}
#[doc = "`write(|w| ..)` method takes [adc12mem16::W](adc12mem16::W) writer structure"]
impl crate::Writable for ADC12MEM16 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem16;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem17](adc12mem17) module"]
pub type ADC12MEM17 = crate::Reg<u16, _ADC12MEM17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM17;
#[doc = "`read()` method returns [adc12mem17::R](adc12mem17::R) reader structure"]
impl crate::Readable for ADC12MEM17 {}
#[doc = "`write(|w| ..)` method takes [adc12mem17::W](adc12mem17::W) writer structure"]
impl crate::Writable for ADC12MEM17 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem17;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem18](adc12mem18) module"]
pub type ADC12MEM18 = crate::Reg<u16, _ADC12MEM18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM18;
#[doc = "`read()` method returns [adc12mem18::R](adc12mem18::R) reader structure"]
impl crate::Readable for ADC12MEM18 {}
#[doc = "`write(|w| ..)` method takes [adc12mem18::W](adc12mem18::W) writer structure"]
impl crate::Writable for ADC12MEM18 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem18;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem19](adc12mem19) module"]
pub type ADC12MEM19 = crate::Reg<u16, _ADC12MEM19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM19;
#[doc = "`read()` method returns [adc12mem19::R](adc12mem19::R) reader structure"]
impl crate::Readable for ADC12MEM19 {}
#[doc = "`write(|w| ..)` method takes [adc12mem19::W](adc12mem19::W) writer structure"]
impl crate::Writable for ADC12MEM19 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem19;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem20](adc12mem20) module"]
pub type ADC12MEM20 = crate::Reg<u16, _ADC12MEM20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM20;
#[doc = "`read()` method returns [adc12mem20::R](adc12mem20::R) reader structure"]
impl crate::Readable for ADC12MEM20 {}
#[doc = "`write(|w| ..)` method takes [adc12mem20::W](adc12mem20::W) writer structure"]
impl crate::Writable for ADC12MEM20 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem20;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem21](adc12mem21) module"]
pub type ADC12MEM21 = crate::Reg<u16, _ADC12MEM21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM21;
#[doc = "`read()` method returns [adc12mem21::R](adc12mem21::R) reader structure"]
impl crate::Readable for ADC12MEM21 {}
#[doc = "`write(|w| ..)` method takes [adc12mem21::W](adc12mem21::W) writer structure"]
impl crate::Writable for ADC12MEM21 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem21;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem22](adc12mem22) module"]
pub type ADC12MEM22 = crate::Reg<u16, _ADC12MEM22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM22;
#[doc = "`read()` method returns [adc12mem22::R](adc12mem22::R) reader structure"]
impl crate::Readable for ADC12MEM22 {}
#[doc = "`write(|w| ..)` method takes [adc12mem22::W](adc12mem22::W) writer structure"]
impl crate::Writable for ADC12MEM22 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem22;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem23](adc12mem23) module"]
pub type ADC12MEM23 = crate::Reg<u16, _ADC12MEM23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM23;
#[doc = "`read()` method returns [adc12mem23::R](adc12mem23::R) reader structure"]
impl crate::Readable for ADC12MEM23 {}
#[doc = "`write(|w| ..)` method takes [adc12mem23::W](adc12mem23::W) writer structure"]
impl crate::Writable for ADC12MEM23 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem23;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem24](adc12mem24) module"]
pub type ADC12MEM24 = crate::Reg<u16, _ADC12MEM24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM24;
#[doc = "`read()` method returns [adc12mem24::R](adc12mem24::R) reader structure"]
impl crate::Readable for ADC12MEM24 {}
#[doc = "`write(|w| ..)` method takes [adc12mem24::W](adc12mem24::W) writer structure"]
impl crate::Writable for ADC12MEM24 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem24;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem25](adc12mem25) module"]
pub type ADC12MEM25 = crate::Reg<u16, _ADC12MEM25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM25;
#[doc = "`read()` method returns [adc12mem25::R](adc12mem25::R) reader structure"]
impl crate::Readable for ADC12MEM25 {}
#[doc = "`write(|w| ..)` method takes [adc12mem25::W](adc12mem25::W) writer structure"]
impl crate::Writable for ADC12MEM25 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem25;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem26](adc12mem26) module"]
pub type ADC12MEM26 = crate::Reg<u16, _ADC12MEM26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM26;
#[doc = "`read()` method returns [adc12mem26::R](adc12mem26::R) reader structure"]
impl crate::Readable for ADC12MEM26 {}
#[doc = "`write(|w| ..)` method takes [adc12mem26::W](adc12mem26::W) writer structure"]
impl crate::Writable for ADC12MEM26 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem26;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem27](adc12mem27) module"]
pub type ADC12MEM27 = crate::Reg<u16, _ADC12MEM27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM27;
#[doc = "`read()` method returns [adc12mem27::R](adc12mem27::R) reader structure"]
impl crate::Readable for ADC12MEM27 {}
#[doc = "`write(|w| ..)` method takes [adc12mem27::W](adc12mem27::W) writer structure"]
impl crate::Writable for ADC12MEM27 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem27;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem28](adc12mem28) module"]
pub type ADC12MEM28 = crate::Reg<u16, _ADC12MEM28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM28;
#[doc = "`read()` method returns [adc12mem28::R](adc12mem28::R) reader structure"]
impl crate::Readable for ADC12MEM28 {}
#[doc = "`write(|w| ..)` method takes [adc12mem28::W](adc12mem28::W) writer structure"]
impl crate::Writable for ADC12MEM28 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem28;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem29](adc12mem29) module"]
pub type ADC12MEM29 = crate::Reg<u16, _ADC12MEM29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM29;
#[doc = "`read()` method returns [adc12mem29::R](adc12mem29::R) reader structure"]
impl crate::Readable for ADC12MEM29 {}
#[doc = "`write(|w| ..)` method takes [adc12mem29::W](adc12mem29::W) writer structure"]
impl crate::Writable for ADC12MEM29 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem29;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem30](adc12mem30) module"]
pub type ADC12MEM30 = crate::Reg<u16, _ADC12MEM30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM30;
#[doc = "`read()` method returns [adc12mem30::R](adc12mem30::R) reader structure"]
impl crate::Readable for ADC12MEM30 {}
#[doc = "`write(|w| ..)` method takes [adc12mem30::W](adc12mem30::W) writer structure"]
impl crate::Writable for ADC12MEM30 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem30;
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12mem31](adc12mem31) module"]
pub type ADC12MEM31 = crate::Reg<u16, _ADC12MEM31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC12MEM31;
#[doc = "`read()` method returns [adc12mem31::R](adc12mem31::R) reader structure"]
impl crate::Readable for ADC12MEM31 {}
#[doc = "`write(|w| ..)` method takes [adc12mem31::W](adc12mem31::W) writer structure"]
impl crate::Writable for ADC12MEM31 {}
#[doc = "ADC12_B Memory 0 Register to ADC12_B Memory 31 Register"]
pub mod adc12mem31;
