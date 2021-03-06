#[doc = "Reader of register UCA0MCTLW"]
pub type R = crate::R<u16, super::UCA0MCTLW>;
#[doc = "Writer for register UCA0MCTLW"]
pub type W = crate::W<u16, super::UCA0MCTLW>;
#[doc = "Register UCA0MCTLW `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0MCTLW {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
Oversampling mode enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCOS16_A {
    #[doc = "0: Disabled"]
    UCOS16_0 = 0,
    #[doc = "1: Enabled"]
    UCOS16_1 = 1,
}
impl From<UCOS16_A> for bool {
    #[inline(always)]
    fn from(variant: UCOS16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCOS16`"]
pub type UCOS16_R = crate::R<bool, UCOS16_A>;
impl UCOS16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOS16_A {
        match self.bits {
            false => UCOS16_A::UCOS16_0,
            true => UCOS16_A::UCOS16_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCOS16_0`"]
    #[inline(always)]
    pub fn is_ucos16_0(&self) -> bool {
        *self == UCOS16_A::UCOS16_0
    }
    #[doc = "Checks if the value of the field is `UCOS16_1`"]
    #[inline(always)]
    pub fn is_ucos16_1(&self) -> bool {
        *self == UCOS16_A::UCOS16_1
    }
}
#[doc = "Write proxy for field `UCOS16`"]
pub struct UCOS16_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOS16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCOS16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn ucos16_0(self) -> &'a mut W {
        self.variant(UCOS16_A::UCOS16_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ucos16_1(self) -> &'a mut W {
        self.variant(UCOS16_A::UCOS16_1)
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
#[doc = "Reader of field `UCBRF`"]
pub type UCBRF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCBRF`"]
pub struct UCBRF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `UCBRS`"]
pub type UCBRS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCBRS`"]
pub struct UCBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&self) -> UCOS16_R {
        UCOS16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UCBRF_R {
        UCBRF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Second modulation stage select"]
    #[inline(always)]
    pub fn ucbrs(&self) -> UCBRS_R {
        UCBRS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> UCOS16_W {
        UCOS16_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UCBRF_W {
        UCBRF_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Second modulation stage select"]
    #[inline(always)]
    pub fn ucbrs(&mut self) -> UCBRS_W {
        UCBRS_W { w: self }
    }
}
