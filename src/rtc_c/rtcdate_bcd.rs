#[doc = "Reader of register RTCDATE_BCD"]
pub type R = crate::R<u16, super::RTCDATE_BCD>;
#[doc = "Writer for register RTCDATE_BCD"]
pub type W = crate::W<u16, super::RTCDATE_BCD>;
#[doc = "Register RTCDATE_BCD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCDATE_BCD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAYLOWDIGIT`"]
pub type DAYLOWDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYLOWDIGIT`"]
pub struct DAYLOWDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYLOWDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DAYHIGHDIGIT`"]
pub type DAYHIGHDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYHIGHDIGIT`"]
pub struct DAYHIGHDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYHIGHDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `MONTHLOWDIGIT`"]
pub type MONTHLOWDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MONTHLOWDIGIT`"]
pub struct MONTHLOWDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTHLOWDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MONTHHIGHDIGIT`"]
pub type MONTHHIGHDIGIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTHHIGHDIGIT`"]
pub struct MONTHHIGHDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTHHIGHDIGIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Day of month low digit (0 to 9)"]
    #[inline(always)]
    pub fn daylowdigit(&self) -> DAYLOWDIGIT_R {
        DAYLOWDIGIT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Day of month high digit (0 to 3)"]
    #[inline(always)]
    pub fn dayhighdigit(&self) -> DAYHIGHDIGIT_R {
        DAYHIGHDIGIT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Month low digit (0 to 9)"]
    #[inline(always)]
    pub fn monthlowdigit(&self) -> MONTHLOWDIGIT_R {
        MONTHLOWDIGIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Month high digit (0 or 1)"]
    #[inline(always)]
    pub fn monthhighdigit(&self) -> MONTHHIGHDIGIT_R {
        MONTHHIGHDIGIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Day of month low digit (0 to 9)"]
    #[inline(always)]
    pub fn daylowdigit(&mut self) -> DAYLOWDIGIT_W {
        DAYLOWDIGIT_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Day of month high digit (0 to 3)"]
    #[inline(always)]
    pub fn dayhighdigit(&mut self) -> DAYHIGHDIGIT_W {
        DAYHIGHDIGIT_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Month low digit (0 to 9)"]
    #[inline(always)]
    pub fn monthlowdigit(&mut self) -> MONTHLOWDIGIT_W {
        MONTHLOWDIGIT_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Month high digit (0 or 1)"]
    #[inline(always)]
    pub fn monthhighdigit(&mut self) -> MONTHHIGHDIGIT_W {
        MONTHHIGHDIGIT_W { w: self }
    }
}
