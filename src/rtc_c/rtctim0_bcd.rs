#[doc = "Reader of register RTCTIM0_BCD"]
pub type R = crate::R<u16, super::RTCTIM0_BCD>;
#[doc = "Writer for register RTCTIM0_BCD"]
pub type W = crate::W<u16, super::RTCTIM0_BCD>;
#[doc = "Register RTCTIM0_BCD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCTIM0_BCD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECONDSLOWDIGIT`"]
pub type SECONDSLOWDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECONDSLOWDIGIT`"]
pub struct SECONDSLOWDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDSLOWDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SECONDSHIGHDIGIT`"]
pub type SECONDSHIGHDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECONDSHIGHDIGIT`"]
pub struct SECONDSHIGHDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDSHIGHDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `MINUTESLOWDIGIT`"]
pub type MINUTESLOWDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINUTESLOWDIGIT`"]
pub struct MINUTESLOWDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTESLOWDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MINUTESHIGHDIGIT`"]
pub type MINUTESHIGHDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINUTESHIGHDIGIT`"]
pub struct MINUTESHIGHDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTESHIGHDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Seconds low digit (0 to 9)"]
    #[inline(always)]
    pub fn secondslowdigit(&self) -> SECONDSLOWDIGIT_R {
        SECONDSLOWDIGIT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Seconds high digit (0 to 5)"]
    #[inline(always)]
    pub fn secondshighdigit(&self) -> SECONDSHIGHDIGIT_R {
        SECONDSHIGHDIGIT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Minutes low digit (0 to 9)"]
    #[inline(always)]
    pub fn minuteslowdigit(&self) -> MINUTESLOWDIGIT_R {
        MINUTESLOWDIGIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Minutes high digit (0 to 5)"]
    #[inline(always)]
    pub fn minuteshighdigit(&self) -> MINUTESHIGHDIGIT_R {
        MINUTESHIGHDIGIT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Seconds low digit (0 to 9)"]
    #[inline(always)]
    pub fn secondslowdigit(&mut self) -> SECONDSLOWDIGIT_W {
        SECONDSLOWDIGIT_W { w: self }
    }
    #[doc = "Bits 4:6 - 6:4\\]
Seconds high digit (0 to 5)"]
    #[inline(always)]
    pub fn secondshighdigit(&mut self) -> SECONDSHIGHDIGIT_W {
        SECONDSHIGHDIGIT_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Minutes low digit (0 to 9)"]
    #[inline(always)]
    pub fn minuteslowdigit(&mut self) -> MINUTESLOWDIGIT_W {
        MINUTESLOWDIGIT_W { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\]
Minutes high digit (0 to 5)"]
    #[inline(always)]
    pub fn minuteshighdigit(&mut self) -> MINUTESHIGHDIGIT_W {
        MINUTESHIGHDIGIT_W { w: self }
    }
}
