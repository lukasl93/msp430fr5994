#[doc = "Reader of register AESAXIN"]
pub type R = crate::R<u16, super::AESAXIN>;
#[doc = "Writer for register AESAXIN"]
pub type W = crate::W<u16, super::AESAXIN>;
#[doc = "Register AESAXIN `reset()`'s with value 0"]
impl crate::ResetValue for super::AESAXIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESXIN0`"]
pub type AESXIN0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESXIN0`"]
pub struct AESXIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `AESXIN1`"]
pub type AESXIN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESXIN1`"]
pub struct AESXIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
AES data in byte n when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin0(&self) -> AESXIN0_R {
        AESXIN0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES data in byte n+1 when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin1(&self) -> AESXIN1_R {
        AESXIN1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
AES data in byte n when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin0(&mut self) -> AESXIN0_W {
        AESXIN0_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES data in byte n+1 when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin1(&mut self) -> AESXIN1_W {
        AESXIN1_W { w: self }
    }
}
