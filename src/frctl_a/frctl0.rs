#[doc = "Reader of register FRCTL0"]
pub type R = crate::R<u16, super::FRCTL0>;
#[doc = "Writer for register FRCTL0"]
pub type W = crate::W<u16, super::FRCTL0>;
#[doc = "Register FRCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FRCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "7:4\\]
Wait state numbers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NWAITS_A {
    #[doc = "0: FRAM wait states: 0"]
    NWAITS_0 = 0,
    #[doc = "1: FRAM wait states: 1"]
    NWAITS_1 = 1,
    #[doc = "2: FRAM wait states: 2"]
    NWAITS_2 = 2,
    #[doc = "3: FRAM wait states: 3"]
    NWAITS_3 = 3,
    #[doc = "4: FRAM wait states: 4"]
    NWAITS_4 = 4,
    #[doc = "5: FRAM wait states: 5"]
    NWAITS_5 = 5,
    #[doc = "6: FRAM wait states: 6"]
    NWAITS_6 = 6,
    #[doc = "7: FRAM wait states: 7"]
    NWAITS_7 = 7,
    #[doc = "8: FRAM wait states: 8"]
    NWAITS_8 = 8,
    #[doc = "9: FRAM wait states: 9"]
    NWAITS_9 = 9,
    #[doc = "10: FRAM wait states: 10"]
    NWAITS_10 = 10,
    #[doc = "11: FRAM wait states: 11"]
    NWAITS_11 = 11,
    #[doc = "12: FRAM wait states: 12"]
    NWAITS_12 = 12,
    #[doc = "13: FRAM wait states: 13"]
    NWAITS_13 = 13,
    #[doc = "14: FRAM wait states: 14"]
    NWAITS_14 = 14,
    #[doc = "15: FRAM wait states: 15"]
    NWAITS_15 = 15,
}
impl From<NWAITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NWAITS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NWAITS`"]
pub type NWAITS_R = crate::R<u8, NWAITS_A>;
impl NWAITS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NWAITS_A {
        match self.bits {
            0 => NWAITS_A::NWAITS_0,
            1 => NWAITS_A::NWAITS_1,
            2 => NWAITS_A::NWAITS_2,
            3 => NWAITS_A::NWAITS_3,
            4 => NWAITS_A::NWAITS_4,
            5 => NWAITS_A::NWAITS_5,
            6 => NWAITS_A::NWAITS_6,
            7 => NWAITS_A::NWAITS_7,
            8 => NWAITS_A::NWAITS_8,
            9 => NWAITS_A::NWAITS_9,
            10 => NWAITS_A::NWAITS_10,
            11 => NWAITS_A::NWAITS_11,
            12 => NWAITS_A::NWAITS_12,
            13 => NWAITS_A::NWAITS_13,
            14 => NWAITS_A::NWAITS_14,
            15 => NWAITS_A::NWAITS_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NWAITS_0`"]
    #[inline(always)]
    pub fn is_nwaits_0(&self) -> bool {
        *self == NWAITS_A::NWAITS_0
    }
    #[doc = "Checks if the value of the field is `NWAITS_1`"]
    #[inline(always)]
    pub fn is_nwaits_1(&self) -> bool {
        *self == NWAITS_A::NWAITS_1
    }
    #[doc = "Checks if the value of the field is `NWAITS_2`"]
    #[inline(always)]
    pub fn is_nwaits_2(&self) -> bool {
        *self == NWAITS_A::NWAITS_2
    }
    #[doc = "Checks if the value of the field is `NWAITS_3`"]
    #[inline(always)]
    pub fn is_nwaits_3(&self) -> bool {
        *self == NWAITS_A::NWAITS_3
    }
    #[doc = "Checks if the value of the field is `NWAITS_4`"]
    #[inline(always)]
    pub fn is_nwaits_4(&self) -> bool {
        *self == NWAITS_A::NWAITS_4
    }
    #[doc = "Checks if the value of the field is `NWAITS_5`"]
    #[inline(always)]
    pub fn is_nwaits_5(&self) -> bool {
        *self == NWAITS_A::NWAITS_5
    }
    #[doc = "Checks if the value of the field is `NWAITS_6`"]
    #[inline(always)]
    pub fn is_nwaits_6(&self) -> bool {
        *self == NWAITS_A::NWAITS_6
    }
    #[doc = "Checks if the value of the field is `NWAITS_7`"]
    #[inline(always)]
    pub fn is_nwaits_7(&self) -> bool {
        *self == NWAITS_A::NWAITS_7
    }
    #[doc = "Checks if the value of the field is `NWAITS_8`"]
    #[inline(always)]
    pub fn is_nwaits_8(&self) -> bool {
        *self == NWAITS_A::NWAITS_8
    }
    #[doc = "Checks if the value of the field is `NWAITS_9`"]
    #[inline(always)]
    pub fn is_nwaits_9(&self) -> bool {
        *self == NWAITS_A::NWAITS_9
    }
    #[doc = "Checks if the value of the field is `NWAITS_10`"]
    #[inline(always)]
    pub fn is_nwaits_10(&self) -> bool {
        *self == NWAITS_A::NWAITS_10
    }
    #[doc = "Checks if the value of the field is `NWAITS_11`"]
    #[inline(always)]
    pub fn is_nwaits_11(&self) -> bool {
        *self == NWAITS_A::NWAITS_11
    }
    #[doc = "Checks if the value of the field is `NWAITS_12`"]
    #[inline(always)]
    pub fn is_nwaits_12(&self) -> bool {
        *self == NWAITS_A::NWAITS_12
    }
    #[doc = "Checks if the value of the field is `NWAITS_13`"]
    #[inline(always)]
    pub fn is_nwaits_13(&self) -> bool {
        *self == NWAITS_A::NWAITS_13
    }
    #[doc = "Checks if the value of the field is `NWAITS_14`"]
    #[inline(always)]
    pub fn is_nwaits_14(&self) -> bool {
        *self == NWAITS_A::NWAITS_14
    }
    #[doc = "Checks if the value of the field is `NWAITS_15`"]
    #[inline(always)]
    pub fn is_nwaits_15(&self) -> bool {
        *self == NWAITS_A::NWAITS_15
    }
}
#[doc = "Write proxy for field `NWAITS`"]
pub struct NWAITS_W<'a> {
    w: &'a mut W,
}
impl<'a> NWAITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NWAITS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FRAM wait states: 0"]
    #[inline(always)]
    pub fn nwaits_0(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_0)
    }
    #[doc = "FRAM wait states: 1"]
    #[inline(always)]
    pub fn nwaits_1(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_1)
    }
    #[doc = "FRAM wait states: 2"]
    #[inline(always)]
    pub fn nwaits_2(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_2)
    }
    #[doc = "FRAM wait states: 3"]
    #[inline(always)]
    pub fn nwaits_3(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_3)
    }
    #[doc = "FRAM wait states: 4"]
    #[inline(always)]
    pub fn nwaits_4(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_4)
    }
    #[doc = "FRAM wait states: 5"]
    #[inline(always)]
    pub fn nwaits_5(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_5)
    }
    #[doc = "FRAM wait states: 6"]
    #[inline(always)]
    pub fn nwaits_6(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_6)
    }
    #[doc = "FRAM wait states: 7"]
    #[inline(always)]
    pub fn nwaits_7(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_7)
    }
    #[doc = "FRAM wait states: 8"]
    #[inline(always)]
    pub fn nwaits_8(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_8)
    }
    #[doc = "FRAM wait states: 9"]
    #[inline(always)]
    pub fn nwaits_9(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_9)
    }
    #[doc = "FRAM wait states: 10"]
    #[inline(always)]
    pub fn nwaits_10(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_10)
    }
    #[doc = "FRAM wait states: 11"]
    #[inline(always)]
    pub fn nwaits_11(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_11)
    }
    #[doc = "FRAM wait states: 12"]
    #[inline(always)]
    pub fn nwaits_12(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_12)
    }
    #[doc = "FRAM wait states: 13"]
    #[inline(always)]
    pub fn nwaits_13(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_13)
    }
    #[doc = "FRAM wait states: 14"]
    #[inline(always)]
    pub fn nwaits_14(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_14)
    }
    #[doc = "FRAM wait states: 15"]
    #[inline(always)]
    pub fn nwaits_15(self) -> &'a mut W {
        self.variant(NWAITS_A::NWAITS_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `FRCTLPW`"]
pub type FRCTLPW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRCTLPW`"]
pub struct FRCTLPW_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCTLPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
#[doc = "3:3\\]
Enable automatic Wait State Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_A {
    #[doc = "0: User Wait State Mode. The NWAITS\\[3:0\\]
is used for the FRAM wait state."]
    AUTO_0 = 0,
    #[doc = "1: Auto mode. The NWAITS\\[3:0\\]
is ignored. Wait states are generated automatically by the internal FRAM controller state machine."]
    AUTO_1 = 1,
}
impl From<AUTO_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTO`"]
pub type AUTO_R = crate::R<bool, AUTO_A>;
impl AUTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_A {
        match self.bits {
            false => AUTO_A::AUTO_0,
            true => AUTO_A::AUTO_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_0`"]
    #[inline(always)]
    pub fn is_auto_0(&self) -> bool {
        *self == AUTO_A::AUTO_0
    }
    #[doc = "Checks if the value of the field is `AUTO_1`"]
    #[inline(always)]
    pub fn is_auto_1(&self) -> bool {
        *self == AUTO_A::AUTO_1
    }
}
#[doc = "Write proxy for field `AUTO`"]
pub struct AUTO_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "User Wait State Mode. The NWAITS\\[3:0\\]
is used for the FRAM wait state."]
    #[inline(always)]
    pub fn auto_0(self) -> &'a mut W {
        self.variant(AUTO_A::AUTO_0)
    }
    #[doc = "Auto mode. The NWAITS\\[3:0\\]
is ignored. Wait states are generated automatically by the internal FRAM controller state machine."]
    #[inline(always)]
    pub fn auto_1(self) -> &'a mut W {
        self.variant(AUTO_A::AUTO_1)
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
#[doc = "0:0\\]
Write Protection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPROT_A {
    #[doc = "0: Disable Write Protection. Write to FRAM memory is allowed."]
    WPROT_0 = 0,
    #[doc = "1: Enable Write Protection. Write to FRAM memory is not allowed. In case a write access is attempted, the WPIFG (Write Protection Flag) bit will be set."]
    WPROT_1 = 1,
}
impl From<WPROT_A> for bool {
    #[inline(always)]
    fn from(variant: WPROT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WPROT`"]
pub type WPROT_R = crate::R<bool, WPROT_A>;
impl WPROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPROT_A {
        match self.bits {
            false => WPROT_A::WPROT_0,
            true => WPROT_A::WPROT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPROT_0`"]
    #[inline(always)]
    pub fn is_wprot_0(&self) -> bool {
        *self == WPROT_A::WPROT_0
    }
    #[doc = "Checks if the value of the field is `WPROT_1`"]
    #[inline(always)]
    pub fn is_wprot_1(&self) -> bool {
        *self == WPROT_A::WPROT_1
    }
}
#[doc = "Write proxy for field `WPROT`"]
pub struct WPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> WPROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPROT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Write Protection. Write to FRAM memory is allowed."]
    #[inline(always)]
    pub fn wprot_0(self) -> &'a mut W {
        self.variant(WPROT_A::WPROT_0)
    }
    #[doc = "Enable Write Protection. Write to FRAM memory is not allowed. In case a write access is attempted, the WPIFG (Write Protection Flag) bit will be set."]
    #[inline(always)]
    pub fn wprot_1(self) -> &'a mut W {
        self.variant(WPROT_A::WPROT_1)
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
impl R {
    #[doc = "Bits 4:7 - 7:4\\]
Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&self) -> NWAITS_R {
        NWAITS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&self) -> FRCTLPW_R {
        FRCTLPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable automatic Wait State Mode"]
    #[inline(always)]
    pub fn auto(&self) -> AUTO_R {
        AUTO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Write Protection Enable"]
    #[inline(always)]
    pub fn wprot(&self) -> WPROT_R {
        WPROT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - 7:4\\]
Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&mut self) -> NWAITS_W {
        NWAITS_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&mut self) -> FRCTLPW_W {
        FRCTLPW_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Enable automatic Wait State Mode"]
    #[inline(always)]
    pub fn auto(&mut self) -> AUTO_W {
        AUTO_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Write Protection Enable"]
    #[inline(always)]
    pub fn wprot(&mut self) -> WPROT_W {
        WPROT_W { w: self }
    }
}
