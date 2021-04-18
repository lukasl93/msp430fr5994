#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    pub cectl0: CECTL0,
    #[doc = "0x02 - Comparator Control Register 1"]
    pub cectl1: CECTL1,
    #[doc = "0x04 - Comparator Control Register 2"]
    pub cectl2: CECTL2,
    #[doc = "0x06 - Comparator Control Register 3"]
    pub cectl3: CECTL3,
    _reserved4: [u8; 4usize],
    #[doc = "0x0c - Comparator Interrupt Control Register"]
    pub ceint: CEINT,
    #[doc = "0x0e - Comparator Interrupt Vector Word Register"]
    pub ceiv: CEIV,
}
#[doc = "Comparator Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cectl0](cectl0) module"]
pub type CECTL0 = crate::Reg<u16, _CECTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CECTL0;
#[doc = "`read()` method returns [cectl0::R](cectl0::R) reader structure"]
impl crate::Readable for CECTL0 {}
#[doc = "`write(|w| ..)` method takes [cectl0::W](cectl0::W) writer structure"]
impl crate::Writable for CECTL0 {}
#[doc = "Comparator Control Register 0"]
pub mod cectl0;
#[doc = "Comparator Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cectl1](cectl1) module"]
pub type CECTL1 = crate::Reg<u16, _CECTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CECTL1;
#[doc = "`read()` method returns [cectl1::R](cectl1::R) reader structure"]
impl crate::Readable for CECTL1 {}
#[doc = "`write(|w| ..)` method takes [cectl1::W](cectl1::W) writer structure"]
impl crate::Writable for CECTL1 {}
#[doc = "Comparator Control Register 1"]
pub mod cectl1;
#[doc = "Comparator Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cectl2](cectl2) module"]
pub type CECTL2 = crate::Reg<u16, _CECTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CECTL2;
#[doc = "`read()` method returns [cectl2::R](cectl2::R) reader structure"]
impl crate::Readable for CECTL2 {}
#[doc = "`write(|w| ..)` method takes [cectl2::W](cectl2::W) writer structure"]
impl crate::Writable for CECTL2 {}
#[doc = "Comparator Control Register 2"]
pub mod cectl2;
#[doc = "Comparator Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cectl3](cectl3) module"]
pub type CECTL3 = crate::Reg<u16, _CECTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CECTL3;
#[doc = "`read()` method returns [cectl3::R](cectl3::R) reader structure"]
impl crate::Readable for CECTL3 {}
#[doc = "`write(|w| ..)` method takes [cectl3::W](cectl3::W) writer structure"]
impl crate::Writable for CECTL3 {}
#[doc = "Comparator Control Register 3"]
pub mod cectl3;
#[doc = "Comparator Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceint](ceint) module"]
pub type CEINT = crate::Reg<u16, _CEINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEINT;
#[doc = "`read()` method returns [ceint::R](ceint::R) reader structure"]
impl crate::Readable for CEINT {}
#[doc = "`write(|w| ..)` method takes [ceint::W](ceint::W) writer structure"]
impl crate::Writable for CEINT {}
#[doc = "Comparator Interrupt Control Register"]
pub mod ceint;
#[doc = "Comparator Interrupt Vector Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceiv](ceiv) module"]
pub type CEIV = crate::Reg<u16, _CEIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEIV;
#[doc = "`read()` method returns [ceiv::R](ceiv::R) reader structure"]
impl crate::Readable for CEIV {}
#[doc = "`write(|w| ..)` method takes [ceiv::W](ceiv::W) writer structure"]
impl crate::Writable for CEIV {}
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod ceiv;
