#[doc = "Reader of register LEAIFGSET"]
pub type R = crate::R<u32, super::LEAIFGSET>;
#[doc = "Writer for register LEAIFGSET"]
pub type W = crate::W<u32, super::LEAIFGSET>;
#[doc = "Register LEAIFGSET `reset()`'s with value 0"]
impl crate::ResetValue for super::LEAIFGSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
LEA command overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEACOVLIS_A {
    #[doc = "0: No interrupt pending"]
    LEACOVLIS_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEACOVLIS_1 = 1,
}
impl From<LEACOVLIS_A> for bool {
    #[inline(always)]
    fn from(variant: LEACOVLIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEACOVLIS`"]
pub type LEACOVLIS_R = crate::R<bool, LEACOVLIS_A>;
impl LEACOVLIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEACOVLIS_A {
        match self.bits {
            false => LEACOVLIS_A::LEACOVLIS_0,
            true => LEACOVLIS_A::LEACOVLIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEACOVLIS_0`"]
    #[inline(always)]
    pub fn is_leacovlis_0(&self) -> bool {
        *self == LEACOVLIS_A::LEACOVLIS_0
    }
    #[doc = "Checks if the value of the field is `LEACOVLIS_1`"]
    #[inline(always)]
    pub fn is_leacovlis_1(&self) -> bool {
        *self == LEACOVLIS_A::LEACOVLIS_1
    }
}
#[doc = "Write proxy for field `LEACOVLIS`"]
pub struct LEACOVLIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACOVLIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACOVLIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leacovlis_0(self) -> &'a mut W {
        self.variant(LEACOVLIS_A::LEACOVLIS_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leacovlis_1(self) -> &'a mut W {
        self.variant(LEACOVLIS_A::LEACOVLIS_1)
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
LEA timer interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEATIS_A {
    #[doc = "0: No interrupt pending"]
    LEATIS_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEATIS_1 = 1,
}
impl From<LEATIS_A> for bool {
    #[inline(always)]
    fn from(variant: LEATIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEATIS`"]
pub type LEATIS_R = crate::R<bool, LEATIS_A>;
impl LEATIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEATIS_A {
        match self.bits {
            false => LEATIS_A::LEATIS_0,
            true => LEATIS_A::LEATIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEATIS_0`"]
    #[inline(always)]
    pub fn is_leatis_0(&self) -> bool {
        *self == LEATIS_A::LEATIS_0
    }
    #[doc = "Checks if the value of the field is `LEATIS_1`"]
    #[inline(always)]
    pub fn is_leatis_1(&self) -> bool {
        *self == LEATIS_A::LEATIS_1
    }
}
#[doc = "Write proxy for field `LEATIS`"]
pub struct LEATIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEATIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leatis_0(self) -> &'a mut W {
        self.variant(LEATIS_A::LEATIS_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leatis_1(self) -> &'a mut W {
        self.variant(LEATIS_A::LEATIS_1)
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
LEA out of address range interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAOORIS_A {
    #[doc = "0: No interrupt pending"]
    LEAOORIS_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEAOORIS_1 = 1,
}
impl From<LEAOORIS_A> for bool {
    #[inline(always)]
    fn from(variant: LEAOORIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAOORIS`"]
pub type LEAOORIS_R = crate::R<bool, LEAOORIS_A>;
impl LEAOORIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAOORIS_A {
        match self.bits {
            false => LEAOORIS_A::LEAOORIS_0,
            true => LEAOORIS_A::LEAOORIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAOORIS_0`"]
    #[inline(always)]
    pub fn is_leaooris_0(&self) -> bool {
        *self == LEAOORIS_A::LEAOORIS_0
    }
    #[doc = "Checks if the value of the field is `LEAOORIS_1`"]
    #[inline(always)]
    pub fn is_leaooris_1(&self) -> bool {
        *self == LEAOORIS_A::LEAOORIS_1
    }
}
#[doc = "Write proxy for field `LEAOORIS`"]
pub struct LEAOORIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAOORIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAOORIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leaooris_0(self) -> &'a mut W {
        self.variant(LEAOORIS_A::LEAOORIS_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leaooris_1(self) -> &'a mut W {
        self.variant(LEAOORIS_A::LEAOORIS_1)
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
LEA scalar data inconsistency interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEASDIIS_A {
    #[doc = "0: No interrupt pending"]
    LEASDIIS_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEASDIIS_1 = 1,
}
impl From<LEASDIIS_A> for bool {
    #[inline(always)]
    fn from(variant: LEASDIIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEASDIIS`"]
pub type LEASDIIS_R = crate::R<bool, LEASDIIS_A>;
impl LEASDIIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEASDIIS_A {
        match self.bits {
            false => LEASDIIS_A::LEASDIIS_0,
            true => LEASDIIS_A::LEASDIIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEASDIIS_0`"]
    #[inline(always)]
    pub fn is_leasdiis_0(&self) -> bool {
        *self == LEASDIIS_A::LEASDIIS_0
    }
    #[doc = "Checks if the value of the field is `LEASDIIS_1`"]
    #[inline(always)]
    pub fn is_leasdiis_1(&self) -> bool {
        *self == LEASDIIS_A::LEASDIIS_1
    }
}
#[doc = "Write proxy for field `LEASDIIS`"]
pub struct LEASDIIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEASDIIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEASDIIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leasdiis_0(self) -> &'a mut W {
        self.variant(LEASDIIS_A::LEASDIIS_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leasdiis_1(self) -> &'a mut W {
        self.variant(LEASDIIS_A::LEASDIIS_1)
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
PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAPMCMDIS_A {
    #[doc = "0: No interrupt pending"]
    LEAPMCMDIS_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEAPMCMDIS_1 = 1,
}
impl From<LEAPMCMDIS_A> for bool {
    #[inline(always)]
    fn from(variant: LEAPMCMDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAPMCMDIS`"]
pub type LEAPMCMDIS_R = crate::R<bool, LEAPMCMDIS_A>;
impl LEAPMCMDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAPMCMDIS_A {
        match self.bits {
            false => LEAPMCMDIS_A::LEAPMCMDIS_0,
            true => LEAPMCMDIS_A::LEAPMCMDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAPMCMDIS_0`"]
    #[inline(always)]
    pub fn is_leapmcmdis_0(&self) -> bool {
        *self == LEAPMCMDIS_A::LEAPMCMDIS_0
    }
    #[doc = "Checks if the value of the field is `LEAPMCMDIS_1`"]
    #[inline(always)]
    pub fn is_leapmcmdis_1(&self) -> bool {
        *self == LEAPMCMDIS_A::LEAPMCMDIS_1
    }
}
#[doc = "Write proxy for field `LEAPMCMDIS`"]
pub struct LEAPMCMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAPMCMDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAPMCMDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leapmcmdis_0(self) -> &'a mut W {
        self.variant(LEAPMCMDIS_A::LEAPMCMDIS_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leapmcmdis_1(self) -> &'a mut W {
        self.variant(LEAPMCMDIS_A::LEAPMCMDIS_1)
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
LEA command overflow interrupt flag"]
    #[inline(always)]
    pub fn leacovlis(&self) -> LEACOVLIS_R {
        LEACOVLIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
LEA timer interrupt flag"]
    #[inline(always)]
    pub fn leatis(&self) -> LEATIS_R {
        LEATIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
LEA out of address range interrupt flag."]
    #[inline(always)]
    pub fn leaooris(&self) -> LEAOORIS_R {
        LEAOORIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
LEA scalar data inconsistency interrupt flag"]
    #[inline(always)]
    pub fn leasdiis(&self) -> LEASDIIS_R {
        LEASDIIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag."]
    #[inline(always)]
    pub fn leapmcmdis(&self) -> LEAPMCMDIS_R {
        LEAPMCMDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LEA command overflow interrupt flag"]
    #[inline(always)]
    pub fn leacovlis(&mut self) -> LEACOVLIS_W {
        LEACOVLIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
LEA timer interrupt flag"]
    #[inline(always)]
    pub fn leatis(&mut self) -> LEATIS_W {
        LEATIS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
LEA out of address range interrupt flag."]
    #[inline(always)]
    pub fn leaooris(&mut self) -> LEAOORIS_W {
        LEAOORIS_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
LEA scalar data inconsistency interrupt flag"]
    #[inline(always)]
    pub fn leasdiis(&mut self) -> LEASDIIS_W {
        LEASDIIS_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag."]
    #[inline(always)]
    pub fn leapmcmdis(&mut self) -> LEAPMCMDIS_W {
        LEAPMCMDIS_W { w: self }
    }
}
