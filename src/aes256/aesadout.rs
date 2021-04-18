#[doc = "Reader of register AESADOUT"]
pub type R = crate::R<u16, super::AESADOUT>;
#[doc = "Writer for register AESADOUT"]
pub type W = crate::W<u16, super::AESADOUT>;
#[doc = "Register AESADOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::AESADOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESDOUT0`"]
pub type AESDOUT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESDOUT0`"]
pub struct AESDOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `AESDOUT1`"]
pub type AESDOUT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESDOUT1`"]
pub struct AESDOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
AES data out byte n when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout0(&self) -> AESDOUT0_R {
        AESDOUT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES data out byte n+1 when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout1(&self) -> AESDOUT1_R {
        AESDOUT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
AES data out byte n when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout0(&mut self) -> AESDOUT0_W {
        AESDOUT0_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
AES data out byte n+1 when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout1(&mut self) -> AESDOUT1_W {
        AESDOUT1_W { w: self }
    }
}
