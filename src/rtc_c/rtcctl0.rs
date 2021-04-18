#[doc = "Reader of register RTCCTL0"]
pub type R = crate::R<u16, super::RTCCTL0>;
#[doc = "Writer for register RTCCTL0"]
pub type W = crate::W<u16, super::RTCCTL0>;
#[doc = "Register RTCCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCRDYIFG`"]
pub type RTCRDYIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCRDYIFG`"]
pub struct RTCRDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCRDYIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RTCAIFG`"]
pub type RTCAIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCAIFG`"]
pub struct RTCAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RTCTEVIFG`"]
pub type RTCTEVIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTEVIFG`"]
pub struct RTCTEVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTEVIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTCOFIFG`"]
pub type RTCOFIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOFIFG`"]
pub struct RTCOFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOFIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RTCRDYIE`"]
pub type RTCRDYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCRDYIE`"]
pub struct RTCRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RTCAIE`"]
pub type RTCAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCAIE`"]
pub struct RTCAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTCTEVIE`"]
pub type RTCTEVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTEVIE`"]
pub struct RTCTEVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTEVIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RTCOFIE`"]
pub type RTCOFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOFIE`"]
pub struct RTCOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOFIE_W<'a> {
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
#[doc = "Reader of field `RTCKEY`"]
pub type RTCKEY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCKEY`"]
pub struct RTCKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Real-time clock ready interrupt flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&self) -> RTCRDYIFG_R {
        RTCRDYIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Real-time clock alarm interrupt flag"]
    #[inline(always)]
    pub fn rtcaifg(&self) -> RTCAIFG_R {
        RTCAIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Real-time clock time event interrupt flag"]
    #[inline(always)]
    pub fn rtctevifg(&self) -> RTCTEVIFG_R {
        RTCTEVIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&self) -> RTCOFIFG_R {
        RTCOFIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Real-time clock ready interrupt enable"]
    #[inline(always)]
    pub fn rtcrdyie(&self) -> RTCRDYIE_R {
        RTCRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Real-time clock alarm interrupt enable"]
    #[inline(always)]
    pub fn rtcaie(&self) -> RTCAIE_R {
        RTCAIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Real-time clock time event interrupt enable"]
    #[inline(always)]
    pub fn rtctevie(&self) -> RTCTEVIE_R {
        RTCTEVIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&self) -> RTCOFIE_R {
        RTCOFIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Real-time clock key"]
    #[inline(always)]
    pub fn rtckey(&self) -> RTCKEY_R {
        RTCKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Real-time clock ready interrupt flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&mut self) -> RTCRDYIFG_W {
        RTCRDYIFG_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Real-time clock alarm interrupt flag"]
    #[inline(always)]
    pub fn rtcaifg(&mut self) -> RTCAIFG_W {
        RTCAIFG_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Real-time clock time event interrupt flag"]
    #[inline(always)]
    pub fn rtctevifg(&mut self) -> RTCTEVIFG_W {
        RTCTEVIFG_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&mut self) -> RTCOFIFG_W {
        RTCOFIFG_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Real-time clock ready interrupt enable"]
    #[inline(always)]
    pub fn rtcrdyie(&mut self) -> RTCRDYIE_W {
        RTCRDYIE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Real-time clock alarm interrupt enable"]
    #[inline(always)]
    pub fn rtcaie(&mut self) -> RTCAIE_W {
        RTCAIE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Real-time clock time event interrupt enable"]
    #[inline(always)]
    pub fn rtctevie(&mut self) -> RTCTEVIE_W {
        RTCTEVIE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&mut self) -> RTCOFIE_W {
        RTCOFIE_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Real-time clock key"]
    #[inline(always)]
    pub fn rtckey(&mut self) -> RTCKEY_W {
        RTCKEY_W { w: self }
    }
}
