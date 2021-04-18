#[doc = "Reader of register RTCAMINHR_BCD"]
pub type R = crate::R<u16, super::RTCAMINHR_BCD>;
#[doc = "Writer for register RTCAMINHR_BCD"]
pub type W = crate::W<u16, super::RTCAMINHR_BCD>;
#[doc = "Register RTCAMINHR_BCD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCAMINHR_BCD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
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
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `MINAE`"]
pub type MINAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINAE`"]
pub struct MINAE_W<'a> {
    w: &'a mut W,
}
impl<'a> MINAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `HOURSLOWDIGIT`"]
pub type HOURSLOWDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOURSLOWDIGIT`"]
pub struct HOURSLOWDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURSLOWDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HOURSHIGHDIGIT`"]
pub type HOURSHIGHDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOURSHIGHDIGIT`"]
pub struct HOURSHIGHDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURSHIGHDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `HOURAE`"]
pub type HOURAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOURAE`"]
pub struct HOURAE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minutes low digit (0 to 9)"]
    #[inline(always)]
    pub fn minuteslowdigit(&self) -> MINUTESLOWDIGIT_R {
        MINUTESLOWDIGIT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Minutes high digit (0 to 5)"]
    #[inline(always)]
    pub fn minuteshighdigit(&self) -> MINUTESHIGHDIGIT_R {
        MINUTESHIGHDIGIT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Alarm enable"]
    #[inline(always)]
    pub fn minae(&self) -> MINAE_R {
        MINAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Hours low digit (0 to 9)"]
    #[inline(always)]
    pub fn hourslowdigit(&self) -> HOURSLOWDIGIT_R {
        HOURSLOWDIGIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Hours high digit (0 to 2)"]
    #[inline(always)]
    pub fn hourshighdigit(&self) -> HOURSHIGHDIGIT_R {
        HOURSHIGHDIGIT_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Alarm enable"]
    #[inline(always)]
    pub fn hourae(&self) -> HOURAE_R {
        HOURAE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Minutes low digit (0 to 9)"]
    #[inline(always)]
    pub fn minuteslowdigit(&mut self) -> MINUTESLOWDIGIT_W {
        MINUTESLOWDIGIT_W { w: self }
    }
    #[doc = "Bits 4:6 - 6:4\\]
Minutes high digit (0 to 5)"]
    #[inline(always)]
    pub fn minuteshighdigit(&mut self) -> MINUTESHIGHDIGIT_W {
        MINUTESHIGHDIGIT_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Alarm enable"]
    #[inline(always)]
    pub fn minae(&mut self) -> MINAE_W {
        MINAE_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Hours low digit (0 to 9)"]
    #[inline(always)]
    pub fn hourslowdigit(&mut self) -> HOURSLOWDIGIT_W {
        HOURSLOWDIGIT_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Hours high digit (0 to 2)"]
    #[inline(always)]
    pub fn hourshighdigit(&mut self) -> HOURSHIGHDIGIT_W {
        HOURSHIGHDIGIT_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Alarm enable"]
    #[inline(always)]
    pub fn hourae(&mut self) -> HOURAE_W {
        HOURAE_W { w: self }
    }
}
