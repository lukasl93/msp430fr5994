#[doc = "Reader of register ADC12IER0"]
pub type R = crate::R<u16, super::ADC12IER0>;
#[doc = "Writer for register ADC12IER0"]
pub type W = crate::W<u16, super::ADC12IER0>;
#[doc = "Register ADC12IER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IER0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
Interrupt enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE0_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE0_1 = 1,
}
impl From<ADC12IE0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE0`"]
pub type ADC12IE0_R = crate::R<bool, ADC12IE0_A>;
impl ADC12IE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE0_A {
        match self.bits {
            false => ADC12IE0_A::ADC12IE0_0,
            true => ADC12IE0_A::ADC12IE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE0_0`"]
    #[inline(always)]
    pub fn is_adc12ie0_0(&self) -> bool {
        *self == ADC12IE0_A::ADC12IE0_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE0_1`"]
    #[inline(always)]
    pub fn is_adc12ie0_1(&self) -> bool {
        *self == ADC12IE0_A::ADC12IE0_1
    }
}
#[doc = "Write proxy for field `ADC12IE0`"]
pub struct ADC12IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie0_0(self) -> &'a mut W {
        self.variant(ADC12IE0_A::ADC12IE0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie0_1(self) -> &'a mut W {
        self.variant(ADC12IE0_A::ADC12IE0_1)
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
interrupt enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE1_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE1_1 = 1,
}
impl From<ADC12IE1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE1`"]
pub type ADC12IE1_R = crate::R<bool, ADC12IE1_A>;
impl ADC12IE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE1_A {
        match self.bits {
            false => ADC12IE1_A::ADC12IE1_0,
            true => ADC12IE1_A::ADC12IE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE1_0`"]
    #[inline(always)]
    pub fn is_adc12ie1_0(&self) -> bool {
        *self == ADC12IE1_A::ADC12IE1_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE1_1`"]
    #[inline(always)]
    pub fn is_adc12ie1_1(&self) -> bool {
        *self == ADC12IE1_A::ADC12IE1_1
    }
}
#[doc = "Write proxy for field `ADC12IE1`"]
pub struct ADC12IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie1_0(self) -> &'a mut W {
        self.variant(ADC12IE1_A::ADC12IE1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie1_1(self) -> &'a mut W {
        self.variant(ADC12IE1_A::ADC12IE1_1)
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
interrupt enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE2_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE2_1 = 1,
}
impl From<ADC12IE2_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE2`"]
pub type ADC12IE2_R = crate::R<bool, ADC12IE2_A>;
impl ADC12IE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE2_A {
        match self.bits {
            false => ADC12IE2_A::ADC12IE2_0,
            true => ADC12IE2_A::ADC12IE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE2_0`"]
    #[inline(always)]
    pub fn is_adc12ie2_0(&self) -> bool {
        *self == ADC12IE2_A::ADC12IE2_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE2_1`"]
    #[inline(always)]
    pub fn is_adc12ie2_1(&self) -> bool {
        *self == ADC12IE2_A::ADC12IE2_1
    }
}
#[doc = "Write proxy for field `ADC12IE2`"]
pub struct ADC12IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie2_0(self) -> &'a mut W {
        self.variant(ADC12IE2_A::ADC12IE2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie2_1(self) -> &'a mut W {
        self.variant(ADC12IE2_A::ADC12IE2_1)
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
interrupt enable 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE3_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE3_1 = 1,
}
impl From<ADC12IE3_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE3`"]
pub type ADC12IE3_R = crate::R<bool, ADC12IE3_A>;
impl ADC12IE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE3_A {
        match self.bits {
            false => ADC12IE3_A::ADC12IE3_0,
            true => ADC12IE3_A::ADC12IE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE3_0`"]
    #[inline(always)]
    pub fn is_adc12ie3_0(&self) -> bool {
        *self == ADC12IE3_A::ADC12IE3_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE3_1`"]
    #[inline(always)]
    pub fn is_adc12ie3_1(&self) -> bool {
        *self == ADC12IE3_A::ADC12IE3_1
    }
}
#[doc = "Write proxy for field `ADC12IE3`"]
pub struct ADC12IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie3_0(self) -> &'a mut W {
        self.variant(ADC12IE3_A::ADC12IE3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie3_1(self) -> &'a mut W {
        self.variant(ADC12IE3_A::ADC12IE3_1)
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
interrupt enable 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE4_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE4_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE4_1 = 1,
}
impl From<ADC12IE4_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE4`"]
pub type ADC12IE4_R = crate::R<bool, ADC12IE4_A>;
impl ADC12IE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE4_A {
        match self.bits {
            false => ADC12IE4_A::ADC12IE4_0,
            true => ADC12IE4_A::ADC12IE4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE4_0`"]
    #[inline(always)]
    pub fn is_adc12ie4_0(&self) -> bool {
        *self == ADC12IE4_A::ADC12IE4_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE4_1`"]
    #[inline(always)]
    pub fn is_adc12ie4_1(&self) -> bool {
        *self == ADC12IE4_A::ADC12IE4_1
    }
}
#[doc = "Write proxy for field `ADC12IE4`"]
pub struct ADC12IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie4_0(self) -> &'a mut W {
        self.variant(ADC12IE4_A::ADC12IE4_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie4_1(self) -> &'a mut W {
        self.variant(ADC12IE4_A::ADC12IE4_1)
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
interrupt enable 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE5_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE5_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE5_1 = 1,
}
impl From<ADC12IE5_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE5`"]
pub type ADC12IE5_R = crate::R<bool, ADC12IE5_A>;
impl ADC12IE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE5_A {
        match self.bits {
            false => ADC12IE5_A::ADC12IE5_0,
            true => ADC12IE5_A::ADC12IE5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE5_0`"]
    #[inline(always)]
    pub fn is_adc12ie5_0(&self) -> bool {
        *self == ADC12IE5_A::ADC12IE5_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE5_1`"]
    #[inline(always)]
    pub fn is_adc12ie5_1(&self) -> bool {
        *self == ADC12IE5_A::ADC12IE5_1
    }
}
#[doc = "Write proxy for field `ADC12IE5`"]
pub struct ADC12IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie5_0(self) -> &'a mut W {
        self.variant(ADC12IE5_A::ADC12IE5_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie5_1(self) -> &'a mut W {
        self.variant(ADC12IE5_A::ADC12IE5_1)
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
interrupt enable 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE6_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE6_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE6_1 = 1,
}
impl From<ADC12IE6_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE6`"]
pub type ADC12IE6_R = crate::R<bool, ADC12IE6_A>;
impl ADC12IE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE6_A {
        match self.bits {
            false => ADC12IE6_A::ADC12IE6_0,
            true => ADC12IE6_A::ADC12IE6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE6_0`"]
    #[inline(always)]
    pub fn is_adc12ie6_0(&self) -> bool {
        *self == ADC12IE6_A::ADC12IE6_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE6_1`"]
    #[inline(always)]
    pub fn is_adc12ie6_1(&self) -> bool {
        *self == ADC12IE6_A::ADC12IE6_1
    }
}
#[doc = "Write proxy for field `ADC12IE6`"]
pub struct ADC12IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie6_0(self) -> &'a mut W {
        self.variant(ADC12IE6_A::ADC12IE6_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie6_1(self) -> &'a mut W {
        self.variant(ADC12IE6_A::ADC12IE6_1)
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
interrupt enable 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE7_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE7_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE7_1 = 1,
}
impl From<ADC12IE7_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE7`"]
pub type ADC12IE7_R = crate::R<bool, ADC12IE7_A>;
impl ADC12IE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE7_A {
        match self.bits {
            false => ADC12IE7_A::ADC12IE7_0,
            true => ADC12IE7_A::ADC12IE7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE7_0`"]
    #[inline(always)]
    pub fn is_adc12ie7_0(&self) -> bool {
        *self == ADC12IE7_A::ADC12IE7_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE7_1`"]
    #[inline(always)]
    pub fn is_adc12ie7_1(&self) -> bool {
        *self == ADC12IE7_A::ADC12IE7_1
    }
}
#[doc = "Write proxy for field `ADC12IE7`"]
pub struct ADC12IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie7_0(self) -> &'a mut W {
        self.variant(ADC12IE7_A::ADC12IE7_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie7_1(self) -> &'a mut W {
        self.variant(ADC12IE7_A::ADC12IE7_1)
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
interrupt enable 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE8_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE8_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE8_1 = 1,
}
impl From<ADC12IE8_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE8`"]
pub type ADC12IE8_R = crate::R<bool, ADC12IE8_A>;
impl ADC12IE8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE8_A {
        match self.bits {
            false => ADC12IE8_A::ADC12IE8_0,
            true => ADC12IE8_A::ADC12IE8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE8_0`"]
    #[inline(always)]
    pub fn is_adc12ie8_0(&self) -> bool {
        *self == ADC12IE8_A::ADC12IE8_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE8_1`"]
    #[inline(always)]
    pub fn is_adc12ie8_1(&self) -> bool {
        *self == ADC12IE8_A::ADC12IE8_1
    }
}
#[doc = "Write proxy for field `ADC12IE8`"]
pub struct ADC12IE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie8_0(self) -> &'a mut W {
        self.variant(ADC12IE8_A::ADC12IE8_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie8_1(self) -> &'a mut W {
        self.variant(ADC12IE8_A::ADC12IE8_1)
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
interrupt enable 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE9_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE9_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE9_1 = 1,
}
impl From<ADC12IE9_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE9`"]
pub type ADC12IE9_R = crate::R<bool, ADC12IE9_A>;
impl ADC12IE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE9_A {
        match self.bits {
            false => ADC12IE9_A::ADC12IE9_0,
            true => ADC12IE9_A::ADC12IE9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE9_0`"]
    #[inline(always)]
    pub fn is_adc12ie9_0(&self) -> bool {
        *self == ADC12IE9_A::ADC12IE9_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE9_1`"]
    #[inline(always)]
    pub fn is_adc12ie9_1(&self) -> bool {
        *self == ADC12IE9_A::ADC12IE9_1
    }
}
#[doc = "Write proxy for field `ADC12IE9`"]
pub struct ADC12IE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie9_0(self) -> &'a mut W {
        self.variant(ADC12IE9_A::ADC12IE9_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie9_1(self) -> &'a mut W {
        self.variant(ADC12IE9_A::ADC12IE9_1)
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
interrupt enable 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE10_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE10_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE10_1 = 1,
}
impl From<ADC12IE10_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE10`"]
pub type ADC12IE10_R = crate::R<bool, ADC12IE10_A>;
impl ADC12IE10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE10_A {
        match self.bits {
            false => ADC12IE10_A::ADC12IE10_0,
            true => ADC12IE10_A::ADC12IE10_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE10_0`"]
    #[inline(always)]
    pub fn is_adc12ie10_0(&self) -> bool {
        *self == ADC12IE10_A::ADC12IE10_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE10_1`"]
    #[inline(always)]
    pub fn is_adc12ie10_1(&self) -> bool {
        *self == ADC12IE10_A::ADC12IE10_1
    }
}
#[doc = "Write proxy for field `ADC12IE10`"]
pub struct ADC12IE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie10_0(self) -> &'a mut W {
        self.variant(ADC12IE10_A::ADC12IE10_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie10_1(self) -> &'a mut W {
        self.variant(ADC12IE10_A::ADC12IE10_1)
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
interrupt enable 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE11_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE11_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE11_1 = 1,
}
impl From<ADC12IE11_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE11`"]
pub type ADC12IE11_R = crate::R<bool, ADC12IE11_A>;
impl ADC12IE11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE11_A {
        match self.bits {
            false => ADC12IE11_A::ADC12IE11_0,
            true => ADC12IE11_A::ADC12IE11_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE11_0`"]
    #[inline(always)]
    pub fn is_adc12ie11_0(&self) -> bool {
        *self == ADC12IE11_A::ADC12IE11_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE11_1`"]
    #[inline(always)]
    pub fn is_adc12ie11_1(&self) -> bool {
        *self == ADC12IE11_A::ADC12IE11_1
    }
}
#[doc = "Write proxy for field `ADC12IE11`"]
pub struct ADC12IE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie11_0(self) -> &'a mut W {
        self.variant(ADC12IE11_A::ADC12IE11_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie11_1(self) -> &'a mut W {
        self.variant(ADC12IE11_A::ADC12IE11_1)
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
interrupt enable 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE12_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE12_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE12_1 = 1,
}
impl From<ADC12IE12_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE12`"]
pub type ADC12IE12_R = crate::R<bool, ADC12IE12_A>;
impl ADC12IE12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE12_A {
        match self.bits {
            false => ADC12IE12_A::ADC12IE12_0,
            true => ADC12IE12_A::ADC12IE12_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE12_0`"]
    #[inline(always)]
    pub fn is_adc12ie12_0(&self) -> bool {
        *self == ADC12IE12_A::ADC12IE12_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE12_1`"]
    #[inline(always)]
    pub fn is_adc12ie12_1(&self) -> bool {
        *self == ADC12IE12_A::ADC12IE12_1
    }
}
#[doc = "Write proxy for field `ADC12IE12`"]
pub struct ADC12IE12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie12_0(self) -> &'a mut W {
        self.variant(ADC12IE12_A::ADC12IE12_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie12_1(self) -> &'a mut W {
        self.variant(ADC12IE12_A::ADC12IE12_1)
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
interrupt enable 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE13_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE13_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE13_1 = 1,
}
impl From<ADC12IE13_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE13`"]
pub type ADC12IE13_R = crate::R<bool, ADC12IE13_A>;
impl ADC12IE13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE13_A {
        match self.bits {
            false => ADC12IE13_A::ADC12IE13_0,
            true => ADC12IE13_A::ADC12IE13_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE13_0`"]
    #[inline(always)]
    pub fn is_adc12ie13_0(&self) -> bool {
        *self == ADC12IE13_A::ADC12IE13_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE13_1`"]
    #[inline(always)]
    pub fn is_adc12ie13_1(&self) -> bool {
        *self == ADC12IE13_A::ADC12IE13_1
    }
}
#[doc = "Write proxy for field `ADC12IE13`"]
pub struct ADC12IE13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie13_0(self) -> &'a mut W {
        self.variant(ADC12IE13_A::ADC12IE13_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie13_1(self) -> &'a mut W {
        self.variant(ADC12IE13_A::ADC12IE13_1)
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
interrupt enable 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE14_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE14_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE14_1 = 1,
}
impl From<ADC12IE14_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE14`"]
pub type ADC12IE14_R = crate::R<bool, ADC12IE14_A>;
impl ADC12IE14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE14_A {
        match self.bits {
            false => ADC12IE14_A::ADC12IE14_0,
            true => ADC12IE14_A::ADC12IE14_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE14_0`"]
    #[inline(always)]
    pub fn is_adc12ie14_0(&self) -> bool {
        *self == ADC12IE14_A::ADC12IE14_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE14_1`"]
    #[inline(always)]
    pub fn is_adc12ie14_1(&self) -> bool {
        *self == ADC12IE14_A::ADC12IE14_1
    }
}
#[doc = "Write proxy for field `ADC12IE14`"]
pub struct ADC12IE14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie14_0(self) -> &'a mut W {
        self.variant(ADC12IE14_A::ADC12IE14_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie14_1(self) -> &'a mut W {
        self.variant(ADC12IE14_A::ADC12IE14_1)
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
interrupt enable 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IE15_A {
    #[doc = "0: Interrupt disabled"]
    ADC12IE15_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12IE15_1 = 1,
}
impl From<ADC12IE15_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IE15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IE15`"]
pub type ADC12IE15_R = crate::R<bool, ADC12IE15_A>;
impl ADC12IE15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IE15_A {
        match self.bits {
            false => ADC12IE15_A::ADC12IE15_0,
            true => ADC12IE15_A::ADC12IE15_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IE15_0`"]
    #[inline(always)]
    pub fn is_adc12ie15_0(&self) -> bool {
        *self == ADC12IE15_A::ADC12IE15_0
    }
    #[doc = "Checks if the value of the field is `ADC12IE15_1`"]
    #[inline(always)]
    pub fn is_adc12ie15_1(&self) -> bool {
        *self == ADC12IE15_A::ADC12IE15_1
    }
}
#[doc = "Write proxy for field `ADC12IE15`"]
pub struct ADC12IE15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IE15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ie15_0(self) -> &'a mut W {
        self.variant(ADC12IE15_A::ADC12IE15_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ie15_1(self) -> &'a mut W {
        self.variant(ADC12IE15_A::ADC12IE15_1)
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
Interrupt enable 0"]
    #[inline(always)]
    pub fn adc12ie0(&self) -> ADC12IE0_R {
        ADC12IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable 1"]
    #[inline(always)]
    pub fn adc12ie1(&self) -> ADC12IE1_R {
        ADC12IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
interrupt enable 2"]
    #[inline(always)]
    pub fn adc12ie2(&self) -> ADC12IE2_R {
        ADC12IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
interrupt enable 3"]
    #[inline(always)]
    pub fn adc12ie3(&self) -> ADC12IE3_R {
        ADC12IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
interrupt enable 4"]
    #[inline(always)]
    pub fn adc12ie4(&self) -> ADC12IE4_R {
        ADC12IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
interrupt enable 5"]
    #[inline(always)]
    pub fn adc12ie5(&self) -> ADC12IE5_R {
        ADC12IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
interrupt enable 6"]
    #[inline(always)]
    pub fn adc12ie6(&self) -> ADC12IE6_R {
        ADC12IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
interrupt enable 7"]
    #[inline(always)]
    pub fn adc12ie7(&self) -> ADC12IE7_R {
        ADC12IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
interrupt enable 8"]
    #[inline(always)]
    pub fn adc12ie8(&self) -> ADC12IE8_R {
        ADC12IE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
interrupt enable 9"]
    #[inline(always)]
    pub fn adc12ie9(&self) -> ADC12IE9_R {
        ADC12IE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
interrupt enable 10"]
    #[inline(always)]
    pub fn adc12ie10(&self) -> ADC12IE10_R {
        ADC12IE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
interrupt enable 11"]
    #[inline(always)]
    pub fn adc12ie11(&self) -> ADC12IE11_R {
        ADC12IE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
interrupt enable 12"]
    #[inline(always)]
    pub fn adc12ie12(&self) -> ADC12IE12_R {
        ADC12IE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
interrupt enable 13"]
    #[inline(always)]
    pub fn adc12ie13(&self) -> ADC12IE13_R {
        ADC12IE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
interrupt enable 14"]
    #[inline(always)]
    pub fn adc12ie14(&self) -> ADC12IE14_R {
        ADC12IE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
interrupt enable 15"]
    #[inline(always)]
    pub fn adc12ie15(&self) -> ADC12IE15_R {
        ADC12IE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt enable 0"]
    #[inline(always)]
    pub fn adc12ie0(&mut self) -> ADC12IE0_W {
        ADC12IE0_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable 1"]
    #[inline(always)]
    pub fn adc12ie1(&mut self) -> ADC12IE1_W {
        ADC12IE1_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
interrupt enable 2"]
    #[inline(always)]
    pub fn adc12ie2(&mut self) -> ADC12IE2_W {
        ADC12IE2_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
interrupt enable 3"]
    #[inline(always)]
    pub fn adc12ie3(&mut self) -> ADC12IE3_W {
        ADC12IE3_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
interrupt enable 4"]
    #[inline(always)]
    pub fn adc12ie4(&mut self) -> ADC12IE4_W {
        ADC12IE4_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
interrupt enable 5"]
    #[inline(always)]
    pub fn adc12ie5(&mut self) -> ADC12IE5_W {
        ADC12IE5_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
interrupt enable 6"]
    #[inline(always)]
    pub fn adc12ie6(&mut self) -> ADC12IE6_W {
        ADC12IE6_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
interrupt enable 7"]
    #[inline(always)]
    pub fn adc12ie7(&mut self) -> ADC12IE7_W {
        ADC12IE7_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
interrupt enable 8"]
    #[inline(always)]
    pub fn adc12ie8(&mut self) -> ADC12IE8_W {
        ADC12IE8_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
interrupt enable 9"]
    #[inline(always)]
    pub fn adc12ie9(&mut self) -> ADC12IE9_W {
        ADC12IE9_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
interrupt enable 10"]
    #[inline(always)]
    pub fn adc12ie10(&mut self) -> ADC12IE10_W {
        ADC12IE10_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
interrupt enable 11"]
    #[inline(always)]
    pub fn adc12ie11(&mut self) -> ADC12IE11_W {
        ADC12IE11_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
interrupt enable 12"]
    #[inline(always)]
    pub fn adc12ie12(&mut self) -> ADC12IE12_W {
        ADC12IE12_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
interrupt enable 13"]
    #[inline(always)]
    pub fn adc12ie13(&mut self) -> ADC12IE13_W {
        ADC12IE13_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
interrupt enable 14"]
    #[inline(always)]
    pub fn adc12ie14(&mut self) -> ADC12IE14_W {
        ADC12IE14_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
interrupt enable 15"]
    #[inline(always)]
    pub fn adc12ie15(&mut self) -> ADC12IE15_W {
        ADC12IE15_W { w: self }
    }
}
