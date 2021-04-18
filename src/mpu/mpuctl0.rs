#[doc = "Reader of register MPUCTL0"]
pub type R = crate::R<u16, super::MPUCTL0>;
#[doc = "Writer for register MPUCTL0"]
pub type W = crate::W<u16, super::MPUCTL0>;
#[doc = "Register MPUCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
MPU Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUENA_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<MPUENA_A> for bool {
    #[inline(always)]
    fn from(variant: MPUENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUENA`"]
pub type MPUENA_R = crate::R<bool, MPUENA_A>;
impl MPUENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUENA_A {
        match self.bits {
            false => MPUENA_A::DISABLE,
            true => MPUENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUENA`"]
pub struct MPUENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUENA_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUENA_A::ENABLE)
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
MPU Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPULOCK_A {
    #[doc = "0: Open"]
    OPEN = 0,
    #[doc = "1: Locked"]
    LOCK = 1,
}
impl From<MPULOCK_A> for bool {
    #[inline(always)]
    fn from(variant: MPULOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPULOCK`"]
pub type MPULOCK_R = crate::R<bool, MPULOCK_A>;
impl MPULOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPULOCK_A {
        match self.bits {
            false => MPULOCK_A::OPEN,
            true => MPULOCK_A::LOCK,
        }
    }
    #[doc = "Checks if the value of the field is `OPEN`"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == MPULOCK_A::OPEN
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == MPULOCK_A::LOCK
    }
}
#[doc = "Write proxy for field `MPULOCK`"]
pub struct MPULOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> MPULOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPULOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut W {
        self.variant(MPULOCK_A::OPEN)
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(MPULOCK_A::LOCK)
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
#[doc = "4:4\\]
Enable NMI Event if a Segment violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSEGIE_A {
    #[doc = "0: Segment violation interrupt disabled"]
    DISABLE = 0,
    #[doc = "1: Segment violation interrupt enabled"]
    ENABLE = 1,
}
impl From<MPUSEGIE_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSEGIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSEGIE`"]
pub type MPUSEGIE_R = crate::R<bool, MPUSEGIE_A>;
impl MPUSEGIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSEGIE_A {
        match self.bits {
            false => MPUSEGIE_A::DISABLE,
            true => MPUSEGIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUSEGIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUSEGIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUSEGIE`"]
pub struct MPUSEGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSEGIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Segment violation interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUSEGIE_A::DISABLE)
    }
    #[doc = "Segment violation interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUSEGIE_A::ENABLE)
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
#[doc = "Reader of field `MPUPW`"]
pub type MPUPW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MPUPW`"]
pub struct MPUPW_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
MPU Enable"]
    #[inline(always)]
    pub fn mpuena(&self) -> MPUENA_R {
        MPUENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MPU Lock"]
    #[inline(always)]
    pub fn mpulock(&self) -> MPULOCK_R {
        MPULOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable NMI Event if a Segment violation"]
    #[inline(always)]
    pub fn mpusegie(&self) -> MPUSEGIE_R {
        MPUSEGIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
MPU Password"]
    #[inline(always)]
    pub fn mpupw(&self) -> MPUPW_R {
        MPUPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MPU Enable"]
    #[inline(always)]
    pub fn mpuena(&mut self) -> MPUENA_W {
        MPUENA_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
MPU Lock"]
    #[inline(always)]
    pub fn mpulock(&mut self) -> MPULOCK_W {
        MPULOCK_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Enable NMI Event if a Segment violation"]
    #[inline(always)]
    pub fn mpusegie(&mut self) -> MPUSEGIE_W {
        MPUSEGIE_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
MPU Password"]
    #[inline(always)]
    pub fn mpupw(&mut self) -> MPUPW_W {
        MPUPW_W { w: self }
    }
}
