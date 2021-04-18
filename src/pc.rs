#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 64usize],
    #[doc = "0x40 - Port C Input"]
    pub pcin: PCIN,
    #[doc = "0x42 - Port C Output"]
    pub pcout: PCOUT,
    #[doc = "0x44 - Port C Direction"]
    pub pcdir: PCDIR,
    #[doc = "0x46 - Port C Resistor Enable"]
    pub pcren: PCREN,
    _reserved4: [u8; 2usize],
    #[doc = "0x4a - Port C Select 0"]
    pub pcsel0: PCSEL0,
    #[doc = "0x4c - Port C Select 1"]
    pub pcsel1: PCSEL1,
    _reserved6: [u8; 8usize],
    #[doc = "0x56 - Port C Complement Select"]
    pub pcselc: PCSELC,
    #[doc = "0x58 - Port C Interrupt Edge Select"]
    pub pcies: PCIES,
    #[doc = "0x5a - Port C Interrupt Enable"]
    pub pcie: PCIE,
    #[doc = "0x5c - Port C Interrupt Flag"]
    pub pcifg: PCIFG,
}
#[doc = "Port C Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcin](pcin) module"]
pub type PCIN = crate::Reg<u16, _PCIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCIN;
#[doc = "`read()` method returns [pcin::R](pcin::R) reader structure"]
impl crate::Readable for PCIN {}
#[doc = "`write(|w| ..)` method takes [pcin::W](pcin::W) writer structure"]
impl crate::Writable for PCIN {}
#[doc = "Port C Input"]
pub mod pcin;
#[doc = "Port C Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcout](pcout) module"]
pub type PCOUT = crate::Reg<u16, _PCOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCOUT;
#[doc = "`read()` method returns [pcout::R](pcout::R) reader structure"]
impl crate::Readable for PCOUT {}
#[doc = "`write(|w| ..)` method takes [pcout::W](pcout::W) writer structure"]
impl crate::Writable for PCOUT {}
#[doc = "Port C Output"]
pub mod pcout;
#[doc = "Port C Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdir](pcdir) module"]
pub type PCDIR = crate::Reg<u16, _PCDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCDIR;
#[doc = "`read()` method returns [pcdir::R](pcdir::R) reader structure"]
impl crate::Readable for PCDIR {}
#[doc = "`write(|w| ..)` method takes [pcdir::W](pcdir::W) writer structure"]
impl crate::Writable for PCDIR {}
#[doc = "Port C Direction"]
pub mod pcdir;
#[doc = "Port C Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcren](pcren) module"]
pub type PCREN = crate::Reg<u16, _PCREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCREN;
#[doc = "`read()` method returns [pcren::R](pcren::R) reader structure"]
impl crate::Readable for PCREN {}
#[doc = "`write(|w| ..)` method takes [pcren::W](pcren::W) writer structure"]
impl crate::Writable for PCREN {}
#[doc = "Port C Resistor Enable"]
pub mod pcren;
#[doc = "Port C Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsel0](pcsel0) module"]
pub type PCSEL0 = crate::Reg<u16, _PCSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSEL0;
#[doc = "`read()` method returns [pcsel0::R](pcsel0::R) reader structure"]
impl crate::Readable for PCSEL0 {}
#[doc = "`write(|w| ..)` method takes [pcsel0::W](pcsel0::W) writer structure"]
impl crate::Writable for PCSEL0 {}
#[doc = "Port C Select 0"]
pub mod pcsel0;
#[doc = "Port C Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsel1](pcsel1) module"]
pub type PCSEL1 = crate::Reg<u16, _PCSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSEL1;
#[doc = "`read()` method returns [pcsel1::R](pcsel1::R) reader structure"]
impl crate::Readable for PCSEL1 {}
#[doc = "`write(|w| ..)` method takes [pcsel1::W](pcsel1::W) writer structure"]
impl crate::Writable for PCSEL1 {}
#[doc = "Port C Select 1"]
pub mod pcsel1;
#[doc = "Port C Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcselc](pcselc) module"]
pub type PCSELC = crate::Reg<u16, _PCSELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSELC;
#[doc = "`read()` method returns [pcselc::R](pcselc::R) reader structure"]
impl crate::Readable for PCSELC {}
#[doc = "`write(|w| ..)` method takes [pcselc::W](pcselc::W) writer structure"]
impl crate::Writable for PCSELC {}
#[doc = "Port C Complement Select"]
pub mod pcselc;
#[doc = "Port C Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcies](pcies) module"]
pub type PCIES = crate::Reg<u16, _PCIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCIES;
#[doc = "`read()` method returns [pcies::R](pcies::R) reader structure"]
impl crate::Readable for PCIES {}
#[doc = "`write(|w| ..)` method takes [pcies::W](pcies::W) writer structure"]
impl crate::Writable for PCIES {}
#[doc = "Port C Interrupt Edge Select"]
pub mod pcies;
#[doc = "Port C Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcie](pcie) module"]
pub type PCIE = crate::Reg<u16, _PCIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCIE;
#[doc = "`read()` method returns [pcie::R](pcie::R) reader structure"]
impl crate::Readable for PCIE {}
#[doc = "`write(|w| ..)` method takes [pcie::W](pcie::W) writer structure"]
impl crate::Writable for PCIE {}
#[doc = "Port C Interrupt Enable"]
pub mod pcie;
#[doc = "Port C Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcifg](pcifg) module"]
pub type PCIFG = crate::Reg<u16, _PCIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCIFG;
#[doc = "`read()` method returns [pcifg::R](pcifg::R) reader structure"]
impl crate::Readable for PCIFG {}
#[doc = "`write(|w| ..)` method takes [pcifg::W](pcifg::W) writer structure"]
impl crate::Writable for PCIFG {}
#[doc = "Port C Interrupt Flag"]
pub mod pcifg;
