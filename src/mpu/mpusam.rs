#[doc = "Reader of register MPUSAM"]
pub type R = crate::R<u16, super::MPUSAM>;
#[doc = "Writer for register MPUSAM"]
pub type W = crate::W<u16, super::MPUSAM>;
#[doc = "Register MPUSAM `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUSAM {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
MPU Main Memory Segment 1 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG1RE_A {
    #[doc = "0: Read on Main Memory Segment 1 causes a violation if MPUSEG1WE = MPUSEG1XE = 0"]
    DISABLE = 0,
    #[doc = "1: Read on Main Memory Segment 1 is allowed"]
    ENABLE = 1,
}
impl From<MPUSEG1RE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG1RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG1RE`"]
pub type MPUSEG1RE_R = crate::R<bool, MPUSEG1RE_A>;
impl MPUSEG1RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG1RE_A {
        match self.bits {
            false => MPUSEG1RE_A::DISABLE,
            true => MPUSEG1RE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEG1RE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEG1RE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEG1RE`"]
pub struct MPUSEG1RE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG1RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read on Main Memory Segment 1 causes a violation if MPUSEG1WE = MPUSEG1XE = 0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEG1RE_A::DISABLE)
    }
    #[doc = "Read on Main Memory Segment 1 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEG1RE_A::ENABLE)
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
MPU Main Memory Segment 1 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG1WE_A {
    #[doc = "0: Write on Main Memory Segment 1 causes a violation"]
    DISABLE = 0,
    #[doc = "1: Write on Main Memory Segment 1 is allowed"]
    ENABLE = 1,
}
impl From<MPUSEG1WE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG1WE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG1WE`"]
pub type MPUSEG1WE_R = crate::R<bool, MPUSEG1WE_A>;
impl MPUSEG1WE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG1WE_A {
        match self.bits {
            false => MPUSEG1WE_A::DISABLE,
            true => MPUSEG1WE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEG1WE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEG1WE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEG1WE`"]
pub struct MPUSEG1WE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG1WE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write on Main Memory Segment 1 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEG1WE_A::DISABLE)
    }
    #[doc = "Write on Main Memory Segment 1 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEG1WE_A::ENABLE)
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
MPU Main Memory Segment 1 Execute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG1XE_A {
    #[doc = "0: Execute code on Main Memory Segment 1 causes a violation"]
    DISABLE = 0,
    #[doc = "1: Execute code on Main Memory Segment 1 is allowed"]
    ENABLE = 1,
}
impl From<MPUSEG1XE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG1XE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG1XE`"]
pub type MPUSEG1XE_R = crate::R<bool, MPUSEG1XE_A>;
impl MPUSEG1XE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG1XE_A {
        match self.bits {
            false => MPUSEG1XE_A::DISABLE,
            true => MPUSEG1XE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEG1XE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEG1XE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEG1XE`"]
pub struct MPUSEG1XE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1XE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG1XE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Execute code on Main Memory Segment 1 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEG1XE_A::DISABLE)
    }
    #[doc = "Execute code on Main Memory Segment 1 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEG1XE_A::ENABLE)
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
MPU Main Memory Segment 1 Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG1VS_A {
    #[doc = "0: Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    MPUSEG1VS_0 = 0,
    #[doc = "1: Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a PUC"]
    MPUSEG1VS_1 = 1,
}
impl From<MPUSEG1VS_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG1VS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG1VS`"]
pub type MPUSEG1VS_R = crate::R<bool, MPUSEG1VS_A>;
impl MPUSEG1VS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG1VS_A {
        match self.bits {
            false => MPUSEG1VS_A::MPUSEG1VS_0,
            true => MPUSEG1VS_A::MPUSEG1VS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUSEG1VS_0`"]
    #[inline(always)]
    pub fn is_mpuseg1vs_0(&self) -> bool {
        *self == MPUSEG1VS_A::MPUSEG1VS_0
    }
    #[doc = "Checks if the value of the field is `MPUSEG1VS_1`"]
    #[inline(always)]
    pub fn is_mpuseg1vs_1(&self) -> bool {
        *self == MPUSEG1VS_A::MPUSEG1VS_1
    }
}
#[doc = "Write proxy for field `MPUSEG1VS`"]
pub struct MPUSEG1VS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1VS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG1VS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn mpuseg1vs_0(self) -> &'a mut W {
        self.variant(MPUSEG1VS_A::MPUSEG1VS_0)
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEG1IFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpuseg1vs_1(self) -> &'a mut W {
        self.variant(MPUSEG1VS_A::MPUSEG1VS_1)
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
MPU Main Memory Segment 2 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG2RE_A {
    #[doc = "0: Read on Main Memory Segment 2 causes a violation if MPUSEG2WE = MPUSEG2XE = 0"]
    DISABLE = 0,
    #[doc = "1: Read on Main Memory Segment 2 is allowed"]
    ENABLE = 1,
}
impl From<MPUSEG2RE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG2RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG2RE`"]
pub type MPUSEG2RE_R = crate::R<bool, MPUSEG2RE_A>;
impl MPUSEG2RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG2RE_A {
        match self.bits {
            false => MPUSEG2RE_A::DISABLE,
            true => MPUSEG2RE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEG2RE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEG2RE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEG2RE`"]
pub struct MPUSEG2RE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG2RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read on Main Memory Segment 2 causes a violation if MPUSEG2WE = MPUSEG2XE = 0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEG2RE_A::DISABLE)
    }
    #[doc = "Read on Main Memory Segment 2 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEG2RE_A::ENABLE)
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
MPU Main Memory Segment 2 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG2WE_A {
    #[doc = "0: Write on Main Memory Segment 2 causes a violation"]
    DISABLE = 0,
    #[doc = "1: Write on Main Memory Segment 2 is allowed"]
    ENABLE = 1,
}
impl From<MPUSEG2WE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG2WE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG2WE`"]
pub type MPUSEG2WE_R = crate::R<bool, MPUSEG2WE_A>;
impl MPUSEG2WE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG2WE_A {
        match self.bits {
            false => MPUSEG2WE_A::DISABLE,
            true => MPUSEG2WE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEG2WE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEG2WE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEG2WE`"]
pub struct MPUSEG2WE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG2WE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write on Main Memory Segment 2 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEG2WE_A::DISABLE)
    }
    #[doc = "Write on Main Memory Segment 2 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEG2WE_A::ENABLE)
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
MPU Main Memory Segment 2 Execute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG2XE_A {
    #[doc = "0: Execute code on Main Memory Segment 2 causes a violation"]
    DISABLE = 0,
    #[doc = "1: Execute code on Main Memory Segment 2 is allowed"]
    ENABLE = 1,
}
impl From<MPUSEG2XE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG2XE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG2XE`"]
pub type MPUSEG2XE_R = crate::R<bool, MPUSEG2XE_A>;
impl MPUSEG2XE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG2XE_A {
        match self.bits {
            false => MPUSEG2XE_A::DISABLE,
            true => MPUSEG2XE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEG2XE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEG2XE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEG2XE`"]
pub struct MPUSEG2XE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2XE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG2XE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Execute code on Main Memory Segment 2 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEG2XE_A::DISABLE)
    }
    #[doc = "Execute code on Main Memory Segment 2 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEG2XE_A::ENABLE)
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
MPU Main Memory Segment 2 Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG2VS_A {
    #[doc = "0: Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    MPUSEG2VS_0 = 0,
    #[doc = "1: Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a PUC"]
    MPUSEG2VS_1 = 1,
}
impl From<MPUSEG2VS_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG2VS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG2VS`"]
pub type MPUSEG2VS_R = crate::R<bool, MPUSEG2VS_A>;
impl MPUSEG2VS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG2VS_A {
        match self.bits {
            false => MPUSEG2VS_A::MPUSEG2VS_0,
            true => MPUSEG2VS_A::MPUSEG2VS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUSEG2VS_0`"]
    #[inline(always)]
    pub fn is_mpuseg2vs_0(&self) -> bool {
        *self == MPUSEG2VS_A::MPUSEG2VS_0
    }
    #[doc = "Checks if the value of the field is `MPUSEG2VS_1`"]
    #[inline(always)]
    pub fn is_mpuseg2vs_1(&self) -> bool {
        *self == MPUSEG2VS_A::MPUSEG2VS_1
    }
}
#[doc = "Write proxy for field `MPUSEG2VS`"]
pub struct MPUSEG2VS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2VS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG2VS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn mpuseg2vs_0(self) -> &'a mut W {
        self.variant(MPUSEG2VS_A::MPUSEG2VS_0)
    }
    #[doc = "Violation in Main Memory Segment 2 asserts the MPUSEG2IFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpuseg2vs_1(self) -> &'a mut W {
        self.variant(MPUSEG2VS_A::MPUSEG2VS_1)
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
#[doc = "8:8\\]
MPU Main Memory Segment 3 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG3RE_A {
    #[doc = "0: Read on Main Memory Segment 3 causes a violation if MPUSEG3WE = MPUSEG3XE = 0"]
    DISABLE = 0,
    #[doc = "1: Read on Main Memory Segment 3 is allowed"]
    ENABLE = 1,
}
impl From<MPUSEG3RE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG3RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG3RE`"]
pub type MPUSEG3RE_R = crate::R<bool, MPUSEG3RE_A>;
impl MPUSEG3RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG3RE_A {
        match self.bits {
            false => MPUSEG3RE_A::DISABLE,
            true => MPUSEG3RE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEG3RE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEG3RE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEG3RE`"]
pub struct MPUSEG3RE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG3RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read on Main Memory Segment 3 causes a violation if MPUSEG3WE = MPUSEG3XE = 0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEG3RE_A::DISABLE)
    }
    #[doc = "Read on Main Memory Segment 3 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEG3RE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "9:9\\]
MPU Main Memory Segment 3 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG3WE_A {
    #[doc = "0: Write on Main Memory Segment 3 causes a violation"]
    DISABLE = 0,
    #[doc = "1: Write on Main Memory Segment 3 is allowed"]
    ENABLE = 1,
}
impl From<MPUSEG3WE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG3WE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG3WE`"]
pub type MPUSEG3WE_R = crate::R<bool, MPUSEG3WE_A>;
impl MPUSEG3WE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG3WE_A {
        match self.bits {
            false => MPUSEG3WE_A::DISABLE,
            true => MPUSEG3WE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEG3WE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEG3WE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEG3WE`"]
pub struct MPUSEG3WE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG3WE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write on Main Memory Segment 3 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEG3WE_A::DISABLE)
    }
    #[doc = "Write on Main Memory Segment 3 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEG3WE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "10:10\\]
MPU Main Memory Segment 3 Execute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG3XE_A {
    #[doc = "0: Execute code on Main Memory Segment 3 causes a violation"]
    DISABLE = 0,
    #[doc = "1: Execute code on Main Memory Segment 3 is allowed"]
    ENABLE = 1,
}
impl From<MPUSEG3XE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG3XE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG3XE`"]
pub type MPUSEG3XE_R = crate::R<bool, MPUSEG3XE_A>;
impl MPUSEG3XE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG3XE_A {
        match self.bits {
            false => MPUSEG3XE_A::DISABLE,
            true => MPUSEG3XE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEG3XE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEG3XE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEG3XE`"]
pub struct MPUSEG3XE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3XE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG3XE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Execute code on Main Memory Segment 3 causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEG3XE_A::DISABLE)
    }
    #[doc = "Execute code on Main Memory Segment 3 is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEG3XE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "11:11\\]
MPU Main Memory Segment 3 Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEG3VS_A {
    #[doc = "0: Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    MPUSEG3VS_0 = 0,
    #[doc = "1: Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a PUC"]
    MPUSEG3VS_1 = 1,
}
impl From<MPUSEG3VS_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEG3VS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEG3VS`"]
pub type MPUSEG3VS_R = crate::R<bool, MPUSEG3VS_A>;
impl MPUSEG3VS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEG3VS_A {
        match self.bits {
            false => MPUSEG3VS_A::MPUSEG3VS_0,
            true => MPUSEG3VS_A::MPUSEG3VS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUSEG3VS_0`"]
    #[inline(always)]
    pub fn is_mpuseg3vs_0(&self) -> bool {
        *self == MPUSEG3VS_A::MPUSEG3VS_0
    }
    #[doc = "Checks if the value of the field is `MPUSEG3VS_1`"]
    #[inline(always)]
    pub fn is_mpuseg3vs_1(&self) -> bool {
        *self == MPUSEG3VS_A::MPUSEG3VS_1
    }
}
#[doc = "Write proxy for field `MPUSEG3VS`"]
pub struct MPUSEG3VS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3VS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEG3VS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn mpuseg3vs_0(self) -> &'a mut W {
        self.variant(MPUSEG3VS_A::MPUSEG3VS_0)
    }
    #[doc = "Violation in Main Memory Segment 3 asserts the MPUSEG3IFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpuseg3vs_1(self) -> &'a mut W {
        self.variant(MPUSEG3VS_A::MPUSEG3VS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "12:12\\]
MPU User Information Memory Segment Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEGIRE_A {
    #[doc = "0: Read on User Information Memory causes a violation if MPUSEGIWE=MPUSEGIXE=0"]
    DISABLE = 0,
    #[doc = "1: Read on User Information Memory is allowed"]
    ENABLE = 1,
}
impl From<MPUSEGIRE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEGIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEGIRE`"]
pub type MPUSEGIRE_R = crate::R<bool, MPUSEGIRE_A>;
impl MPUSEGIRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEGIRE_A {
        match self.bits {
            false => MPUSEGIRE_A::DISABLE,
            true => MPUSEGIRE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEGIRE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEGIRE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEGIRE`"]
pub struct MPUSEGIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEGIRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read on User Information Memory causes a violation if MPUSEGIWE=MPUSEGIXE=0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEGIRE_A::DISABLE)
    }
    #[doc = "Read on User Information Memory is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEGIRE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "13:13\\]
MPU User Information Memory Segment Write Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEGIWE_A {
    #[doc = "0: Write on User Information Memory causes a violation"]
    DISABLE = 0,
    #[doc = "1: Write on User Information Memory is allowed"]
    ENABLE = 1,
}
impl From<MPUSEGIWE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEGIWE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEGIWE`"]
pub type MPUSEGIWE_R = crate::R<bool, MPUSEGIWE_A>;
impl MPUSEGIWE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEGIWE_A {
        match self.bits {
            false => MPUSEGIWE_A::DISABLE,
            true => MPUSEGIWE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEGIWE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEGIWE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEGIWE`"]
pub struct MPUSEGIWE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEGIWE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write on User Information Memory causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEGIWE_A::DISABLE)
    }
    #[doc = "Write on User Information Memory is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEGIWE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "14:14\\]
MPU User Information Memory Segment Execute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEGIXE_A {
    #[doc = "0: Execute code on User Information Memory causes a violation"]
    DISABLE = 0,
    #[doc = "1: Execute code on User Information Memory is allowed"]
    ENABLE = 1,
}
impl From<MPUSEGIXE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEGIXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEGIXE`"]
pub type MPUSEGIXE_R = crate::R<bool, MPUSEGIXE_A>;
impl MPUSEGIXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEGIXE_A {
        match self.bits {
            false => MPUSEGIXE_A::DISABLE,
            true => MPUSEGIXE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEGIXE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEGIXE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEGIXE`"]
pub struct MPUSEGIXE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEGIXE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Execute code on User Information Memory causes a violation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEGIXE_A::DISABLE)
    }
    #[doc = "Execute code on User Information Memory is allowed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEGIXE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "15:15\\]
MPU User Information Memory Segment Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEGIVS_A {
    #[doc = "0: Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a SNMI if enabled by MPUSEGIE =1"]
    MPUSEGIVS_0 = 0,
    #[doc = "1: Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a PUC"]
    MPUSEGIVS_1 = 1,
}
impl From<MPUSEGIVS_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEGIVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEGIVS`"]
pub type MPUSEGIVS_R = crate::R<bool, MPUSEGIVS_A>;
impl MPUSEGIVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEGIVS_A {
        match self.bits {
            false => MPUSEGIVS_A::MPUSEGIVS_0,
            true => MPUSEGIVS_A::MPUSEGIVS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUSEGIVS_0`"]
    #[inline(always)]
    pub fn is_mpusegivs_0(&self) -> bool {
        *self == MPUSEGIVS_A::MPUSEGIVS_0
    }
    #[doc = "Checks if the value of the field is `MPUSEGIVS_1`"]
    #[inline(always)]
    pub fn is_mpusegivs_1(&self) -> bool {
        *self == MPUSEGIVS_A::MPUSEGIVS_1
    }
}
#[doc = "Write proxy for field `MPUSEGIVS`"]
pub struct MPUSEGIVS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEGIVS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a SNMI if enabled by MPUSEGIE =1"]
    #[inline(always)]
    pub fn mpusegivs_0(self) -> &'a mut W {
        self.variant(MPUSEGIVS_A::MPUSEGIVS_0)
    }
    #[doc = "Violation in User Information Memory asserts the MPUSEGIIFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpusegivs_1(self) -> &'a mut W {
        self.variant(MPUSEGIVS_A::MPUSEGIVS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
MPU Main Memory Segment 1 Read Enable"]
    #[inline(always)]
    pub fn mpuseg1re(&self) -> MPUSEG1RE_R {
        MPUSEG1RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MPU Main Memory Segment 1 Write Enable"]
    #[inline(always)]
    pub fn mpuseg1we(&self) -> MPUSEG1WE_R {
        MPUSEG1WE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MPU Main Memory Segment 1 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg1xe(&self) -> MPUSEG1XE_R {
        MPUSEG1XE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
MPU Main Memory Segment 1 Violation Select"]
    #[inline(always)]
    pub fn mpuseg1vs(&self) -> MPUSEG1VS_R {
        MPUSEG1VS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
MPU Main Memory Segment 2 Read Enable"]
    #[inline(always)]
    pub fn mpuseg2re(&self) -> MPUSEG2RE_R {
        MPUSEG2RE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
MPU Main Memory Segment 2 Write Enable"]
    #[inline(always)]
    pub fn mpuseg2we(&self) -> MPUSEG2WE_R {
        MPUSEG2WE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
MPU Main Memory Segment 2 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg2xe(&self) -> MPUSEG2XE_R {
        MPUSEG2XE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
MPU Main Memory Segment 2 Violation Select"]
    #[inline(always)]
    pub fn mpuseg2vs(&self) -> MPUSEG2VS_R {
        MPUSEG2VS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MPU Main Memory Segment 3 Read Enable"]
    #[inline(always)]
    pub fn mpuseg3re(&self) -> MPUSEG3RE_R {
        MPUSEG3RE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
MPU Main Memory Segment 3 Write Enable"]
    #[inline(always)]
    pub fn mpuseg3we(&self) -> MPUSEG3WE_R {
        MPUSEG3WE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
MPU Main Memory Segment 3 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg3xe(&self) -> MPUSEG3XE_R {
        MPUSEG3XE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
MPU Main Memory Segment 3 Violation Select"]
    #[inline(always)]
    pub fn mpuseg3vs(&self) -> MPUSEG3VS_R {
        MPUSEG3VS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
MPU User Information Memory Segment Read Enable"]
    #[inline(always)]
    pub fn mpusegire(&self) -> MPUSEGIRE_R {
        MPUSEGIRE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
MPU User Information Memory Segment Write Enable."]
    #[inline(always)]
    pub fn mpusegiwe(&self) -> MPUSEGIWE_R {
        MPUSEGIWE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
MPU User Information Memory Segment Execute Enable"]
    #[inline(always)]
    pub fn mpusegixe(&self) -> MPUSEGIXE_R {
        MPUSEGIXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
MPU User Information Memory Segment Violation Select"]
    #[inline(always)]
    pub fn mpusegivs(&self) -> MPUSEGIVS_R {
        MPUSEGIVS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MPU Main Memory Segment 1 Read Enable"]
    #[inline(always)]
    pub fn mpuseg1re(&mut self) -> MPUSEG1RE_W {
        MPUSEG1RE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
MPU Main Memory Segment 1 Write Enable"]
    #[inline(always)]
    pub fn mpuseg1we(&mut self) -> MPUSEG1WE_W {
        MPUSEG1WE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
MPU Main Memory Segment 1 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg1xe(&mut self) -> MPUSEG1XE_W {
        MPUSEG1XE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
MPU Main Memory Segment 1 Violation Select"]
    #[inline(always)]
    pub fn mpuseg1vs(&mut self) -> MPUSEG1VS_W {
        MPUSEG1VS_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
MPU Main Memory Segment 2 Read Enable"]
    #[inline(always)]
    pub fn mpuseg2re(&mut self) -> MPUSEG2RE_W {
        MPUSEG2RE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
MPU Main Memory Segment 2 Write Enable"]
    #[inline(always)]
    pub fn mpuseg2we(&mut self) -> MPUSEG2WE_W {
        MPUSEG2WE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
MPU Main Memory Segment 2 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg2xe(&mut self) -> MPUSEG2XE_W {
        MPUSEG2XE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
MPU Main Memory Segment 2 Violation Select"]
    #[inline(always)]
    pub fn mpuseg2vs(&mut self) -> MPUSEG2VS_W {
        MPUSEG2VS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
MPU Main Memory Segment 3 Read Enable"]
    #[inline(always)]
    pub fn mpuseg3re(&mut self) -> MPUSEG3RE_W {
        MPUSEG3RE_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
MPU Main Memory Segment 3 Write Enable"]
    #[inline(always)]
    pub fn mpuseg3we(&mut self) -> MPUSEG3WE_W {
        MPUSEG3WE_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
MPU Main Memory Segment 3 Execute Enable"]
    #[inline(always)]
    pub fn mpuseg3xe(&mut self) -> MPUSEG3XE_W {
        MPUSEG3XE_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
MPU Main Memory Segment 3 Violation Select"]
    #[inline(always)]
    pub fn mpuseg3vs(&mut self) -> MPUSEG3VS_W {
        MPUSEG3VS_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
MPU User Information Memory Segment Read Enable"]
    #[inline(always)]
    pub fn mpusegire(&mut self) -> MPUSEGIRE_W {
        MPUSEGIRE_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
MPU User Information Memory Segment Write Enable."]
    #[inline(always)]
    pub fn mpusegiwe(&mut self) -> MPUSEGIWE_W {
        MPUSEGIWE_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
MPU User Information Memory Segment Execute Enable"]
    #[inline(always)]
    pub fn mpusegixe(&mut self) -> MPUSEGIXE_W {
        MPUSEGIXE_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
MPU User Information Memory Segment Violation Select"]
    #[inline(always)]
    pub fn mpusegivs(&mut self) -> MPUSEGIVS_W {
        MPUSEGIVS_W { w: self }
    }
}
