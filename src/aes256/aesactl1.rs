#[doc = "Reader of register AESACTL1"]
pub type R = crate::R<u16, super::AESACTL1>;
#[doc = "Writer for register AESACTL1"]
pub type W = crate::W<u16, super::AESACTL1>;
#[doc = "Register AESACTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AESACTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESBLKCNT`"]
pub type AESBLKCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESBLKCNT`"]
pub struct AESBLKCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcnt(&self) -> AESBLKCNT_R {
        AESBLKCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcnt(&mut self) -> AESBLKCNT_W {
        AESBLKCNT_W { w: self }
    }
}
