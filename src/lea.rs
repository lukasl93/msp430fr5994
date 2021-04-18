#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LEA Capability Register"]
    pub leacap: LEACAP,
    #[doc = "0x04 - Configuration Register 0"]
    pub leacnf0: LEACNF0,
    #[doc = "0x08 - Configuration Register 1"]
    pub leacnf1: LEACNF1,
    #[doc = "0x0c - Configuration Register 2"]
    pub leacnf2: LEACNF2,
    #[doc = "0x10 - Memory Bottom Register"]
    pub leamb: LEAMB,
    #[doc = "0x14 - Memory Top Register"]
    pub leamt: LEAMT,
    #[doc = "0x18 - Code Memory Access Register"]
    pub leacma: LEACMA,
    #[doc = "0x1c - Code Memory Control Register"]
    pub leacmctl: LEACMCTL,
    _reserved8: [u8; 8usize],
    #[doc = "0x28 - LEA Command Status Register"]
    pub leacmdstat: LEACMDSTAT,
    #[doc = "0x2c - LEA Source 1 Status Register"]
    pub leas1stat: LEAS1STAT,
    #[doc = "0x30 - LEA Source 0 Status Register"]
    pub leas0stat: LEAS0STAT,
    #[doc = "0x34 - LEA Result Status Register"]
    pub leadststat: LEADSTSTAT,
    _reserved12: [u8; 8usize],
    #[doc = "0x40 - PM Control Register"]
    pub leapmctl: LEAPMCTL,
    #[doc = "0x44 - PM Result Register"]
    pub leapmdst: LEAPMDST,
    #[doc = "0x48 - PM Source 1 Register"]
    pub leapms1: LEAPMS1,
    #[doc = "0x4c - PM Source 0 Register"]
    pub leapms0: LEAPMS0,
    #[doc = "0x50 - PM Command Buffer Register"]
    pub leapmcb: LEAPMCB,
    _reserved17: [u8; 28usize],
    #[doc = "0x70 - Interrupt Flag and Set Register"]
    pub leaifgset: LEAIFGSET,
    #[doc = "0x74 - Interrupt Enable Register"]
    pub leaie: LEAIE,
    #[doc = "0x78 - Interrupt Flag and Clear Register"]
    pub leaifg: LEAIFG,
    #[doc = "0x7c - Interrupt Vector Register"]
    pub leaiv: LEAIV,
}
#[doc = "LEA Capability Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leacap](leacap) module"]
pub type LEACAP = crate::Reg<u32, _LEACAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEACAP;
#[doc = "`read()` method returns [leacap::R](leacap::R) reader structure"]
impl crate::Readable for LEACAP {}
#[doc = "`write(|w| ..)` method takes [leacap::W](leacap::W) writer structure"]
impl crate::Writable for LEACAP {}
#[doc = "LEA Capability Register"]
pub mod leacap;
#[doc = "Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leacnf0](leacnf0) module"]
pub type LEACNF0 = crate::Reg<u32, _LEACNF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEACNF0;
#[doc = "`read()` method returns [leacnf0::R](leacnf0::R) reader structure"]
impl crate::Readable for LEACNF0 {}
#[doc = "`write(|w| ..)` method takes [leacnf0::W](leacnf0::W) writer structure"]
impl crate::Writable for LEACNF0 {}
#[doc = "Configuration Register 0"]
pub mod leacnf0;
#[doc = "Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leacnf1](leacnf1) module"]
pub type LEACNF1 = crate::Reg<u32, _LEACNF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEACNF1;
#[doc = "`read()` method returns [leacnf1::R](leacnf1::R) reader structure"]
impl crate::Readable for LEACNF1 {}
#[doc = "`write(|w| ..)` method takes [leacnf1::W](leacnf1::W) writer structure"]
impl crate::Writable for LEACNF1 {}
#[doc = "Configuration Register 1"]
pub mod leacnf1;
#[doc = "Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leacnf2](leacnf2) module"]
pub type LEACNF2 = crate::Reg<u32, _LEACNF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEACNF2;
#[doc = "`read()` method returns [leacnf2::R](leacnf2::R) reader structure"]
impl crate::Readable for LEACNF2 {}
#[doc = "`write(|w| ..)` method takes [leacnf2::W](leacnf2::W) writer structure"]
impl crate::Writable for LEACNF2 {}
#[doc = "Configuration Register 2"]
pub mod leacnf2;
#[doc = "Memory Bottom Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leamb](leamb) module"]
pub type LEAMB = crate::Reg<u32, _LEAMB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAMB;
#[doc = "`read()` method returns [leamb::R](leamb::R) reader structure"]
impl crate::Readable for LEAMB {}
#[doc = "`write(|w| ..)` method takes [leamb::W](leamb::W) writer structure"]
impl crate::Writable for LEAMB {}
#[doc = "Memory Bottom Register"]
pub mod leamb;
#[doc = "Memory Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leamt](leamt) module"]
pub type LEAMT = crate::Reg<u32, _LEAMT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAMT;
#[doc = "`read()` method returns [leamt::R](leamt::R) reader structure"]
impl crate::Readable for LEAMT {}
#[doc = "`write(|w| ..)` method takes [leamt::W](leamt::W) writer structure"]
impl crate::Writable for LEAMT {}
#[doc = "Memory Top Register"]
pub mod leamt;
#[doc = "Code Memory Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leacma](leacma) module"]
pub type LEACMA = crate::Reg<u32, _LEACMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEACMA;
#[doc = "`read()` method returns [leacma::R](leacma::R) reader structure"]
impl crate::Readable for LEACMA {}
#[doc = "`write(|w| ..)` method takes [leacma::W](leacma::W) writer structure"]
impl crate::Writable for LEACMA {}
#[doc = "Code Memory Access Register"]
pub mod leacma;
#[doc = "Code Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leacmctl](leacmctl) module"]
pub type LEACMCTL = crate::Reg<u32, _LEACMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEACMCTL;
#[doc = "`read()` method returns [leacmctl::R](leacmctl::R) reader structure"]
impl crate::Readable for LEACMCTL {}
#[doc = "`write(|w| ..)` method takes [leacmctl::W](leacmctl::W) writer structure"]
impl crate::Writable for LEACMCTL {}
#[doc = "Code Memory Control Register"]
pub mod leacmctl;
#[doc = "LEA Command Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leacmdstat](leacmdstat) module"]
pub type LEACMDSTAT = crate::Reg<u32, _LEACMDSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEACMDSTAT;
#[doc = "`read()` method returns [leacmdstat::R](leacmdstat::R) reader structure"]
impl crate::Readable for LEACMDSTAT {}
#[doc = "`write(|w| ..)` method takes [leacmdstat::W](leacmdstat::W) writer structure"]
impl crate::Writable for LEACMDSTAT {}
#[doc = "LEA Command Status Register"]
pub mod leacmdstat;
#[doc = "LEA Source 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leas1stat](leas1stat) module"]
pub type LEAS1STAT = crate::Reg<u32, _LEAS1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAS1STAT;
#[doc = "`read()` method returns [leas1stat::R](leas1stat::R) reader structure"]
impl crate::Readable for LEAS1STAT {}
#[doc = "`write(|w| ..)` method takes [leas1stat::W](leas1stat::W) writer structure"]
impl crate::Writable for LEAS1STAT {}
#[doc = "LEA Source 1 Status Register"]
pub mod leas1stat;
#[doc = "LEA Source 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leas0stat](leas0stat) module"]
pub type LEAS0STAT = crate::Reg<u32, _LEAS0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAS0STAT;
#[doc = "`read()` method returns [leas0stat::R](leas0stat::R) reader structure"]
impl crate::Readable for LEAS0STAT {}
#[doc = "`write(|w| ..)` method takes [leas0stat::W](leas0stat::W) writer structure"]
impl crate::Writable for LEAS0STAT {}
#[doc = "LEA Source 0 Status Register"]
pub mod leas0stat;
#[doc = "LEA Result Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leadststat](leadststat) module"]
pub type LEADSTSTAT = crate::Reg<u32, _LEADSTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEADSTSTAT;
#[doc = "`read()` method returns [leadststat::R](leadststat::R) reader structure"]
impl crate::Readable for LEADSTSTAT {}
#[doc = "`write(|w| ..)` method takes [leadststat::W](leadststat::W) writer structure"]
impl crate::Writable for LEADSTSTAT {}
#[doc = "LEA Result Status Register"]
pub mod leadststat;
#[doc = "PM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leapmctl](leapmctl) module"]
pub type LEAPMCTL = crate::Reg<u32, _LEAPMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAPMCTL;
#[doc = "`read()` method returns [leapmctl::R](leapmctl::R) reader structure"]
impl crate::Readable for LEAPMCTL {}
#[doc = "`write(|w| ..)` method takes [leapmctl::W](leapmctl::W) writer structure"]
impl crate::Writable for LEAPMCTL {}
#[doc = "PM Control Register"]
pub mod leapmctl;
#[doc = "PM Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leapmdst](leapmdst) module"]
pub type LEAPMDST = crate::Reg<u32, _LEAPMDST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAPMDST;
#[doc = "`read()` method returns [leapmdst::R](leapmdst::R) reader structure"]
impl crate::Readable for LEAPMDST {}
#[doc = "`write(|w| ..)` method takes [leapmdst::W](leapmdst::W) writer structure"]
impl crate::Writable for LEAPMDST {}
#[doc = "PM Result Register"]
pub mod leapmdst;
#[doc = "PM Source 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leapms1](leapms1) module"]
pub type LEAPMS1 = crate::Reg<u32, _LEAPMS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAPMS1;
#[doc = "`read()` method returns [leapms1::R](leapms1::R) reader structure"]
impl crate::Readable for LEAPMS1 {}
#[doc = "`write(|w| ..)` method takes [leapms1::W](leapms1::W) writer structure"]
impl crate::Writable for LEAPMS1 {}
#[doc = "PM Source 1 Register"]
pub mod leapms1;
#[doc = "PM Source 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leapms0](leapms0) module"]
pub type LEAPMS0 = crate::Reg<u32, _LEAPMS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAPMS0;
#[doc = "`read()` method returns [leapms0::R](leapms0::R) reader structure"]
impl crate::Readable for LEAPMS0 {}
#[doc = "`write(|w| ..)` method takes [leapms0::W](leapms0::W) writer structure"]
impl crate::Writable for LEAPMS0 {}
#[doc = "PM Source 0 Register"]
pub mod leapms0;
#[doc = "PM Command Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leapmcb](leapmcb) module"]
pub type LEAPMCB = crate::Reg<u32, _LEAPMCB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAPMCB;
#[doc = "`read()` method returns [leapmcb::R](leapmcb::R) reader structure"]
impl crate::Readable for LEAPMCB {}
#[doc = "`write(|w| ..)` method takes [leapmcb::W](leapmcb::W) writer structure"]
impl crate::Writable for LEAPMCB {}
#[doc = "PM Command Buffer Register"]
pub mod leapmcb;
#[doc = "Interrupt Flag and Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leaifgset](leaifgset) module"]
pub type LEAIFGSET = crate::Reg<u32, _LEAIFGSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAIFGSET;
#[doc = "`read()` method returns [leaifgset::R](leaifgset::R) reader structure"]
impl crate::Readable for LEAIFGSET {}
#[doc = "`write(|w| ..)` method takes [leaifgset::W](leaifgset::W) writer structure"]
impl crate::Writable for LEAIFGSET {}
#[doc = "Interrupt Flag and Set Register"]
pub mod leaifgset;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leaie](leaie) module"]
pub type LEAIE = crate::Reg<u32, _LEAIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAIE;
#[doc = "`read()` method returns [leaie::R](leaie::R) reader structure"]
impl crate::Readable for LEAIE {}
#[doc = "`write(|w| ..)` method takes [leaie::W](leaie::W) writer structure"]
impl crate::Writable for LEAIE {}
#[doc = "Interrupt Enable Register"]
pub mod leaie;
#[doc = "Interrupt Flag and Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leaifg](leaifg) module"]
pub type LEAIFG = crate::Reg<u32, _LEAIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAIFG;
#[doc = "`read()` method returns [leaifg::R](leaifg::R) reader structure"]
impl crate::Readable for LEAIFG {}
#[doc = "`write(|w| ..)` method takes [leaifg::W](leaifg::W) writer structure"]
impl crate::Writable for LEAIFG {}
#[doc = "Interrupt Flag and Clear Register"]
pub mod leaifg;
#[doc = "Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [leaiv](leaiv) module"]
pub type LEAIV = crate::Reg<u32, _LEAIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEAIV;
#[doc = "`read()` method returns [leaiv::R](leaiv::R) reader structure"]
impl crate::Readable for LEAIV {}
#[doc = "`write(|w| ..)` method takes [leaiv::W](leaiv::W) writer structure"]
impl crate::Writable for LEAIV {}
#[doc = "Interrupt Vector Register"]
pub mod leaiv;
