#[doc = "Reader of register MPUCTL1"]
pub type R = crate::R<u16, super::MPUCTL1>;
#[doc = "Writer for register MPUCTL1"]
pub type W = crate::W<u16, super::MPUCTL1>;
#[doc = "Register MPUCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
Main Memory Segment 1 Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG1IFG_A {
    #[doc = "0: No interrupt pending"]
    MPUSEG1IFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    MPUSEG1IFG_1 = 1,
}
impl From<MPUSEG1IFG_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG1IFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG1IFG`"]
pub type MPUSEG1IFG_R = crate::R<bool, MPUSEG1IFG_A>;
impl MPUSEG1IFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG1IFG_A {
        match self.bits {
            false => MPUSEG1IFG_A::MPUSEG1IFG_0,
            true => MPUSEG1IFG_A::MPUSEG1IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUSEG1IFG_0`"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_0(&self) -> bool {
        *self == MPUSEG1IFG_A::MPUSEG1IFG_0
    }
    #[doc = "Checks if the value of the field is `MPUSEG1IFG_1`"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_1(&self) -> bool {
        *self == MPUSEG1IFG_A::MPUSEG1IFG_1
    }
}
#[doc = "Write proxy for field `MPUSEG1IFG`"]
pub struct MPUSEG1IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG1IFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_0(self) -> &'a mut W {
        self.variant(MPUSEG1IFG_A::MPUSEG1IFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_1(self) -> &'a mut W {
        self.variant(MPUSEG1IFG_A::MPUSEG1IFG_1)
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
Main Memory Segment 2 Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG2IFG_A {
    #[doc = "0: No interrupt pending"]
    MPUSEG2IFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    MPUSEG2IFG_1 = 1,
}
impl From<MPUSEG2IFG_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG2IFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG2IFG`"]
pub type MPUSEG2IFG_R = crate::R<bool, MPUSEG2IFG_A>;
impl MPUSEG2IFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG2IFG_A {
        match self.bits {
            false => MPUSEG2IFG_A::MPUSEG2IFG_0,
            true => MPUSEG2IFG_A::MPUSEG2IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUSEG2IFG_0`"]
    #[inline(always)]
    pub fn is_mpuseg2ifg_0(&self) -> bool {
        *self == MPUSEG2IFG_A::MPUSEG2IFG_0
    }
    #[doc = "Checks if the value of the field is `MPUSEG2IFG_1`"]
    #[inline(always)]
    pub fn is_mpuseg2ifg_1(&self) -> bool {
        *self == MPUSEG2IFG_A::MPUSEG2IFG_1
    }
}
#[doc = "Write proxy for field `MPUSEG2IFG`"]
pub struct MPUSEG2IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG2IFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpuseg2ifg_0(self) -> &'a mut W {
        self.variant(MPUSEG2IFG_A::MPUSEG2IFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpuseg2ifg_1(self) -> &'a mut W {
        self.variant(MPUSEG2IFG_A::MPUSEG2IFG_1)
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
Main Memory Segment 3 Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG3IFG_A {
    #[doc = "0: No interrupt pending"]
    MPUSEG1IFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    MPUSEG1IFG_1 = 1,
}
impl From<MPUSEG3IFG_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG3IFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG3IFG`"]
pub type MPUSEG3IFG_R = crate::R<bool, MPUSEG3IFG_A>;
impl MPUSEG3IFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG3IFG_A {
        match self.bits {
            false => MPUSEG3IFG_A::MPUSEG1IFG_0,
            true => MPUSEG3IFG_A::MPUSEG1IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUSEG1IFG_0`"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_0(&self) -> bool {
        *self == MPUSEG3IFG_A::MPUSEG1IFG_0
    }
    #[doc = "Checks if the value of the field is `MPUSEG1IFG_1`"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_1(&self) -> bool {
        *self == MPUSEG3IFG_A::MPUSEG1IFG_1
    }
}
#[doc = "Write proxy for field `MPUSEG3IFG`"]
pub struct MPUSEG3IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG3IFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_0(self) -> &'a mut W {
        self.variant(MPUSEG3IFG_A::MPUSEG1IFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_1(self) -> &'a mut W {
        self.variant(MPUSEG3IFG_A::MPUSEG1IFG_1)
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
User Information Memory Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEGIIFG_A {
    #[doc = "0: No interrupt pending"]
    MPUSEGIIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    MPUSEGIIFG_1 = 1,
}
impl From<MPUSEGIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEGIIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEGIIFG`"]
pub type MPUSEGIIFG_R = crate::R<bool, MPUSEGIIFG_A>;
impl MPUSEGIIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEGIIFG_A {
        match self.bits {
            false => MPUSEGIIFG_A::MPUSEGIIFG_0,
            true => MPUSEGIIFG_A::MPUSEGIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUSEGIIFG_0`"]
    #[inline(always)]
    pub fn is_mpusegiifg_0(&self) -> bool {
        *self == MPUSEGIIFG_A::MPUSEGIIFG_0
    }
    #[doc = "Checks if the value of the field is `MPUSEGIIFG_1`"]
    #[inline(always)]
    pub fn is_mpusegiifg_1(&self) -> bool {
        *self == MPUSEGIIFG_A::MPUSEGIIFG_1
    }
}
#[doc = "Write proxy for field `MPUSEGIIFG`"]
pub struct MPUSEGIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEGIIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpusegiifg_0(self) -> &'a mut W {
        self.variant(MPUSEGIIFG_A::MPUSEGIIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpusegiifg_1(self) -> &'a mut W {
        self.variant(MPUSEGIIFG_A::MPUSEGIIFG_1)
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
#[doc = "4:4\\]
IP Encapsulation Access Violation Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEGIPIFG_A {
    #[doc = "0: No interrupt pending"]
    MPUSEG1IFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    MPUSEG1IFG_1 = 1,
}
impl From<MPUSEGIPIFG_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEGIPIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEGIPIFG`"]
pub type MPUSEGIPIFG_R = crate::R<bool, MPUSEGIPIFG_A>;
impl MPUSEGIPIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEGIPIFG_A {
        match self.bits {
            false => MPUSEGIPIFG_A::MPUSEG1IFG_0,
            true => MPUSEGIPIFG_A::MPUSEG1IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUSEG1IFG_0`"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_0(&self) -> bool {
        *self == MPUSEGIPIFG_A::MPUSEG1IFG_0
    }
    #[doc = "Checks if the value of the field is `MPUSEG1IFG_1`"]
    #[inline(always)]
    pub fn is_mpuseg1ifg_1(&self) -> bool {
        *self == MPUSEGIPIFG_A::MPUSEG1IFG_1
    }
}
#[doc = "Write proxy for field `MPUSEGIPIFG`"]
pub struct MPUSEGIPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIPIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEGIPIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_0(self) -> &'a mut W {
        self.variant(MPUSEGIPIFG_A::MPUSEG1IFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn mpuseg1ifg_1(self) -> &'a mut W {
        self.variant(MPUSEGIPIFG_A::MPUSEG1IFG_1)
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
impl R {
    #[doc = "Bit 0 - 0:0\\]
Main Memory Segment 1 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg1ifg(&self) -> MPUSEG1IFG_R {
        MPUSEG1IFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Main Memory Segment 2 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg2ifg(&self) -> MPUSEG2IFG_R {
        MPUSEG2IFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Main Memory Segment 3 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg3ifg(&self) -> MPUSEG3IFG_R {
        MPUSEG3IFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
User Information Memory Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpusegiifg(&self) -> MPUSEGIIFG_R {
        MPUSEGIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
IP Encapsulation Access Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpusegipifg(&self) -> MPUSEGIPIFG_R {
        MPUSEGIPIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Main Memory Segment 1 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg1ifg(&mut self) -> MPUSEG1IFG_W {
        MPUSEG1IFG_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Main Memory Segment 2 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg2ifg(&mut self) -> MPUSEG2IFG_W {
        MPUSEG2IFG_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Main Memory Segment 3 Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpuseg3ifg(&mut self) -> MPUSEG3IFG_W {
        MPUSEG3IFG_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
User Information Memory Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpusegiifg(&mut self) -> MPUSEGIIFG_W {
        MPUSEGIIFG_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
IP Encapsulation Access Violation Interrupt Flag"]
    #[inline(always)]
    pub fn mpusegipifg(&mut self) -> MPUSEGIPIFG_W {
        MPUSEGIPIFG_W { w: self }
    }
}
