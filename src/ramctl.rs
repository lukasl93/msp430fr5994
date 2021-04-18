#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RAM Controller Control 0"]
    pub rcctl0: RCCTL0,
}
#[doc = "RAM Controller Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcctl0](rcctl0) module"]
pub type RCCTL0 = crate::Reg<u16, _RCCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCCTL0;
#[doc = "`read()` method returns [rcctl0::R](rcctl0::R) reader structure"]
impl crate::Readable for RCCTL0 {}
#[doc = "`write(|w| ..)` method takes [rcctl0::W](rcctl0::W) writer structure"]
impl crate::Writable for RCCTL0 {}
#[doc = "RAM Controller Control 0"]
pub mod rcctl0;
