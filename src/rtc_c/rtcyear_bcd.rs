#[doc = "Reader of register RTCYEAR_BCD"]
pub type R = crate::R<u16, super::RTCYEAR_BCD>;
#[doc = "Writer for register RTCYEAR_BCD"]
pub type W = crate::W<u16, super::RTCYEAR_BCD>;
#[doc = "Register RTCYEAR_BCD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCYEAR_BCD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `YEAR`"]
pub type YEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YEAR`"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DECADE`"]
pub type DECADE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECADE`"]
pub struct DECADE_W<'a> {
    w: &'a mut W,
}
impl<'a> DECADE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CENTURYLOWDIGIT`"]
pub type CENTURYLOWDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CENTURYLOWDIGIT`"]
pub struct CENTURYLOWDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTURYLOWDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CENTURYHIGHDIGIT`"]
pub type CENTURYHIGHDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CENTURYHIGHDIGIT`"]
pub struct CENTURYHIGHDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTURYHIGHDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Year lowest digit (0 to 9)"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Decade (0 to 9)"]
    #[inline(always)]
    pub fn decade(&self) -> DECADE_R {
        DECADE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Century low digit (0 to 9)"]
    #[inline(always)]
    pub fn centurylowdigit(&self) -> CENTURYLOWDIGIT_R {
        CENTURYLOWDIGIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Century high digit (0 to 4)"]
    #[inline(always)]
    pub fn centuryhighdigit(&self) -> CENTURYHIGHDIGIT_R {
        CENTURYHIGHDIGIT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Year lowest digit (0 to 9)"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Decade (0 to 9)"]
    #[inline(always)]
    pub fn decade(&mut self) -> DECADE_W {
        DECADE_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Century low digit (0 to 9)"]
    #[inline(always)]
    pub fn centurylowdigit(&mut self) -> CENTURYLOWDIGIT_W {
        CENTURYLOWDIGIT_W { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\]
Century high digit (0 to 4)"]
    #[inline(always)]
    pub fn centuryhighdigit(&mut self) -> CENTURYHIGHDIGIT_W {
        CENTURYHIGHDIGIT_W { w: self }
    }
}
