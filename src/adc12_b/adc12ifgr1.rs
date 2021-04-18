#[doc = "Reader of register ADC12IFGR1"]
pub type R = crate::R<u16, super::ADC12IFGR1>;
#[doc = "Writer for register ADC12IFGR1"]
pub type W = crate::W<u16, super::ADC12IFGR1>;
#[doc = "Register ADC12IFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IFGR1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
ADC12MEM16 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG16_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG16_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG16_1 = 1,
}
impl From<ADC12IFG16_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG16`"]
pub type ADC12IFG16_R = crate::R<bool, ADC12IFG16_A>;
impl ADC12IFG16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG16_A {
        match self.bits {
            false => ADC12IFG16_A::ADC12IFG16_0,
            true => ADC12IFG16_A::ADC12IFG16_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG16_0`"]
    #[inline(always)]
    pub fn is_adc12ifg16_0(&self) -> bool {
        *self == ADC12IFG16_A::ADC12IFG16_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG16_1`"]
    #[inline(always)]
    pub fn is_adc12ifg16_1(&self) -> bool {
        *self == ADC12IFG16_A::ADC12IFG16_1
    }
}
#[doc = "Write proxy for field `ADC12IFG16`"]
pub struct ADC12IFG16_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg16_0(self) -> &'a mut W {
        self.variant(ADC12IFG16_A::ADC12IFG16_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg16_1(self) -> &'a mut W {
        self.variant(ADC12IFG16_A::ADC12IFG16_1)
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
ADC12MEM17 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG17_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG17_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG17_1 = 1,
}
impl From<ADC12IFG17_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG17`"]
pub type ADC12IFG17_R = crate::R<bool, ADC12IFG17_A>;
impl ADC12IFG17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG17_A {
        match self.bits {
            false => ADC12IFG17_A::ADC12IFG17_0,
            true => ADC12IFG17_A::ADC12IFG17_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG17_0`"]
    #[inline(always)]
    pub fn is_adc12ifg17_0(&self) -> bool {
        *self == ADC12IFG17_A::ADC12IFG17_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG17_1`"]
    #[inline(always)]
    pub fn is_adc12ifg17_1(&self) -> bool {
        *self == ADC12IFG17_A::ADC12IFG17_1
    }
}
#[doc = "Write proxy for field `ADC12IFG17`"]
pub struct ADC12IFG17_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg17_0(self) -> &'a mut W {
        self.variant(ADC12IFG17_A::ADC12IFG17_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg17_1(self) -> &'a mut W {
        self.variant(ADC12IFG17_A::ADC12IFG17_1)
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
ADC12MEM18 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG18_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG18_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG18_1 = 1,
}
impl From<ADC12IFG18_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG18`"]
pub type ADC12IFG18_R = crate::R<bool, ADC12IFG18_A>;
impl ADC12IFG18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG18_A {
        match self.bits {
            false => ADC12IFG18_A::ADC12IFG18_0,
            true => ADC12IFG18_A::ADC12IFG18_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG18_0`"]
    #[inline(always)]
    pub fn is_adc12ifg18_0(&self) -> bool {
        *self == ADC12IFG18_A::ADC12IFG18_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG18_1`"]
    #[inline(always)]
    pub fn is_adc12ifg18_1(&self) -> bool {
        *self == ADC12IFG18_A::ADC12IFG18_1
    }
}
#[doc = "Write proxy for field `ADC12IFG18`"]
pub struct ADC12IFG18_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg18_0(self) -> &'a mut W {
        self.variant(ADC12IFG18_A::ADC12IFG18_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg18_1(self) -> &'a mut W {
        self.variant(ADC12IFG18_A::ADC12IFG18_1)
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
ADC12MEM19 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG19_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG19_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG19_1 = 1,
}
impl From<ADC12IFG19_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG19`"]
pub type ADC12IFG19_R = crate::R<bool, ADC12IFG19_A>;
impl ADC12IFG19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG19_A {
        match self.bits {
            false => ADC12IFG19_A::ADC12IFG19_0,
            true => ADC12IFG19_A::ADC12IFG19_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG19_0`"]
    #[inline(always)]
    pub fn is_adc12ifg19_0(&self) -> bool {
        *self == ADC12IFG19_A::ADC12IFG19_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG19_1`"]
    #[inline(always)]
    pub fn is_adc12ifg19_1(&self) -> bool {
        *self == ADC12IFG19_A::ADC12IFG19_1
    }
}
#[doc = "Write proxy for field `ADC12IFG19`"]
pub struct ADC12IFG19_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg19_0(self) -> &'a mut W {
        self.variant(ADC12IFG19_A::ADC12IFG19_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg19_1(self) -> &'a mut W {
        self.variant(ADC12IFG19_A::ADC12IFG19_1)
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
ADC12MEM20 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG20_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG20_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG20_1 = 1,
}
impl From<ADC12IFG20_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG20`"]
pub type ADC12IFG20_R = crate::R<bool, ADC12IFG20_A>;
impl ADC12IFG20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG20_A {
        match self.bits {
            false => ADC12IFG20_A::ADC12IFG20_0,
            true => ADC12IFG20_A::ADC12IFG20_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG20_0`"]
    #[inline(always)]
    pub fn is_adc12ifg20_0(&self) -> bool {
        *self == ADC12IFG20_A::ADC12IFG20_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG20_1`"]
    #[inline(always)]
    pub fn is_adc12ifg20_1(&self) -> bool {
        *self == ADC12IFG20_A::ADC12IFG20_1
    }
}
#[doc = "Write proxy for field `ADC12IFG20`"]
pub struct ADC12IFG20_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg20_0(self) -> &'a mut W {
        self.variant(ADC12IFG20_A::ADC12IFG20_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg20_1(self) -> &'a mut W {
        self.variant(ADC12IFG20_A::ADC12IFG20_1)
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
ADC12MEM21 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG21_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG21_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG21_1 = 1,
}
impl From<ADC12IFG21_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG21`"]
pub type ADC12IFG21_R = crate::R<bool, ADC12IFG21_A>;
impl ADC12IFG21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG21_A {
        match self.bits {
            false => ADC12IFG21_A::ADC12IFG21_0,
            true => ADC12IFG21_A::ADC12IFG21_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG21_0`"]
    #[inline(always)]
    pub fn is_adc12ifg21_0(&self) -> bool {
        *self == ADC12IFG21_A::ADC12IFG21_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG21_1`"]
    #[inline(always)]
    pub fn is_adc12ifg21_1(&self) -> bool {
        *self == ADC12IFG21_A::ADC12IFG21_1
    }
}
#[doc = "Write proxy for field `ADC12IFG21`"]
pub struct ADC12IFG21_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg21_0(self) -> &'a mut W {
        self.variant(ADC12IFG21_A::ADC12IFG21_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg21_1(self) -> &'a mut W {
        self.variant(ADC12IFG21_A::ADC12IFG21_1)
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
ADC12MEM22 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG22_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG22_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG22_1 = 1,
}
impl From<ADC12IFG22_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG22`"]
pub type ADC12IFG22_R = crate::R<bool, ADC12IFG22_A>;
impl ADC12IFG22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG22_A {
        match self.bits {
            false => ADC12IFG22_A::ADC12IFG22_0,
            true => ADC12IFG22_A::ADC12IFG22_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG22_0`"]
    #[inline(always)]
    pub fn is_adc12ifg22_0(&self) -> bool {
        *self == ADC12IFG22_A::ADC12IFG22_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG22_1`"]
    #[inline(always)]
    pub fn is_adc12ifg22_1(&self) -> bool {
        *self == ADC12IFG22_A::ADC12IFG22_1
    }
}
#[doc = "Write proxy for field `ADC12IFG22`"]
pub struct ADC12IFG22_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg22_0(self) -> &'a mut W {
        self.variant(ADC12IFG22_A::ADC12IFG22_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg22_1(self) -> &'a mut W {
        self.variant(ADC12IFG22_A::ADC12IFG22_1)
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
ADC12MEM23 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG23_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG23_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG23_1 = 1,
}
impl From<ADC12IFG23_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG23`"]
pub type ADC12IFG23_R = crate::R<bool, ADC12IFG23_A>;
impl ADC12IFG23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG23_A {
        match self.bits {
            false => ADC12IFG23_A::ADC12IFG23_0,
            true => ADC12IFG23_A::ADC12IFG23_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG23_0`"]
    #[inline(always)]
    pub fn is_adc12ifg23_0(&self) -> bool {
        *self == ADC12IFG23_A::ADC12IFG23_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG23_1`"]
    #[inline(always)]
    pub fn is_adc12ifg23_1(&self) -> bool {
        *self == ADC12IFG23_A::ADC12IFG23_1
    }
}
#[doc = "Write proxy for field `ADC12IFG23`"]
pub struct ADC12IFG23_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg23_0(self) -> &'a mut W {
        self.variant(ADC12IFG23_A::ADC12IFG23_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg23_1(self) -> &'a mut W {
        self.variant(ADC12IFG23_A::ADC12IFG23_1)
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
ADC12MEM24 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG24_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG24_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG24_1 = 1,
}
impl From<ADC12IFG24_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG24`"]
pub type ADC12IFG24_R = crate::R<bool, ADC12IFG24_A>;
impl ADC12IFG24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG24_A {
        match self.bits {
            false => ADC12IFG24_A::ADC12IFG24_0,
            true => ADC12IFG24_A::ADC12IFG24_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG24_0`"]
    #[inline(always)]
    pub fn is_adc12ifg24_0(&self) -> bool {
        *self == ADC12IFG24_A::ADC12IFG24_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG24_1`"]
    #[inline(always)]
    pub fn is_adc12ifg24_1(&self) -> bool {
        *self == ADC12IFG24_A::ADC12IFG24_1
    }
}
#[doc = "Write proxy for field `ADC12IFG24`"]
pub struct ADC12IFG24_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg24_0(self) -> &'a mut W {
        self.variant(ADC12IFG24_A::ADC12IFG24_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg24_1(self) -> &'a mut W {
        self.variant(ADC12IFG24_A::ADC12IFG24_1)
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
ADC12MEM25 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG25_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG25_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG25_1 = 1,
}
impl From<ADC12IFG25_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG25`"]
pub type ADC12IFG25_R = crate::R<bool, ADC12IFG25_A>;
impl ADC12IFG25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG25_A {
        match self.bits {
            false => ADC12IFG25_A::ADC12IFG25_0,
            true => ADC12IFG25_A::ADC12IFG25_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG25_0`"]
    #[inline(always)]
    pub fn is_adc12ifg25_0(&self) -> bool {
        *self == ADC12IFG25_A::ADC12IFG25_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG25_1`"]
    #[inline(always)]
    pub fn is_adc12ifg25_1(&self) -> bool {
        *self == ADC12IFG25_A::ADC12IFG25_1
    }
}
#[doc = "Write proxy for field `ADC12IFG25`"]
pub struct ADC12IFG25_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg25_0(self) -> &'a mut W {
        self.variant(ADC12IFG25_A::ADC12IFG25_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg25_1(self) -> &'a mut W {
        self.variant(ADC12IFG25_A::ADC12IFG25_1)
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
ADC12MEM26 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG26_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG26_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG26_1 = 1,
}
impl From<ADC12IFG26_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG26`"]
pub type ADC12IFG26_R = crate::R<bool, ADC12IFG26_A>;
impl ADC12IFG26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG26_A {
        match self.bits {
            false => ADC12IFG26_A::ADC12IFG26_0,
            true => ADC12IFG26_A::ADC12IFG26_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG26_0`"]
    #[inline(always)]
    pub fn is_adc12ifg26_0(&self) -> bool {
        *self == ADC12IFG26_A::ADC12IFG26_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG26_1`"]
    #[inline(always)]
    pub fn is_adc12ifg26_1(&self) -> bool {
        *self == ADC12IFG26_A::ADC12IFG26_1
    }
}
#[doc = "Write proxy for field `ADC12IFG26`"]
pub struct ADC12IFG26_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg26_0(self) -> &'a mut W {
        self.variant(ADC12IFG26_A::ADC12IFG26_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg26_1(self) -> &'a mut W {
        self.variant(ADC12IFG26_A::ADC12IFG26_1)
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
ADC12MEM27 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG27_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG27_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG27_1 = 1,
}
impl From<ADC12IFG27_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG27`"]
pub type ADC12IFG27_R = crate::R<bool, ADC12IFG27_A>;
impl ADC12IFG27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG27_A {
        match self.bits {
            false => ADC12IFG27_A::ADC12IFG27_0,
            true => ADC12IFG27_A::ADC12IFG27_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG27_0`"]
    #[inline(always)]
    pub fn is_adc12ifg27_0(&self) -> bool {
        *self == ADC12IFG27_A::ADC12IFG27_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG27_1`"]
    #[inline(always)]
    pub fn is_adc12ifg27_1(&self) -> bool {
        *self == ADC12IFG27_A::ADC12IFG27_1
    }
}
#[doc = "Write proxy for field `ADC12IFG27`"]
pub struct ADC12IFG27_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg27_0(self) -> &'a mut W {
        self.variant(ADC12IFG27_A::ADC12IFG27_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg27_1(self) -> &'a mut W {
        self.variant(ADC12IFG27_A::ADC12IFG27_1)
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
ADC12MEM28 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG28_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG28_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG28_1 = 1,
}
impl From<ADC12IFG28_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG28`"]
pub type ADC12IFG28_R = crate::R<bool, ADC12IFG28_A>;
impl ADC12IFG28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG28_A {
        match self.bits {
            false => ADC12IFG28_A::ADC12IFG28_0,
            true => ADC12IFG28_A::ADC12IFG28_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG28_0`"]
    #[inline(always)]
    pub fn is_adc12ifg28_0(&self) -> bool {
        *self == ADC12IFG28_A::ADC12IFG28_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG28_1`"]
    #[inline(always)]
    pub fn is_adc12ifg28_1(&self) -> bool {
        *self == ADC12IFG28_A::ADC12IFG28_1
    }
}
#[doc = "Write proxy for field `ADC12IFG28`"]
pub struct ADC12IFG28_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg28_0(self) -> &'a mut W {
        self.variant(ADC12IFG28_A::ADC12IFG28_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg28_1(self) -> &'a mut W {
        self.variant(ADC12IFG28_A::ADC12IFG28_1)
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
ADC12MEM29 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG29_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG29_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG29_1 = 1,
}
impl From<ADC12IFG29_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG29`"]
pub type ADC12IFG29_R = crate::R<bool, ADC12IFG29_A>;
impl ADC12IFG29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG29_A {
        match self.bits {
            false => ADC12IFG29_A::ADC12IFG29_0,
            true => ADC12IFG29_A::ADC12IFG29_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG29_0`"]
    #[inline(always)]
    pub fn is_adc12ifg29_0(&self) -> bool {
        *self == ADC12IFG29_A::ADC12IFG29_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG29_1`"]
    #[inline(always)]
    pub fn is_adc12ifg29_1(&self) -> bool {
        *self == ADC12IFG29_A::ADC12IFG29_1
    }
}
#[doc = "Write proxy for field `ADC12IFG29`"]
pub struct ADC12IFG29_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg29_0(self) -> &'a mut W {
        self.variant(ADC12IFG29_A::ADC12IFG29_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg29_1(self) -> &'a mut W {
        self.variant(ADC12IFG29_A::ADC12IFG29_1)
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
ADC12MEM30 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG30_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG30_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG30_1 = 1,
}
impl From<ADC12IFG30_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG30`"]
pub type ADC12IFG30_R = crate::R<bool, ADC12IFG30_A>;
impl ADC12IFG30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG30_A {
        match self.bits {
            false => ADC12IFG30_A::ADC12IFG30_0,
            true => ADC12IFG30_A::ADC12IFG30_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG30_0`"]
    #[inline(always)]
    pub fn is_adc12ifg30_0(&self) -> bool {
        *self == ADC12IFG30_A::ADC12IFG30_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG30_1`"]
    #[inline(always)]
    pub fn is_adc12ifg30_1(&self) -> bool {
        *self == ADC12IFG30_A::ADC12IFG30_1
    }
}
#[doc = "Write proxy for field `ADC12IFG30`"]
pub struct ADC12IFG30_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg30_0(self) -> &'a mut W {
        self.variant(ADC12IFG30_A::ADC12IFG30_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg30_1(self) -> &'a mut W {
        self.variant(ADC12IFG30_A::ADC12IFG30_1)
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
ADC12MEM31 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12IFG31_A {
    #[doc = "0: No interrupt pending"]
    ADC12IFG31_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12IFG31_1 = 1,
}
impl From<ADC12IFG31_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12IFG31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12IFG31`"]
pub type ADC12IFG31_R = crate::R<bool, ADC12IFG31_A>;
impl ADC12IFG31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12IFG31_A {
        match self.bits {
            false => ADC12IFG31_A::ADC12IFG31_0,
            true => ADC12IFG31_A::ADC12IFG31_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12IFG31_0`"]
    #[inline(always)]
    pub fn is_adc12ifg31_0(&self) -> bool {
        *self == ADC12IFG31_A::ADC12IFG31_0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG31_1`"]
    #[inline(always)]
    pub fn is_adc12ifg31_1(&self) -> bool {
        *self == ADC12IFG31_A::ADC12IFG31_1
    }
}
#[doc = "Write proxy for field `ADC12IFG31`"]
pub struct ADC12IFG31_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IFG31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg31_0(self) -> &'a mut W {
        self.variant(ADC12IFG31_A::ADC12IFG31_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ifg31_1(self) -> &'a mut W {
        self.variant(ADC12IFG31_A::ADC12IFG31_1)
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
ADC12MEM16 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg16(&self) -> ADC12IFG16_R {
        ADC12IFG16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ADC12MEM17 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg17(&self) -> ADC12IFG17_R {
        ADC12IFG17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
ADC12MEM18 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg18(&self) -> ADC12IFG18_R {
        ADC12IFG18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
ADC12MEM19 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg19(&self) -> ADC12IFG19_R {
        ADC12IFG19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ADC12MEM20 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg20(&self) -> ADC12IFG20_R {
        ADC12IFG20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
ADC12MEM21 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg21(&self) -> ADC12IFG21_R {
        ADC12IFG21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
ADC12MEM22 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg22(&self) -> ADC12IFG22_R {
        ADC12IFG22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
ADC12MEM23 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg23(&self) -> ADC12IFG23_R {
        ADC12IFG23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
ADC12MEM24 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg24(&self) -> ADC12IFG24_R {
        ADC12IFG24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
ADC12MEM25 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg25(&self) -> ADC12IFG25_R {
        ADC12IFG25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
ADC12MEM26 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg26(&self) -> ADC12IFG26_R {
        ADC12IFG26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
ADC12MEM27 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg27(&self) -> ADC12IFG27_R {
        ADC12IFG27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
ADC12MEM28 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg28(&self) -> ADC12IFG28_R {
        ADC12IFG28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
ADC12MEM29 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg29(&self) -> ADC12IFG29_R {
        ADC12IFG29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
ADC12MEM30 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg30(&self) -> ADC12IFG30_R {
        ADC12IFG30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
ADC12MEM31 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg31(&self) -> ADC12IFG31_R {
        ADC12IFG31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ADC12MEM16 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg16(&mut self) -> ADC12IFG16_W {
        ADC12IFG16_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
ADC12MEM17 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg17(&mut self) -> ADC12IFG17_W {
        ADC12IFG17_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
ADC12MEM18 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg18(&mut self) -> ADC12IFG18_W {
        ADC12IFG18_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
ADC12MEM19 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg19(&mut self) -> ADC12IFG19_W {
        ADC12IFG19_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
ADC12MEM20 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg20(&mut self) -> ADC12IFG20_W {
        ADC12IFG20_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
ADC12MEM21 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg21(&mut self) -> ADC12IFG21_W {
        ADC12IFG21_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
ADC12MEM22 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg22(&mut self) -> ADC12IFG22_W {
        ADC12IFG22_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
ADC12MEM23 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg23(&mut self) -> ADC12IFG23_W {
        ADC12IFG23_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
ADC12MEM24 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg24(&mut self) -> ADC12IFG24_W {
        ADC12IFG24_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
ADC12MEM25 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg25(&mut self) -> ADC12IFG25_W {
        ADC12IFG25_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
ADC12MEM26 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg26(&mut self) -> ADC12IFG26_W {
        ADC12IFG26_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
ADC12MEM27 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg27(&mut self) -> ADC12IFG27_W {
        ADC12IFG27_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
ADC12MEM28 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg28(&mut self) -> ADC12IFG28_W {
        ADC12IFG28_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
ADC12MEM29 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg29(&mut self) -> ADC12IFG29_W {
        ADC12IFG29_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
ADC12MEM30 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg30(&mut self) -> ADC12IFG30_W {
        ADC12IFG30_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
ADC12MEM31 interrupt flag"]
    #[inline(always)]
    pub fn adc12ifg31(&mut self) -> ADC12IFG31_W {
        ADC12IFG31_W { w: self }
    }
}
