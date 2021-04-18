#[doc = "Reader of register SYSUNIV"]
pub type R = crate::R<u16, super::SYSUNIV>;
#[doc = "Writer for register SYSUNIV"]
pub type W = crate::W<u16, super::SYSUNIV>;
#[doc = "Register SYSUNIV `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSUNIV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
User NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SYSUNIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: NMIIFG NMI pin"]
    NMIIFG = 2,
    #[doc = "4: OFIFG oscillator fault"]
    OFIFG = 4,
}
impl From<SYSUNIV_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSUNIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSUNIV`"]
pub type SYSUNIV_R = crate::R<u16, SYSUNIV_A>;
impl SYSUNIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SYSUNIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSUNIV_A::NONE),
            2 => Val(SYSUNIV_A::NMIIFG),
            4 => Val(SYSUNIV_A::OFIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYSUNIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `NMIIFG`"]
    #[inline(always)]
    pub fn is_nmiifg(&self) -> bool {
        *self == SYSUNIV_A::NMIIFG
    }
    #[doc = "Checks if the value of the field is `OFIFG`"]
    #[inline(always)]
    pub fn is_ofifg(&self) -> bool {
        *self == SYSUNIV_A::OFIFG
    }
}
#[doc = "Write proxy for field `SYSUNIV`"]
pub struct SYSUNIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSUNIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSUNIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYSUNIV_A::NONE)
    }
    #[doc = "NMIIFG NMI pin"]
    #[inline(always)]
    pub fn nmiifg(self) -> &'a mut W {
        self.variant(SYSUNIV_A::NMIIFG)
    }
    #[doc = "OFIFG oscillator fault"]
    #[inline(always)]
    pub fn ofifg(self) -> &'a mut W {
        self.variant(SYSUNIV_A::OFIFG)
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
User NMI vector"]
    #[inline(always)]
    pub fn sysuniv(&self) -> SYSUNIV_R {
        SYSUNIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
User NMI vector"]
    #[inline(always)]
    pub fn sysuniv(&mut self) -> SYSUNIV_W {
        SYSUNIV_W { w: self }
    }
}
