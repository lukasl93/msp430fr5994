#[doc = "Reader of register MPUIPC0"]
pub type R = crate::R<u16, super::MPUIPC0>;
#[doc = "Writer for register MPUIPC0"]
pub type W = crate::W<u16, super::MPUIPC0>;
#[doc = "Register MPUIPC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUIPC0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "5:5\\]
MPU IP Encapsulation segment Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUIPVS_A {
    #[doc = "0: Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    MPUIPVS_0 = 0,
    #[doc = "1: Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a PUC"]
    MPUIPVS_1 = 1,
}
impl From<MPUIPVS_A> for bool {
    #[inline(always)]
    fn from(variant: MPUIPVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUIPVS`"]
pub type MPUIPVS_R = crate::R<bool, MPUIPVS_A>;
impl MPUIPVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUIPVS_A {
        match self.bits {
            false => MPUIPVS_A::MPUIPVS_0,
            true => MPUIPVS_A::MPUIPVS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPUIPVS_0`"]
    #[inline(always)]
    pub fn is_mpuipvs_0(&self) -> bool {
        *self == MPUIPVS_A::MPUIPVS_0
    }
    #[doc = "Checks if the value of the field is `MPUIPVS_1`"]
    #[inline(always)]
    pub fn is_mpuipvs_1(&self) -> bool {
        *self == MPUIPVS_A::MPUIPVS_1
    }
}
#[doc = "Write proxy for field `MPUIPVS`"]
pub struct MPUIPVS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUIPVS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn mpuipvs_0(self) -> &'a mut W {
        self.variant(MPUIPVS_A::MPUIPVS_0)
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpuipvs_1(self) -> &'a mut W {
        self.variant(MPUIPVS_A::MPUIPVS_1)
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
MPU IP Encapsulation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUIPENA_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<MPUIPENA_A> for bool {
    #[inline(always)]
    fn from(variant: MPUIPENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUIPENA`"]
pub type MPUIPENA_R = crate::R<bool, MPUIPENA_A>;
impl MPUIPENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUIPENA_A {
        match self.bits {
            false => MPUIPENA_A::DISABLE,
            true => MPUIPENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MPUIPENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MPUIPENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `MPUIPENA`"]
pub struct MPUIPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUIPENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MPUIPENA_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MPUIPENA_A::ENABLE)
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
MPU IP Encapsulation Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUIPLOCK_A {
    #[doc = "0: Open"]
    OPEN = 0,
    #[doc = "1: Locked"]
    LOCK = 1,
}
impl From<MPUIPLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: MPUIPLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUIPLOCK`"]
pub type MPUIPLOCK_R = crate::R<bool, MPUIPLOCK_A>;
impl MPUIPLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUIPLOCK_A {
        match self.bits {
            false => MPUIPLOCK_A::OPEN,
            true => MPUIPLOCK_A::LOCK,
        }
    }
    #[doc = "Checks if the value of the field is `OPEN`"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == MPUIPLOCK_A::OPEN
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == MPUIPLOCK_A::LOCK
    }
}
#[doc = "Write proxy for field `MPUIPLOCK`"]
pub struct MPUIPLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUIPLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut W {
        self.variant(MPUIPLOCK_A::OPEN)
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(MPUIPLOCK_A::LOCK)
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
impl R {
    #[doc = "Bit 5 - 5:5\\]
MPU IP Encapsulation segment Violation Select"]
    #[inline(always)]
    pub fn mpuipvs(&self) -> MPUIPVS_R {
        MPUIPVS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
MPU IP Encapsulation Enable"]
    #[inline(always)]
    pub fn mpuipena(&self) -> MPUIPENA_R {
        MPUIPENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
MPU IP Encapsulation Lock"]
    #[inline(always)]
    pub fn mpuiplock(&self) -> MPUIPLOCK_R {
        MPUIPLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - 5:5\\]
MPU IP Encapsulation segment Violation Select"]
    #[inline(always)]
    pub fn mpuipvs(&mut self) -> MPUIPVS_W {
        MPUIPVS_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
MPU IP Encapsulation Enable"]
    #[inline(always)]
    pub fn mpuipena(&mut self) -> MPUIPENA_W {
        MPUIPENA_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
MPU IP Encapsulation Lock"]
    #[inline(always)]
    pub fn mpuiplock(&mut self) -> MPUIPLOCK_W {
        MPUIPLOCK_W { w: self }
    }
}
