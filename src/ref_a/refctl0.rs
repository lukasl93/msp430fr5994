#[doc = "Reader of register REFCTL0"]
pub type R = crate::R<u16, super::REFCTL0>;
#[doc = "Writer for register REFCTL0"]
pub type W = crate::W<u16, super::REFCTL0>;
#[doc = "Register REFCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REFCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
Reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFON_A {
    #[doc = "0: Disables reference if no other reference requests are pending"]
    REFON_0 = 0,
    #[doc = "1: Enables reference in static mode"]
    REFON_1 = 1,
}
impl From<REFON_A> for bool {
    #[inline(always)]
    fn from(variant: REFON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFON`"]
pub type REFON_R = crate::R<bool, REFON_A>;
impl REFON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFON_A {
        match self.bits {
            false => REFON_A::REFON_0,
            true => REFON_A::REFON_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFON_0`"]
    #[inline(always)]
    pub fn is_refon_0(&self) -> bool {
        *self == REFON_A::REFON_0
    }
    #[doc = "Checks if the value of the field is `REFON_1`"]
    #[inline(always)]
    pub fn is_refon_1(&self) -> bool {
        *self == REFON_A::REFON_1
    }
}
#[doc = "Write proxy for field `REFON`"]
pub struct REFON_W<'a> {
    w: &'a mut W,
}
impl<'a> REFON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables reference if no other reference requests are pending"]
    #[inline(always)]
    pub fn refon_0(self) -> &'a mut W {
        self.variant(REFON_A::REFON_0)
    }
    #[doc = "Enables reference in static mode"]
    #[inline(always)]
    pub fn refon_1(self) -> &'a mut W {
        self.variant(REFON_A::REFON_1)
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
Reference output buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFOUT_A {
    #[doc = "0: Reference output not available externally"]
    REFOUT_0 = 0,
    #[doc = "1: Reference output available externally. If ADC14REFBURST = 0, output is available continuously. If ADC14REFBURST = 1, output is available only during an ADC14 conversion."]
    REFOUT_1 = 1,
}
impl From<REFOUT_A> for bool {
    #[inline(always)]
    fn from(variant: REFOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFOUT`"]
pub type REFOUT_R = crate::R<bool, REFOUT_A>;
impl REFOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFOUT_A {
        match self.bits {
            false => REFOUT_A::REFOUT_0,
            true => REFOUT_A::REFOUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFOUT_0`"]
    #[inline(always)]
    pub fn is_refout_0(&self) -> bool {
        *self == REFOUT_A::REFOUT_0
    }
    #[doc = "Checks if the value of the field is `REFOUT_1`"]
    #[inline(always)]
    pub fn is_refout_1(&self) -> bool {
        *self == REFOUT_A::REFOUT_1
    }
}
#[doc = "Write proxy for field `REFOUT`"]
pub struct REFOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reference output not available externally"]
    #[inline(always)]
    pub fn refout_0(self) -> &'a mut W {
        self.variant(REFOUT_A::REFOUT_0)
    }
    #[doc = "Reference output available externally. If ADC14REFBURST = 0, output is available continuously. If ADC14REFBURST = 1, output is available only during an ADC14 conversion."]
    #[inline(always)]
    pub fn refout_1(self) -> &'a mut W {
        self.variant(REFOUT_A::REFOUT_1)
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
#[doc = "3:3\\]
Temperature sensor disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFTCOFF_A {
    #[doc = "0: Temperature sensor enabled"]
    REFTCOFF_0 = 0,
    #[doc = "1: Temperature sensor disabled to save power"]
    REFTCOFF_1 = 1,
}
impl From<REFTCOFF_A> for bool {
    #[inline(always)]
    fn from(variant: REFTCOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFTCOFF`"]
pub type REFTCOFF_R = crate::R<bool, REFTCOFF_A>;
impl REFTCOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFTCOFF_A {
        match self.bits {
            false => REFTCOFF_A::REFTCOFF_0,
            true => REFTCOFF_A::REFTCOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFTCOFF_0`"]
    #[inline(always)]
    pub fn is_reftcoff_0(&self) -> bool {
        *self == REFTCOFF_A::REFTCOFF_0
    }
    #[doc = "Checks if the value of the field is `REFTCOFF_1`"]
    #[inline(always)]
    pub fn is_reftcoff_1(&self) -> bool {
        *self == REFTCOFF_A::REFTCOFF_1
    }
}
#[doc = "Write proxy for field `REFTCOFF`"]
pub struct REFTCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> REFTCOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFTCOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn reftcoff_0(self) -> &'a mut W {
        self.variant(REFTCOFF_A::REFTCOFF_0)
    }
    #[doc = "Temperature sensor disabled to save power"]
    #[inline(always)]
    pub fn reftcoff_1(self) -> &'a mut W {
        self.variant(REFTCOFF_A::REFTCOFF_1)
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
#[doc = "5:4\\]
Reference voltage level select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFVSEL_A {
    #[doc = "0: 1.2 V available when reference requested or REFON = 1"]
    REFVSEL_0 = 0,
    #[doc = "1: 2.0 V available when reference requested or REFON = 1"]
    REFVSEL_1 = 1,
    #[doc = "2: 2.5 V available when reference requested or REFON = 1"]
    REFVSEL_2 = 2,
    #[doc = "3: 2.5 V available when reference requested or REFON = 1"]
    REFVSEL_3 = 3,
}
impl From<REFVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFVSEL`"]
pub type REFVSEL_R = crate::R<u8, REFVSEL_A>;
impl REFVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFVSEL_A {
        match self.bits {
            0 => REFVSEL_A::REFVSEL_0,
            1 => REFVSEL_A::REFVSEL_1,
            2 => REFVSEL_A::REFVSEL_2,
            3 => REFVSEL_A::REFVSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFVSEL_0`"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_0
    }
    #[doc = "Checks if the value of the field is `REFVSEL_1`"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_1
    }
    #[doc = "Checks if the value of the field is `REFVSEL_2`"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_2
    }
    #[doc = "Checks if the value of the field is `REFVSEL_3`"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_3
    }
}
#[doc = "Write proxy for field `REFVSEL`"]
pub struct REFVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFVSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1.2 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_0)
    }
    #[doc = "2.0 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_1)
    }
    #[doc = "2.5 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_2(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_2)
    }
    #[doc = "2.5 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "6:6\\]
Reference generator one-time trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFGENOT_A {
    #[doc = "0: No trigger"]
    REFGENOT_0 = 0,
    #[doc = "1: Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    REFGENOT_1 = 1,
}
impl From<REFGENOT_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFGENOT`"]
pub type REFGENOT_R = crate::R<bool, REFGENOT_A>;
impl REFGENOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENOT_A {
        match self.bits {
            false => REFGENOT_A::REFGENOT_0,
            true => REFGENOT_A::REFGENOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENOT_0`"]
    #[inline(always)]
    pub fn is_refgenot_0(&self) -> bool {
        *self == REFGENOT_A::REFGENOT_0
    }
    #[doc = "Checks if the value of the field is `REFGENOT_1`"]
    #[inline(always)]
    pub fn is_refgenot_1(&self) -> bool {
        *self == REFGENOT_A::REFGENOT_1
    }
}
#[doc = "Write proxy for field `REFGENOT`"]
pub struct REFGENOT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGENOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFGENOT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refgenot_0(self) -> &'a mut W {
        self.variant(REFGENOT_A::REFGENOT_0)
    }
    #[doc = "Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refgenot_1(self) -> &'a mut W {
        self.variant(REFGENOT_A::REFGENOT_1)
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
Bandgap and bandgap buffer one-time trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFBGOT_A {
    #[doc = "0: No trigger"]
    REFBGOT_0 = 0,
    #[doc = "1: Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    REFBGOT_1 = 1,
}
impl From<REFBGOT_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFBGOT`"]
pub type REFBGOT_R = crate::R<bool, REFBGOT_A>;
impl REFBGOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBGOT_A {
        match self.bits {
            false => REFBGOT_A::REFBGOT_0,
            true => REFBGOT_A::REFBGOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFBGOT_0`"]
    #[inline(always)]
    pub fn is_refbgot_0(&self) -> bool {
        *self == REFBGOT_A::REFBGOT_0
    }
    #[doc = "Checks if the value of the field is `REFBGOT_1`"]
    #[inline(always)]
    pub fn is_refbgot_1(&self) -> bool {
        *self == REFBGOT_A::REFBGOT_1
    }
}
#[doc = "Write proxy for field `REFBGOT`"]
pub struct REFBGOT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBGOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFBGOT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refbgot_0(self) -> &'a mut W {
        self.variant(REFBGOT_A::REFBGOT_0)
    }
    #[doc = "Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refbgot_1(self) -> &'a mut W {
        self.variant(REFBGOT_A::REFBGOT_1)
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
Reference generator active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFGENACT_A {
    #[doc = "0: Reference generator not active"]
    REFGENACT_0 = 0,
    #[doc = "1: Reference generator active"]
    REFGENACT_1 = 1,
}
impl From<REFGENACT_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFGENACT`"]
pub type REFGENACT_R = crate::R<bool, REFGENACT_A>;
impl REFGENACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENACT_A {
        match self.bits {
            false => REFGENACT_A::REFGENACT_0,
            true => REFGENACT_A::REFGENACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENACT_0`"]
    #[inline(always)]
    pub fn is_refgenact_0(&self) -> bool {
        *self == REFGENACT_A::REFGENACT_0
    }
    #[doc = "Checks if the value of the field is `REFGENACT_1`"]
    #[inline(always)]
    pub fn is_refgenact_1(&self) -> bool {
        *self == REFGENACT_A::REFGENACT_1
    }
}
#[doc = "Write proxy for field `REFGENACT`"]
pub struct REFGENACT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGENACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFGENACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reference generator not active"]
    #[inline(always)]
    pub fn refgenact_0(self) -> &'a mut W {
        self.variant(REFGENACT_A::REFGENACT_0)
    }
    #[doc = "Reference generator active"]
    #[inline(always)]
    pub fn refgenact_1(self) -> &'a mut W {
        self.variant(REFGENACT_A::REFGENACT_1)
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
Reference bandgap active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFBGACT_A {
    #[doc = "0: Reference bandgap buffer not active"]
    REFBGACT_0 = 0,
    #[doc = "1: Reference bandgap buffer active"]
    REFBGACT_1 = 1,
}
impl From<REFBGACT_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFBGACT`"]
pub type REFBGACT_R = crate::R<bool, REFBGACT_A>;
impl REFBGACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBGACT_A {
        match self.bits {
            false => REFBGACT_A::REFBGACT_0,
            true => REFBGACT_A::REFBGACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFBGACT_0`"]
    #[inline(always)]
    pub fn is_refbgact_0(&self) -> bool {
        *self == REFBGACT_A::REFBGACT_0
    }
    #[doc = "Checks if the value of the field is `REFBGACT_1`"]
    #[inline(always)]
    pub fn is_refbgact_1(&self) -> bool {
        *self == REFBGACT_A::REFBGACT_1
    }
}
#[doc = "Write proxy for field `REFBGACT`"]
pub struct REFBGACT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBGACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFBGACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reference bandgap buffer not active"]
    #[inline(always)]
    pub fn refbgact_0(self) -> &'a mut W {
        self.variant(REFBGACT_A::REFBGACT_0)
    }
    #[doc = "Reference bandgap buffer active"]
    #[inline(always)]
    pub fn refbgact_1(self) -> &'a mut W {
        self.variant(REFBGACT_A::REFBGACT_1)
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
Reference generator busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFGENBUSY_A {
    #[doc = "0: Reference generator not busy"]
    REFGENBUSY_0 = 0,
    #[doc = "1: Reference generator busy"]
    REFGENBUSY_1 = 1,
}
impl From<REFGENBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFGENBUSY`"]
pub type REFGENBUSY_R = crate::R<bool, REFGENBUSY_A>;
impl REFGENBUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENBUSY_A {
        match self.bits {
            false => REFGENBUSY_A::REFGENBUSY_0,
            true => REFGENBUSY_A::REFGENBUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENBUSY_0`"]
    #[inline(always)]
    pub fn is_refgenbusy_0(&self) -> bool {
        *self == REFGENBUSY_A::REFGENBUSY_0
    }
    #[doc = "Checks if the value of the field is `REFGENBUSY_1`"]
    #[inline(always)]
    pub fn is_refgenbusy_1(&self) -> bool {
        *self == REFGENBUSY_A::REFGENBUSY_1
    }
}
#[doc = "Write proxy for field `REFGENBUSY`"]
pub struct REFGENBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGENBUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFGENBUSY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reference generator not busy"]
    #[inline(always)]
    pub fn refgenbusy_0(self) -> &'a mut W {
        self.variant(REFGENBUSY_A::REFGENBUSY_0)
    }
    #[doc = "Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy_1(self) -> &'a mut W {
        self.variant(REFGENBUSY_A::REFGENBUSY_1)
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
Bandgap mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGMODE_A {
    #[doc = "0: Static mode"]
    BGMODE_0 = 0,
    #[doc = "1: Sampled mode"]
    BGMODE_1 = 1,
}
impl From<BGMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BGMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BGMODE`"]
pub type BGMODE_R = crate::R<bool, BGMODE_A>;
impl BGMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGMODE_A {
        match self.bits {
            false => BGMODE_A::BGMODE_0,
            true => BGMODE_A::BGMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGMODE_0`"]
    #[inline(always)]
    pub fn is_bgmode_0(&self) -> bool {
        *self == BGMODE_A::BGMODE_0
    }
    #[doc = "Checks if the value of the field is `BGMODE_1`"]
    #[inline(always)]
    pub fn is_bgmode_1(&self) -> bool {
        *self == BGMODE_A::BGMODE_1
    }
}
#[doc = "Write proxy for field `BGMODE`"]
pub struct BGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn bgmode_0(self) -> &'a mut W {
        self.variant(BGMODE_A::BGMODE_0)
    }
    #[doc = "Sampled mode"]
    #[inline(always)]
    pub fn bgmode_1(self) -> &'a mut W {
        self.variant(BGMODE_A::BGMODE_1)
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
Variable reference voltage ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFGENRDY_A {
    #[doc = "0: Reference voltage output is not ready to be used"]
    REFGENRDY_0 = 0,
    #[doc = "1: Reference voltage output is ready to be used"]
    REFGENRDY_1 = 1,
}
impl From<REFGENRDY_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFGENRDY`"]
pub type REFGENRDY_R = crate::R<bool, REFGENRDY_A>;
impl REFGENRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENRDY_A {
        match self.bits {
            false => REFGENRDY_A::REFGENRDY_0,
            true => REFGENRDY_A::REFGENRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENRDY_0`"]
    #[inline(always)]
    pub fn is_refgenrdy_0(&self) -> bool {
        *self == REFGENRDY_A::REFGENRDY_0
    }
    #[doc = "Checks if the value of the field is `REFGENRDY_1`"]
    #[inline(always)]
    pub fn is_refgenrdy_1(&self) -> bool {
        *self == REFGENRDY_A::REFGENRDY_1
    }
}
#[doc = "Write proxy for field `REFGENRDY`"]
pub struct REFGENRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> REFGENRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFGENRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reference voltage output is not ready to be used"]
    #[inline(always)]
    pub fn refgenrdy_0(self) -> &'a mut W {
        self.variant(REFGENRDY_A::REFGENRDY_0)
    }
    #[doc = "Reference voltage output is ready to be used"]
    #[inline(always)]
    pub fn refgenrdy_1(self) -> &'a mut W {
        self.variant(REFGENRDY_A::REFGENRDY_1)
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
Buffered bandgap voltage ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFBGRDY_A {
    #[doc = "0: Buffered bandgap voltage is not ready to be used"]
    REFBGRDY_0 = 0,
    #[doc = "1: Buffered bandgap voltage is ready to be used"]
    REFBGRDY_1 = 1,
}
impl From<REFBGRDY_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFBGRDY`"]
pub type REFBGRDY_R = crate::R<bool, REFBGRDY_A>;
impl REFBGRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBGRDY_A {
        match self.bits {
            false => REFBGRDY_A::REFBGRDY_0,
            true => REFBGRDY_A::REFBGRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFBGRDY_0`"]
    #[inline(always)]
    pub fn is_refbgrdy_0(&self) -> bool {
        *self == REFBGRDY_A::REFBGRDY_0
    }
    #[doc = "Checks if the value of the field is `REFBGRDY_1`"]
    #[inline(always)]
    pub fn is_refbgrdy_1(&self) -> bool {
        *self == REFBGRDY_A::REFBGRDY_1
    }
}
#[doc = "Write proxy for field `REFBGRDY`"]
pub struct REFBGRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBGRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFBGRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Buffered bandgap voltage is not ready to be used"]
    #[inline(always)]
    pub fn refbgrdy_0(self) -> &'a mut W {
        self.variant(REFBGRDY_A::REFBGRDY_0)
    }
    #[doc = "Buffered bandgap voltage is ready to be used"]
    #[inline(always)]
    pub fn refbgrdy_1(self) -> &'a mut W {
        self.variant(REFBGRDY_A::REFBGRDY_1)
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
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reference enable"]
    #[inline(always)]
    pub fn refon(&self) -> REFON_R {
        REFON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reference output buffer"]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Temperature sensor disabled"]
    #[inline(always)]
    pub fn reftcoff(&self) -> REFTCOFF_R {
        REFTCOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Reference voltage level select"]
    #[inline(always)]
    pub fn refvsel(&self) -> REFVSEL_R {
        REFVSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Reference generator one-time trigger"]
    #[inline(always)]
    pub fn refgenot(&self) -> REFGENOT_R {
        REFGENOT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Bandgap and bandgap buffer one-time trigger"]
    #[inline(always)]
    pub fn refbgot(&self) -> REFBGOT_R {
        REFBGOT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy(&self) -> REFGENBUSY_R {
        REFGENBUSY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Variable reference voltage ready status"]
    #[inline(always)]
    pub fn refgenrdy(&self) -> REFGENRDY_R {
        REFGENRDY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Buffered bandgap voltage ready status"]
    #[inline(always)]
    pub fn refbgrdy(&self) -> REFBGRDY_R {
        REFBGRDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reference enable"]
    #[inline(always)]
    pub fn refon(&mut self) -> REFON_W {
        REFON_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Reference output buffer"]
    #[inline(always)]
    pub fn refout(&mut self) -> REFOUT_W {
        REFOUT_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Temperature sensor disabled"]
    #[inline(always)]
    pub fn reftcoff(&mut self) -> REFTCOFF_W {
        REFTCOFF_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Reference voltage level select"]
    #[inline(always)]
    pub fn refvsel(&mut self) -> REFVSEL_W {
        REFVSEL_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Reference generator one-time trigger"]
    #[inline(always)]
    pub fn refgenot(&mut self) -> REFGENOT_W {
        REFGENOT_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Bandgap and bandgap buffer one-time trigger"]
    #[inline(always)]
    pub fn refbgot(&mut self) -> REFBGOT_W {
        REFBGOT_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&mut self) -> REFGENACT_W {
        REFGENACT_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&mut self) -> REFBGACT_W {
        REFBGACT_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy(&mut self) -> REFGENBUSY_W {
        REFGENBUSY_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BGMODE_W {
        BGMODE_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Variable reference voltage ready status"]
    #[inline(always)]
    pub fn refgenrdy(&mut self) -> REFGENRDY_W {
        REFGENRDY_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Buffered bandgap voltage ready status"]
    #[inline(always)]
    pub fn refbgrdy(&mut self) -> REFBGRDY_W {
        REFBGRDY_W { w: self }
    }
}
