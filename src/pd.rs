#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 96usize],
    #[doc = "0x60 - Port D Input"]
    pub pdin: PDIN,
    #[doc = "0x62 - Port D Output"]
    pub pdout: PDOUT,
    #[doc = "0x64 - Port D Direction"]
    pub pddir: PDDIR,
    #[doc = "0x66 - Port D Resistor Enable"]
    pub pdren: PDREN,
    _reserved4: [u8; 2usize],
    #[doc = "0x6a - Port D Select 0"]
    pub pdsel0: PDSEL0,
    #[doc = "0x6c - Port D Select 1"]
    pub pdsel1: PDSEL1,
    _reserved6: [u8; 8usize],
    #[doc = "0x76 - Port D Complement Select"]
    pub pdselc: PDSELC,
    #[doc = "0x78 - Port D Interrupt Edge Select"]
    pub pdies: PDIES,
    #[doc = "0x7a - Port D Interrupt Enable"]
    pub pdie: PDIE,
    #[doc = "0x7c - Port D Interrupt Flag"]
    pub pdifg: PDIFG,
}
#[doc = "Port D Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdin](pdin) module"]
pub type PDIN = crate::Reg<u16, _PDIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIN;
#[doc = "`read()` method returns [pdin::R](pdin::R) reader structure"]
impl crate::Readable for PDIN {}
#[doc = "`write(|w| ..)` method takes [pdin::W](pdin::W) writer structure"]
impl crate::Writable for PDIN {}
#[doc = "Port D Input"]
pub mod pdin;
#[doc = "Port D Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdout](pdout) module"]
pub type PDOUT = crate::Reg<u16, _PDOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDOUT;
#[doc = "`read()` method returns [pdout::R](pdout::R) reader structure"]
impl crate::Readable for PDOUT {}
#[doc = "`write(|w| ..)` method takes [pdout::W](pdout::W) writer structure"]
impl crate::Writable for PDOUT {}
#[doc = "Port D Output"]
pub mod pdout;
#[doc = "Port D Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddir](pddir) module"]
pub type PDDIR = crate::Reg<u16, _PDDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDDIR;
#[doc = "`read()` method returns [pddir::R](pddir::R) reader structure"]
impl crate::Readable for PDDIR {}
#[doc = "`write(|w| ..)` method takes [pddir::W](pddir::W) writer structure"]
impl crate::Writable for PDDIR {}
#[doc = "Port D Direction"]
pub mod pddir;
#[doc = "Port D Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdren](pdren) module"]
pub type PDREN = crate::Reg<u16, _PDREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDREN;
#[doc = "`read()` method returns [pdren::R](pdren::R) reader structure"]
impl crate::Readable for PDREN {}
#[doc = "`write(|w| ..)` method takes [pdren::W](pdren::W) writer structure"]
impl crate::Writable for PDREN {}
#[doc = "Port D Resistor Enable"]
pub mod pdren;
#[doc = "Port D Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsel0](pdsel0) module"]
pub type PDSEL0 = crate::Reg<u16, _PDSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSEL0;
#[doc = "`read()` method returns [pdsel0::R](pdsel0::R) reader structure"]
impl crate::Readable for PDSEL0 {}
#[doc = "`write(|w| ..)` method takes [pdsel0::W](pdsel0::W) writer structure"]
impl crate::Writable for PDSEL0 {}
#[doc = "Port D Select 0"]
pub mod pdsel0;
#[doc = "Port D Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsel1](pdsel1) module"]
pub type PDSEL1 = crate::Reg<u16, _PDSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSEL1;
#[doc = "`read()` method returns [pdsel1::R](pdsel1::R) reader structure"]
impl crate::Readable for PDSEL1 {}
#[doc = "`write(|w| ..)` method takes [pdsel1::W](pdsel1::W) writer structure"]
impl crate::Writable for PDSEL1 {}
#[doc = "Port D Select 1"]
pub mod pdsel1;
#[doc = "Port D Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdselc](pdselc) module"]
pub type PDSELC = crate::Reg<u16, _PDSELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSELC;
#[doc = "`read()` method returns [pdselc::R](pdselc::R) reader structure"]
impl crate::Readable for PDSELC {}
#[doc = "`write(|w| ..)` method takes [pdselc::W](pdselc::W) writer structure"]
impl crate::Writable for PDSELC {}
#[doc = "Port D Complement Select"]
pub mod pdselc;
#[doc = "Port D Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdies](pdies) module"]
pub type PDIES = crate::Reg<u16, _PDIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIES;
#[doc = "`read()` method returns [pdies::R](pdies::R) reader structure"]
impl crate::Readable for PDIES {}
#[doc = "`write(|w| ..)` method takes [pdies::W](pdies::W) writer structure"]
impl crate::Writable for PDIES {}
#[doc = "Port D Interrupt Edge Select"]
pub mod pdies;
#[doc = "Port D Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdie](pdie) module"]
pub type PDIE = crate::Reg<u16, _PDIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIE;
#[doc = "`read()` method returns [pdie::R](pdie::R) reader structure"]
impl crate::Readable for PDIE {}
#[doc = "`write(|w| ..)` method takes [pdie::W](pdie::W) writer structure"]
impl crate::Writable for PDIE {}
#[doc = "Port D Interrupt Enable"]
pub mod pdie;
#[doc = "Port D Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdifg](pdifg) module"]
pub type PDIFG = crate::Reg<u16, _PDIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIFG;
#[doc = "`read()` method returns [pdifg::R](pdifg::R) reader structure"]
impl crate::Readable for PDIFG {}
#[doc = "`write(|w| ..)` method takes [pdifg::W](pdifg::W) writer structure"]
impl crate::Writable for PDIFG {}
#[doc = "Port D Interrupt Flag"]
pub mod pdifg;
