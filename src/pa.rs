#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port A Input"]
    pub pain: PAIN,
    #[doc = "0x02 - Port A Output"]
    pub paout: PAOUT,
    #[doc = "0x04 - Port A Direction"]
    pub padir: PADIR,
    #[doc = "0x06 - Port A Resistor Enable"]
    pub paren: PAREN,
    _reserved4: [u8; 2usize],
    #[doc = "0x0a - Port A Select 0"]
    pub pasel0: PASEL0,
    #[doc = "0x0c - Port A Select 1"]
    pub pasel1: PASEL1,
    _reserved6: [u8; 8usize],
    #[doc = "0x16 - Port A Complement Select"]
    pub paselc: PASELC,
    #[doc = "0x18 - Port A Interrupt Edge Select"]
    pub paies: PAIES,
    #[doc = "0x1a - Port A Interrupt Enable"]
    pub paie: PAIE,
    #[doc = "0x1c - Port A Interrupt Flag"]
    pub paifg: PAIFG,
}
#[doc = "Port A Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pain](pain) module"]
pub type PAIN = crate::Reg<u16, _PAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIN;
#[doc = "`read()` method returns [pain::R](pain::R) reader structure"]
impl crate::Readable for PAIN {}
#[doc = "`write(|w| ..)` method takes [pain::W](pain::W) writer structure"]
impl crate::Writable for PAIN {}
#[doc = "Port A Input"]
pub mod pain;
#[doc = "Port A Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paout](paout) module"]
pub type PAOUT = crate::Reg<u16, _PAOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAOUT;
#[doc = "`read()` method returns [paout::R](paout::R) reader structure"]
impl crate::Readable for PAOUT {}
#[doc = "`write(|w| ..)` method takes [paout::W](paout::W) writer structure"]
impl crate::Writable for PAOUT {}
#[doc = "Port A Output"]
pub mod paout;
#[doc = "Port A Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padir](padir) module"]
pub type PADIR = crate::Reg<u16, _PADIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADIR;
#[doc = "`read()` method returns [padir::R](padir::R) reader structure"]
impl crate::Readable for PADIR {}
#[doc = "`write(|w| ..)` method takes [padir::W](padir::W) writer structure"]
impl crate::Writable for PADIR {}
#[doc = "Port A Direction"]
pub mod padir;
#[doc = "Port A Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paren](paren) module"]
pub type PAREN = crate::Reg<u16, _PAREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAREN;
#[doc = "`read()` method returns [paren::R](paren::R) reader structure"]
impl crate::Readable for PAREN {}
#[doc = "`write(|w| ..)` method takes [paren::W](paren::W) writer structure"]
impl crate::Writable for PAREN {}
#[doc = "Port A Resistor Enable"]
pub mod paren;
#[doc = "Port A Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pasel0](pasel0) module"]
pub type PASEL0 = crate::Reg<u16, _PASEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PASEL0;
#[doc = "`read()` method returns [pasel0::R](pasel0::R) reader structure"]
impl crate::Readable for PASEL0 {}
#[doc = "`write(|w| ..)` method takes [pasel0::W](pasel0::W) writer structure"]
impl crate::Writable for PASEL0 {}
#[doc = "Port A Select 0"]
pub mod pasel0;
#[doc = "Port A Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pasel1](pasel1) module"]
pub type PASEL1 = crate::Reg<u16, _PASEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PASEL1;
#[doc = "`read()` method returns [pasel1::R](pasel1::R) reader structure"]
impl crate::Readable for PASEL1 {}
#[doc = "`write(|w| ..)` method takes [pasel1::W](pasel1::W) writer structure"]
impl crate::Writable for PASEL1 {}
#[doc = "Port A Select 1"]
pub mod pasel1;
#[doc = "Port A Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paselc](paselc) module"]
pub type PASELC = crate::Reg<u16, _PASELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PASELC;
#[doc = "`read()` method returns [paselc::R](paselc::R) reader structure"]
impl crate::Readable for PASELC {}
#[doc = "`write(|w| ..)` method takes [paselc::W](paselc::W) writer structure"]
impl crate::Writable for PASELC {}
#[doc = "Port A Complement Select"]
pub mod paselc;
#[doc = "Port A Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paies](paies) module"]
pub type PAIES = crate::Reg<u16, _PAIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIES;
#[doc = "`read()` method returns [paies::R](paies::R) reader structure"]
impl crate::Readable for PAIES {}
#[doc = "`write(|w| ..)` method takes [paies::W](paies::W) writer structure"]
impl crate::Writable for PAIES {}
#[doc = "Port A Interrupt Edge Select"]
pub mod paies;
#[doc = "Port A Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paie](paie) module"]
pub type PAIE = crate::Reg<u16, _PAIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIE;
#[doc = "`read()` method returns [paie::R](paie::R) reader structure"]
impl crate::Readable for PAIE {}
#[doc = "`write(|w| ..)` method takes [paie::W](paie::W) writer structure"]
impl crate::Writable for PAIE {}
#[doc = "Port A Interrupt Enable"]
pub mod paie;
#[doc = "Port A Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paifg](paifg) module"]
pub type PAIFG = crate::Reg<u16, _PAIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIFG;
#[doc = "`read()` method returns [paifg::R](paifg::R) reader structure"]
impl crate::Readable for PAIFG {}
#[doc = "`write(|w| ..)` method takes [paifg::W](paifg::W) writer structure"]
impl crate::Writable for PAIFG {}
#[doc = "Port A Interrupt Flag"]
pub mod paifg;
