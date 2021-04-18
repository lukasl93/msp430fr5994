#[doc = "Reader of register TA4IV"]
pub type R = crate::R<u16, super::TA4IV>;
#[doc = "Writer for register TA4IV"]
pub type W = crate::W<u16, super::TA4IV>;
#[doc = "Register TA4IV `reset()`'s with value 0"]
impl crate::ResetValue for super::TA4IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
TimerA interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TAIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    TACCR1 = 2,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    TACCR2 = 4,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    TACCR3 = 6,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    TACCR4 = 8,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    TACCR5 = 10,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    TACCR6 = 12,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    TAIFG = 14,
}
impl From<TAIV_A> for u16 {
    #[inline(always)]
    fn from(variant: TAIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TAIV`"]
pub type TAIV_R = crate::R<u16, TAIV_A>;
impl TAIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TAIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TAIV_A::NONE),
            2 => Val(TAIV_A::TACCR1),
            4 => Val(TAIV_A::TACCR2),
            6 => Val(TAIV_A::TACCR3),
            8 => Val(TAIV_A::TACCR4),
            10 => Val(TAIV_A::TACCR5),
            12 => Val(TAIV_A::TACCR6),
            14 => Val(TAIV_A::TAIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TAIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `TACCR1`"]
    #[inline(always)]
    pub fn is_taccr1(&self) -> bool {
        *self == TAIV_A::TACCR1
    }
    #[doc = "Checks if the value of the field is `TACCR2`"]
    #[inline(always)]
    pub fn is_taccr2(&self) -> bool {
        *self == TAIV_A::TACCR2
    }
    #[doc = "Checks if the value of the field is `TACCR3`"]
    #[inline(always)]
    pub fn is_taccr3(&self) -> bool {
        *self == TAIV_A::TACCR3
    }
    #[doc = "Checks if the value of the field is `TACCR4`"]
    #[inline(always)]
    pub fn is_taccr4(&self) -> bool {
        *self == TAIV_A::TACCR4
    }
    #[doc = "Checks if the value of the field is `TACCR5`"]
    #[inline(always)]
    pub fn is_taccr5(&self) -> bool {
        *self == TAIV_A::TACCR5
    }
    #[doc = "Checks if the value of the field is `TACCR6`"]
    #[inline(always)]
    pub fn is_taccr6(&self) -> bool {
        *self == TAIV_A::TACCR6
    }
    #[doc = "Checks if the value of the field is `TAIFG`"]
    #[inline(always)]
    pub fn is_taifg(&self) -> bool {
        *self == TAIV_A::TAIFG
    }
}
#[doc = "Write proxy for field `TAIV`"]
pub struct TAIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TAIV_A::NONE)
    }
    #[doc = "Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn taccr1(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR1)
    }
    #[doc = "Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    #[inline(always)]
    pub fn taccr2(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR2)
    }
    #[doc = "Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    #[inline(always)]
    pub fn taccr3(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR3)
    }
    #[doc = "Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    #[inline(always)]
    pub fn taccr4(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR4)
    }
    #[doc = "Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    #[inline(always)]
    pub fn taccr5(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR5)
    }
    #[doc = "Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    #[inline(always)]
    pub fn taccr6(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR6)
    }
    #[doc = "Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn taifg(self) -> &'a mut W {
        self.variant(TAIV_A::TAIFG)
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
TimerA interrupt vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TAIV_R {
        TAIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TimerA interrupt vector value"]
    #[inline(always)]
    pub fn taiv(&mut self) -> TAIV_W {
        TAIV_W { w: self }
    }
}
