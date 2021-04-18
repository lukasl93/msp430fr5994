#[doc = "Reader of register PCIE"]
pub type R = crate::R<u16, super::PCIE>;
#[doc = "Writer for register PCIE"]
pub type W = crate::W<u16, super::PCIE>;
#[doc = "Register PCIE `reset()`'s with value 0"]
impl crate::ResetValue for super::PCIE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
