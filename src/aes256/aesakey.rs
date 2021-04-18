#[doc = "Reader of register AESAKEY"]
pub type R = crate::R<u16, super::AESAKEY>;
#[doc = "Writer for register AESAKEY"]
pub type W = crate::W<u16, super::AESAKEY>;
#[doc = "Register AESAKEY `reset()`'s with value 0"]
impl crate::ResetValue for super::AESAKEY {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESKEY0`"]
pub type AESKEY0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESKEY0`"]
pub struct AESKEY0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `AESKEY1`"]
pub type AESKEY1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESKEY1`"]
pub struct AESKEY1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
AES key byte n when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey0(&self) -> AESKEY0_R {
        AESKEY0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES key byte n+1 when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey1(&self) -> AESKEY1_R {
        AESKEY1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
AES key byte n when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey0(&mut self) -> AESKEY0_W {
        AESKEY0_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES key byte n+1 when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey1(&mut self) -> AESKEY1_W {
        AESKEY1_W { w: self }
    }
}
