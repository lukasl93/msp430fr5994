#[doc = "Reader of register RTCADOWDAY"]
pub type R = crate::R<u16, super::RTCADOWDAY>;
#[doc = "Writer for register RTCADOWDAY"]
pub type W = crate::W<u16, super::RTCADOWDAY>;
#[doc = "Register RTCADOWDAY `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCADOWDAY {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOW`"]
pub type DOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOW`"]
pub struct DOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DOWAE`"]
pub type DOWAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOWAE`"]
pub struct DOWAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWAE_W<'a> {
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
#[doc = "Reader of field `DAY`"]
pub type DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAY`"]
pub struct DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DAYAE`"]
pub type DAYAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAYAE`"]
pub struct DAYAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYAE_W<'a> {
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
    #[doc = "Bits 0:2 - 2:0\\]
Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Alarm enable"]
    #[inline(always)]
    pub fn dowae(&self) -> DOWAE_R {
        DOWAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Alarm enable"]
    #[inline(always)]
    pub fn dayae(&self) -> DAYAE_R {
        DAYAE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dow(&mut self) -> DOW_W {
        DOW_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Alarm enable"]
    #[inline(always)]
    pub fn dowae(&mut self) -> DOWAE_W {
        DOWAE_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Alarm enable"]
    #[inline(always)]
    pub fn dayae(&mut self) -> DAYAE_W {
        DAYAE_W { w: self }
    }
}
