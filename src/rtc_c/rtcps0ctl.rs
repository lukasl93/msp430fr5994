#[doc = "Reader of register RTCPS0CTL"]
pub type R = crate::R<u16, super::RTCPS0CTL>;
#[doc = "Writer for register RTCPS0CTL"]
pub type W = crate::W<u16, super::RTCPS0CTL>;
#[doc = "Register RTCPS0CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCPS0CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
Prescale timer 0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT0PSIFG_A {
    #[doc = "0: No time event occurred"]
    RT0PSIFG_0 = 0,
    #[doc = "1: Time event occurred"]
    RT0PSIFG_1 = 1,
}
impl From<RT0PSIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RT0PSIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RT0PSIFG`"]
pub type RT0PSIFG_R = crate::R<bool, RT0PSIFG_A>;
impl RT0PSIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0PSIFG_A {
        match self.bits {
            false => RT0PSIFG_A::RT0PSIFG_0,
            true => RT0PSIFG_A::RT0PSIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RT0PSIFG_0`"]
    #[inline(always)]
    pub fn is_rt0psifg_0(&self) -> bool {
        *self == RT0PSIFG_A::RT0PSIFG_0
    }
    #[doc = "Checks if the value of the field is `RT0PSIFG_1`"]
    #[inline(always)]
    pub fn is_rt0psifg_1(&self) -> bool {
        *self == RT0PSIFG_A::RT0PSIFG_1
    }
}
#[doc = "Write proxy for field `RT0PSIFG`"]
pub struct RT0PSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0PSIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rt0psifg_0(self) -> &'a mut W {
        self.variant(RT0PSIFG_A::RT0PSIFG_0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rt0psifg_1(self) -> &'a mut W {
        self.variant(RT0PSIFG_A::RT0PSIFG_1)
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
Prescale timer 0 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT0PSIE_A {
    #[doc = "0: Interrupt not enabled"]
    DISABLE = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLE = 1,
}
impl From<RT0PSIE_A> for bool {
    #[inline(always)]
    fn from(variant: RT0PSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RT0PSIE`"]
pub type RT0PSIE_R = crate::R<bool, RT0PSIE_A>;
impl RT0PSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0PSIE_A {
        match self.bits {
            false => RT0PSIE_A::DISABLE,
            true => RT0PSIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RT0PSIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RT0PSIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `RT0PSIE`"]
pub struct RT0PSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0PSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RT0PSIE_A::DISABLE)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RT0PSIE_A::ENABLE)
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
Prescale timer 0 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT0IP_A {
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
impl From<RT0IP_A> for u8 {
    #[inline(always)]
    fn from(variant: RT0IP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RT0IP`"]
pub type RT0IP_R = crate::R<u8, RT0IP_A>;
impl RT0IP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0IP_A {
        match self.bits {
            0 => RT0IP_A::_2,
            1 => RT0IP_A::_4,
            2 => RT0IP_A::_8,
            3 => RT0IP_A::_16,
            4 => RT0IP_A::_32,
            5 => RT0IP_A::_64,
            6 => RT0IP_A::_128,
            7 => RT0IP_A::_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RT0IP_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == RT0IP_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == RT0IP_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == RT0IP_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == RT0IP_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == RT0IP_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == RT0IP_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == RT0IP_A::_256
    }
}
#[doc = "Write proxy for field `RT0IP`"]
pub struct RT0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0IP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0IP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RT0IP_A::_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(RT0IP_A::_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(RT0IP_A::_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(RT0IP_A::_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(RT0IP_A::_32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(RT0IP_A::_64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(RT0IP_A::_128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(RT0IP_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u16) & 0x07) << 2);
        self.w
    }
}
#[doc = "8:8\\]
Prescale timer 0 hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT0PSHOLD_A {
    #[doc = "0: RT0PS is operational"]
    RT0PSHOLD_0 = 0,
    #[doc = "1: RT0PS is held"]
    RT0PSHOLD_1 = 1,
}
impl From<RT0PSHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: RT0PSHOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RT0PSHOLD`"]
pub type RT0PSHOLD_R = crate::R<bool, RT0PSHOLD_A>;
impl RT0PSHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0PSHOLD_A {
        match self.bits {
            false => RT0PSHOLD_A::RT0PSHOLD_0,
            true => RT0PSHOLD_A::RT0PSHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RT0PSHOLD_0`"]
    #[inline(always)]
    pub fn is_rt0pshold_0(&self) -> bool {
        *self == RT0PSHOLD_A::RT0PSHOLD_0
    }
    #[doc = "Checks if the value of the field is `RT0PSHOLD_1`"]
    #[inline(always)]
    pub fn is_rt0pshold_1(&self) -> bool {
        *self == RT0PSHOLD_A::RT0PSHOLD_1
    }
}
#[doc = "Write proxy for field `RT0PSHOLD`"]
pub struct RT0PSHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0PSHOLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RT0PS is operational"]
    #[inline(always)]
    pub fn rt0pshold_0(self) -> &'a mut W {
        self.variant(RT0PSHOLD_A::RT0PSHOLD_0)
    }
    #[doc = "RT0PS is held"]
    #[inline(always)]
    pub fn rt0pshold_1(self) -> &'a mut W {
        self.variant(RT0PSHOLD_A::RT0PSHOLD_1)
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
Prescale timer 0 clock divide\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT0PSDIV_A {
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
impl From<RT0PSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: RT0PSDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RT0PSDIV`"]
pub type RT0PSDIV_R = crate::R<u8, RT0PSDIV_A>;
impl RT0PSDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT0PSDIV_A {
        match self.bits {
            0 => RT0PSDIV_A::_2,
            1 => RT0PSDIV_A::_4,
            2 => RT0PSDIV_A::_8,
            3 => RT0PSDIV_A::_16,
            4 => RT0PSDIV_A::_32,
            5 => RT0PSDIV_A::_64,
            6 => RT0PSDIV_A::_128,
            7 => RT0PSDIV_A::_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RT0PSDIV_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == RT0PSDIV_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == RT0PSDIV_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == RT0PSDIV_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == RT0PSDIV_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == RT0PSDIV_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == RT0PSDIV_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == RT0PSDIV_A::_256
    }
}
#[doc = "Write proxy for field `RT0PSDIV`"]
pub struct RT0PSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0PSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT0PSDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::_32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::_64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::_128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(RT0PSDIV_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u16) & 0x07) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Prescale timer 0 interrupt flag"]
    #[inline(always)]
    pub fn rt0psifg(&self) -> RT0PSIFG_R {
        RT0PSIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Prescale timer 0 interrupt enable"]
    #[inline(always)]
    pub fn rt0psie(&self) -> RT0PSIE_R {
        RT0PSIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&self) -> RT0IP_R {
        RT0IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Prescale timer 0 hold"]
    #[inline(always)]
    pub fn rt0pshold(&self) -> RT0PSHOLD_R {
        RT0PSHOLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Prescale timer 0 clock divide"]
    #[inline(always)]
    pub fn rt0psdiv(&self) -> RT0PSDIV_R {
        RT0PSDIV_R::new(((self.bits >> 11) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Prescale timer 0 interrupt flag"]
    #[inline(always)]
    pub fn rt0psifg(&mut self) -> RT0PSIFG_W {
        RT0PSIFG_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Prescale timer 0 interrupt enable"]
    #[inline(always)]
    pub fn rt0psie(&mut self) -> RT0PSIE_W {
        RT0PSIE_W { w: self }
    }
    #[doc = "Bits 2:4 - 4:2\\]
Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&mut self) -> RT0IP_W {
        RT0IP_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Prescale timer 0 hold"]
    #[inline(always)]
    pub fn rt0pshold(&mut self) -> RT0PSHOLD_W {
        RT0PSHOLD_W { w: self }
    }
    #[doc = "Bits 11:13 - 13:11\\]
Prescale timer 0 clock divide"]
    #[inline(always)]
    pub fn rt0psdiv(&mut self) -> RT0PSDIV_W {
        RT0PSDIV_W { w: self }
    }
}
