#[doc = "Reader of register CEIV"]
pub type R = crate::R<u16, super::CEIV>;
#[doc = "Writer for register CEIV"]
pub type W = crate::W<u16, super::CEIV>;
#[doc = "Register CEIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CEIV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
Comparator interrupt vector word register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CEIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest"]
    CEIFG = 2,
    #[doc = "4: Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG"]
    CEIIFG = 4,
    #[doc = "10: Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest"]
    CERDYIFG = 10,
}
impl From<CEIV_A> for u16 {
    #[inline(always)]
    fn from(variant: CEIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEIV`"]
pub type CEIV_R = crate::R<u16, CEIV_A>;
impl CEIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CEIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CEIV_A::NONE),
            2 => Val(CEIV_A::CEIFG),
            4 => Val(CEIV_A::CEIIFG),
            10 => Val(CEIV_A::CERDYIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CEIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `CEIFG`"]
    #[inline(always)]
    pub fn is_ceifg(&self) -> bool {
        *self == CEIV_A::CEIFG
    }
    #[doc = "Checks if the value of the field is `CEIIFG`"]
    #[inline(always)]
    pub fn is_ceiifg(&self) -> bool {
        *self == CEIV_A::CEIIFG
    }
    #[doc = "Checks if the value of the field is `CERDYIFG`"]
    #[inline(always)]
    pub fn is_cerdyifg(&self) -> bool {
        *self == CEIV_A::CERDYIFG
    }
}
#[doc = "Write proxy for field `CEIV`"]
pub struct CEIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CEIV_A::NONE)
    }
    #[doc = "Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn ceifg(self) -> &'a mut W {
        self.variant(CEIV_A::CEIFG)
    }
    #[doc = "Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG"]
    #[inline(always)]
    pub fn ceiifg(self) -> &'a mut W {
        self.variant(CEIV_A::CEIIFG)
    }
    #[doc = "Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn cerdyifg(self) -> &'a mut W {
        self.variant(CEIV_A::CERDYIFG)
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
Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn ceiv(&self) -> CEIV_R {
        CEIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn ceiv(&mut self) -> CEIV_W {
        CEIV_W { w: self }
    }
}
