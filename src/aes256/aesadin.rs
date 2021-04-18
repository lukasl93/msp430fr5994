#[doc = "Reader of register AESADIN"]
pub type R = crate::R<u16, super::AESADIN>;
#[doc = "Writer for register AESADIN"]
pub type W = crate::W<u16, super::AESADIN>;
#[doc = "Register AESADIN `reset()`'s with value 0"]
impl crate::ResetValue for super::AESADIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESDIN0`"]
pub type AESDIN0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESDIN0`"]
pub struct AESDIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `AESDIN1`"]
pub type AESDIN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESDIN1`"]
pub struct AESDIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
AES data in byte n when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin0(&self) -> AESDIN0_R {
        AESDIN0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES data in byte n+1 when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin1(&self) -> AESDIN1_R {
        AESDIN1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
AES data in byte n when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin0(&mut self) -> AESDIN0_W {
        AESDIN0_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES data in byte n+1 when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin1(&mut self) -> AESDIN1_W {
        AESDIN1_W { w: self }
    }
}
