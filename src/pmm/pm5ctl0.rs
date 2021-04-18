#[doc = "Reader of register PM5CTL0"]
pub type R = crate::R<u16, super::PM5CTL0>;
#[doc = "Writer for register PM5CTL0"]
pub type W = crate::W<u16, super::PM5CTL0>;
#[doc = "Register PM5CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PM5CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
LPMx.5 Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKLPM5_A {
    #[doc = "0: LPMx.5 configuration is not locked and defaults to its reset condition."]
    LOCKLPM5_0 = 0,
    #[doc = "1: LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    LOCKLPM5_1 = 1,
}
impl From<LOCKLPM5_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKLPM5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCKLPM5`"]
pub type LOCKLPM5_R = crate::R<bool, LOCKLPM5_A>;
impl LOCKLPM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKLPM5_A {
        match self.bits {
            false => LOCKLPM5_A::LOCKLPM5_0,
            true => LOCKLPM5_A::LOCKLPM5_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_0`"]
    #[inline(always)]
    pub fn is_locklpm5_0(&self) -> bool {
        *self == LOCKLPM5_A::LOCKLPM5_0
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_1`"]
    #[inline(always)]
    pub fn is_locklpm5_1(&self) -> bool {
        *self == LOCKLPM5_A::LOCKLPM5_1
    }
}
#[doc = "Write proxy for field `LOCKLPM5`"]
pub struct LOCKLPM5_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKLPM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKLPM5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPMx.5 configuration is not locked and defaults to its reset condition."]
    #[inline(always)]
    pub fn locklpm5_0(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_0)
    }
    #[doc = "LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    #[inline(always)]
    pub fn locklpm5_1(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_1)
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
impl R {
    #[doc = "Bit 0 - 0:0\\]
LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&self) -> LOCKLPM5_R {
        LOCKLPM5_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> LOCKLPM5_W {
        LOCKLPM5_W { w: self }
    }
}
