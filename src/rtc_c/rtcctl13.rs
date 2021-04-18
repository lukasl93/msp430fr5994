#[doc = "Reader of register RTCCTL13"]
pub type R = crate::R<u16, super::RTCCTL13>;
#[doc = "Writer for register RTCCTL13"]
pub type W = crate::W<u16, super::RTCCTL13>;
#[doc = "Register RTCCTL13 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCTL13 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "1:0\\]
Real-time clock time event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCTEV_A {
    #[doc = "0: Minute changed"]
    MIN = 0,
    #[doc = "1: Hour changed"]
    HOUR = 1,
    #[doc = "2: Every day at midnight (00:00)"]
    _0000 = 2,
    #[doc = "3: Every day at noon (12:00)"]
    _1200 = 3,
}
impl From<RTCTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCTEV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCTEV`"]
pub type RTCTEV_R = crate::R<u8, RTCTEV_A>;
impl RTCTEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTEV_A {
        match self.bits {
            0 => RTCTEV_A::MIN,
            1 => RTCTEV_A::HOUR,
            2 => RTCTEV_A::_0000,
            3 => RTCTEV_A::_1200,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == RTCTEV_A::MIN
    }
    #[doc = "Checks if the value of the field is `HOUR`"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == RTCTEV_A::HOUR
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == RTCTEV_A::_0000
    }
    #[doc = "Checks if the value of the field is `_1200`"]
    #[inline(always)]
    pub fn is_1200(&self) -> bool {
        *self == RTCTEV_A::_1200
    }
}
#[doc = "Write proxy for field `RTCTEV`"]
pub struct RTCTEV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCTEV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Minute changed"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(RTCTEV_A::MIN)
    }
    #[doc = "Hour changed"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut W {
        self.variant(RTCTEV_A::HOUR)
    }
    #[doc = "Every day at midnight (00:00)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(RTCTEV_A::_0000)
    }
    #[doc = "Every day at noon (12:00)"]
    #[inline(always)]
    pub fn _1200(self) -> &'a mut W {
        self.variant(RTCTEV_A::_1200)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "3:2\\]
Real-time clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSSEL_A {
    #[doc = "0: 32-kHz crystal oscillator clock"]
    LFXT = 0,
    #[doc = "1: 32-kHz crystal oscillator clock"]
    LFXT = 1,
    #[doc = "2: Output from RT1PS"]
    RT1PS = 2,
    #[doc = "3: Output from RT1PS"]
    RT1PS = 3,
}
impl From<RTCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCSSEL`"]
pub type RTCSSEL_R = crate::R<u8, RTCSSEL_A>;
impl RTCSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSSEL_A {
        match self.bits {
            0 => RTCSSEL_A::LFXT,
            1 => RTCSSEL_A::LFXT,
            2 => RTCSSEL_A::RT1PS,
            3 => RTCSSEL_A::RT1PS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LFXT`"]
    #[inline(always)]
    pub fn is_lfxt(&self) -> bool {
        *self == RTCSSEL_A::LFXT
    }
    #[doc = "Checks if the value of the field is `LFXT`"]
    #[inline(always)]
    pub fn is_lfxt(&self) -> bool {
        *self == RTCSSEL_A::LFXT
    }
    #[doc = "Checks if the value of the field is `RT1PS`"]
    #[inline(always)]
    pub fn is_rt1ps(&self) -> bool {
        *self == RTCSSEL_A::RT1PS
    }
    #[doc = "Checks if the value of the field is `RT1PS`"]
    #[inline(always)]
    pub fn is_rt1ps(&self) -> bool {
        *self == RTCSSEL_A::RT1PS
    }
}
#[doc = "Write proxy for field `RTCSSEL`"]
pub struct RTCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn lfxt(self) -> &'a mut W {
        self.variant(RTCSSEL_A::LFXT)
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn lfxt(self) -> &'a mut W {
        self.variant(RTCSSEL_A::LFXT)
    }
    #[doc = "Output from RT1PS"]
    #[inline(always)]
    pub fn rt1ps(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RT1PS)
    }
    #[doc = "Output from RT1PS"]
    #[inline(always)]
    pub fn rt1ps(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RT1PS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "4:4\\]
Real-time clock ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCRDY_A {
    #[doc = "0: RTC time values in transition"]
    RTCRDY_0 = 0,
    #[doc = "1: RTC time values safe for reading. This bit indicates when the real-time clock time values are safe for reading."]
    RTCRDY_1 = 1,
}
impl From<RTCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RTCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCRDY`"]
pub type RTCRDY_R = crate::R<bool, RTCRDY_A>;
impl RTCRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCRDY_A {
        match self.bits {
            false => RTCRDY_A::RTCRDY_0,
            true => RTCRDY_A::RTCRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCRDY_0`"]
    #[inline(always)]
    pub fn is_rtcrdy_0(&self) -> bool {
        *self == RTCRDY_A::RTCRDY_0
    }
    #[doc = "Checks if the value of the field is `RTCRDY_1`"]
    #[inline(always)]
    pub fn is_rtcrdy_1(&self) -> bool {
        *self == RTCRDY_A::RTCRDY_1
    }
}
#[doc = "Write proxy for field `RTCRDY`"]
pub struct RTCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC time values in transition"]
    #[inline(always)]
    pub fn rtcrdy_0(self) -> &'a mut W {
        self.variant(RTCRDY_A::RTCRDY_0)
    }
    #[doc = "RTC time values safe for reading. This bit indicates when the real-time clock time values are safe for reading."]
    #[inline(always)]
    pub fn rtcrdy_1(self) -> &'a mut W {
        self.variant(RTCRDY_A::RTCRDY_1)
    }
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCMODE_A {
    #[doc = "1: Calendar mode. Always reads a value of 1."]
    RTCMODE_1 = 1,
}
impl From<RTCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCMODE`"]
pub type RTCMODE_R = crate::R<bool, RTCMODE_A>;
impl RTCMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RTCMODE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RTCMODE_A::RTCMODE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RTCMODE_1`"]
    #[inline(always)]
    pub fn is_rtcmode_1(&self) -> bool {
        *self == RTCMODE_A::RTCMODE_1
    }
}
#[doc = "Write proxy for field `RTCMODE`"]
pub struct RTCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calendar mode. Always reads a value of 1."]
    #[inline(always)]
    pub fn rtcmode_1(self) -> &'a mut W {
        self.variant(RTCMODE_A::RTCMODE_1)
    }
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
#[doc = "6:6\\]
Real-time clock hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCHOLD_A {
    #[doc = "0: Real-time clock is operational"]
    RTCHOLD_0 = 0,
    #[doc = "1: When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    RTCHOLD_1 = 1,
}
impl From<RTCHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: RTCHOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCHOLD`"]
pub type RTCHOLD_R = crate::R<bool, RTCHOLD_A>;
impl RTCHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCHOLD_A {
        match self.bits {
            false => RTCHOLD_A::RTCHOLD_0,
            true => RTCHOLD_A::RTCHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCHOLD_0`"]
    #[inline(always)]
    pub fn is_rtchold_0(&self) -> bool {
        *self == RTCHOLD_A::RTCHOLD_0
    }
    #[doc = "Checks if the value of the field is `RTCHOLD_1`"]
    #[inline(always)]
    pub fn is_rtchold_1(&self) -> bool {
        *self == RTCHOLD_A::RTCHOLD_1
    }
}
#[doc = "Write proxy for field `RTCHOLD`"]
pub struct RTCHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCHOLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Real-time clock is operational"]
    #[inline(always)]
    pub fn rtchold_0(self) -> &'a mut W {
        self.variant(RTCHOLD_A::RTCHOLD_0)
    }
    #[doc = "When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    #[inline(always)]
    pub fn rtchold_1(self) -> &'a mut W {
        self.variant(RTCHOLD_A::RTCHOLD_1)
    }
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
#[doc = "7:7\\]
Real-time clock BCD select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCBCD_A {
    #[doc = "0: Binary (hexadecimal) code selected"]
    HEX = 0,
    #[doc = "1: Binary coded decimal (BCD) code selected"]
    BCD = 1,
}
impl From<RTCBCD_A> for bool {
    #[inline(always)]
    fn from(variant: RTCBCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCBCD`"]
pub type RTCBCD_R = crate::R<bool, RTCBCD_A>;
impl RTCBCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCBCD_A {
        match self.bits {
            false => RTCBCD_A::HEX,
            true => RTCBCD_A::BCD,
        }
    }
    #[doc = "Checks if the value of the field is `HEX`"]
    #[inline(always)]
    pub fn is_hex(&self) -> bool {
        *self == RTCBCD_A::HEX
    }
    #[doc = "Checks if the value of the field is `BCD`"]
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == RTCBCD_A::BCD
    }
}
#[doc = "Write proxy for field `RTCBCD`"]
pub struct RTCBCD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCBCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCBCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Binary (hexadecimal) code selected"]
    #[inline(always)]
    pub fn hex(self) -> &'a mut W {
        self.variant(RTCBCD_A::HEX)
    }
    #[doc = "Binary coded decimal (BCD) code selected"]
    #[inline(always)]
    pub fn bcd(self) -> &'a mut W {
        self.variant(RTCBCD_A::BCD)
    }
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
#[doc = "9:8\\]
Real-time clock calibration frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCCALF_A {
    #[doc = "0: No frequency output to RTCCLK pin"]
    NONE = 0,
    #[doc = "1: 512 Hz"]
    _512 = 1,
    #[doc = "2: 256 Hz"]
    _256 = 2,
    #[doc = "3: 1 Hz"]
    _1 = 3,
}
impl From<RTCCALF_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCCALF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCCALF`"]
pub type RTCCALF_R = crate::R<u8, RTCCALF_A>;
impl RTCCALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCALF_A {
        match self.bits {
            0 => RTCCALF_A::NONE,
            1 => RTCCALF_A::_512,
            2 => RTCCALF_A::_256,
            3 => RTCCALF_A::_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RTCCALF_A::NONE
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == RTCCALF_A::_512
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == RTCCALF_A::_256
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCCALF_A::_1
    }
}
#[doc = "Write proxy for field `RTCCALF`"]
pub struct RTCCALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCALF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No frequency output to RTCCLK pin"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RTCCALF_A::NONE)
    }
    #[doc = "512 Hz"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(RTCCALF_A::_512)
    }
    #[doc = "256 Hz"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(RTCCALF_A::_256)
    }
    #[doc = "1 Hz"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCALF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Real-time clock time event"]
    #[inline(always)]
    pub fn rtctev(&self) -> RTCTEV_R {
        RTCTEV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Real-time clock source select"]
    #[inline(always)]
    pub fn rtcssel(&self) -> RTCSSEL_R {
        RTCSSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Real-time clock ready"]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RTCRDY_R {
        RTCRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtcmode(&self) -> RTCMODE_R {
        RTCMODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Real-time clock hold"]
    #[inline(always)]
    pub fn rtchold(&self) -> RTCHOLD_R {
        RTCHOLD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Real-time clock BCD select"]
    #[inline(always)]
    pub fn rtcbcd(&self) -> RTCBCD_R {
        RTCBCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Real-time clock calibration frequency"]
    #[inline(always)]
    pub fn rtccalf(&self) -> RTCCALF_R {
        RTCCALF_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Real-time clock time event"]
    #[inline(always)]
    pub fn rtctev(&mut self) -> RTCTEV_W {
        RTCTEV_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Real-time clock source select"]
    #[inline(always)]
    pub fn rtcssel(&mut self) -> RTCSSEL_W {
        RTCSSEL_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Real-time clock ready"]
    #[inline(always)]
    pub fn rtcrdy(&mut self) -> RTCRDY_W {
        RTCRDY_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtcmode(&mut self) -> RTCMODE_W {
        RTCMODE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Real-time clock hold"]
    #[inline(always)]
    pub fn rtchold(&mut self) -> RTCHOLD_W {
        RTCHOLD_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Real-time clock BCD select"]
    #[inline(always)]
    pub fn rtcbcd(&mut self) -> RTCBCD_W {
        RTCBCD_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Real-time clock calibration frequency"]
    #[inline(always)]
    pub fn rtccalf(&mut self) -> RTCCALF_W {
        RTCCALF_W { w: self }
    }
}
