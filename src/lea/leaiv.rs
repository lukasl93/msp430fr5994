#[doc = "Reader of register LEAIV"]
pub type R = crate::R<u32, super::LEAIV>;
#[doc = "Writer for register LEAIV"]
pub type W = crate::W<u32, super::LEAIV>;
#[doc = "Register LEAIV `reset()`'s with value 0"]
impl crate::ResetValue for super::LEAIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "7:0\\]
LEA interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEAIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: LEA command overflow"]
    COVLIFG = 2,
    #[doc = "4: LEA timer interrupt"]
    TIFG = 4,
    #[doc = "6: LEA out of range interrupt"]
    OORIFG = 6,
    #[doc = "8: LEA scalar data inconsistency"]
    SDIIFG = 8,
    #[doc = "10: PMCMD complete interrupt"]
    PMCMDIFG = 10,
}
impl From<LEAIV_A> for u8 {
    #[inline(always)]
    fn from(variant: LEAIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEAIV`"]
pub type LEAIV_R = crate::R<u8, LEAIV_A>;
impl LEAIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LEAIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LEAIV_A::NONE),
            2 => Val(LEAIV_A::COVLIFG),
            4 => Val(LEAIV_A::TIFG),
            6 => Val(LEAIV_A::OORIFG),
            8 => Val(LEAIV_A::SDIIFG),
            10 => Val(LEAIV_A::PMCMDIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LEAIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `COVLIFG`"]
    #[inline(always)]
    pub fn is_covlifg(&self) -> bool {
        *self == LEAIV_A::COVLIFG
    }
    #[doc = "Checks if the value of the field is `TIFG`"]
    #[inline(always)]
    pub fn is_tifg(&self) -> bool {
        *self == LEAIV_A::TIFG
    }
    #[doc = "Checks if the value of the field is `OORIFG`"]
    #[inline(always)]
    pub fn is_oorifg(&self) -> bool {
        *self == LEAIV_A::OORIFG
    }
    #[doc = "Checks if the value of the field is `SDIIFG`"]
    #[inline(always)]
    pub fn is_sdiifg(&self) -> bool {
        *self == LEAIV_A::SDIIFG
    }
    #[doc = "Checks if the value of the field is `PMCMDIFG`"]
    #[inline(always)]
    pub fn is_pmcmdifg(&self) -> bool {
        *self == LEAIV_A::PMCMDIFG
    }
}
#[doc = "Write proxy for field `LEAIV`"]
pub struct LEAIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LEAIV_A::NONE)
    }
    #[doc = "LEA command overflow"]
    #[inline(always)]
    pub fn covlifg(self) -> &'a mut W {
        self.variant(LEAIV_A::COVLIFG)
    }
    #[doc = "LEA timer interrupt"]
    #[inline(always)]
    pub fn tifg(self) -> &'a mut W {
        self.variant(LEAIV_A::TIFG)
    }
    #[doc = "LEA out of range interrupt"]
    #[inline(always)]
    pub fn oorifg(self) -> &'a mut W {
        self.variant(LEAIV_A::OORIFG)
    }
    #[doc = "LEA scalar data inconsistency"]
    #[inline(always)]
    pub fn sdiifg(self) -> &'a mut W {
        self.variant(LEAIV_A::SDIIFG)
    }
    #[doc = "PMCMD complete interrupt"]
    #[inline(always)]
    pub fn pmcmdifg(self) -> &'a mut W {
        self.variant(LEAIV_A::PMCMDIFG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
LEA interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary"]
    #[inline(always)]
    pub fn leaiv(&self) -> LEAIV_R {
        LEAIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
LEA interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary"]
    #[inline(always)]
    pub fn leaiv(&mut self) -> LEAIV_W {
        LEAIV_W { w: self }
    }
}
