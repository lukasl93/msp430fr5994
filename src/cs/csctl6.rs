#[doc = "Reader of register CSCTL6"]
pub type R = crate::R<u16, super::CSCTL6>;
#[doc = "Writer for register CSCTL6"]
pub type W = crate::W<u16, super::CSCTL6>;
#[doc = "Register CSCTL6 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL6 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
ACLK clock request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACLKREQEN_A {
    #[doc = "0: ACLK conditional requests are disabled"]
    DISABLE = 0,
    #[doc = "1: ACLK conditional requests are enabled"]
    ENABLE = 1,
}
impl From<ACLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACLKREQEN`"]
pub type ACLKREQEN_R = crate::R<bool, ACLKREQEN_A>;
impl ACLKREQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACLKREQEN_A {
        match self.bits {
            false => ACLKREQEN_A::DISABLE,
            true => ACLKREQEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACLKREQEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ACLKREQEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `ACLKREQEN`"]
pub struct ACLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLKREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACLKREQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACLK conditional requests are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACLKREQEN_A::DISABLE)
    }
    #[doc = "ACLK conditional requests are enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACLKREQEN_A::ENABLE)
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
MCLK clock request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKREQEN_A {
    #[doc = "0: MCLK conditional requests are disabled"]
    DISABLE = 0,
    #[doc = "1: MCLK conditional requests are enabled"]
    ENABLE = 1,
}
impl From<MCLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCLKREQEN`"]
pub type MCLKREQEN_R = crate::R<bool, MCLKREQEN_A>;
impl MCLKREQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKREQEN_A {
        match self.bits {
            false => MCLKREQEN_A::DISABLE,
            true => MCLKREQEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MCLKREQEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MCLKREQEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `MCLKREQEN`"]
pub struct MCLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKREQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCLK conditional requests are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MCLKREQEN_A::DISABLE)
    }
    #[doc = "MCLK conditional requests are enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MCLKREQEN_A::ENABLE)
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
#[doc = "2:2\\]
SMCLK clock request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMCLKREQEN_A {
    #[doc = "0: SMCLK conditional requests are disabled"]
    DISABLE = 0,
    #[doc = "1: SMCLK conditional requests are enabled"]
    ENABLE = 1,
}
impl From<SMCLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMCLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMCLKREQEN`"]
pub type SMCLKREQEN_R = crate::R<bool, SMCLKREQEN_A>;
impl SMCLKREQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMCLKREQEN_A {
        match self.bits {
            false => SMCLKREQEN_A::DISABLE,
            true => SMCLKREQEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SMCLKREQEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMCLKREQEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `SMCLKREQEN`"]
pub struct SMCLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLKREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMCLKREQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SMCLK conditional requests are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SMCLKREQEN_A::DISABLE)
    }
    #[doc = "SMCLK conditional requests are enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMCLKREQEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "3:3\\]
MODCLK clock request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODCLKREQEN_A {
    #[doc = "0: MODCLK conditional requests are disabled"]
    DISABLE = 0,
    #[doc = "1: MODCLK conditional requests are enabled"]
    ENABLE = 1,
}
impl From<MODCLKREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODCLKREQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODCLKREQEN`"]
pub type MODCLKREQEN_R = crate::R<bool, MODCLKREQEN_A>;
impl MODCLKREQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODCLKREQEN_A {
        match self.bits {
            false => MODCLKREQEN_A::DISABLE,
            true => MODCLKREQEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODCLKREQEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MODCLKREQEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `MODCLKREQEN`"]
pub struct MODCLKREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODCLKREQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODCLKREQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MODCLK conditional requests are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MODCLKREQEN_A::DISABLE)
    }
    #[doc = "MODCLK conditional requests are enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MODCLKREQEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
ACLK clock request enable"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> ACLKREQEN_R {
        ACLKREQEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MCLK clock request enable"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MCLKREQEN_R {
        MCLKREQEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SMCLK clock request enable"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SMCLKREQEN_R {
        SMCLKREQEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
MODCLK clock request enable"]
    #[inline(always)]
    pub fn modclkreqen(&self) -> MODCLKREQEN_R {
        MODCLKREQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ACLK clock request enable"]
    #[inline(always)]
    pub fn aclkreqen(&mut self) -> ACLKREQEN_W {
        ACLKREQEN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
MCLK clock request enable"]
    #[inline(always)]
    pub fn mclkreqen(&mut self) -> MCLKREQEN_W {
        MCLKREQEN_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
SMCLK clock request enable"]
    #[inline(always)]
    pub fn smclkreqen(&mut self) -> SMCLKREQEN_W {
        SMCLKREQEN_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
MODCLK clock request enable"]
    #[inline(always)]
    pub fn modclkreqen(&mut self) -> MODCLKREQEN_W {
        MODCLKREQEN_W { w: self }
    }
}
