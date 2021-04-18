#[doc = "Reader of register LEAIE"]
pub type R = crate::R<u32, super::LEAIE>;
#[doc = "Writer for register LEAIE"]
pub type W = crate::W<u32, super::LEAIE>;
#[doc = "Register LEAIE `reset()`'s with value 0"]
impl crate::ResetValue for super::LEAIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
LEA command overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEACOVLIE_A {
    #[doc = "0: Interrupt disabled"]
    LEACOVLIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LEACOVLIE_1 = 1,
}
impl From<LEACOVLIE_A> for bool {
    #[inline(always)]
    fn from(variant: LEACOVLIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEACOVLIE`"]
pub type LEACOVLIE_R = crate::R<bool, LEACOVLIE_A>;
impl LEACOVLIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEACOVLIE_A {
        match self.bits {
            false => LEACOVLIE_A::LEACOVLIE_0,
            true => LEACOVLIE_A::LEACOVLIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEACOVLIE_0`"]
    #[inline(always)]
    pub fn is_leacovlie_0(&self) -> bool {
        *self == LEACOVLIE_A::LEACOVLIE_0
    }
    #[doc = "Checks if the value of the field is `LEACOVLIE_1`"]
    #[inline(always)]
    pub fn is_leacovlie_1(&self) -> bool {
        *self == LEACOVLIE_A::LEACOVLIE_1
    }
}
#[doc = "Write proxy for field `LEACOVLIE`"]
pub struct LEACOVLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACOVLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACOVLIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leacovlie_0(self) -> &'a mut W {
        self.variant(LEACOVLIE_A::LEACOVLIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leacovlie_1(self) -> &'a mut W {
        self.variant(LEACOVLIE_A::LEACOVLIE_1)
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
#[doc = "1:1\\]
LEA timer event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEATIE_A {
    #[doc = "0: Interrupt disabled"]
    LEATIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LEATIE_1 = 1,
}
impl From<LEATIE_A> for bool {
    #[inline(always)]
    fn from(variant: LEATIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEATIE`"]
pub type LEATIE_R = crate::R<bool, LEATIE_A>;
impl LEATIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEATIE_A {
        match self.bits {
            false => LEATIE_A::LEATIE_0,
            true => LEATIE_A::LEATIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEATIE_0`"]
    #[inline(always)]
    pub fn is_leatie_0(&self) -> bool {
        *self == LEATIE_A::LEATIE_0
    }
    #[doc = "Checks if the value of the field is `LEATIE_1`"]
    #[inline(always)]
    pub fn is_leatie_1(&self) -> bool {
        *self == LEATIE_A::LEATIE_1
    }
}
#[doc = "Write proxy for field `LEATIE`"]
pub struct LEATIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEATIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leatie_0(self) -> &'a mut W {
        self.variant(LEATIE_A::LEATIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leatie_1(self) -> &'a mut W {
        self.variant(LEATIE_A::LEATIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "2:2\\]
LEA out of address range interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAOORIE_A {
    #[doc = "0: Interrupt disabled"]
    LEAOORIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LEAOORIE_1 = 1,
}
impl From<LEAOORIE_A> for bool {
    #[inline(always)]
    fn from(variant: LEAOORIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAOORIE`"]
pub type LEAOORIE_R = crate::R<bool, LEAOORIE_A>;
impl LEAOORIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAOORIE_A {
        match self.bits {
            false => LEAOORIE_A::LEAOORIE_0,
            true => LEAOORIE_A::LEAOORIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAOORIE_0`"]
    #[inline(always)]
    pub fn is_leaoorie_0(&self) -> bool {
        *self == LEAOORIE_A::LEAOORIE_0
    }
    #[doc = "Checks if the value of the field is `LEAOORIE_1`"]
    #[inline(always)]
    pub fn is_leaoorie_1(&self) -> bool {
        *self == LEAOORIE_A::LEAOORIE_1
    }
}
#[doc = "Write proxy for field `LEAOORIE`"]
pub struct LEAOORIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAOORIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAOORIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leaoorie_0(self) -> &'a mut W {
        self.variant(LEAOORIE_A::LEAOORIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leaoorie_1(self) -> &'a mut W {
        self.variant(LEAOORIE_A::LEAOORIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "3:3\\]
LEA scalar data inconsistency interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEASDIIE_A {
    #[doc = "0: Interrupt disabled"]
    LEASDIIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LEASDIIE_1 = 1,
}
impl From<LEASDIIE_A> for bool {
    #[inline(always)]
    fn from(variant: LEASDIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEASDIIE`"]
pub type LEASDIIE_R = crate::R<bool, LEASDIIE_A>;
impl LEASDIIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEASDIIE_A {
        match self.bits {
            false => LEASDIIE_A::LEASDIIE_0,
            true => LEASDIIE_A::LEASDIIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEASDIIE_0`"]
    #[inline(always)]
    pub fn is_leasdiie_0(&self) -> bool {
        *self == LEASDIIE_A::LEASDIIE_0
    }
    #[doc = "Checks if the value of the field is `LEASDIIE_1`"]
    #[inline(always)]
    pub fn is_leasdiie_1(&self) -> bool {
        *self == LEASDIIE_A::LEASDIIE_1
    }
}
#[doc = "Write proxy for field `LEASDIIE`"]
pub struct LEASDIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEASDIIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEASDIIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leasdiie_0(self) -> &'a mut W {
        self.variant(LEASDIIE_A::LEASDIIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leasdiie_1(self) -> &'a mut W {
        self.variant(LEASDIIE_A::LEASDIIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "4:4\\]
PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAPMCMDIE_A {
    #[doc = "0: Interrupt disabled"]
    LEAPMCMDIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LEAPMCMDIE_1 = 1,
}
impl From<LEAPMCMDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LEAPMCMDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAPMCMDIE`"]
pub type LEAPMCMDIE_R = crate::R<bool, LEAPMCMDIE_A>;
impl LEAPMCMDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAPMCMDIE_A {
        match self.bits {
            false => LEAPMCMDIE_A::LEAPMCMDIE_0,
            true => LEAPMCMDIE_A::LEAPMCMDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAPMCMDIE_0`"]
    #[inline(always)]
    pub fn is_leapmcmdie_0(&self) -> bool {
        *self == LEAPMCMDIE_A::LEAPMCMDIE_0
    }
    #[doc = "Checks if the value of the field is `LEAPMCMDIE_1`"]
    #[inline(always)]
    pub fn is_leapmcmdie_1(&self) -> bool {
        *self == LEAPMCMDIE_A::LEAPMCMDIE_1
    }
}
#[doc = "Write proxy for field `LEAPMCMDIE`"]
pub struct LEAPMCMDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAPMCMDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAPMCMDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leapmcmdie_0(self) -> &'a mut W {
        self.variant(LEAPMCMDIE_A::LEAPMCMDIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leapmcmdie_1(self) -> &'a mut W {
        self.variant(LEAPMCMDIE_A::LEAPMCMDIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
LEA command overflow interrupt enable"]
    #[inline(always)]
    pub fn leacovlie(&self) -> LEACOVLIE_R {
        LEACOVLIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
LEA timer event interrupt enable"]
    #[inline(always)]
    pub fn leatie(&self) -> LEATIE_R {
        LEATIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
LEA out of address range interrupt enable."]
    #[inline(always)]
    pub fn leaoorie(&self) -> LEAOORIE_R {
        LEAOORIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
LEA scalar data inconsistency interrupt enable"]
    #[inline(always)]
    pub fn leasdiie(&self) -> LEASDIIE_R {
        LEASDIIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable."]
    #[inline(always)]
    pub fn leapmcmdie(&self) -> LEAPMCMDIE_R {
        LEAPMCMDIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LEA command overflow interrupt enable"]
    #[inline(always)]
    pub fn leacovlie(&mut self) -> LEACOVLIE_W {
        LEACOVLIE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
LEA timer event interrupt enable"]
    #[inline(always)]
    pub fn leatie(&mut self) -> LEATIE_W {
        LEATIE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
LEA out of address range interrupt enable."]
    #[inline(always)]
    pub fn leaoorie(&mut self) -> LEAOORIE_W {
        LEAOORIE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
LEA scalar data inconsistency interrupt enable"]
    #[inline(always)]
    pub fn leasdiie(&mut self) -> LEASDIIE_W {
        LEASDIIE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable."]
    #[inline(always)]
    pub fn leapmcmdie(&mut self) -> LEAPMCMDIE_W {
        LEAPMCMDIE_W { w: self }
    }
}
