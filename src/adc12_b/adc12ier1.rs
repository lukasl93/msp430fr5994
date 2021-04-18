#[doc = "Reader of register ADC12IER1"]
pub type R = crate::R<u16, super::ADC12IER1>;
#[doc = "Writer for register ADC12IER1"]
pub type W = crate::W<u16, super::ADC12IER1>;
#[doc = "Register ADC12IER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IER1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
interrupt enable 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE16_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE16_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE16_1 = 1,
}
impl From<ADC12IE16_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE16`"]
pub type ADC12IE16_R = crate::R<bool, ADC12IE16_A>;
impl ADC12IE16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE16_A {
        match self.bits {
            false => ADC12IE16_A::ADC12IE16_0,
            true => ADC12IE16_A::ADC12IE16_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE16_0`"]
    #[inline(always)]
    pub fn is_adc12ie16_0(&self) -> bool {
        *self == ADC12IE16_A::ADC12IE16_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE16_1`"]
    #[inline(always)]
    pub fn is_adc12ie16_1(&self) -> bool {
        *self == ADC12IE16_A::ADC12IE16_1
    }
}
#[doc = "Write proxy for field `ADC12IE16`"]
pub struct ADC12IE16_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie16_0(self) -> &'a mut W {
        self.variant(ADC12IE16_A::ADC12IE16_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie16_1(self) -> &'a mut W {
        self.variant(ADC12IE16_A::ADC12IE16_1)
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
interrupt enable 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE17_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE17_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE17_1 = 1,
}
impl From<ADC12IE17_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE17`"]
pub type ADC12IE17_R = crate::R<bool, ADC12IE17_A>;
impl ADC12IE17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE17_A {
        match self.bits {
            false => ADC12IE17_A::ADC12IE17_0,
            true => ADC12IE17_A::ADC12IE17_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE17_0`"]
    #[inline(always)]
    pub fn is_adc12ie17_0(&self) -> bool {
        *self == ADC12IE17_A::ADC12IE17_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE17_1`"]
    #[inline(always)]
    pub fn is_adc12ie17_1(&self) -> bool {
        *self == ADC12IE17_A::ADC12IE17_1
    }
}
#[doc = "Write proxy for field `ADC12IE17`"]
pub struct ADC12IE17_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie17_0(self) -> &'a mut W {
        self.variant(ADC12IE17_A::ADC12IE17_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie17_1(self) -> &'a mut W {
        self.variant(ADC12IE17_A::ADC12IE17_1)
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
interrupt enable 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE18_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE18_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE18_1 = 1,
}
impl From<ADC12IE18_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE18`"]
pub type ADC12IE18_R = crate::R<bool, ADC12IE18_A>;
impl ADC12IE18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE18_A {
        match self.bits {
            false => ADC12IE18_A::ADC12IE18_0,
            true => ADC12IE18_A::ADC12IE18_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE18_0`"]
    #[inline(always)]
    pub fn is_adc12ie18_0(&self) -> bool {
        *self == ADC12IE18_A::ADC12IE18_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE18_1`"]
    #[inline(always)]
    pub fn is_adc12ie18_1(&self) -> bool {
        *self == ADC12IE18_A::ADC12IE18_1
    }
}
#[doc = "Write proxy for field `ADC12IE18`"]
pub struct ADC12IE18_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie18_0(self) -> &'a mut W {
        self.variant(ADC12IE18_A::ADC12IE18_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie18_1(self) -> &'a mut W {
        self.variant(ADC12IE18_A::ADC12IE18_1)
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
interrupt enable 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE19_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE19_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE19_1 = 1,
}
impl From<ADC12IE19_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE19`"]
pub type ADC12IE19_R = crate::R<bool, ADC12IE19_A>;
impl ADC12IE19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE19_A {
        match self.bits {
            false => ADC12IE19_A::ADC12IE19_0,
            true => ADC12IE19_A::ADC12IE19_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE19_0`"]
    #[inline(always)]
    pub fn is_adc12ie19_0(&self) -> bool {
        *self == ADC12IE19_A::ADC12IE19_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE19_1`"]
    #[inline(always)]
    pub fn is_adc12ie19_1(&self) -> bool {
        *self == ADC12IE19_A::ADC12IE19_1
    }
}
#[doc = "Write proxy for field `ADC12IE19`"]
pub struct ADC12IE19_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie19_0(self) -> &'a mut W {
        self.variant(ADC12IE19_A::ADC12IE19_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie19_1(self) -> &'a mut W {
        self.variant(ADC12IE19_A::ADC12IE19_1)
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
interrupt enable 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE20_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE20_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE20_1 = 1,
}
impl From<ADC12IE20_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE20`"]
pub type ADC12IE20_R = crate::R<bool, ADC12IE20_A>;
impl ADC12IE20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE20_A {
        match self.bits {
            false => ADC12IE20_A::ADC12IE20_0,
            true => ADC12IE20_A::ADC12IE20_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE20_0`"]
    #[inline(always)]
    pub fn is_adc12ie20_0(&self) -> bool {
        *self == ADC12IE20_A::ADC12IE20_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE20_1`"]
    #[inline(always)]
    pub fn is_adc12ie20_1(&self) -> bool {
        *self == ADC12IE20_A::ADC12IE20_1
    }
}
#[doc = "Write proxy for field `ADC12IE20`"]
pub struct ADC12IE20_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie20_0(self) -> &'a mut W {
        self.variant(ADC12IE20_A::ADC12IE20_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie20_1(self) -> &'a mut W {
        self.variant(ADC12IE20_A::ADC12IE20_1)
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
interrupt enable 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE21_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE21_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE21_1 = 1,
}
impl From<ADC12IE21_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE21`"]
pub type ADC12IE21_R = crate::R<bool, ADC12IE21_A>;
impl ADC12IE21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE21_A {
        match self.bits {
            false => ADC12IE21_A::ADC12IE21_0,
            true => ADC12IE21_A::ADC12IE21_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE21_0`"]
    #[inline(always)]
    pub fn is_adc12ie21_0(&self) -> bool {
        *self == ADC12IE21_A::ADC12IE21_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE21_1`"]
    #[inline(always)]
    pub fn is_adc12ie21_1(&self) -> bool {
        *self == ADC12IE21_A::ADC12IE21_1
    }
}
#[doc = "Write proxy for field `ADC12IE21`"]
pub struct ADC12IE21_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie21_0(self) -> &'a mut W {
        self.variant(ADC12IE21_A::ADC12IE21_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie21_1(self) -> &'a mut W {
        self.variant(ADC12IE21_A::ADC12IE21_1)
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
interrupt enable 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE22_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE22_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE22_1 = 1,
}
impl From<ADC12IE22_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE22`"]
pub type ADC12IE22_R = crate::R<bool, ADC12IE22_A>;
impl ADC12IE22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE22_A {
        match self.bits {
            false => ADC12IE22_A::ADC12IE22_0,
            true => ADC12IE22_A::ADC12IE22_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE22_0`"]
    #[inline(always)]
    pub fn is_adc12ie22_0(&self) -> bool {
        *self == ADC12IE22_A::ADC12IE22_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE22_1`"]
    #[inline(always)]
    pub fn is_adc12ie22_1(&self) -> bool {
        *self == ADC12IE22_A::ADC12IE22_1
    }
}
#[doc = "Write proxy for field `ADC12IE22`"]
pub struct ADC12IE22_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie22_0(self) -> &'a mut W {
        self.variant(ADC12IE22_A::ADC12IE22_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie22_1(self) -> &'a mut W {
        self.variant(ADC12IE22_A::ADC12IE22_1)
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
interrupt enable 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE23_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE23_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE23_1 = 1,
}
impl From<ADC12IE23_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE23`"]
pub type ADC12IE23_R = crate::R<bool, ADC12IE23_A>;
impl ADC12IE23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE23_A {
        match self.bits {
            false => ADC12IE23_A::ADC12IE23_0,
            true => ADC12IE23_A::ADC12IE23_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE23_0`"]
    #[inline(always)]
    pub fn is_adc12ie23_0(&self) -> bool {
        *self == ADC12IE23_A::ADC12IE23_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE23_1`"]
    #[inline(always)]
    pub fn is_adc12ie23_1(&self) -> bool {
        *self == ADC12IE23_A::ADC12IE23_1
    }
}
#[doc = "Write proxy for field `ADC12IE23`"]
pub struct ADC12IE23_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie23_0(self) -> &'a mut W {
        self.variant(ADC12IE23_A::ADC12IE23_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie23_1(self) -> &'a mut W {
        self.variant(ADC12IE23_A::ADC12IE23_1)
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
interrupt enable 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE24_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE24_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE24_1 = 1,
}
impl From<ADC12IE24_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE24`"]
pub type ADC12IE24_R = crate::R<bool, ADC12IE24_A>;
impl ADC12IE24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE24_A {
        match self.bits {
            false => ADC12IE24_A::ADC12IE24_0,
            true => ADC12IE24_A::ADC12IE24_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE24_0`"]
    #[inline(always)]
    pub fn is_adc12ie24_0(&self) -> bool {
        *self == ADC12IE24_A::ADC12IE24_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE24_1`"]
    #[inline(always)]
    pub fn is_adc12ie24_1(&self) -> bool {
        *self == ADC12IE24_A::ADC12IE24_1
    }
}
#[doc = "Write proxy for field `ADC12IE24`"]
pub struct ADC12IE24_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie24_0(self) -> &'a mut W {
        self.variant(ADC12IE24_A::ADC12IE24_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie24_1(self) -> &'a mut W {
        self.variant(ADC12IE24_A::ADC12IE24_1)
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
interrupt enable 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE25_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE25_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE25_1 = 1,
}
impl From<ADC12IE25_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE25`"]
pub type ADC12IE25_R = crate::R<bool, ADC12IE25_A>;
impl ADC12IE25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE25_A {
        match self.bits {
            false => ADC12IE25_A::ADC12IE25_0,
            true => ADC12IE25_A::ADC12IE25_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE25_0`"]
    #[inline(always)]
    pub fn is_adc12ie25_0(&self) -> bool {
        *self == ADC12IE25_A::ADC12IE25_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE25_1`"]
    #[inline(always)]
    pub fn is_adc12ie25_1(&self) -> bool {
        *self == ADC12IE25_A::ADC12IE25_1
    }
}
#[doc = "Write proxy for field `ADC12IE25`"]
pub struct ADC12IE25_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie25_0(self) -> &'a mut W {
        self.variant(ADC12IE25_A::ADC12IE25_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie25_1(self) -> &'a mut W {
        self.variant(ADC12IE25_A::ADC12IE25_1)
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
interrupt enable 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE26_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE26_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE26_1 = 1,
}
impl From<ADC12IE26_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE26`"]
pub type ADC12IE26_R = crate::R<bool, ADC12IE26_A>;
impl ADC12IE26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE26_A {
        match self.bits {
            false => ADC12IE26_A::ADC12IE26_0,
            true => ADC12IE26_A::ADC12IE26_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE26_0`"]
    #[inline(always)]
    pub fn is_adc12ie26_0(&self) -> bool {
        *self == ADC12IE26_A::ADC12IE26_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE26_1`"]
    #[inline(always)]
    pub fn is_adc12ie26_1(&self) -> bool {
        *self == ADC12IE26_A::ADC12IE26_1
    }
}
#[doc = "Write proxy for field `ADC12IE26`"]
pub struct ADC12IE26_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie26_0(self) -> &'a mut W {
        self.variant(ADC12IE26_A::ADC12IE26_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie26_1(self) -> &'a mut W {
        self.variant(ADC12IE26_A::ADC12IE26_1)
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
interrupt enable 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE27_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE27_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE27_1 = 1,
}
impl From<ADC12IE27_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE27`"]
pub type ADC12IE27_R = crate::R<bool, ADC12IE27_A>;
impl ADC12IE27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE27_A {
        match self.bits {
            false => ADC12IE27_A::ADC12IE27_0,
            true => ADC12IE27_A::ADC12IE27_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE27_0`"]
    #[inline(always)]
    pub fn is_adc12ie27_0(&self) -> bool {
        *self == ADC12IE27_A::ADC12IE27_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE27_1`"]
    #[inline(always)]
    pub fn is_adc12ie27_1(&self) -> bool {
        *self == ADC12IE27_A::ADC12IE27_1
    }
}
#[doc = "Write proxy for field `ADC12IE27`"]
pub struct ADC12IE27_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie27_0(self) -> &'a mut W {
        self.variant(ADC12IE27_A::ADC12IE27_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie27_1(self) -> &'a mut W {
        self.variant(ADC12IE27_A::ADC12IE27_1)
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
interrupt enable 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE28_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE28_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE28_1 = 1,
}
impl From<ADC12IE28_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE28`"]
pub type ADC12IE28_R = crate::R<bool, ADC12IE28_A>;
impl ADC12IE28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE28_A {
        match self.bits {
            false => ADC12IE28_A::ADC12IE28_0,
            true => ADC12IE28_A::ADC12IE28_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE28_0`"]
    #[inline(always)]
    pub fn is_adc12ie28_0(&self) -> bool {
        *self == ADC12IE28_A::ADC12IE28_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE28_1`"]
    #[inline(always)]
    pub fn is_adc12ie28_1(&self) -> bool {
        *self == ADC12IE28_A::ADC12IE28_1
    }
}
#[doc = "Write proxy for field `ADC12IE28`"]
pub struct ADC12IE28_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie28_0(self) -> &'a mut W {
        self.variant(ADC12IE28_A::ADC12IE28_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie28_1(self) -> &'a mut W {
        self.variant(ADC12IE28_A::ADC12IE28_1)
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
interrupt enable 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE29_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE29_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE29_1 = 1,
}
impl From<ADC12IE29_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE29`"]
pub type ADC12IE29_R = crate::R<bool, ADC12IE29_A>;
impl ADC12IE29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE29_A {
        match self.bits {
            false => ADC12IE29_A::ADC12IE29_0,
            true => ADC12IE29_A::ADC12IE29_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE29_0`"]
    #[inline(always)]
    pub fn is_adc12ie29_0(&self) -> bool {
        *self == ADC12IE29_A::ADC12IE29_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE29_1`"]
    #[inline(always)]
    pub fn is_adc12ie29_1(&self) -> bool {
        *self == ADC12IE29_A::ADC12IE29_1
    }
}
#[doc = "Write proxy for field `ADC12IE29`"]
pub struct ADC12IE29_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie29_0(self) -> &'a mut W {
        self.variant(ADC12IE29_A::ADC12IE29_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie29_1(self) -> &'a mut W {
        self.variant(ADC12IE29_A::ADC12IE29_1)
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
interrupt enable 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE30_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE30_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE30_1 = 1,
}
impl From<ADC12IE30_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE30`"]
pub type ADC12IE30_R = crate::R<bool, ADC12IE30_A>;
impl ADC12IE30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE30_A {
        match self.bits {
            false => ADC12IE30_A::ADC12IE30_0,
            true => ADC12IE30_A::ADC12IE30_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE30_0`"]
    #[inline(always)]
    pub fn is_adc12ie30_0(&self) -> bool {
        *self == ADC12IE30_A::ADC12IE30_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE30_1`"]
    #[inline(always)]
    pub fn is_adc12ie30_1(&self) -> bool {
        *self == ADC12IE30_A::ADC12IE30_1
    }
}
#[doc = "Write proxy for field `ADC12IE30`"]
pub struct ADC12IE30_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie30_0(self) -> &'a mut W {
        self.variant(ADC12IE30_A::ADC12IE30_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie30_1(self) -> &'a mut W {
        self.variant(ADC12IE30_A::ADC12IE30_1)
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
interrupt enable 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE31_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE31_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE31_1 = 1,
}
impl From<ADC12IE31_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE31`"]
pub type ADC12IE31_R = crate::R<bool, ADC12IE31_A>;
impl ADC12IE31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE31_A {
        match self.bits {
            false => ADC12IE31_A::ADC12IE31_0,
            true => ADC12IE31_A::ADC12IE31_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE31_0`"]
    #[inline(always)]
    pub fn is_adc12ie31_0(&self) -> bool {
        *self == ADC12IE31_A::ADC12IE31_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE31_1`"]
    #[inline(always)]
    pub fn is_adc12ie31_1(&self) -> bool {
        *self == ADC12IE31_A::ADC12IE31_1
    }
}
#[doc = "Write proxy for field `ADC12IE31`"]
pub struct ADC12IE31_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie31_0(self) -> &'a mut W {
        self.variant(ADC12IE31_A::ADC12IE31_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie31_1(self) -> &'a mut W {
        self.variant(ADC12IE31_A::ADC12IE31_1)
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
interrupt enable 16"]
    #[inline(always)]
    pub fn adc12ie16(&self) -> ADC12IE16_R {
        ADC12IE16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable 17"]
    #[inline(always)]
    pub fn adc12ie17(&self) -> ADC12IE17_R {
        ADC12IE17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
interrupt enable 18"]
    #[inline(always)]
    pub fn adc12ie18(&self) -> ADC12IE18_R {
        ADC12IE18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
interrupt enable 19"]
    #[inline(always)]
    pub fn adc12ie19(&self) -> ADC12IE19_R {
        ADC12IE19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
interrupt enable 19"]
    #[inline(always)]
    pub fn adc12ie20(&self) -> ADC12IE20_R {
        ADC12IE20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
interrupt enable 21"]
    #[inline(always)]
    pub fn adc12ie21(&self) -> ADC12IE21_R {
        ADC12IE21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
interrupt enable 22"]
    #[inline(always)]
    pub fn adc12ie22(&self) -> ADC12IE22_R {
        ADC12IE22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
interrupt enable 23"]
    #[inline(always)]
    pub fn adc12ie23(&self) -> ADC12IE23_R {
        ADC12IE23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
interrupt enable 24"]
    #[inline(always)]
    pub fn adc12ie24(&self) -> ADC12IE24_R {
        ADC12IE24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
interrupt enable 25"]
    #[inline(always)]
    pub fn adc12ie25(&self) -> ADC12IE25_R {
        ADC12IE25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
interrupt enable 26"]
    #[inline(always)]
    pub fn adc12ie26(&self) -> ADC12IE26_R {
        ADC12IE26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
interrupt enable 27"]
    #[inline(always)]
    pub fn adc12ie27(&self) -> ADC12IE27_R {
        ADC12IE27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
interrupt enable 28"]
    #[inline(always)]
    pub fn adc12ie28(&self) -> ADC12IE28_R {
        ADC12IE28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
interrupt enable 29"]
    #[inline(always)]
    pub fn adc12ie29(&self) -> ADC12IE29_R {
        ADC12IE29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
interrupt enable 30"]
    #[inline(always)]
    pub fn adc12ie30(&self) -> ADC12IE30_R {
        ADC12IE30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
interrupt enable 30"]
    #[inline(always)]
    pub fn adc12ie31(&self) -> ADC12IE31_R {
        ADC12IE31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable 16"]
    #[inline(always)]
    pub fn adc12ie16(&mut self) -> ADC12IE16_W {
        ADC12IE16_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable 17"]
    #[inline(always)]
    pub fn adc12ie17(&mut self) -> ADC12IE17_W {
        ADC12IE17_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
interrupt enable 18"]
    #[inline(always)]
    pub fn adc12ie18(&mut self) -> ADC12IE18_W {
        ADC12IE18_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
interrupt enable 19"]
    #[inline(always)]
    pub fn adc12ie19(&mut self) -> ADC12IE19_W {
        ADC12IE19_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
interrupt enable 19"]
    #[inline(always)]
    pub fn adc12ie20(&mut self) -> ADC12IE20_W {
        ADC12IE20_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
interrupt enable 21"]
    #[inline(always)]
    pub fn adc12ie21(&mut self) -> ADC12IE21_W {
        ADC12IE21_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
interrupt enable 22"]
    #[inline(always)]
    pub fn adc12ie22(&mut self) -> ADC12IE22_W {
        ADC12IE22_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
interrupt enable 23"]
    #[inline(always)]
    pub fn adc12ie23(&mut self) -> ADC12IE23_W {
        ADC12IE23_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
interrupt enable 24"]
    #[inline(always)]
    pub fn adc12ie24(&mut self) -> ADC12IE24_W {
        ADC12IE24_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
interrupt enable 25"]
    #[inline(always)]
    pub fn adc12ie25(&mut self) -> ADC12IE25_W {
        ADC12IE25_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
interrupt enable 26"]
    #[inline(always)]
    pub fn adc12ie26(&mut self) -> ADC12IE26_W {
        ADC12IE26_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
interrupt enable 27"]
    #[inline(always)]
    pub fn adc12ie27(&mut self) -> ADC12IE27_W {
        ADC12IE27_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
interrupt enable 28"]
    #[inline(always)]
    pub fn adc12ie28(&mut self) -> ADC12IE28_W {
        ADC12IE28_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
interrupt enable 29"]
    #[inline(always)]
    pub fn adc12ie29(&mut self) -> ADC12IE29_W {
        ADC12IE29_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
interrupt enable 30"]
    #[inline(always)]
    pub fn adc12ie30(&mut self) -> ADC12IE30_W {
        ADC12IE30_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
interrupt enable 30"]
    #[inline(always)]
    pub fn adc12ie31(&mut self) -> ADC12IE31_W {
        ADC12IE31_W { w: self }
    }
}
