#[doc = "Reader of register RTCPS1CTL"]
pub type R = crate::R<u16, super::RTCPS1CTL>;
#[doc = "Writer for register RTCPS1CTL"]
pub type W = crate::W<u16, super::RTCPS1CTL>;
#[doc = "Register RTCPS1CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCPS1CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
Prescale timer 1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT1PSIFG_A {
    #[doc = "0: No time event occurred"]
    RT1PSIFG_0 = 0,
    #[doc = "1: Time event occurred"]
    RT1PSIFG_1 = 1,
}
impl From<RT1PSIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RT1PSIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RT1PSIFG`"]
pub type RT1PSIFG_R = crate::R<bool, RT1PSIFG_A>;
impl RT1PSIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1PSIFG_A {
        match self.bits {
            false => RT1PSIFG_A::RT1PSIFG_0,
            true => RT1PSIFG_A::RT1PSIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RT1PSIFG_0`"]
    #[inline(always)]
    pub fn is_rt1psifg_0(&self) -> bool {
        *self == RT1PSIFG_A::RT1PSIFG_0
    }
    #[doc = "Checks if the value of the field is `RT1PSIFG_1`"]
    #[inline(always)]
    pub fn is_rt1psifg_1(&self) -> bool {
        *self == RT1PSIFG_A::RT1PSIFG_1
    }
}
#[doc = "Write proxy for field `RT1PSIFG`"]
pub struct RT1PSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1PSIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rt1psifg_0(self) -> &'a mut W {
        self.variant(RT1PSIFG_A::RT1PSIFG_0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rt1psifg_1(self) -> &'a mut W {
        self.variant(RT1PSIFG_A::RT1PSIFG_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "1:1\\]
Prescale timer 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT1PSIE_A {
    #[doc = "0: Interrupt not enabled"]
    DISABLE = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    ENABLE = 1,
}
impl From<RT1PSIE_A> for bool {
    #[inline(always)]
    fn from(variant: RT1PSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RT1PSIE`"]
pub type RT1PSIE_R = crate::R<bool, RT1PSIE_A>;
impl RT1PSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1PSIE_A {
        match self.bits {
            false => RT1PSIE_A::DISABLE,
            true => RT1PSIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RT1PSIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RT1PSIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `RT1PSIE`"]
pub struct RT1PSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1PSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RT1PSIE_A::DISABLE)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RT1PSIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "4:2\\]
Prescale timer 1 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT1IP_A {
    #[doc = "0: Divide by 2"]
    _2 = 0,
    #[doc = "1: Divide by 4"]
    _4 = 1,
    #[doc = "2: Divide by 8"]
    _8 = 2,
    #[doc = "3: Divide by 16"]
    _16 = 3,
    #[doc = "4: Divide by 32"]
    _32 = 4,
    #[doc = "5: Divide by 64"]
    _64 = 5,
    #[doc = "6: Divide by 128"]
    _128 = 6,
    #[doc = "7: Divide by 256"]
    _256 = 7,
}
impl From<RT1IP_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1IP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RT1IP`"]
pub type RT1IP_R = crate::R<u8, RT1IP_A>;
impl RT1IP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1IP_A {
        match self.bits {
            0 => RT1IP_A::_2,
            1 => RT1IP_A::_4,
            2 => RT1IP_A::_8,
            3 => RT1IP_A::_16,
            4 => RT1IP_A::_32,
            5 => RT1IP_A::_64,
            6 => RT1IP_A::_128,
            7 => RT1IP_A::_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RT1IP_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == RT1IP_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == RT1IP_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == RT1IP_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == RT1IP_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == RT1IP_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == RT1IP_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == RT1IP_A::_256
    }
}
#[doc = "Write proxy for field `RT1IP`"]
pub struct RT1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1IP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1IP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RT1IP_A::_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(RT1IP_A::_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(RT1IP_A::_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(RT1IP_A::_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(RT1IP_A::_32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(RT1IP_A::_64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(RT1IP_A::_128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(RT1IP_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u16) & 0x07) << 2);
        self.w
    }
}
#[doc = "8:8\\]
Prescale timer 1 hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT1PSHOLD_A {
    #[doc = "0: RT1PS is operational"]
    RT1PSHOLD_0 = 0,
    #[doc = "1: RT1PS is held"]
    RT1PSHOLD_1 = 1,
}
impl From<RT1PSHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: RT1PSHOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RT1PSHOLD`"]
pub type RT1PSHOLD_R = crate::R<bool, RT1PSHOLD_A>;
impl RT1PSHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1PSHOLD_A {
        match self.bits {
            false => RT1PSHOLD_A::RT1PSHOLD_0,
            true => RT1PSHOLD_A::RT1PSHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RT1PSHOLD_0`"]
    #[inline(always)]
    pub fn is_rt1pshold_0(&self) -> bool {
        *self == RT1PSHOLD_A::RT1PSHOLD_0
    }
    #[doc = "Checks if the value of the field is `RT1PSHOLD_1`"]
    #[inline(always)]
    pub fn is_rt1pshold_1(&self) -> bool {
        *self == RT1PSHOLD_A::RT1PSHOLD_1
    }
}
#[doc = "Write proxy for field `RT1PSHOLD`"]
pub struct RT1PSHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1PSHOLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RT1PS is operational"]
    #[inline(always)]
    pub fn rt1pshold_0(self) -> &'a mut W {
        self.variant(RT1PSHOLD_A::RT1PSHOLD_0)
    }
    #[doc = "RT1PS is held"]
    #[inline(always)]
    pub fn rt1pshold_1(self) -> &'a mut W {
        self.variant(RT1PSHOLD_A::RT1PSHOLD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "13:11\\]
Prescale timer 1 clock divide\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT1PSDIV_A {
    #[doc = "0: Divide by 2"]
    _2 = 0,
    #[doc = "1: Divide by 4"]
    _4 = 1,
    #[doc = "2: Divide by 8"]
    _8 = 2,
    #[doc = "3: Divide by 16"]
    _16 = 3,
    #[doc = "4: Divide by 32"]
    _32 = 4,
    #[doc = "5: Divide by 64"]
    _64 = 5,
    #[doc = "6: Divide by 128"]
    _128 = 6,
    #[doc = "7: Divide by 256"]
    _256 = 7,
}
impl From<RT1PSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1PSDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RT1PSDIV`"]
pub type RT1PSDIV_R = crate::R<u8, RT1PSDIV_A>;
impl RT1PSDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1PSDIV_A {
        match self.bits {
            0 => RT1PSDIV_A::_2,
            1 => RT1PSDIV_A::_4,
            2 => RT1PSDIV_A::_8,
            3 => RT1PSDIV_A::_16,
            4 => RT1PSDIV_A::_32,
            5 => RT1PSDIV_A::_64,
            6 => RT1PSDIV_A::_128,
            7 => RT1PSDIV_A::_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RT1PSDIV_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == RT1PSDIV_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == RT1PSDIV_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == RT1PSDIV_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == RT1PSDIV_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == RT1PSDIV_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == RT1PSDIV_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == RT1PSDIV_A::_256
    }
}
#[doc = "Write proxy for field `RT1PSDIV`"]
pub struct RT1PSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1PSDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::_32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::_64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::_128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(RT1PSDIV_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u16) & 0x07) << 11);
        self.w
    }
}
#[doc = "15:14\\]
Prescale timer 1 clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT1SSEL_A {
    #[doc = "0: 32-kHz crystal oscillator clock"]
    RT1SSEL_0 = 0,
    #[doc = "1: 32-kHz crystal oscillator clock"]
    RT1SSEL_1 = 1,
    #[doc = "2: Output from RT0PS"]
    RT0PS = 2,
    #[doc = "3: Output from RT0PS"]
    RT0PS = 3,
}
impl From<RT1SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1SSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RT1SSEL`"]
pub type RT1SSEL_R = crate::R<u8, RT1SSEL_A>;
impl RT1SSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1SSEL_A {
        match self.bits {
            0 => RT1SSEL_A::RT1SSEL_0,
            1 => RT1SSEL_A::RT1SSEL_1,
            2 => RT1SSEL_A::RT0PS,
            3 => RT1SSEL_A::RT0PS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_0`"]
    #[inline(always)]
    pub fn is_rt1ssel_0(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_0
    }
    #[doc = "Checks if the value of the field is `RT1SSEL_1`"]
    #[inline(always)]
    pub fn is_rt1ssel_1(&self) -> bool {
        *self == RT1SSEL_A::RT1SSEL_1
    }
    #[doc = "Checks if the value of the field is `RT0PS`"]
    #[inline(always)]
    pub fn is_rt0ps(&self) -> bool {
        *self == RT1SSEL_A::RT0PS
    }
    #[doc = "Checks if the value of the field is `RT0PS`"]
    #[inline(always)]
    pub fn is_rt0ps(&self) -> bool {
        *self == RT1SSEL_A::RT0PS
    }
}
#[doc = "Write proxy for field `RT1SSEL`"]
pub struct RT1SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1SSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1SSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn rt1ssel_0(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_0)
    }
    #[doc = "32-kHz crystal oscillator clock"]
    #[inline(always)]
    pub fn rt1ssel_1(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT1SSEL_1)
    }
    #[doc = "Output from RT0PS"]
    #[inline(always)]
    pub fn rt0ps(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT0PS)
    }
    #[doc = "Output from RT0PS"]
    #[inline(always)]
    pub fn rt0ps(self) -> &'a mut W {
        self.variant(RT1SSEL_A::RT0PS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Prescale timer 1 interrupt flag"]
    #[inline(always)]
    pub fn rt1psifg(&self) -> RT1PSIFG_R {
        RT1PSIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Prescale timer 1 interrupt enable"]
    #[inline(always)]
    pub fn rt1psie(&self) -> RT1PSIE_R {
        RT1PSIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&self) -> RT1IP_R {
        RT1IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Prescale timer 1 hold"]
    #[inline(always)]
    pub fn rt1pshold(&self) -> RT1PSHOLD_R {
        RT1PSHOLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Prescale timer 1 clock divide"]
    #[inline(always)]
    pub fn rt1psdiv(&self) -> RT1PSDIV_R {
        RT1PSDIV_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Prescale timer 1 clock source select"]
    #[inline(always)]
    pub fn rt1ssel(&self) -> RT1SSEL_R {
        RT1SSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Prescale timer 1 interrupt flag"]
    #[inline(always)]
    pub fn rt1psifg(&mut self) -> RT1PSIFG_W {
        RT1PSIFG_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Prescale timer 1 interrupt enable"]
    #[inline(always)]
    pub fn rt1psie(&mut self) -> RT1PSIE_W {
        RT1PSIE_W { w: self }
    }
    #[doc = "Bits 2:4 - 4:2\\]
Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&mut self) -> RT1IP_W {
        RT1IP_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Prescale timer 1 hold"]
    #[inline(always)]
    pub fn rt1pshold(&mut self) -> RT1PSHOLD_W {
        RT1PSHOLD_W { w: self }
    }
    #[doc = "Bits 11:13 - 13:11\\]
Prescale timer 1 clock divide"]
    #[inline(always)]
    pub fn rt1psdiv(&mut self) -> RT1PSDIV_W {
        RT1PSDIV_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
Prescale timer 1 clock source select"]
    #[inline(always)]
    pub fn rt1ssel(&mut self) -> RT1SSEL_W {
        RT1SSEL_W { w: self }
    }
}
