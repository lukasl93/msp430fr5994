#[doc = "Reader of register UCA0IV"]
pub type R = crate::R<u16, super::UCA0IV>;
#[doc = "Writer for register UCA0IV"]
pub type W = crate::W<u16, super::UCA0IV>;
#[doc = "Register UCA0IV `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
eUSCI_A interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum UCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    UCRXIFG = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    UCTXIFG = 4,
    #[doc = "6: Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    UCSTTIFG = 6,
    #[doc = "8: Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    UCTXCPTIFG = 8,
}
impl From<UCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: UCIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UCIV`"]
pub type UCIV_R = crate::R<u16, UCIV_A>;
impl UCIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, UCIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UCIV_A::NONE),
            2 => Val(UCIV_A::UCRXIFG),
            4 => Val(UCIV_A::UCTXIFG),
            6 => Val(UCIV_A::UCSTTIFG),
            8 => Val(UCIV_A::UCTXCPTIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `UCRXIFG`"]
    #[inline(always)]
    pub fn is_ucrxifg(&self) -> bool {
        *self == UCIV_A::UCRXIFG
    }
    #[doc = "Checks if the value of the field is `UCTXIFG`"]
    #[inline(always)]
    pub fn is_uctxifg(&self) -> bool {
        *self == UCIV_A::UCTXIFG
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG`"]
    #[inline(always)]
    pub fn is_ucsttifg(&self) -> bool {
        *self == UCIV_A::UCSTTIFG
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIFG`"]
    #[inline(always)]
    pub fn is_uctxcptifg(&self) -> bool {
        *self == UCIV_A::UCTXCPTIFG
    }
}
#[doc = "Write proxy for field `UCIV`"]
pub struct UCIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(UCIV_A::NONE)
    }
    #[doc = "Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn ucrxifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCRXIFG)
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    #[inline(always)]
    pub fn uctxifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCTXIFG)
    }
    #[doc = "Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    #[inline(always)]
    pub fn ucsttifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCSTTIFG)
    }
    #[doc = "Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn uctxcptifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCTXCPTIFG)
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
eUSCI_A interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
eUSCI_A interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&mut self) -> UCIV_W {
        UCIV_W { w: self }
    }
}
