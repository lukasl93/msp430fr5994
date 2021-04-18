#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 32usize],
    #[doc = "0x20 - Port B Input"]
    pub pbin: PBIN,
    #[doc = "0x22 - Port B Output"]
    pub pbout: PBOUT,
    #[doc = "0x24 - Port B Direction"]
    pub pbdir: PBDIR,
    #[doc = "0x26 - Port B Resistor Enable"]
    pub pbren: PBREN,
    _reserved4: [u8; 2usize],
    #[doc = "0x2a - Port B Select 0"]
    pub pbsel0: PBSEL0,
    #[doc = "0x2c - Port B Select 1"]
    pub pbsel1: PBSEL1,
    _reserved6: [u8; 8usize],
    #[doc = "0x36 - Port B Complement Select"]
    pub pbselc: PBSELC,
    #[doc = "0x38 - Port B Interrupt Edge Select"]
    pub pbies: PBIES,
    #[doc = "0x3a - Port B Interrupt Enable"]
    pub pbie: PBIE,
    #[doc = "0x3c - Port B Interrupt Flag"]
    pub pbifg: PBIFG,
}
#[doc = "Port B Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbin](pbin) module"]
pub type PBIN = crate::Reg<u16, _PBIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBIN;
#[doc = "`read()` method returns [pbin::R](pbin::R) reader structure"]
impl crate::Readable for PBIN {}
#[doc = "`write(|w| ..)` method takes [pbin::W](pbin::W) writer structure"]
impl crate::Writable for PBIN {}
#[doc = "Port B Input"]
pub mod pbin;
#[doc = "Port B Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbout](pbout) module"]
pub type PBOUT = crate::Reg<u16, _PBOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBOUT;
#[doc = "`read()` method returns [pbout::R](pbout::R) reader structure"]
impl crate::Readable for PBOUT {}
#[doc = "`write(|w| ..)` method takes [pbout::W](pbout::W) writer structure"]
impl crate::Writable for PBOUT {}
#[doc = "Port B Output"]
pub mod pbout;
#[doc = "Port B Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdir](pbdir) module"]
pub type PBDIR = crate::Reg<u16, _PBDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBDIR;
#[doc = "`read()` method returns [pbdir::R](pbdir::R) reader structure"]
impl crate::Readable for PBDIR {}
#[doc = "`write(|w| ..)` method takes [pbdir::W](pbdir::W) writer structure"]
impl crate::Writable for PBDIR {}
#[doc = "Port B Direction"]
pub mod pbdir;
#[doc = "Port B Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbren](pbren) module"]
pub type PBREN = crate::Reg<u16, _PBREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBREN;
#[doc = "`read()` method returns [pbren::R](pbren::R) reader structure"]
impl crate::Readable for PBREN {}
#[doc = "`write(|w| ..)` method takes [pbren::W](pbren::W) writer structure"]
impl crate::Writable for PBREN {}
#[doc = "Port B Resistor Enable"]
pub mod pbren;
#[doc = "Port B Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbsel0](pbsel0) module"]
pub type PBSEL0 = crate::Reg<u16, _PBSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBSEL0;
#[doc = "`read()` method returns [pbsel0::R](pbsel0::R) reader structure"]
impl crate::Readable for PBSEL0 {}
#[doc = "`write(|w| ..)` method takes [pbsel0::W](pbsel0::W) writer structure"]
impl crate::Writable for PBSEL0 {}
#[doc = "Port B Select 0"]
pub mod pbsel0;
#[doc = "Port B Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbsel1](pbsel1) module"]
pub type PBSEL1 = crate::Reg<u16, _PBSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBSEL1;
#[doc = "`read()` method returns [pbsel1::R](pbsel1::R) reader structure"]
impl crate::Readable for PBSEL1 {}
#[doc = "`write(|w| ..)` method takes [pbsel1::W](pbsel1::W) writer structure"]
impl crate::Writable for PBSEL1 {}
#[doc = "Port B Select 1"]
pub mod pbsel1;
#[doc = "Port B Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbselc](pbselc) module"]
pub type PBSELC = crate::Reg<u16, _PBSELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBSELC;
#[doc = "`read()` method returns [pbselc::R](pbselc::R) reader structure"]
impl crate::Readable for PBSELC {}
#[doc = "`write(|w| ..)` method takes [pbselc::W](pbselc::W) writer structure"]
impl crate::Writable for PBSELC {}
#[doc = "Port B Complement Select"]
pub mod pbselc;
#[doc = "Port B Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbies](pbies) module"]
pub type PBIES = crate::Reg<u16, _PBIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBIES;
#[doc = "`read()` method returns [pbies::R](pbies::R) reader structure"]
impl crate::Readable for PBIES {}
#[doc = "`write(|w| ..)` method takes [pbies::W](pbies::W) writer structure"]
impl crate::Writable for PBIES {}
#[doc = "Port B Interrupt Edge Select"]
pub mod pbies;
#[doc = "Port B Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbie](pbie) module"]
pub type PBIE = crate::Reg<u16, _PBIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBIE;
#[doc = "`read()` method returns [pbie::R](pbie::R) reader structure"]
impl crate::Readable for PBIE {}
#[doc = "`write(|w| ..)` method takes [pbie::W](pbie::W) writer structure"]
impl crate::Writable for PBIE {}
#[doc = "Port B Interrupt Enable"]
pub mod pbie;
#[doc = "Port B Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbifg](pbifg) module"]
pub type PBIFG = crate::Reg<u16, _PBIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBIFG;
#[doc = "`read()` method returns [pbifg::R](pbifg::R) reader structure"]
impl crate::Readable for PBIFG {}
#[doc = "`write(|w| ..)` method takes [pbifg::W](pbifg::W) writer structure"]
impl crate::Writable for PBIFG {}
#[doc = "Port B Interrupt Flag"]
pub mod pbifg;
