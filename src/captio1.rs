#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 14usize],
    #[doc = "0x0e - Capacitive Touch IO 0 Control Register"]
    pub captio1ctl: CAPTIO1CTL,
}
#[doc = "Capacitive Touch IO 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [captio1ctl](captio1ctl) module"]
pub type CAPTIO1CTL = crate::Reg<u16, _CAPTIO1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTIO1CTL;
#[doc = "`read()` method returns [captio1ctl::R](captio1ctl::R) reader structure"]
impl crate::Readable for CAPTIO1CTL {}
#[doc = "`write(|w| ..)` method takes [captio1ctl::W](captio1ctl::W) writer structure"]
impl crate::Writable for CAPTIO1CTL {}
#[doc = "Capacitive Touch IO 0 Control Register"]
pub mod captio1ctl;
