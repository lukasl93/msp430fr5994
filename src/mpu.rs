#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory Protection Unit Control 0"]
    pub mpuctl0: MPUCTL0,
    #[doc = "0x02 - Memory Protection Unit Control 1"]
    pub mpuctl1: MPUCTL1,
    #[doc = "0x04 - Memory Protection Unit Segmentation Border 2 Register"]
    pub mpusegb2: MPUSEGB2,
    #[doc = "0x06 - Memory Protection Unit Segmentation Border 1 Register"]
    pub mpusegb1: MPUSEGB1,
    #[doc = "0x08 - Memory Protection Unit Segmentation Access Management Register"]
    pub mpusam: MPUSAM,
    #[doc = "0x0a - Memory Protection Unit IP Control 0 Register"]
    pub mpuipc0: MPUIPC0,
    #[doc = "0x0c - Memory Protection Unit IP Encapsulation Segment Border 2 Register"]
    pub mpuipsegb2: MPUIPSEGB2,
    #[doc = "0x0e - Memory Protection Unit IP Encapsulation Segment Border 1 Register"]
    pub mpuipsegb1: MPUIPSEGB1,
}
#[doc = "Memory Protection Unit Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpuctl0](mpuctl0) module"]
pub type MPUCTL0 = crate::Reg<u16, _MPUCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPUCTL0;
#[doc = "`read()` method returns [mpuctl0::R](mpuctl0::R) reader structure"]
impl crate::Readable for MPUCTL0 {}
#[doc = "`write(|w| ..)` method takes [mpuctl0::W](mpuctl0::W) writer structure"]
impl crate::Writable for MPUCTL0 {}
#[doc = "Memory Protection Unit Control 0"]
pub mod mpuctl0;
#[doc = "Memory Protection Unit Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpuctl1](mpuctl1) module"]
pub type MPUCTL1 = crate::Reg<u16, _MPUCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPUCTL1;
#[doc = "`read()` method returns [mpuctl1::R](mpuctl1::R) reader structure"]
impl crate::Readable for MPUCTL1 {}
#[doc = "`write(|w| ..)` method takes [mpuctl1::W](mpuctl1::W) writer structure"]
impl crate::Writable for MPUCTL1 {}
#[doc = "Memory Protection Unit Control 1"]
pub mod mpuctl1;
#[doc = "Memory Protection Unit Segmentation Border 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpusegb2](mpusegb2) module"]
pub type MPUSEGB2 = crate::Reg<u16, _MPUSEGB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPUSEGB2;
#[doc = "`read()` method returns [mpusegb2::R](mpusegb2::R) reader structure"]
impl crate::Readable for MPUSEGB2 {}
#[doc = "`write(|w| ..)` method takes [mpusegb2::W](mpusegb2::W) writer structure"]
impl crate::Writable for MPUSEGB2 {}
#[doc = "Memory Protection Unit Segmentation Border 2 Register"]
pub mod mpusegb2;
#[doc = "Memory Protection Unit Segmentation Border 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpusegb1](mpusegb1) module"]
pub type MPUSEGB1 = crate::Reg<u16, _MPUSEGB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPUSEGB1;
#[doc = "`read()` method returns [mpusegb1::R](mpusegb1::R) reader structure"]
impl crate::Readable for MPUSEGB1 {}
#[doc = "`write(|w| ..)` method takes [mpusegb1::W](mpusegb1::W) writer structure"]
impl crate::Writable for MPUSEGB1 {}
#[doc = "Memory Protection Unit Segmentation Border 1 Register"]
pub mod mpusegb1;
#[doc = "Memory Protection Unit Segmentation Access Management Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpusam](mpusam) module"]
pub type MPUSAM = crate::Reg<u16, _MPUSAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPUSAM;
#[doc = "`read()` method returns [mpusam::R](mpusam::R) reader structure"]
impl crate::Readable for MPUSAM {}
#[doc = "`write(|w| ..)` method takes [mpusam::W](mpusam::W) writer structure"]
impl crate::Writable for MPUSAM {}
#[doc = "Memory Protection Unit Segmentation Access Management Register"]
pub mod mpusam;
#[doc = "Memory Protection Unit IP Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpuipc0](mpuipc0) module"]
pub type MPUIPC0 = crate::Reg<u16, _MPUIPC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPUIPC0;
#[doc = "`read()` method returns [mpuipc0::R](mpuipc0::R) reader structure"]
impl crate::Readable for MPUIPC0 {}
#[doc = "`write(|w| ..)` method takes [mpuipc0::W](mpuipc0::W) writer structure"]
impl crate::Writable for MPUIPC0 {}
#[doc = "Memory Protection Unit IP Control 0 Register"]
pub mod mpuipc0;
#[doc = "Memory Protection Unit IP Encapsulation Segment Border 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpuipsegb2](mpuipsegb2) module"]
pub type MPUIPSEGB2 = crate::Reg<u16, _MPUIPSEGB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPUIPSEGB2;
#[doc = "`read()` method returns [mpuipsegb2::R](mpuipsegb2::R) reader structure"]
impl crate::Readable for MPUIPSEGB2 {}
#[doc = "`write(|w| ..)` method takes [mpuipsegb2::W](mpuipsegb2::W) writer structure"]
impl crate::Writable for MPUIPSEGB2 {}
#[doc = "Memory Protection Unit IP Encapsulation Segment Border 2 Register"]
pub mod mpuipsegb2;
#[doc = "Memory Protection Unit IP Encapsulation Segment Border 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpuipsegb1](mpuipsegb1) module"]
pub type MPUIPSEGB1 = crate::Reg<u16, _MPUIPSEGB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPUIPSEGB1;
#[doc = "`read()` method returns [mpuipsegb1::R](mpuipsegb1::R) reader structure"]
impl crate::Readable for MPUIPSEGB1 {}
#[doc = "`write(|w| ..)` method takes [mpuipsegb1::W](mpuipsegb1::W) writer structure"]
impl crate::Writable for MPUIPSEGB1 {}
#[doc = "Memory Protection Unit IP Encapsulation Segment Border 1 Register"]
pub mod mpuipsegb1;
