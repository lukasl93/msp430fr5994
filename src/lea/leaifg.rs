#[doc = "Reader of register LEAIFG"]
pub type R = crate::R<u32, super::LEAIFG>;
#[doc = "Writer for register LEAIFG"]
pub type W = crate::W<u32, super::LEAIFG>;
#[doc = "Register LEAIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::LEAIFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
LEA command overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEACOVLIFG_A {
    #[doc = "0: No interrupt pending"]
    LEACOVLIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEACOVLIFG_1 = 1,
}
impl From<LEACOVLIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LEACOVLIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEACOVLIFG`"]
pub type LEACOVLIFG_R = crate::R<bool, LEACOVLIFG_A>;
impl LEACOVLIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEACOVLIFG_A {
        match self.bits {
            false => LEACOVLIFG_A::LEACOVLIFG_0,
            true => LEACOVLIFG_A::LEACOVLIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEACOVLIFG_0`"]
    #[inline(always)]
    pub fn is_leacovlifg_0(&self) -> bool {
        *self == LEACOVLIFG_A::LEACOVLIFG_0
    }
    #[doc = "Checks if the value of the field is `LEACOVLIFG_1`"]
    #[inline(always)]
    pub fn is_leacovlifg_1(&self) -> bool {
        *self == LEACOVLIFG_A::LEACOVLIFG_1
    }
}
#[doc = "Write proxy for field `LEACOVLIFG`"]
pub struct LEACOVLIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACOVLIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACOVLIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leacovlifg_0(self) -> &'a mut W {
        self.variant(LEACOVLIFG_A::LEACOVLIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leacovlifg_1(self) -> &'a mut W {
        self.variant(LEACOVLIFG_A::LEACOVLIFG_1)
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
pub enum LEATIFG_A {
    #[doc = "0: No interrupt pending"]
    LEATIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEATIFG_1 = 1,
}
impl From<LEATIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LEATIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEATIFG`"]
pub type LEATIFG_R = crate::R<bool, LEATIFG_A>;
impl LEATIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEATIFG_A {
        match self.bits {
            false => LEATIFG_A::LEATIFG_0,
            true => LEATIFG_A::LEATIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEATIFG_0`"]
    #[inline(always)]
    pub fn is_leatifg_0(&self) -> bool {
        *self == LEATIFG_A::LEATIFG_0
    }
    #[doc = "Checks if the value of the field is `LEATIFG_1`"]
    #[inline(always)]
    pub fn is_leatifg_1(&self) -> bool {
        *self == LEATIFG_A::LEATIFG_1
    }
}
#[doc = "Write proxy for field `LEATIFG`"]
pub struct LEATIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEATIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leatifg_0(self) -> &'a mut W {
        self.variant(LEATIFG_A::LEATIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leatifg_1(self) -> &'a mut W {
        self.variant(LEATIFG_A::LEATIFG_1)
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
pub enum LEAOORIFG_A {
    #[doc = "0: No interrupt pending"]
    LEAOORIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEAOORIFG_1 = 1,
}
impl From<LEAOORIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LEAOORIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAOORIFG`"]
pub type LEAOORIFG_R = crate::R<bool, LEAOORIFG_A>;
impl LEAOORIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAOORIFG_A {
        match self.bits {
            false => LEAOORIFG_A::LEAOORIFG_0,
            true => LEAOORIFG_A::LEAOORIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAOORIFG_0`"]
    #[inline(always)]
    pub fn is_leaoorifg_0(&self) -> bool {
        *self == LEAOORIFG_A::LEAOORIFG_0
    }
    #[doc = "Checks if the value of the field is `LEAOORIFG_1`"]
    #[inline(always)]
    pub fn is_leaoorifg_1(&self) -> bool {
        *self == LEAOORIFG_A::LEAOORIFG_1
    }
}
#[doc = "Write proxy for field `LEAOORIFG`"]
pub struct LEAOORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAOORIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAOORIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leaoorifg_0(self) -> &'a mut W {
        self.variant(LEAOORIFG_A::LEAOORIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leaoorifg_1(self) -> &'a mut W {
        self.variant(LEAOORIFG_A::LEAOORIFG_1)
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
pub enum LEASDIIFG_A {
    #[doc = "0: No interrupt pending"]
    LEASDIIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEASDIIFG_1 = 1,
}
impl From<LEASDIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LEASDIIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEASDIIFG`"]
pub type LEASDIIFG_R = crate::R<bool, LEASDIIFG_A>;
impl LEASDIIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEASDIIFG_A {
        match self.bits {
            false => LEASDIIFG_A::LEASDIIFG_0,
            true => LEASDIIFG_A::LEASDIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEASDIIFG_0`"]
    #[inline(always)]
    pub fn is_leasdiifg_0(&self) -> bool {
        *self == LEASDIIFG_A::LEASDIIFG_0
    }
    #[doc = "Checks if the value of the field is `LEASDIIFG_1`"]
    #[inline(always)]
    pub fn is_leasdiifg_1(&self) -> bool {
        *self == LEASDIIFG_A::LEASDIIFG_1
    }
}
#[doc = "Write proxy for field `LEASDIIFG`"]
pub struct LEASDIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LEASDIIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEASDIIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leasdiifg_0(self) -> &'a mut W {
        self.variant(LEASDIIFG_A::LEASDIIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leasdiifg_1(self) -> &'a mut W {
        self.variant(LEASDIIFG_A::LEASDIIFG_1)
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
PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAPMCMDIFG_A {
    #[doc = "0: No interrupt pending"]
    LEAPMCMDIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    LEAPMCMDIFG_1 = 1,
}
impl From<LEAPMCMDIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LEAPMCMDIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAPMCMDIFG`"]
pub type LEAPMCMDIFG_R = crate::R<bool, LEAPMCMDIFG_A>;
impl LEAPMCMDIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAPMCMDIFG_A {
        match self.bits {
            false => LEAPMCMDIFG_A::LEAPMCMDIFG_0,
            true => LEAPMCMDIFG_A::LEAPMCMDIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAPMCMDIFG_0`"]
    #[inline(always)]
    pub fn is_leapmcmdifg_0(&self) -> bool {
        *self == LEAPMCMDIFG_A::LEAPMCMDIFG_0
    }
    #[doc = "Checks if the value of the field is `LEAPMCMDIFG_1`"]
    #[inline(always)]
    pub fn is_leapmcmdifg_1(&self) -> bool {
        *self == LEAPMCMDIFG_A::LEAPMCMDIFG_1
    }
}
#[doc = "Write proxy for field `LEAPMCMDIFG`"]
pub struct LEAPMCMDIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAPMCMDIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAPMCMDIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leapmcmdifg_0(self) -> &'a mut W {
        self.variant(LEAPMCMDIFG_A::LEAPMCMDIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leapmcmdifg_1(self) -> &'a mut W {
        self.variant(LEAPMCMDIFG_A::LEAPMCMDIFG_1)
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
    pub fn leacovlifg(&self) -> LEACOVLIFG_R {
        LEACOVLIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
LEA timer interrupt flag"]
    #[inline(always)]
    pub fn leatifg(&self) -> LEATIFG_R {
        LEATIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
LEA out of address range interrupt flag."]
    #[inline(always)]
    pub fn leaoorifg(&self) -> LEAOORIFG_R {
        LEAOORIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
LEA scalar data inconsistency interrupt flag"]
    #[inline(always)]
    pub fn leasdiifg(&self) -> LEASDIIFG_R {
        LEASDIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag."]
    #[inline(always)]
    pub fn leapmcmdifg(&self) -> LEAPMCMDIFG_R {
        LEAPMCMDIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LEA command overflow interrupt flag"]
    #[inline(always)]
    pub fn leacovlifg(&mut self) -> LEACOVLIFG_W {
        LEACOVLIFG_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
LEA timer interrupt flag"]
    #[inline(always)]
    pub fn leatifg(&mut self) -> LEATIFG_W {
        LEATIFG_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
LEA out of address range interrupt flag."]
    #[inline(always)]
    pub fn leaoorifg(&mut self) -> LEAOORIFG_W {
        LEAOORIFG_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
LEA scalar data inconsistency interrupt flag"]
    #[inline(always)]
    pub fn leasdiifg(&mut self) -> LEASDIIFG_W {
        LEASDIIFG_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag."]
    #[inline(always)]
    pub fn leapmcmdifg(&mut self) -> LEAPMCMDIFG_W {
        LEAPMCMDIFG_W { w: self }
    }
}
