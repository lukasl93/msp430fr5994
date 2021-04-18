#[doc = "Reader of register RTCADOWDAY_BCD"]
pub type R = crate::R<u16, super::RTCADOWDAY_BCD>;
#[doc = "Writer for register RTCADOWDAY_BCD"]
pub type W = crate::W<u16, super::RTCADOWDAY_BCD>;
#[doc = "Register RTCADOWDAY_BCD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCADOWDAY_BCD {
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
#[doc = "Reader of field `DAY_LD`"]
pub type DAY_LD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAY_LD`"]
pub struct DAY_LD_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_LD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DAY_HD`"]
pub type DAY_HD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAY_HD`"]
pub struct DAY_HD_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_HD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
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
    #[doc = "Bits 8:11 - 11:8\\]
Day of month low digit (0 to 9)"]
    #[inline(always)]
    pub fn day_ld(&self) -> DAY_LD_R {
        DAY_LD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Day of month high digit (0 to 3)"]
    #[inline(always)]
    pub fn day_hd(&self) -> DAY_HD_R {
        DAY_HD_R::new(((self.bits >> 12) & 0x03) as u8)
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
    #[doc = "Bits 8:11 - 11:8\\]
Day of month low digit (0 to 9)"]
    #[inline(always)]
    pub fn day_ld(&mut self) -> DAY_LD_W {
        DAY_LD_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Day of month high digit (0 to 3)"]
    #[inline(always)]
    pub fn day_hd(&mut self) -> DAY_HD_W {
        DAY_HD_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Alarm enable"]
    #[inline(always)]
    pub fn dayae(&mut self) -> DAYAE_W {
        DAYAE_W { w: self }
    }
}
