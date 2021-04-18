#[doc = "Reader of register AESAXDIN"]
pub type R = crate::R<u16, super::AESAXDIN>;
#[doc = "Writer for register AESAXDIN"]
pub type W = crate::W<u16, super::AESAXDIN>;
#[doc = "Register AESAXDIN `reset()`'s with value 0"]
impl crate::ResetValue for super::AESAXDIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESXDIN0`"]
pub type AESXDIN0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESXDIN0`"]
pub struct AESXDIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXDIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `AESXDIN1`"]
pub type AESXDIN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESXDIN1`"]
pub struct AESXDIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXDIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
AES data in byte n when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin0(&self) -> AESXDIN0_R {
        AESXDIN0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES data in byte n+1 when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin1(&self) -> AESXDIN1_R {
        AESXDIN1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
AES data in byte n when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin0(&mut self) -> AESXDIN0_W {
        AESXDIN0_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES data in byte n+1 when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin1(&mut self) -> AESXDIN1_W {
        AESXDIN1_W { w: self }
    }
}
