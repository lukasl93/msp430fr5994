#[doc = "Reader of register CSCTL5"]
pub type R = crate::R<u16, super::CSCTL5>;
#[doc = "Writer for register CSCTL5"]
pub type W = crate::W<u16, super::CSCTL5>;
#[doc = "Register CSCTL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL5 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
LFXT oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXTOFFG_A {
    #[doc = "0: No fault condition occurred after the last reset"]
    LFXTOFFG_0 = 0,
    #[doc = "1: LFXT fault; an LFXT fault occurred after the last reset"]
    LFXTOFFG_1 = 1,
}
impl From<LFXTOFFG_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTOFFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXTOFFG`"]
pub type LFXTOFFG_R = crate::R<bool, LFXTOFFG_A>;
impl LFXTOFFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTOFFG_A {
        match self.bits {
            false => LFXTOFFG_A::LFXTOFFG_0,
            true => LFXTOFFG_A::LFXTOFFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTOFFG_0`"]
    #[inline(always)]
    pub fn is_lfxtoffg_0(&self) -> bool {
        *self == LFXTOFFG_A::LFXTOFFG_0
    }
    #[doc = "Checks if the value of the field is `LFXTOFFG_1`"]
    #[inline(always)]
    pub fn is_lfxtoffg_1(&self) -> bool {
        *self == LFXTOFFG_A::LFXTOFFG_1
    }
}
#[doc = "Write proxy for field `LFXTOFFG`"]
pub struct LFXTOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTOFFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTOFFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No fault condition occurred after the last reset"]
    #[inline(always)]
    pub fn lfxtoffg_0(self) -> &'a mut W {
        self.variant(LFXTOFFG_A::LFXTOFFG_0)
    }
    #[doc = "LFXT fault; an LFXT fault occurred after the last reset"]
    #[inline(always)]
    pub fn lfxtoffg_1(self) -> &'a mut W {
        self.variant(LFXTOFFG_A::LFXTOFFG_1)
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
HFXT oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXTOFFG_A {
    #[doc = "0: No fault condition occurred after the last reset"]
    HFXTOFFG_0 = 0,
    #[doc = "1: HFXT fault; an HFXT fault occurred after the last reset"]
    HFXTOFFG_1 = 1,
}
impl From<HFXTOFFG_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTOFFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXTOFFG`"]
pub type HFXTOFFG_R = crate::R<bool, HFXTOFFG_A>;
impl HFXTOFFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTOFFG_A {
        match self.bits {
            false => HFXTOFFG_A::HFXTOFFG_0,
            true => HFXTOFFG_A::HFXTOFFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTOFFG_0`"]
    #[inline(always)]
    pub fn is_hfxtoffg_0(&self) -> bool {
        *self == HFXTOFFG_A::HFXTOFFG_0
    }
    #[doc = "Checks if the value of the field is `HFXTOFFG_1`"]
    #[inline(always)]
    pub fn is_hfxtoffg_1(&self) -> bool {
        *self == HFXTOFFG_A::HFXTOFFG_1
    }
}
#[doc = "Write proxy for field `HFXTOFFG`"]
pub struct HFXTOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTOFFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTOFFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No fault condition occurred after the last reset"]
    #[inline(always)]
    pub fn hfxtoffg_0(self) -> &'a mut W {
        self.variant(HFXTOFFG_A::HFXTOFFG_0)
    }
    #[doc = "HFXT fault; an HFXT fault occurred after the last reset"]
    #[inline(always)]
    pub fn hfxtoffg_1(self) -> &'a mut W {
        self.variant(HFXTOFFG_A::HFXTOFFG_1)
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
#[doc = "6:6\\]
Enable start counter for LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENSTFCNT1_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    DISABLE = 0,
    #[doc = "1: Startup fault counter enabled"]
    ENABLE = 1,
}
impl From<ENSTFCNT1_A> for bool {
    #[inline(always)]
    fn from(variant: ENSTFCNT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENSTFCNT1`"]
pub type ENSTFCNT1_R = crate::R<bool, ENSTFCNT1_A>;
impl ENSTFCNT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENSTFCNT1_A {
        match self.bits {
            false => ENSTFCNT1_A::DISABLE,
            true => ENSTFCNT1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENSTFCNT1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENSTFCNT1_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENSTFCNT1`"]
pub struct ENSTFCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTFCNT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENSTFCNT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENSTFCNT1_A::DISABLE)
    }
    #[doc = "Startup fault counter enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENSTFCNT1_A::ENABLE)
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
Enable start counter for HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENSTFCNT2_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    DISABLE = 0,
    #[doc = "1: Startup fault counter enabled"]
    ENABLE = 1,
}
impl From<ENSTFCNT2_A> for bool {
    #[inline(always)]
    fn from(variant: ENSTFCNT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENSTFCNT2`"]
pub type ENSTFCNT2_R = crate::R<bool, ENSTFCNT2_A>;
impl ENSTFCNT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENSTFCNT2_A {
        match self.bits {
            false => ENSTFCNT2_A::DISABLE,
            true => ENSTFCNT2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENSTFCNT2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENSTFCNT2_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENSTFCNT2`"]
pub struct ENSTFCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTFCNT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENSTFCNT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENSTFCNT2_A::DISABLE)
    }
    #[doc = "Startup fault counter enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENSTFCNT2_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - 0:0\\]
LFXT oscillator fault flag"]
    #[inline(always)]
    pub fn lfxtoffg(&self) -> LFXTOFFG_R {
        LFXTOFFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
HFXT oscillator fault flag"]
    #[inline(always)]
    pub fn hfxtoffg(&self) -> HFXTOFFG_R {
        HFXTOFFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable start counter for LFXT"]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> ENSTFCNT1_R {
        ENSTFCNT1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable start counter for HFXT"]
    #[inline(always)]
    pub fn enstfcnt2(&self) -> ENSTFCNT2_R {
        ENSTFCNT2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LFXT oscillator fault flag"]
    #[inline(always)]
    pub fn lfxtoffg(&mut self) -> LFXTOFFG_W {
        LFXTOFFG_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
HFXT oscillator fault flag"]
    #[inline(always)]
    pub fn hfxtoffg(&mut self) -> HFXTOFFG_W {
        HFXTOFFG_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Enable start counter for LFXT"]
    #[inline(always)]
    pub fn enstfcnt1(&mut self) -> ENSTFCNT1_W {
        ENSTFCNT1_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Enable start counter for HFXT"]
    #[inline(always)]
    pub fn enstfcnt2(&mut self) -> ENSTFCNT2_W {
        ENSTFCNT2_W { w: self }
    }
}
