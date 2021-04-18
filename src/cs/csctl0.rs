#[doc = "Reader of register CSCTL0"]
pub type R = crate::R<u16, super::CSCTL0>;
#[doc = "Writer for register CSCTL0"]
pub type W = crate::W<u16, super::CSCTL0>;
#[doc = "Register CSCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSKEY`"]
pub type CSKEY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSKEY`"]
pub struct CSKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> CSKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
CSKEY password"]
    #[inline(always)]
    pub fn cskey(&self) -> CSKEY_R {
        CSKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
CSKEY password"]
    #[inline(always)]
    pub fn cskey(&mut self) -> CSKEY_W {
        CSKEY_W { w: self }
    }
}
