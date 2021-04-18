#[doc = "Reader of register RTCAMINHR"]
pub type R = crate::R<u16, super::RTCAMINHR>;
#[doc = "Writer for register RTCAMINHR"]
pub type W = crate::W<u16, super::RTCAMINHR>;
#[doc = "Register RTCAMINHR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCAMINHR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MIN`"]
pub type MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN`"]
pub struct MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
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
#[doc = "Reader of field `HOUR`"]
pub type HOUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOUR`"]
pub struct HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
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
    #[doc = "Bits 0:5 - 5:0\\]
Minutes (0 to 59)"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Alarm enable"]
    #[inline(always)]
    pub fn minae(&self) -> MINAE_R {
        MINAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Hours (0 to 23)"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Alarm enable"]
    #[inline(always)]
    pub fn hourae(&self) -> HOURAE_R {
        HOURAE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minutes (0 to 59)"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W {
        MIN_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Alarm enable"]
    #[inline(always)]
    pub fn minae(&mut self) -> MINAE_W {
        MINAE_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
Hours (0 to 23)"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Alarm enable"]
    #[inline(always)]
    pub fn hourae(&mut self) -> HOURAE_W {
        HOURAE_W { w: self }
    }
}
