#[doc = "Reader of register LEAPMCTL"]
pub type R = crate::R<u32, super::LEAPMCTL>;
#[doc = "Writer for register LEAPMCTL"]
pub type W = crate::W<u32, super::LEAPMCTL>;
#[doc = "Register LEAPMCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LEAPMCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
Command enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEACMDEN_A {
    #[doc = "0: Command triggering by writing to LEAPMCB is disabled"]
    LEACMDEN_0 = 0,
    #[doc = "1: Command triggering by writing to LEAPMCB is enabled"]
    LEACMDEN_1 = 1,
}
impl From<LEACMDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LEACMDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEACMDEN`"]
pub type LEACMDEN_R = crate::R<bool, LEACMDEN_A>;
impl LEACMDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEACMDEN_A {
        match self.bits {
            false => LEACMDEN_A::LEACMDEN_0,
            true => LEACMDEN_A::LEACMDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEACMDEN_0`"]
    #[inline(always)]
    pub fn is_leacmden_0(&self) -> bool {
        *self == LEACMDEN_A::LEACMDEN_0
    }
    #[doc = "Checks if the value of the field is `LEACMDEN_1`"]
    #[inline(always)]
    pub fn is_leacmden_1(&self) -> bool {
        *self == LEACMDEN_A::LEACMDEN_1
    }
}
#[doc = "Write proxy for field `LEACMDEN`"]
pub struct LEACMDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACMDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACMDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Command triggering by writing to LEAPMCB is disabled"]
    #[inline(always)]
    pub fn leacmden_0(self) -> &'a mut W {
        self.variant(LEACMDEN_A::LEACMDEN_0)
    }
    #[doc = "Command triggering by writing to LEAPMCB is enabled"]
    #[inline(always)]
    pub fn leacmden_1(self) -> &'a mut W {
        self.variant(LEACMDEN_A::LEACMDEN_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `LEATRG`"]
pub type LEATRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEATRG`"]
pub struct LEATRG_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATRG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Command enable"]
    #[inline(always)]
    pub fn leacmden(&self) -> LEACMDEN_R {
        LEACMDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Command trigger"]
    #[inline(always)]
    pub fn leatrg(&self) -> LEATRG_R {
        LEATRG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Command enable"]
    #[inline(always)]
    pub fn leacmden(&mut self) -> LEACMDEN_W {
        LEACMDEN_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Command trigger"]
    #[inline(always)]
    pub fn leatrg(&mut self) -> LEATRG_W {
        LEATRG_W { w: self }
    }
}
