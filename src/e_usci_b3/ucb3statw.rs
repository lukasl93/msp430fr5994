#[doc = "Reader of register UCB3STATW"]
pub type R = crate::R<u16, super::UCB3STATW>;
#[doc = "Writer for register UCB3STATW"]
pub type W = crate::W<u16, super::UCB3STATW>;
#[doc = "Register UCB3STATW `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB3STATW {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "4:4\\]
Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBBUSY_A {
    #[doc = "0: Bus inactive"]
    IDLE = 0,
    #[doc = "1: Bus busy"]
    BUSY = 1,
}
impl From<UCBBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: UCBBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCBBUSY`"]
pub type UCBBUSY_R = crate::R<bool, UCBBUSY_A>;
impl UCBBUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBBUSY_A {
        match self.bits {
            false => UCBBUSY_A::IDLE,
            true => UCBBUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == UCBBUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == UCBBUSY_A::BUSY
    }
}
#[doc = "Write proxy for field `UCBBUSY`"]
pub struct UCBBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBBUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBBUSY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus inactive"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(UCBBUSY_A::IDLE)
    }
    #[doc = "Bus busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(UCBBUSY_A::BUSY)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "5:5\\]
General call address received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCGC_A {
    #[doc = "0: No general call address received"]
    UCGC_0 = 0,
    #[doc = "1: General call address received"]
    UCGC_1 = 1,
}
impl From<UCGC_A> for bool {
    #[inline(always)]
    fn from(variant: UCGC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCGC`"]
pub type UCGC_R = crate::R<bool, UCGC_A>;
impl UCGC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCGC_A {
        match self.bits {
            false => UCGC_A::UCGC_0,
            true => UCGC_A::UCGC_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCGC_0`"]
    #[inline(always)]
    pub fn is_ucgc_0(&self) -> bool {
        *self == UCGC_A::UCGC_0
    }
    #[doc = "Checks if the value of the field is `UCGC_1`"]
    #[inline(always)]
    pub fn is_ucgc_1(&self) -> bool {
        *self == UCGC_A::UCGC_1
    }
}
#[doc = "Write proxy for field `UCGC`"]
pub struct UCGC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCGC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCGC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No general call address received"]
    #[inline(always)]
    pub fn ucgc_0(self) -> &'a mut W {
        self.variant(UCGC_A::UCGC_0)
    }
    #[doc = "General call address received"]
    #[inline(always)]
    pub fn ucgc_1(self) -> &'a mut W {
        self.variant(UCGC_A::UCGC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "6:6\\]
SCL low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSCLLOW_A {
    #[doc = "0: SCL is not held low"]
    UCSCLLOW_0 = 0,
    #[doc = "1: SCL is held low"]
    UCSCLLOW_1 = 1,
}
impl From<UCSCLLOW_A> for bool {
    #[inline(always)]
    fn from(variant: UCSCLLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UCSCLLOW`"]
pub type UCSCLLOW_R = crate::R<bool, UCSCLLOW_A>;
impl UCSCLLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSCLLOW_A {
        match self.bits {
            false => UCSCLLOW_A::UCSCLLOW_0,
            true => UCSCLLOW_A::UCSCLLOW_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSCLLOW_0`"]
    #[inline(always)]
    pub fn is_ucscllow_0(&self) -> bool {
        *self == UCSCLLOW_A::UCSCLLOW_0
    }
    #[doc = "Checks if the value of the field is `UCSCLLOW_1`"]
    #[inline(always)]
    pub fn is_ucscllow_1(&self) -> bool {
        *self == UCSCLLOW_A::UCSCLLOW_1
    }
}
#[doc = "Write proxy for field `UCSCLLOW`"]
pub struct UCSCLLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSCLLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSCLLOW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SCL is not held low"]
    #[inline(always)]
    pub fn ucscllow_0(self) -> &'a mut W {
        self.variant(UCSCLLOW_A::UCSCLLOW_0)
    }
    #[doc = "SCL is held low"]
    #[inline(always)]
    pub fn ucscllow_1(self) -> &'a mut W {
        self.variant(UCSCLLOW_A::UCSCLLOW_1)
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
#[doc = "Reader of field `UCBCNT`"]
pub type UCBCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCBCNT`"]
pub struct UCBCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - 4:4\\]
Bus busy"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UCBBUSY_R {
        UCBBUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
General call address received"]
    #[inline(always)]
    pub fn ucgc(&self) -> UCGC_R {
        UCGC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UCSCLLOW_R {
        UCSCLLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Hardware byte counter value"]
    #[inline(always)]
    pub fn ucbcnt(&self) -> UCBCNT_R {
        UCBCNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Bus busy"]
    #[inline(always)]
    pub fn ucbbusy(&mut self) -> UCBBUSY_W {
        UCBBUSY_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
General call address received"]
    #[inline(always)]
    pub fn ucgc(&mut self) -> UCGC_W {
        UCGC_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
SCL low"]
    #[inline(always)]
    pub fn ucscllow(&mut self) -> UCSCLLOW_W {
        UCSCLLOW_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Hardware byte counter value"]
    #[inline(always)]
    pub fn ucbcnt(&mut self) -> UCBCNT_W {
        UCBCNT_W { w: self }
    }
}
