#[doc = "Reader of register CSCTL4"]
pub type R = crate::R<u16, super::CSCTL4>;
#[doc = "Writer for register CSCTL4"]
pub type W = crate::W<u16, super::CSCTL4>;
#[doc = "Register CSCTL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
LFXT off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXTOFF_A {
    #[doc = "0: LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation"]
    LFXTOFF_0 = 0,
    #[doc = "1: LFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    LFXTOFF_1 = 1,
}
impl From<LFXTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXTOFF`"]
pub type LFXTOFF_R = crate::R<bool, LFXTOFF_A>;
impl LFXTOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTOFF_A {
        match self.bits {
            false => LFXTOFF_A::LFXTOFF_0,
            true => LFXTOFF_A::LFXTOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTOFF_0`"]
    #[inline(always)]
    pub fn is_lfxtoff_0(&self) -> bool {
        *self == LFXTOFF_A::LFXTOFF_0
    }
    #[doc = "Checks if the value of the field is `LFXTOFF_1`"]
    #[inline(always)]
    pub fn is_lfxtoff_1(&self) -> bool {
        *self == LFXTOFF_A::LFXTOFF_1
    }
}
#[doc = "Write proxy for field `LFXTOFF`"]
pub struct LFXTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation"]
    #[inline(always)]
    pub fn lfxtoff_0(self) -> &'a mut W {
        self.variant(LFXTOFF_A::LFXTOFF_0)
    }
    #[doc = "LFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    #[inline(always)]
    pub fn lfxtoff_1(self) -> &'a mut W {
        self.variant(LFXTOFF_A::LFXTOFF_1)
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
SMCLK off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMCLKOFF_A {
    #[doc = "0: SMCLK on"]
    SMCLKOFF_0 = 0,
    #[doc = "1: SMCLK off"]
    SMCLKOFF_1 = 1,
}
impl From<SMCLKOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SMCLKOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMCLKOFF`"]
pub type SMCLKOFF_R = crate::R<bool, SMCLKOFF_A>;
impl SMCLKOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMCLKOFF_A {
        match self.bits {
            false => SMCLKOFF_A::SMCLKOFF_0,
            true => SMCLKOFF_A::SMCLKOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMCLKOFF_0`"]
    #[inline(always)]
    pub fn is_smclkoff_0(&self) -> bool {
        *self == SMCLKOFF_A::SMCLKOFF_0
    }
    #[doc = "Checks if the value of the field is `SMCLKOFF_1`"]
    #[inline(always)]
    pub fn is_smclkoff_1(&self) -> bool {
        *self == SMCLKOFF_A::SMCLKOFF_1
    }
}
#[doc = "Write proxy for field `SMCLKOFF`"]
pub struct SMCLKOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLKOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMCLKOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SMCLK on"]
    #[inline(always)]
    pub fn smclkoff_0(self) -> &'a mut W {
        self.variant(SMCLKOFF_A::SMCLKOFF_0)
    }
    #[doc = "SMCLK off"]
    #[inline(always)]
    pub fn smclkoff_1(self) -> &'a mut W {
        self.variant(SMCLKOFF_A::SMCLKOFF_1)
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
VLO off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLOOFF_A {
    #[doc = "0: VLO is on"]
    VLOOFF_0 = 0,
    #[doc = "1: VLO is off if it is not used as a source for ACLK, MCLK, or SMCLK or if not used as a source for the RTC in LPM3.5"]
    VLOOFF_1 = 1,
}
impl From<VLOOFF_A> for bool {
    #[inline(always)]
    fn from(variant: VLOOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VLOOFF`"]
pub type VLOOFF_R = crate::R<bool, VLOOFF_A>;
impl VLOOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLOOFF_A {
        match self.bits {
            false => VLOOFF_A::VLOOFF_0,
            true => VLOOFF_A::VLOOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLOOFF_0`"]
    #[inline(always)]
    pub fn is_vlooff_0(&self) -> bool {
        *self == VLOOFF_A::VLOOFF_0
    }
    #[doc = "Checks if the value of the field is `VLOOFF_1`"]
    #[inline(always)]
    pub fn is_vlooff_1(&self) -> bool {
        *self == VLOOFF_A::VLOOFF_1
    }
}
#[doc = "Write proxy for field `VLOOFF`"]
pub struct VLOOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VLOOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLOOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VLO is on"]
    #[inline(always)]
    pub fn vlooff_0(self) -> &'a mut W {
        self.variant(VLOOFF_A::VLOOFF_0)
    }
    #[doc = "VLO is off if it is not used as a source for ACLK, MCLK, or SMCLK or if not used as a source for the RTC in LPM3.5"]
    #[inline(always)]
    pub fn vlooff_1(self) -> &'a mut W {
        self.variant(VLOOFF_A::VLOOFF_1)
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
LFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFXTBYPASS_A {
    #[doc = "0: LFXT sourced from external crystal"]
    LFXTBYPASS_0 = 0,
    #[doc = "1: LFXT sourced from external clock signal"]
    LFXTBYPASS_1 = 1,
}
impl From<LFXTBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFXTBYPASS`"]
pub type LFXTBYPASS_R = crate::R<bool, LFXTBYPASS_A>;
impl LFXTBYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTBYPASS_A {
        match self.bits {
            false => LFXTBYPASS_A::LFXTBYPASS_0,
            true => LFXTBYPASS_A::LFXTBYPASS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTBYPASS_0`"]
    #[inline(always)]
    pub fn is_lfxtbypass_0(&self) -> bool {
        *self == LFXTBYPASS_A::LFXTBYPASS_0
    }
    #[doc = "Checks if the value of the field is `LFXTBYPASS_1`"]
    #[inline(always)]
    pub fn is_lfxtbypass_1(&self) -> bool {
        *self == LFXTBYPASS_A::LFXTBYPASS_1
    }
}
#[doc = "Write proxy for field `LFXTBYPASS`"]
pub struct LFXTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTBYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LFXT sourced from external crystal"]
    #[inline(always)]
    pub fn lfxtbypass_0(self) -> &'a mut W {
        self.variant(LFXTBYPASS_A::LFXTBYPASS_0)
    }
    #[doc = "LFXT sourced from external clock signal"]
    #[inline(always)]
    pub fn lfxtbypass_1(self) -> &'a mut W {
        self.variant(LFXTBYPASS_A::LFXTBYPASS_1)
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
#[doc = "7:6\\]
LFXT oscillator current\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFXTDRIVE_A {
    #[doc = "0: Lowest drive strength and current consumption LFXT oscillator"]
    LFXTDRIVE_0 = 0,
    #[doc = "1: Increased drive strength LFXT oscillator"]
    LFXTDRIVE_1 = 1,
    #[doc = "2: Increased drive strength LFXT oscillator"]
    LFXTDRIVE_2 = 2,
    #[doc = "3: Maximum drive strength and maximum current consumption LFXT oscillator"]
    LFXTDRIVE_3 = 3,
}
impl From<LFXTDRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXTDRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LFXTDRIVE`"]
pub type LFXTDRIVE_R = crate::R<u8, LFXTDRIVE_A>;
impl LFXTDRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTDRIVE_A {
        match self.bits {
            0 => LFXTDRIVE_A::LFXTDRIVE_0,
            1 => LFXTDRIVE_A::LFXTDRIVE_1,
            2 => LFXTDRIVE_A::LFXTDRIVE_2,
            3 => LFXTDRIVE_A::LFXTDRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_lfxtdrive_0(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_lfxtdrive_1(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_1
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_2`"]
    #[inline(always)]
    pub fn is_lfxtdrive_2(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_2
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_3`"]
    #[inline(always)]
    pub fn is_lfxtdrive_3(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_3
    }
}
#[doc = "Write proxy for field `LFXTDRIVE`"]
pub struct LFXTDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTDRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTDRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Lowest drive strength and current consumption LFXT oscillator"]
    #[inline(always)]
    pub fn lfxtdrive_0(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_0)
    }
    #[doc = "Increased drive strength LFXT oscillator"]
    #[inline(always)]
    pub fn lfxtdrive_1(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_1)
    }
    #[doc = "Increased drive strength LFXT oscillator"]
    #[inline(always)]
    pub fn lfxtdrive_2(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_2)
    }
    #[doc = "Maximum drive strength and maximum current consumption LFXT oscillator"]
    #[inline(always)]
    pub fn lfxtdrive_3(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "8:8\\]
Turns off the HFXT oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXTOFF_A {
    #[doc = "0: HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation"]
    HFXTOFF_0 = 0,
    #[doc = "1: HFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    HFXTOFF_1 = 1,
}
impl From<HFXTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXTOFF`"]
pub type HFXTOFF_R = crate::R<bool, HFXTOFF_A>;
impl HFXTOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTOFF_A {
        match self.bits {
            false => HFXTOFF_A::HFXTOFF_0,
            true => HFXTOFF_A::HFXTOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTOFF_0`"]
    #[inline(always)]
    pub fn is_hfxtoff_0(&self) -> bool {
        *self == HFXTOFF_A::HFXTOFF_0
    }
    #[doc = "Checks if the value of the field is `HFXTOFF_1`"]
    #[inline(always)]
    pub fn is_hfxtoff_1(&self) -> bool {
        *self == HFXTOFF_A::HFXTOFF_1
    }
}
#[doc = "Write proxy for field `HFXTOFF`"]
pub struct HFXTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation"]
    #[inline(always)]
    pub fn hfxtoff_0(self) -> &'a mut W {
        self.variant(HFXTOFF_A::HFXTOFF_0)
    }
    #[doc = "HFXT is off if it is not used as a source for ACLK, MCLK, or SMCLK"]
    #[inline(always)]
    pub fn hfxtoff_1(self) -> &'a mut W {
        self.variant(HFXTOFF_A::HFXTOFF_1)
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
#[doc = "11:10\\]
HFXT frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFFREQ_A {
    #[doc = "0: 0 to 4 MHz"]
    HFFREQ_0 = 0,
    #[doc = "1: Greater than 4 MHz to 8 MHz"]
    HFFREQ_1 = 1,
    #[doc = "2: Greater than 8 MHz to 16 MHz"]
    HFFREQ_2 = 2,
    #[doc = "3: Greater than 16 MHz to 24 MHz"]
    HFFREQ_3 = 3,
}
impl From<HFFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: HFFREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HFFREQ`"]
pub type HFFREQ_R = crate::R<u8, HFFREQ_A>;
impl HFFREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFFREQ_A {
        match self.bits {
            0 => HFFREQ_A::HFFREQ_0,
            1 => HFFREQ_A::HFFREQ_1,
            2 => HFFREQ_A::HFFREQ_2,
            3 => HFFREQ_A::HFFREQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFFREQ_0`"]
    #[inline(always)]
    pub fn is_hffreq_0(&self) -> bool {
        *self == HFFREQ_A::HFFREQ_0
    }
    #[doc = "Checks if the value of the field is `HFFREQ_1`"]
    #[inline(always)]
    pub fn is_hffreq_1(&self) -> bool {
        *self == HFFREQ_A::HFFREQ_1
    }
    #[doc = "Checks if the value of the field is `HFFREQ_2`"]
    #[inline(always)]
    pub fn is_hffreq_2(&self) -> bool {
        *self == HFFREQ_A::HFFREQ_2
    }
    #[doc = "Checks if the value of the field is `HFFREQ_3`"]
    #[inline(always)]
    pub fn is_hffreq_3(&self) -> bool {
        *self == HFFREQ_A::HFFREQ_3
    }
}
#[doc = "Write proxy for field `HFFREQ`"]
pub struct HFFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HFFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFFREQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0 to 4 MHz"]
    #[inline(always)]
    pub fn hffreq_0(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_0)
    }
    #[doc = "Greater than 4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn hffreq_1(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_1)
    }
    #[doc = "Greater than 8 MHz to 16 MHz"]
    #[inline(always)]
    pub fn hffreq_2(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_2)
    }
    #[doc = "Greater than 16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn hffreq_3(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "12:12\\]
HFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXTBYPASS_A {
    #[doc = "0: HFXT sourced from external crystal"]
    HFXTBYPASS_0 = 0,
    #[doc = "1: HFXT sourced from external clock signal"]
    HFXTBYPASS_1 = 1,
}
impl From<HFXTBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXTBYPASS`"]
pub type HFXTBYPASS_R = crate::R<bool, HFXTBYPASS_A>;
impl HFXTBYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTBYPASS_A {
        match self.bits {
            false => HFXTBYPASS_A::HFXTBYPASS_0,
            true => HFXTBYPASS_A::HFXTBYPASS_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTBYPASS_0`"]
    #[inline(always)]
    pub fn is_hfxtbypass_0(&self) -> bool {
        *self == HFXTBYPASS_A::HFXTBYPASS_0
    }
    #[doc = "Checks if the value of the field is `HFXTBYPASS_1`"]
    #[inline(always)]
    pub fn is_hfxtbypass_1(&self) -> bool {
        *self == HFXTBYPASS_A::HFXTBYPASS_1
    }
}
#[doc = "Write proxy for field `HFXTBYPASS`"]
pub struct HFXTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTBYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HFXT sourced from external crystal"]
    #[inline(always)]
    pub fn hfxtbypass_0(self) -> &'a mut W {
        self.variant(HFXTBYPASS_A::HFXTBYPASS_0)
    }
    #[doc = "HFXT sourced from external clock signal"]
    #[inline(always)]
    pub fn hfxtbypass_1(self) -> &'a mut W {
        self.variant(HFXTBYPASS_A::HFXTBYPASS_1)
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
#[doc = "15:14\\]
HFXT oscillator current\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFXTDRIVE_A {
    #[doc = "0: Lowest current consumption"]
    HFXTDRIVE_0 = 0,
    #[doc = "1: Increased drive strength HFXT oscillator"]
    HFXTDRIVE_1 = 1,
    #[doc = "2: Increased drive strength HFXT oscillator"]
    HFXTDRIVE_2 = 2,
    #[doc = "3: Maximum drive strength HFXT oscillator"]
    HFXTDRIVE_3 = 3,
}
impl From<HFXTDRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXTDRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HFXTDRIVE`"]
pub type HFXTDRIVE_R = crate::R<u8, HFXTDRIVE_A>;
impl HFXTDRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTDRIVE_A {
        match self.bits {
            0 => HFXTDRIVE_A::HFXTDRIVE_0,
            1 => HFXTDRIVE_A::HFXTDRIVE_1,
            2 => HFXTDRIVE_A::HFXTDRIVE_2,
            3 => HFXTDRIVE_A::HFXTDRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_hfxtdrive_0(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_hfxtdrive_1(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_1
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_2`"]
    #[inline(always)]
    pub fn is_hfxtdrive_2(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_2
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_3`"]
    #[inline(always)]
    pub fn is_hfxtdrive_3(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_3
    }
}
#[doc = "Write proxy for field `HFXTDRIVE`"]
pub struct HFXTDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTDRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTDRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Lowest current consumption"]
    #[inline(always)]
    pub fn hfxtdrive_0(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_0)
    }
    #[doc = "Increased drive strength HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtdrive_1(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_1)
    }
    #[doc = "Increased drive strength HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtdrive_2(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_2)
    }
    #[doc = "Maximum drive strength HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtdrive_3(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
LFXT off"]
    #[inline(always)]
    pub fn lfxtoff(&self) -> LFXTOFF_R {
        LFXTOFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SMCLKOFF_R {
        SMCLKOFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
VLO off"]
    #[inline(always)]
    pub fn vlooff(&self) -> VLOOFF_R {
        VLOOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&self) -> LFXTBYPASS_R {
        LFXTBYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
LFXT oscillator current"]
    #[inline(always)]
    pub fn lfxtdrive(&self) -> LFXTDRIVE_R {
        LFXTDRIVE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Turns off the HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtoff(&self) -> HFXTOFF_R {
        HFXTOFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
HFXT frequency selection"]
    #[inline(always)]
    pub fn hffreq(&self) -> HFFREQ_R {
        HFFREQ_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&self) -> HFXTBYPASS_R {
        HFXTBYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
HFXT oscillator current"]
    #[inline(always)]
    pub fn hfxtdrive(&self) -> HFXTDRIVE_R {
        HFXTDRIVE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LFXT off"]
    #[inline(always)]
    pub fn lfxtoff(&mut self) -> LFXTOFF_W {
        LFXTOFF_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SMCLKOFF_W {
        SMCLKOFF_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
VLO off"]
    #[inline(always)]
    pub fn vlooff(&mut self) -> VLOOFF_W {
        VLOOFF_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&mut self) -> LFXTBYPASS_W {
        LFXTBYPASS_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
LFXT oscillator current"]
    #[inline(always)]
    pub fn lfxtdrive(&mut self) -> LFXTDRIVE_W {
        LFXTDRIVE_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Turns off the HFXT oscillator"]
    #[inline(always)]
    pub fn hfxtoff(&mut self) -> HFXTOFF_W {
        HFXTOFF_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
HFXT frequency selection"]
    #[inline(always)]
    pub fn hffreq(&mut self) -> HFFREQ_W {
        HFFREQ_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&mut self) -> HFXTBYPASS_W {
        HFXTBYPASS_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
HFXT oscillator current"]
    #[inline(always)]
    pub fn hfxtdrive(&mut self) -> HFXTDRIVE_W {
        HFXTDRIVE_W { w: self }
    }
}
