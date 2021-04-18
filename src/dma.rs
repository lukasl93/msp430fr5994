#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control 0"]
    pub dmactl0: DMACTL0,
    #[doc = "0x02 - DMA Control 1"]
    pub dmactl1: DMACTL1,
    #[doc = "0x04 - DMA Control 2"]
    pub dmactl2: DMACTL2,
    _reserved3: [u8; 2usize],
    #[doc = "0x08 - DMA Control 4"]
    pub dmactl4: DMACTL4,
    _reserved4: [u8; 4usize],
    #[doc = "0x0e - DMA Interrupt Vector"]
    pub dmaiv: DMAIV,
    #[doc = "0x10 - DMA Channel 0 Control"]
    pub dma0ctl: DMA0CTL,
    #[doc = "0x12 - DMA Channel 0 Source Address"]
    pub dma0sa: DMA0SA,
    _reserved7: [u8; 2usize],
    #[doc = "0x16 - DMA Channel 0 Destination Address"]
    pub dma0da: DMA0DA,
    _reserved8: [u8; 2usize],
    #[doc = "0x1a - DMA Channel 0 Transfer Size"]
    pub dma0sz: DMA0SZ,
    _reserved9: [u8; 4usize],
    #[doc = "0x20 - DMA Channel 1 Control"]
    pub dma1ctl: DMA1CTL,
    #[doc = "0x22 - DMA Channel 1 Source Address"]
    pub dma1sa: DMA1SA,
    _reserved11: [u8; 2usize],
    #[doc = "0x26 - DMA Channel 1 Destination Address"]
    pub dma1da: DMA1DA,
    _reserved12: [u8; 2usize],
    #[doc = "0x2a - DMA Channel 1 Transfer Size"]
    pub dma1sz: DMA1SZ,
    _reserved13: [u8; 4usize],
    #[doc = "0x30 - DMA Channel 2 Control"]
    pub dma2ctl: DMA2CTL,
    #[doc = "0x32 - DMA Channel 2 Source Address"]
    pub dma2sa: DMA2SA,
    _reserved15: [u8; 2usize],
    #[doc = "0x36 - DMA Channel 2 Destination Address"]
    pub dma2da: DMA2DA,
    _reserved16: [u8; 2usize],
    #[doc = "0x3a - DMA Channel 2 Transfer Size"]
    pub dma2sz: DMA2SZ,
    _reserved17: [u8; 4usize],
    #[doc = "0x40 - DMA Channel 3 Control"]
    pub dma3ctl: DMA3CTL,
    #[doc = "0x42 - DMA Channel 3 Source Address"]
    pub dma3sa: DMA3SA,
    _reserved19: [u8; 2usize],
    #[doc = "0x46 - DMA Channel 3 Destination Address"]
    pub dma3da: DMA3DA,
    _reserved20: [u8; 2usize],
    #[doc = "0x4a - DMA Channel 3 Transfer Size"]
    pub dma3sz: DMA3SZ,
    _reserved21: [u8; 4usize],
    #[doc = "0x50 - DMA Channel 4 Control"]
    pub dma4ctl: DMA4CTL,
    #[doc = "0x52 - DMA Channel 4 Source Address"]
    pub dma4sa: DMA4SA,
    _reserved23: [u8; 2usize],
    #[doc = "0x56 - DMA Channel 4 Destination Address"]
    pub dma4da: DMA4DA,
    _reserved24: [u8; 2usize],
    #[doc = "0x5a - DMA Channel 4 Transfer Size"]
    pub dma4sz: DMA4SZ,
    _reserved25: [u8; 4usize],
    #[doc = "0x60 - DMA Channel 5 Control"]
    pub dma5ctl: DMA5CTL,
    #[doc = "0x62 - DMA Channel 5 Source Address"]
    pub dma5sa: DMA5SA,
    _reserved27: [u8; 2usize],
    #[doc = "0x66 - DMA Channel 5 Destination Address"]
    pub dma5da: DMA5DA,
    _reserved28: [u8; 2usize],
    #[doc = "0x6a - DMA Channel 5 Transfer Size"]
    pub dma5sz: DMA5SZ,
}
#[doc = "DMA Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl0](dmactl0) module"]
pub type DMACTL0 = crate::Reg<u16, _DMACTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL0;
#[doc = "`read()` method returns [dmactl0::R](dmactl0::R) reader structure"]
impl crate::Readable for DMACTL0 {}
#[doc = "`write(|w| ..)` method takes [dmactl0::W](dmactl0::W) writer structure"]
impl crate::Writable for DMACTL0 {}
#[doc = "DMA Control 0"]
pub mod dmactl0;
#[doc = "DMA Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl1](dmactl1) module"]
pub type DMACTL1 = crate::Reg<u16, _DMACTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL1;
#[doc = "`read()` method returns [dmactl1::R](dmactl1::R) reader structure"]
impl crate::Readable for DMACTL1 {}
#[doc = "`write(|w| ..)` method takes [dmactl1::W](dmactl1::W) writer structure"]
impl crate::Writable for DMACTL1 {}
#[doc = "DMA Control 1"]
pub mod dmactl1;
#[doc = "DMA Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl2](dmactl2) module"]
pub type DMACTL2 = crate::Reg<u16, _DMACTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL2;
#[doc = "`read()` method returns [dmactl2::R](dmactl2::R) reader structure"]
impl crate::Readable for DMACTL2 {}
#[doc = "`write(|w| ..)` method takes [dmactl2::W](dmactl2::W) writer structure"]
impl crate::Writable for DMACTL2 {}
#[doc = "DMA Control 2"]
pub mod dmactl2;
#[doc = "DMA Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl4](dmactl4) module"]
pub type DMACTL4 = crate::Reg<u16, _DMACTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL4;
#[doc = "`read()` method returns [dmactl4::R](dmactl4::R) reader structure"]
impl crate::Readable for DMACTL4 {}
#[doc = "`write(|w| ..)` method takes [dmactl4::W](dmactl4::W) writer structure"]
impl crate::Writable for DMACTL4 {}
#[doc = "DMA Control 4"]
pub mod dmactl4;
#[doc = "DMA Interrupt Vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaiv](dmaiv) module"]
pub type DMAIV = crate::Reg<u16, _DMAIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAIV;
#[doc = "`read()` method returns [dmaiv::R](dmaiv::R) reader structure"]
impl crate::Readable for DMAIV {}
#[doc = "`write(|w| ..)` method takes [dmaiv::W](dmaiv::W) writer structure"]
impl crate::Writable for DMAIV {}
#[doc = "DMA Interrupt Vector"]
pub mod dmaiv;
#[doc = "DMA Channel 0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0ctl](dma0ctl) module"]
pub type DMA0CTL = crate::Reg<u16, _DMA0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0CTL;
#[doc = "`read()` method returns [dma0ctl::R](dma0ctl::R) reader structure"]
impl crate::Readable for DMA0CTL {}
#[doc = "`write(|w| ..)` method takes [dma0ctl::W](dma0ctl::W) writer structure"]
impl crate::Writable for DMA0CTL {}
#[doc = "DMA Channel 0 Control"]
pub mod dma0ctl;
#[doc = "DMA Channel 0 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0sa](dma0sa) module"]
pub type DMA0SA = crate::Reg<u32, _DMA0SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0SA;
#[doc = "`read()` method returns [dma0sa::R](dma0sa::R) reader structure"]
impl crate::Readable for DMA0SA {}
#[doc = "`write(|w| ..)` method takes [dma0sa::W](dma0sa::W) writer structure"]
impl crate::Writable for DMA0SA {}
#[doc = "DMA Channel 0 Source Address"]
pub mod dma0sa;
#[doc = "DMA Channel 0 Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0da](dma0da) module"]
pub type DMA0DA = crate::Reg<u32, _DMA0DA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0DA;
#[doc = "`read()` method returns [dma0da::R](dma0da::R) reader structure"]
impl crate::Readable for DMA0DA {}
#[doc = "`write(|w| ..)` method takes [dma0da::W](dma0da::W) writer structure"]
impl crate::Writable for DMA0DA {}
#[doc = "DMA Channel 0 Destination Address"]
pub mod dma0da;
#[doc = "DMA Channel 0 Transfer Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0sz](dma0sz) module"]
pub type DMA0SZ = crate::Reg<u16, _DMA0SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0SZ;
#[doc = "`read()` method returns [dma0sz::R](dma0sz::R) reader structure"]
impl crate::Readable for DMA0SZ {}
#[doc = "`write(|w| ..)` method takes [dma0sz::W](dma0sz::W) writer structure"]
impl crate::Writable for DMA0SZ {}
#[doc = "DMA Channel 0 Transfer Size"]
pub mod dma0sz;
#[doc = "DMA Channel 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1ctl](dma1ctl) module"]
pub type DMA1CTL = crate::Reg<u16, _DMA1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1CTL;
#[doc = "`read()` method returns [dma1ctl::R](dma1ctl::R) reader structure"]
impl crate::Readable for DMA1CTL {}
#[doc = "`write(|w| ..)` method takes [dma1ctl::W](dma1ctl::W) writer structure"]
impl crate::Writable for DMA1CTL {}
#[doc = "DMA Channel 1 Control"]
pub mod dma1ctl;
#[doc = "DMA Channel 1 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1sa](dma1sa) module"]
pub type DMA1SA = crate::Reg<u32, _DMA1SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1SA;
#[doc = "`read()` method returns [dma1sa::R](dma1sa::R) reader structure"]
impl crate::Readable for DMA1SA {}
#[doc = "`write(|w| ..)` method takes [dma1sa::W](dma1sa::W) writer structure"]
impl crate::Writable for DMA1SA {}
#[doc = "DMA Channel 1 Source Address"]
pub mod dma1sa;
#[doc = "DMA Channel 1 Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1da](dma1da) module"]
pub type DMA1DA = crate::Reg<u32, _DMA1DA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1DA;
#[doc = "`read()` method returns [dma1da::R](dma1da::R) reader structure"]
impl crate::Readable for DMA1DA {}
#[doc = "`write(|w| ..)` method takes [dma1da::W](dma1da::W) writer structure"]
impl crate::Writable for DMA1DA {}
#[doc = "DMA Channel 1 Destination Address"]
pub mod dma1da;
#[doc = "DMA Channel 1 Transfer Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1sz](dma1sz) module"]
pub type DMA1SZ = crate::Reg<u16, _DMA1SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1SZ;
#[doc = "`read()` method returns [dma1sz::R](dma1sz::R) reader structure"]
impl crate::Readable for DMA1SZ {}
#[doc = "`write(|w| ..)` method takes [dma1sz::W](dma1sz::W) writer structure"]
impl crate::Writable for DMA1SZ {}
#[doc = "DMA Channel 1 Transfer Size"]
pub mod dma1sz;
#[doc = "DMA Channel 2 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2ctl](dma2ctl) module"]
pub type DMA2CTL = crate::Reg<u16, _DMA2CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2CTL;
#[doc = "`read()` method returns [dma2ctl::R](dma2ctl::R) reader structure"]
impl crate::Readable for DMA2CTL {}
#[doc = "`write(|w| ..)` method takes [dma2ctl::W](dma2ctl::W) writer structure"]
impl crate::Writable for DMA2CTL {}
#[doc = "DMA Channel 2 Control"]
pub mod dma2ctl;
#[doc = "DMA Channel 2 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2sa](dma2sa) module"]
pub type DMA2SA = crate::Reg<u32, _DMA2SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2SA;
#[doc = "`read()` method returns [dma2sa::R](dma2sa::R) reader structure"]
impl crate::Readable for DMA2SA {}
#[doc = "`write(|w| ..)` method takes [dma2sa::W](dma2sa::W) writer structure"]
impl crate::Writable for DMA2SA {}
#[doc = "DMA Channel 2 Source Address"]
pub mod dma2sa;
#[doc = "DMA Channel 2 Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2da](dma2da) module"]
pub type DMA2DA = crate::Reg<u32, _DMA2DA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2DA;
#[doc = "`read()` method returns [dma2da::R](dma2da::R) reader structure"]
impl crate::Readable for DMA2DA {}
#[doc = "`write(|w| ..)` method takes [dma2da::W](dma2da::W) writer structure"]
impl crate::Writable for DMA2DA {}
#[doc = "DMA Channel 2 Destination Address"]
pub mod dma2da;
#[doc = "DMA Channel 2 Transfer Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2sz](dma2sz) module"]
pub type DMA2SZ = crate::Reg<u16, _DMA2SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA2SZ;
#[doc = "`read()` method returns [dma2sz::R](dma2sz::R) reader structure"]
impl crate::Readable for DMA2SZ {}
#[doc = "`write(|w| ..)` method takes [dma2sz::W](dma2sz::W) writer structure"]
impl crate::Writable for DMA2SZ {}
#[doc = "DMA Channel 2 Transfer Size"]
pub mod dma2sz;
#[doc = "DMA Channel 3 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma3ctl](dma3ctl) module"]
pub type DMA3CTL = crate::Reg<u16, _DMA3CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA3CTL;
#[doc = "`read()` method returns [dma3ctl::R](dma3ctl::R) reader structure"]
impl crate::Readable for DMA3CTL {}
#[doc = "`write(|w| ..)` method takes [dma3ctl::W](dma3ctl::W) writer structure"]
impl crate::Writable for DMA3CTL {}
#[doc = "DMA Channel 3 Control"]
pub mod dma3ctl;
#[doc = "DMA Channel 3 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma3sa](dma3sa) module"]
pub type DMA3SA = crate::Reg<u32, _DMA3SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA3SA;
#[doc = "`read()` method returns [dma3sa::R](dma3sa::R) reader structure"]
impl crate::Readable for DMA3SA {}
#[doc = "`write(|w| ..)` method takes [dma3sa::W](dma3sa::W) writer structure"]
impl crate::Writable for DMA3SA {}
#[doc = "DMA Channel 3 Source Address"]
pub mod dma3sa;
#[doc = "DMA Channel 3 Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma3da](dma3da) module"]
pub type DMA3DA = crate::Reg<u32, _DMA3DA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA3DA;
#[doc = "`read()` method returns [dma3da::R](dma3da::R) reader structure"]
impl crate::Readable for DMA3DA {}
#[doc = "`write(|w| ..)` method takes [dma3da::W](dma3da::W) writer structure"]
impl crate::Writable for DMA3DA {}
#[doc = "DMA Channel 3 Destination Address"]
pub mod dma3da;
#[doc = "DMA Channel 3 Transfer Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma3sz](dma3sz) module"]
pub type DMA3SZ = crate::Reg<u16, _DMA3SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA3SZ;
#[doc = "`read()` method returns [dma3sz::R](dma3sz::R) reader structure"]
impl crate::Readable for DMA3SZ {}
#[doc = "`write(|w| ..)` method takes [dma3sz::W](dma3sz::W) writer structure"]
impl crate::Writable for DMA3SZ {}
#[doc = "DMA Channel 3 Transfer Size"]
pub mod dma3sz;
#[doc = "DMA Channel 4 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma4ctl](dma4ctl) module"]
pub type DMA4CTL = crate::Reg<u16, _DMA4CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA4CTL;
#[doc = "`read()` method returns [dma4ctl::R](dma4ctl::R) reader structure"]
impl crate::Readable for DMA4CTL {}
#[doc = "`write(|w| ..)` method takes [dma4ctl::W](dma4ctl::W) writer structure"]
impl crate::Writable for DMA4CTL {}
#[doc = "DMA Channel 4 Control"]
pub mod dma4ctl;
#[doc = "DMA Channel 4 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma4sa](dma4sa) module"]
pub type DMA4SA = crate::Reg<u32, _DMA4SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA4SA;
#[doc = "`read()` method returns [dma4sa::R](dma4sa::R) reader structure"]
impl crate::Readable for DMA4SA {}
#[doc = "`write(|w| ..)` method takes [dma4sa::W](dma4sa::W) writer structure"]
impl crate::Writable for DMA4SA {}
#[doc = "DMA Channel 4 Source Address"]
pub mod dma4sa;
#[doc = "DMA Channel 4 Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma4da](dma4da) module"]
pub type DMA4DA = crate::Reg<u32, _DMA4DA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA4DA;
#[doc = "`read()` method returns [dma4da::R](dma4da::R) reader structure"]
impl crate::Readable for DMA4DA {}
#[doc = "`write(|w| ..)` method takes [dma4da::W](dma4da::W) writer structure"]
impl crate::Writable for DMA4DA {}
#[doc = "DMA Channel 4 Destination Address"]
pub mod dma4da;
#[doc = "DMA Channel 4 Transfer Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma4sz](dma4sz) module"]
pub type DMA4SZ = crate::Reg<u16, _DMA4SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA4SZ;
#[doc = "`read()` method returns [dma4sz::R](dma4sz::R) reader structure"]
impl crate::Readable for DMA4SZ {}
#[doc = "`write(|w| ..)` method takes [dma4sz::W](dma4sz::W) writer structure"]
impl crate::Writable for DMA4SZ {}
#[doc = "DMA Channel 4 Transfer Size"]
pub mod dma4sz;
#[doc = "DMA Channel 5 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma5ctl](dma5ctl) module"]
pub type DMA5CTL = crate::Reg<u16, _DMA5CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA5CTL;
#[doc = "`read()` method returns [dma5ctl::R](dma5ctl::R) reader structure"]
impl crate::Readable for DMA5CTL {}
#[doc = "`write(|w| ..)` method takes [dma5ctl::W](dma5ctl::W) writer structure"]
impl crate::Writable for DMA5CTL {}
#[doc = "DMA Channel 5 Control"]
pub mod dma5ctl;
#[doc = "DMA Channel 5 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma5sa](dma5sa) module"]
pub type DMA5SA = crate::Reg<u32, _DMA5SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA5SA;
#[doc = "`read()` method returns [dma5sa::R](dma5sa::R) reader structure"]
impl crate::Readable for DMA5SA {}
#[doc = "`write(|w| ..)` method takes [dma5sa::W](dma5sa::W) writer structure"]
impl crate::Writable for DMA5SA {}
#[doc = "DMA Channel 5 Source Address"]
pub mod dma5sa;
#[doc = "DMA Channel 5 Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma5da](dma5da) module"]
pub type DMA5DA = crate::Reg<u32, _DMA5DA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA5DA;
#[doc = "`read()` method returns [dma5da::R](dma5da::R) reader structure"]
impl crate::Readable for DMA5DA {}
#[doc = "`write(|w| ..)` method takes [dma5da::W](dma5da::W) writer structure"]
impl crate::Writable for DMA5DA {}
#[doc = "DMA Channel 5 Destination Address"]
pub mod dma5da;
#[doc = "DMA Channel 5 Transfer Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma5sz](dma5sz) module"]
pub type DMA5SZ = crate::Reg<u16, _DMA5SZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA5SZ;
#[doc = "`read()` method returns [dma5sz::R](dma5sz::R) reader structure"]
impl crate::Readable for DMA5SZ {}
#[doc = "`write(|w| ..)` method takes [dma5sz::W](dma5sz::W) writer structure"]
impl crate::Writable for DMA5SZ {}
#[doc = "DMA Channel 5 Transfer Size"]
pub mod dma5sz;
