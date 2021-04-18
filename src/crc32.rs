#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC32 Data Input Word 0"]
    pub crc32diw0: CRC32DIW0,
    #[doc = "0x02 - CRC32 Data Input Word 1"]
    pub crc32diw1: CRC32DIW1,
    #[doc = "0x04 - CRC32 Data In Reverse Word 1"]
    pub crc32dirbw1: CRC32DIRBW1,
    #[doc = "0x06 - CRC32 Data In Reverse Word 0"]
    pub crc32dirbw0: CRC32DIRBW0,
    #[doc = "0x08 - CRC32 Initialization and Result Word 0"]
    pub crc32iniresw0: CRC32INIRESW0,
    #[doc = "0x0a - CRC32 Initialization and Result Word 1"]
    pub crc32iniresw1: CRC32INIRESW1,
    #[doc = "0x0c - CRC32 Result Reverse Word 1"]
    pub crc32resrw1: CRC32RESRW1,
    #[doc = "0x0e - CRC32 Result Reverse Word 0"]
    pub crc32resrw0: CRC32RESRW0,
    #[doc = "0x10 - CRC16 Data Input"]
    pub crc16diw0: CRC16DIW0,
    _reserved9: [u8; 4usize],
    #[doc = "0x16 - CRC16 Data In Reverse"]
    pub crc16dirbw0: CRC16DIRBW0,
    #[doc = "0x18 - CRC16 Init and Result"]
    pub crc16iniresw0: CRC16INIRESW0,
    _reserved11: [u8; 4usize],
    #[doc = "0x1e - CRC16 Result Reverse"]
    pub crc16resrw0: CRC16RESRW0,
}
#[doc = "CRC32 Data Input Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32diw0](crc32diw0) module"]
pub type CRC32DIW0 = crate::Reg<u16, _CRC32DIW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32DIW0;
#[doc = "`read()` method returns [crc32diw0::R](crc32diw0::R) reader structure"]
impl crate::Readable for CRC32DIW0 {}
#[doc = "`write(|w| ..)` method takes [crc32diw0::W](crc32diw0::W) writer structure"]
impl crate::Writable for CRC32DIW0 {}
#[doc = "CRC32 Data Input Word 0"]
pub mod crc32diw0;
#[doc = "CRC32 Data Input Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32diw1](crc32diw1) module"]
pub type CRC32DIW1 = crate::Reg<u16, _CRC32DIW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32DIW1;
#[doc = "`read()` method returns [crc32diw1::R](crc32diw1::R) reader structure"]
impl crate::Readable for CRC32DIW1 {}
#[doc = "`write(|w| ..)` method takes [crc32diw1::W](crc32diw1::W) writer structure"]
impl crate::Writable for CRC32DIW1 {}
#[doc = "CRC32 Data Input Word 1"]
pub mod crc32diw1;
#[doc = "CRC32 Data In Reverse Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32dirbw1](crc32dirbw1) module"]
pub type CRC32DIRBW1 = crate::Reg<u16, _CRC32DIRBW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32DIRBW1;
#[doc = "`read()` method returns [crc32dirbw1::R](crc32dirbw1::R) reader structure"]
impl crate::Readable for CRC32DIRBW1 {}
#[doc = "`write(|w| ..)` method takes [crc32dirbw1::W](crc32dirbw1::W) writer structure"]
impl crate::Writable for CRC32DIRBW1 {}
#[doc = "CRC32 Data In Reverse Word 1"]
pub mod crc32dirbw1;
#[doc = "CRC32 Data In Reverse Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32dirbw0](crc32dirbw0) module"]
pub type CRC32DIRBW0 = crate::Reg<u16, _CRC32DIRBW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32DIRBW0;
#[doc = "`read()` method returns [crc32dirbw0::R](crc32dirbw0::R) reader structure"]
impl crate::Readable for CRC32DIRBW0 {}
#[doc = "`write(|w| ..)` method takes [crc32dirbw0::W](crc32dirbw0::W) writer structure"]
impl crate::Writable for CRC32DIRBW0 {}
#[doc = "CRC32 Data In Reverse Word 0"]
pub mod crc32dirbw0;
#[doc = "CRC32 Initialization and Result Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32iniresw0](crc32iniresw0) module"]
pub type CRC32INIRESW0 = crate::Reg<u16, _CRC32INIRESW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32INIRESW0;
#[doc = "`read()` method returns [crc32iniresw0::R](crc32iniresw0::R) reader structure"]
impl crate::Readable for CRC32INIRESW0 {}
#[doc = "`write(|w| ..)` method takes [crc32iniresw0::W](crc32iniresw0::W) writer structure"]
impl crate::Writable for CRC32INIRESW0 {}
#[doc = "CRC32 Initialization and Result Word 0"]
pub mod crc32iniresw0;
#[doc = "CRC32 Initialization and Result Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32iniresw1](crc32iniresw1) module"]
pub type CRC32INIRESW1 = crate::Reg<u16, _CRC32INIRESW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32INIRESW1;
#[doc = "`read()` method returns [crc32iniresw1::R](crc32iniresw1::R) reader structure"]
impl crate::Readable for CRC32INIRESW1 {}
#[doc = "`write(|w| ..)` method takes [crc32iniresw1::W](crc32iniresw1::W) writer structure"]
impl crate::Writable for CRC32INIRESW1 {}
#[doc = "CRC32 Initialization and Result Word 1"]
pub mod crc32iniresw1;
#[doc = "CRC32 Result Reverse Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32resrw1](crc32resrw1) module"]
pub type CRC32RESRW1 = crate::Reg<u16, _CRC32RESRW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32RESRW1;
#[doc = "`read()` method returns [crc32resrw1::R](crc32resrw1::R) reader structure"]
impl crate::Readable for CRC32RESRW1 {}
#[doc = "`write(|w| ..)` method takes [crc32resrw1::W](crc32resrw1::W) writer structure"]
impl crate::Writable for CRC32RESRW1 {}
#[doc = "CRC32 Result Reverse Word 1"]
pub mod crc32resrw1;
#[doc = "CRC32 Result Reverse Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32resrw0](crc32resrw0) module"]
pub type CRC32RESRW0 = crate::Reg<u16, _CRC32RESRW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC32RESRW0;
#[doc = "`read()` method returns [crc32resrw0::R](crc32resrw0::R) reader structure"]
impl crate::Readable for CRC32RESRW0 {}
#[doc = "`write(|w| ..)` method takes [crc32resrw0::W](crc32resrw0::W) writer structure"]
impl crate::Writable for CRC32RESRW0 {}
#[doc = "CRC32 Result Reverse Word 0"]
pub mod crc32resrw0;
#[doc = "CRC16 Data Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16diw0](crc16diw0) module"]
pub type CRC16DIW0 = crate::Reg<u16, _CRC16DIW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC16DIW0;
#[doc = "`read()` method returns [crc16diw0::R](crc16diw0::R) reader structure"]
impl crate::Readable for CRC16DIW0 {}
#[doc = "`write(|w| ..)` method takes [crc16diw0::W](crc16diw0::W) writer structure"]
impl crate::Writable for CRC16DIW0 {}
#[doc = "CRC16 Data Input"]
pub mod crc16diw0;
#[doc = "CRC16 Data In Reverse\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16dirbw0](crc16dirbw0) module"]
pub type CRC16DIRBW0 = crate::Reg<u16, _CRC16DIRBW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC16DIRBW0;
#[doc = "`read()` method returns [crc16dirbw0::R](crc16dirbw0::R) reader structure"]
impl crate::Readable for CRC16DIRBW0 {}
#[doc = "`write(|w| ..)` method takes [crc16dirbw0::W](crc16dirbw0::W) writer structure"]
impl crate::Writable for CRC16DIRBW0 {}
#[doc = "CRC16 Data In Reverse"]
pub mod crc16dirbw0;
#[doc = "CRC16 Init and Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16iniresw0](crc16iniresw0) module"]
pub type CRC16INIRESW0 = crate::Reg<u16, _CRC16INIRESW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC16INIRESW0;
#[doc = "`read()` method returns [crc16iniresw0::R](crc16iniresw0::R) reader structure"]
impl crate::Readable for CRC16INIRESW0 {}
#[doc = "`write(|w| ..)` method takes [crc16iniresw0::W](crc16iniresw0::W) writer structure"]
impl crate::Writable for CRC16INIRESW0 {}
#[doc = "CRC16 Init and Result"]
pub mod crc16iniresw0;
#[doc = "CRC16 Result Reverse\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16resrw0](crc16resrw0) module"]
pub type CRC16RESRW0 = crate::Reg<u16, _CRC16RESRW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC16RESRW0;
#[doc = "`read()` method returns [crc16resrw0::R](crc16resrw0::R) reader structure"]
impl crate::Readable for CRC16RESRW0 {}
#[doc = "`write(|w| ..)` method takes [crc16resrw0::W](crc16resrw0::W) writer structure"]
impl crate::Writable for CRC16RESRW0 {}
#[doc = "CRC16 Result Reverse"]
pub mod crc16resrw0;
