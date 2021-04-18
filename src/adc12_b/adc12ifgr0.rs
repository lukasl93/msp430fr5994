#[doc = "Reader of register ADC12IFGR0"]
pub type R = crate::R<u16, super::ADC12IFGR0>;
#[doc = "Writer for register ADC12IFGR0"]
pub type W = crate::W<u16, super::ADC12IFGR0>;
#[doc = "Register ADC12IFGR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IFGR0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
ADC12MEM0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG0_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG0_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG0_1 = 1,
}
impl From<ADC12IFG0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG0`"]
pub type ADC12IFG0_R = crate::R<bool, ADC12IFG0_A>;
impl ADC12IFG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG0_A {
        match self.bits {
            false => ADC12IFG0_A::ADC12IFG0_0,
            true => ADC12IFG0_A::ADC12IFG0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG0_0`"]
    #[inline(always)]
    pub fn is_adc12ifg0_0(&self) -> bool {
        *self == ADC12IFG0_A::ADC12IFG0_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG0_1`"]
    #[inline(always)]
    pub fn is_adc12ifg0_1(&self) -> bool {
        *self == ADC12IFG0_A::ADC12IFG0_1
    }
}
#[doc = "Write proxy for field `ADC12IFG0`"]
pub struct ADC12IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg0_0(self) -> &'a mut W {
        self.variant(ADC12IFG0_A::ADC12IFG0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg0_1(self) -> &'a mut W {
        self.variant(ADC12IFG0_A::ADC12IFG0_1)
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
ADC12MEM1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG1_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG1_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG1_1 = 1,
}
impl From<ADC12IFG1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG1`"]
pub type ADC12IFG1_R = crate::R<bool, ADC12IFG1_A>;
impl ADC12IFG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG1_A {
        match self.bits {
            false => ADC12IFG1_A::ADC12IFG1_0,
            true => ADC12IFG1_A::ADC12IFG1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG1_0`"]
    #[inline(always)]
    pub fn is_adc12ifg1_0(&self) -> bool {
        *self == ADC12IFG1_A::ADC12IFG1_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG1_1`"]
    #[inline(always)]
    pub fn is_adc12ifg1_1(&self) -> bool {
        *self == ADC12IFG1_A::ADC12IFG1_1
    }
}
#[doc = "Write proxy for field `ADC12IFG1`"]
pub struct ADC12IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg1_0(self) -> &'a mut W {
        self.variant(ADC12IFG1_A::ADC12IFG1_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg1_1(self) -> &'a mut W {
        self.variant(ADC12IFG1_A::ADC12IFG1_1)
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
ADC12MEM2 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG2_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG2_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG2_1 = 1,
}
impl From<ADC12IFG2_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG2`"]
pub type ADC12IFG2_R = crate::R<bool, ADC12IFG2_A>;
impl ADC12IFG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG2_A {
        match self.bits {
            false => ADC12IFG2_A::ADC12IFG2_0,
            true => ADC12IFG2_A::ADC12IFG2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG2_0`"]
    #[inline(always)]
    pub fn is_adc12ifg2_0(&self) -> bool {
        *self == ADC12IFG2_A::ADC12IFG2_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG2_1`"]
    #[inline(always)]
    pub fn is_adc12ifg2_1(&self) -> bool {
        *self == ADC12IFG2_A::ADC12IFG2_1
    }
}
#[doc = "Write proxy for field `ADC12IFG2`"]
pub struct ADC12IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg2_0(self) -> &'a mut W {
        self.variant(ADC12IFG2_A::ADC12IFG2_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg2_1(self) -> &'a mut W {
        self.variant(ADC12IFG2_A::ADC12IFG2_1)
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
ADC12MEM3 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG3_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG3_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG3_1 = 1,
}
impl From<ADC12IFG3_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG3`"]
pub type ADC12IFG3_R = crate::R<bool, ADC12IFG3_A>;
impl ADC12IFG3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG3_A {
        match self.bits {
            false => ADC12IFG3_A::ADC12IFG3_0,
            true => ADC12IFG3_A::ADC12IFG3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG3_0`"]
    #[inline(always)]
    pub fn is_adc12ifg3_0(&self) -> bool {
        *self == ADC12IFG3_A::ADC12IFG3_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG3_1`"]
    #[inline(always)]
    pub fn is_adc12ifg3_1(&self) -> bool {
        *self == ADC12IFG3_A::ADC12IFG3_1
    }
}
#[doc = "Write proxy for field `ADC12IFG3`"]
pub struct ADC12IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg3_0(self) -> &'a mut W {
        self.variant(ADC12IFG3_A::ADC12IFG3_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg3_1(self) -> &'a mut W {
        self.variant(ADC12IFG3_A::ADC12IFG3_1)
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
ADC12MEM4 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG4_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG4_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG4_1 = 1,
}
impl From<ADC12IFG4_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG4`"]
pub type ADC12IFG4_R = crate::R<bool, ADC12IFG4_A>;
impl ADC12IFG4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG4_A {
        match self.bits {
            false => ADC12IFG4_A::ADC12IFG4_0,
            true => ADC12IFG4_A::ADC12IFG4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG4_0`"]
    #[inline(always)]
    pub fn is_adc12ifg4_0(&self) -> bool {
        *self == ADC12IFG4_A::ADC12IFG4_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG4_1`"]
    #[inline(always)]
    pub fn is_adc12ifg4_1(&self) -> bool {
        *self == ADC12IFG4_A::ADC12IFG4_1
    }
}
#[doc = "Write proxy for field `ADC12IFG4`"]
pub struct ADC12IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg4_0(self) -> &'a mut W {
        self.variant(ADC12IFG4_A::ADC12IFG4_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg4_1(self) -> &'a mut W {
        self.variant(ADC12IFG4_A::ADC12IFG4_1)
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
ADC12MEM5 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG5_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG5_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG5_1 = 1,
}
impl From<ADC12IFG5_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG5`"]
pub type ADC12IFG5_R = crate::R<bool, ADC12IFG5_A>;
impl ADC12IFG5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG5_A {
        match self.bits {
            false => ADC12IFG5_A::ADC12IFG5_0,
            true => ADC12IFG5_A::ADC12IFG5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG5_0`"]
    #[inline(always)]
    pub fn is_adc12ifg5_0(&self) -> bool {
        *self == ADC12IFG5_A::ADC12IFG5_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG5_1`"]
    #[inline(always)]
    pub fn is_adc12ifg5_1(&self) -> bool {
        *self == ADC12IFG5_A::ADC12IFG5_1
    }
}
#[doc = "Write proxy for field `ADC12IFG5`"]
pub struct ADC12IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg5_0(self) -> &'a mut W {
        self.variant(ADC12IFG5_A::ADC12IFG5_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg5_1(self) -> &'a mut W {
        self.variant(ADC12IFG5_A::ADC12IFG5_1)
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
ADC12MEM6 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG6_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG6_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG6_1 = 1,
}
impl From<ADC12IFG6_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG6`"]
pub type ADC12IFG6_R = crate::R<bool, ADC12IFG6_A>;
impl ADC12IFG6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG6_A {
        match self.bits {
            false => ADC12IFG6_A::ADC12IFG6_0,
            true => ADC12IFG6_A::ADC12IFG6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG6_0`"]
    #[inline(always)]
    pub fn is_adc12ifg6_0(&self) -> bool {
        *self == ADC12IFG6_A::ADC12IFG6_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG6_1`"]
    #[inline(always)]
    pub fn is_adc12ifg6_1(&self) -> bool {
        *self == ADC12IFG6_A::ADC12IFG6_1
    }
}
#[doc = "Write proxy for field `ADC12IFG6`"]
pub struct ADC12IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg6_0(self) -> &'a mut W {
        self.variant(ADC12IFG6_A::ADC12IFG6_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg6_1(self) -> &'a mut W {
        self.variant(ADC12IFG6_A::ADC12IFG6_1)
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
ADC12MEM7 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG7_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG7_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG7_1 = 1,
}
impl From<ADC12IFG7_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG7`"]
pub type ADC12IFG7_R = crate::R<bool, ADC12IFG7_A>;
impl ADC12IFG7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG7_A {
        match self.bits {
            false => ADC12IFG7_A::ADC12IFG7_0,
            true => ADC12IFG7_A::ADC12IFG7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG7_0`"]
    #[inline(always)]
    pub fn is_adc12ifg7_0(&self) -> bool {
        *self == ADC12IFG7_A::ADC12IFG7_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG7_1`"]
    #[inline(always)]
    pub fn is_adc12ifg7_1(&self) -> bool {
        *self == ADC12IFG7_A::ADC12IFG7_1
    }
}
#[doc = "Write proxy for field `ADC12IFG7`"]
pub struct ADC12IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg7_0(self) -> &'a mut W {
        self.variant(ADC12IFG7_A::ADC12IFG7_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg7_1(self) -> &'a mut W {
        self.variant(ADC12IFG7_A::ADC12IFG7_1)
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
ADC12MEM8 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG8_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG8_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG8_1 = 1,
}
impl From<ADC12IFG8_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG8`"]
pub type ADC12IFG8_R = crate::R<bool, ADC12IFG8_A>;
impl ADC12IFG8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG8_A {
        match self.bits {
            false => ADC12IFG8_A::ADC12IFG8_0,
            true => ADC12IFG8_A::ADC12IFG8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG8_0`"]
    #[inline(always)]
    pub fn is_adc12ifg8_0(&self) -> bool {
        *self == ADC12IFG8_A::ADC12IFG8_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG8_1`"]
    #[inline(always)]
    pub fn is_adc12ifg8_1(&self) -> bool {
        *self == ADC12IFG8_A::ADC12IFG8_1
    }
}
#[doc = "Write proxy for field `ADC12IFG8`"]
pub struct ADC12IFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg8_0(self) -> &'a mut W {
        self.variant(ADC12IFG8_A::ADC12IFG8_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg8_1(self) -> &'a mut W {
        self.variant(ADC12IFG8_A::ADC12IFG8_1)
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
ADC12MEM9 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG9_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG9_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG9_1 = 1,
}
impl From<ADC12IFG9_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG9`"]
pub type ADC12IFG9_R = crate::R<bool, ADC12IFG9_A>;
impl ADC12IFG9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG9_A {
        match self.bits {
            false => ADC12IFG9_A::ADC12IFG9_0,
            true => ADC12IFG9_A::ADC12IFG9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG9_0`"]
    #[inline(always)]
    pub fn is_adc12ifg9_0(&self) -> bool {
        *self == ADC12IFG9_A::ADC12IFG9_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG9_1`"]
    #[inline(always)]
    pub fn is_adc12ifg9_1(&self) -> bool {
        *self == ADC12IFG9_A::ADC12IFG9_1
    }
}
#[doc = "Write proxy for field `ADC12IFG9`"]
pub struct ADC12IFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg9_0(self) -> &'a mut W {
        self.variant(ADC12IFG9_A::ADC12IFG9_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg9_1(self) -> &'a mut W {
        self.variant(ADC12IFG9_A::ADC12IFG9_1)
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
ADC12MEM10 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG10_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG10_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG10_1 = 1,
}
impl From<ADC12IFG10_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG10`"]
pub type ADC12IFG10_R = crate::R<bool, ADC12IFG10_A>;
impl ADC12IFG10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG10_A {
        match self.bits {
            false => ADC12IFG10_A::ADC12IFG10_0,
            true => ADC12IFG10_A::ADC12IFG10_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG10_0`"]
    #[inline(always)]
    pub fn is_adc12ifg10_0(&self) -> bool {
        *self == ADC12IFG10_A::ADC12IFG10_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG10_1`"]
    #[inline(always)]
    pub fn is_adc12ifg10_1(&self) -> bool {
        *self == ADC12IFG10_A::ADC12IFG10_1
    }
}
#[doc = "Write proxy for field `ADC12IFG10`"]
pub struct ADC12IFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg10_0(self) -> &'a mut W {
        self.variant(ADC12IFG10_A::ADC12IFG10_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg10_1(self) -> &'a mut W {
        self.variant(ADC12IFG10_A::ADC12IFG10_1)
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
ADC12MEM11 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG11_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG11_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG11_1 = 1,
}
impl From<ADC12IFG11_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG11`"]
pub type ADC12IFG11_R = crate::R<bool, ADC12IFG11_A>;
impl ADC12IFG11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG11_A {
        match self.bits {
            false => ADC12IFG11_A::ADC12IFG11_0,
            true => ADC12IFG11_A::ADC12IFG11_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG11_0`"]
    #[inline(always)]
    pub fn is_adc12ifg11_0(&self) -> bool {
        *self == ADC12IFG11_A::ADC12IFG11_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG11_1`"]
    #[inline(always)]
    pub fn is_adc12ifg11_1(&self) -> bool {
        *self == ADC12IFG11_A::ADC12IFG11_1
    }
}
#[doc = "Write proxy for field `ADC12IFG11`"]
pub struct ADC12IFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg11_0(self) -> &'a mut W {
        self.variant(ADC12IFG11_A::ADC12IFG11_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg11_1(self) -> &'a mut W {
        self.variant(ADC12IFG11_A::ADC12IFG11_1)
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
ADC12MEM12 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG12_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG12_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG12_1 = 1,
}
impl From<ADC12IFG12_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG12`"]
pub type ADC12IFG12_R = crate::R<bool, ADC12IFG12_A>;
impl ADC12IFG12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG12_A {
        match self.bits {
            false => ADC12IFG12_A::ADC12IFG12_0,
            true => ADC12IFG12_A::ADC12IFG12_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG12_0`"]
    #[inline(always)]
    pub fn is_adc12ifg12_0(&self) -> bool {
        *self == ADC12IFG12_A::ADC12IFG12_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG12_1`"]
    #[inline(always)]
    pub fn is_adc12ifg12_1(&self) -> bool {
        *self == ADC12IFG12_A::ADC12IFG12_1
    }
}
#[doc = "Write proxy for field `ADC12IFG12`"]
pub struct ADC12IFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg12_0(self) -> &'a mut W {
        self.variant(ADC12IFG12_A::ADC12IFG12_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg12_1(self) -> &'a mut W {
        self.variant(ADC12IFG12_A::ADC12IFG12_1)
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
ADC12MEM13 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG13_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG13_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG13_1 = 1,
}
impl From<ADC12IFG13_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG13`"]
pub type ADC12IFG13_R = crate::R<bool, ADC12IFG13_A>;
impl ADC12IFG13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG13_A {
        match self.bits {
            false => ADC12IFG13_A::ADC12IFG13_0,
            true => ADC12IFG13_A::ADC12IFG13_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG13_0`"]
    #[inline(always)]
    pub fn is_adc12ifg13_0(&self) -> bool {
        *self == ADC12IFG13_A::ADC12IFG13_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG13_1`"]
    #[inline(always)]
    pub fn is_adc12ifg13_1(&self) -> bool {
        *self == ADC12IFG13_A::ADC12IFG13_1
    }
}
#[doc = "Write proxy for field `ADC12IFG13`"]
pub struct ADC12IFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg13_0(self) -> &'a mut W {
        self.variant(ADC12IFG13_A::ADC12IFG13_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg13_1(self) -> &'a mut W {
        self.variant(ADC12IFG13_A::ADC12IFG13_1)
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
ADC12MEM14 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG14_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG14_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG14_1 = 1,
}
impl From<ADC12IFG14_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG14`"]
pub type ADC12IFG14_R = crate::R<bool, ADC12IFG14_A>;
impl ADC12IFG14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG14_A {
        match self.bits {
            false => ADC12IFG14_A::ADC12IFG14_0,
            true => ADC12IFG14_A::ADC12IFG14_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG14_0`"]
    #[inline(always)]
    pub fn is_adc12ifg14_0(&self) -> bool {
        *self == ADC12IFG14_A::ADC12IFG14_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG14_1`"]
    #[inline(always)]
    pub fn is_adc12ifg14_1(&self) -> bool {
        *self == ADC12IFG14_A::ADC12IFG14_1
    }
}
#[doc = "Write proxy for field `ADC12IFG14`"]
pub struct ADC12IFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg14_0(self) -> &'a mut W {
        self.variant(ADC12IFG14_A::ADC12IFG14_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg14_1(self) -> &'a mut W {
        self.variant(ADC12IFG14_A::ADC12IFG14_1)
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
ADC12MEM15 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG15_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG15_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG15_1 = 1,
}
impl From<ADC12IFG15_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG15`"]
pub type ADC12IFG15_R = crate::R<bool, ADC12IFG15_A>;
impl ADC12IFG15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG15_A {
        match self.bits {
            false => ADC12IFG15_A::ADC12IFG15_0,
            true => ADC12IFG15_A::ADC12IFG15_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG15_0`"]
    #[inline(always)]
    pub fn is_adc12ifg15_0(&self) -> bool {
        *self == ADC12IFG15_A::ADC12IFG15_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG15_1`"]
    #[inline(always)]
    pub fn is_adc12ifg15_1(&self) -> bool {
        *self == ADC12IFG15_A::ADC12IFG15_1
    }
}
#[doc = "Write proxy for field `ADC12IFG15`"]
pub struct ADC12IFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg15_0(self) -> &'a mut W {
        self.variant(ADC12IFG15_A::ADC12IFG15_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg15_1(self) -> &'a mut W {
        self.variant(ADC12IFG15_A::ADC12IFG15_1)
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
ADC12MEM0 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg0(&self) -> ADC12IFG0_R {
        ADC12IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ADC12MEM1 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg1(&self) -> ADC12IFG1_R {
        ADC12IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
ADC12MEM2 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg2(&self) -> ADC12IFG2_R {
        ADC12IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
ADC12MEM3 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg3(&self) -> ADC12IFG3_R {
        ADC12IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ADC12MEM4 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg4(&self) -> ADC12IFG4_R {
        ADC12IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
ADC12MEM5 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg5(&self) -> ADC12IFG5_R {
        ADC12IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
ADC12MEM6 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg6(&self) -> ADC12IFG6_R {
        ADC12IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
ADC12MEM7 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg7(&self) -> ADC12IFG7_R {
        ADC12IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
ADC12MEM8 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg8(&self) -> ADC12IFG8_R {
        ADC12IFG8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
ADC12MEM9 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg9(&self) -> ADC12IFG9_R {
        ADC12IFG9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
ADC12MEM10 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg10(&self) -> ADC12IFG10_R {
        ADC12IFG10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
ADC12MEM11 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg11(&self) -> ADC12IFG11_R {
        ADC12IFG11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
ADC12MEM12 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg12(&self) -> ADC12IFG12_R {
        ADC12IFG12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
ADC12MEM13 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg13(&self) -> ADC12IFG13_R {
        ADC12IFG13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
ADC12MEM14 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg14(&self) -> ADC12IFG14_R {
        ADC12IFG14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
ADC12MEM15 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg15(&self) -> ADC12IFG15_R {
        ADC12IFG15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ADC12MEM0 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg0(&mut self) -> ADC12IFG0_W {
        ADC12IFG0_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
ADC12MEM1 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg1(&mut self) -> ADC12IFG1_W {
        ADC12IFG1_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
ADC12MEM2 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg2(&mut self) -> ADC12IFG2_W {
        ADC12IFG2_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
ADC12MEM3 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg3(&mut self) -> ADC12IFG3_W {
        ADC12IFG3_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
ADC12MEM4 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg4(&mut self) -> ADC12IFG4_W {
        ADC12IFG4_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
ADC12MEM5 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg5(&mut self) -> ADC12IFG5_W {
        ADC12IFG5_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
ADC12MEM6 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg6(&mut self) -> ADC12IFG6_W {
        ADC12IFG6_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
ADC12MEM7 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg7(&mut self) -> ADC12IFG7_W {
        ADC12IFG7_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
ADC12MEM8 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg8(&mut self) -> ADC12IFG8_W {
        ADC12IFG8_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
ADC12MEM9 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg9(&mut self) -> ADC12IFG9_W {
        ADC12IFG9_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
ADC12MEM10 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg10(&mut self) -> ADC12IFG10_W {
        ADC12IFG10_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
ADC12MEM11 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg11(&mut self) -> ADC12IFG11_W {
        ADC12IFG11_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
ADC12MEM12 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg12(&mut self) -> ADC12IFG12_W {
        ADC12IFG12_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
ADC12MEM13 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg13(&mut self) -> ADC12IFG13_W {
        ADC12IFG13_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
ADC12MEM14 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg14(&mut self) -> ADC12IFG14_W {
        ADC12IFG14_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
ADC12MEM15 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg15(&mut self) -> ADC12IFG15_W {
        ADC12IFG15_W { w: self }
    }
}
