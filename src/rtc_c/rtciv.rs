#[doc = "Reader of register RTCIV"]
pub type R = crate::R<u16, super::RTCIV>;
#[doc = "Writer for register RTCIV"]
pub type W = crate::W<u16, super::RTCIV>;
#[doc = "Register RTCIV `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCIV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
Real-time clock interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RTCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest"]
    RTCOFIFG = 2,
    #[doc = "4: Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG"]
    RTCRDYIFG = 4,
    #[doc = "6: Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG"]
    RTCTEVIFG = 6,
    #[doc = "8: Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG"]
    RTCAIFG = 8,
    #[doc = "10: Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG"]
    RT0PSIFG = 10,
    #[doc = "12: Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG"]
    RT1PSIFG = 12,
}
impl From<RTCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: RTCIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCIV`"]
pub type RTCIV_R = crate::R<u16, RTCIV_A>;
impl RTCIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, RTCIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RTCIV_A::NONE),
            2 => Val(RTCIV_A::RTCOFIFG),
            4 => Val(RTCIV_A::RTCRDYIFG),
            6 => Val(RTCIV_A::RTCTEVIFG),
            8 => Val(RTCIV_A::RTCAIFG),
            10 => Val(RTCIV_A::RT0PSIFG),
            12 => Val(RTCIV_A::RT1PSIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RTCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `RTCOFIFG`"]
    #[inline(always)]
    pub fn is_rtcofifg(&self) -> bool {
        *self == RTCIV_A::RTCOFIFG
    }
    #[doc = "Checks if the value of the field is `RTCRDYIFG`"]
    #[inline(always)]
    pub fn is_rtcrdyifg(&self) -> bool {
        *self == RTCIV_A::RTCRDYIFG
    }
    #[doc = "Checks if the value of the field is `RTCTEVIFG`"]
    #[inline(always)]
    pub fn is_rtctevifg(&self) -> bool {
        *self == RTCIV_A::RTCTEVIFG
    }
    #[doc = "Checks if the value of the field is `RTCAIFG`"]
    #[inline(always)]
    pub fn is_rtcaifg(&self) -> bool {
        *self == RTCIV_A::RTCAIFG
    }
    #[doc = "Checks if the value of the field is `RT0PSIFG`"]
    #[inline(always)]
    pub fn is_rt0psifg(&self) -> bool {
        *self == RTCIV_A::RT0PSIFG
    }
    #[doc = "Checks if the value of the field is `RT1PSIFG`"]
    #[inline(always)]
    pub fn is_rt1psifg(&self) -> bool {
        *self == RTCIV_A::RT1PSIFG
    }
}
#[doc = "Write proxy for field `RTCIV`"]
pub struct RTCIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RTCIV_A::NONE)
    }
    #[doc = "Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn rtcofifg(self) -> &'a mut W {
        self.variant(RTCIV_A::RTCOFIFG)
    }
    #[doc = "Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG"]
    #[inline(always)]
    pub fn rtcrdyifg(self) -> &'a mut W {
        self.variant(RTCIV_A::RTCRDYIFG)
    }
    #[doc = "Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG"]
    #[inline(always)]
    pub fn rtctevifg(self) -> &'a mut W {
        self.variant(RTCIV_A::RTCTEVIFG)
    }
    #[doc = "Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG"]
    #[inline(always)]
    pub fn rtcaifg(self) -> &'a mut W {
        self.variant(RTCIV_A::RTCAIFG)
    }
    #[doc = "Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG"]
    #[inline(always)]
    pub fn rt0psifg(self) -> &'a mut W {
        self.variant(RTCIV_A::RT0PSIFG)
    }
    #[doc = "Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG"]
    #[inline(always)]
    pub fn rt1psifg(self) -> &'a mut W {
        self.variant(RTCIV_A::RT1PSIFG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Real-time clock interrupt vector value"]
    #[inline(always)]
    pub fn rtciv(&self) -> RTCIV_R {
        RTCIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Real-time clock interrupt vector value"]
    #[inline(always)]
    pub fn rtciv(&mut self) -> RTCIV_W {
        RTCIV_W { w: self }
    }
}
