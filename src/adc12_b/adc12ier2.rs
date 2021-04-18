#[doc = "Reader of register ADC12IER2"]
pub type R = crate::R<u16, super::ADC12IER2>;
#[doc = "Writer for register ADC12IER2"]
pub type W = crate::W<u16, super::ADC12IER2>;
#[doc = "Register ADC12IER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IER2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "1:1\\]
interrupt enable MEMx between ADC12HI and LO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12INIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC12INIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12INIE_1 = 1,
}
impl From<ADC12INIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12INIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12INIE`"]
pub type ADC12INIE_R = crate::R<bool, ADC12INIE_A>;
impl ADC12INIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12INIE_A {
        match self.bits {
            false => ADC12INIE_A::ADC12INIE_0,
            true => ADC12INIE_A::ADC12INIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12INIE_0`"]
    #[inline(always)]
    pub fn is_adc12inie_0(&self) -> bool {
        *self == ADC12INIE_A::ADC12INIE_0
    }
    #[doc = "Checks if the value of the field is `ADC12INIE_1`"]
    #[inline(always)]
    pub fn is_adc12inie_1(&self) -> bool {
        *self == ADC12INIE_A::ADC12INIE_1
    }
}
#[doc = "Write proxy for field `ADC12INIE`"]
pub struct ADC12INIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12INIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12inie_0(self) -> &'a mut W {
        self.variant(ADC12INIE_A::ADC12INIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12inie_1(self) -> &'a mut W {
        self.variant(ADC12INIE_A::ADC12INIE_1)
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
interrupt enable MEMx ADC12LO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12LOIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC12LOIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12LOIE_1 = 1,
}
impl From<ADC12LOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12LOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12LOIE`"]
pub type ADC12LOIE_R = crate::R<bool, ADC12LOIE_A>;
impl ADC12LOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12LOIE_A {
        match self.bits {
            false => ADC12LOIE_A::ADC12LOIE_0,
            true => ADC12LOIE_A::ADC12LOIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12LOIE_0`"]
    #[inline(always)]
    pub fn is_adc12loie_0(&self) -> bool {
        *self == ADC12LOIE_A::ADC12LOIE_0
    }
    #[doc = "Checks if the value of the field is `ADC12LOIE_1`"]
    #[inline(always)]
    pub fn is_adc12loie_1(&self) -> bool {
        *self == ADC12LOIE_A::ADC12LOIE_1
    }
}
#[doc = "Write proxy for field `ADC12LOIE`"]
pub struct ADC12LOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12LOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12loie_0(self) -> &'a mut W {
        self.variant(ADC12LOIE_A::ADC12LOIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12loie_1(self) -> &'a mut W {
        self.variant(ADC12LOIE_A::ADC12LOIE_1)
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
interrupt enable MEMx ADC12HI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12HIIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC12HIIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12HIIE_1 = 1,
}
impl From<ADC12HIIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12HIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12HIIE`"]
pub type ADC12HIIE_R = crate::R<bool, ADC12HIIE_A>;
impl ADC12HIIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12HIIE_A {
        match self.bits {
            false => ADC12HIIE_A::ADC12HIIE_0,
            true => ADC12HIIE_A::ADC12HIIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12HIIE_0`"]
    #[inline(always)]
    pub fn is_adc12hiie_0(&self) -> bool {
        *self == ADC12HIIE_A::ADC12HIIE_0
    }
    #[doc = "Checks if the value of the field is `ADC12HIIE_1`"]
    #[inline(always)]
    pub fn is_adc12hiie_1(&self) -> bool {
        *self == ADC12HIIE_A::ADC12HIIE_1
    }
}
#[doc = "Write proxy for field `ADC12HIIE`"]
pub struct ADC12HIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12HIIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12HIIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12hiie_0(self) -> &'a mut W {
        self.variant(ADC12HIIE_A::ADC12HIIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12hiie_1(self) -> &'a mut W {
        self.variant(ADC12HIIE_A::ADC12HIIE_1)
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
ADC12MEMx overflow-interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12OVIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC12OVIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12OVIE_1 = 1,
}
impl From<ADC12OVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12OVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12OVIE`"]
pub type ADC12OVIE_R = crate::R<bool, ADC12OVIE_A>;
impl ADC12OVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12OVIE_A {
        match self.bits {
            false => ADC12OVIE_A::ADC12OVIE_0,
            true => ADC12OVIE_A::ADC12OVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12OVIE_0`"]
    #[inline(always)]
    pub fn is_adc12ovie_0(&self) -> bool {
        *self == ADC12OVIE_A::ADC12OVIE_0
    }
    #[doc = "Checks if the value of the field is `ADC12OVIE_1`"]
    #[inline(always)]
    pub fn is_adc12ovie_1(&self) -> bool {
        *self == ADC12OVIE_A::ADC12OVIE_1
    }
}
#[doc = "Write proxy for field `ADC12OVIE`"]
pub struct ADC12OVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12OVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12OVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12ovie_0(self) -> &'a mut W {
        self.variant(ADC12OVIE_A::ADC12OVIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12ovie_1(self) -> &'a mut W {
        self.variant(ADC12OVIE_A::ADC12OVIE_1)
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
conversion-time-overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12TOVIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC12TOVIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12TOVIE_1 = 1,
}
impl From<ADC12TOVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12TOVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12TOVIE`"]
pub type ADC12TOVIE_R = crate::R<bool, ADC12TOVIE_A>;
impl ADC12TOVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12TOVIE_A {
        match self.bits {
            false => ADC12TOVIE_A::ADC12TOVIE_0,
            true => ADC12TOVIE_A::ADC12TOVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12TOVIE_0`"]
    #[inline(always)]
    pub fn is_adc12tovie_0(&self) -> bool {
        *self == ADC12TOVIE_A::ADC12TOVIE_0
    }
    #[doc = "Checks if the value of the field is `ADC12TOVIE_1`"]
    #[inline(always)]
    pub fn is_adc12tovie_1(&self) -> bool {
        *self == ADC12TOVIE_A::ADC12TOVIE_1
    }
}
#[doc = "Write proxy for field `ADC12TOVIE`"]
pub struct ADC12TOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TOVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12TOVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12tovie_0(self) -> &'a mut W {
        self.variant(ADC12TOVIE_A::ADC12TOVIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12tovie_1(self) -> &'a mut W {
        self.variant(ADC12TOVIE_A::ADC12TOVIE_1)
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
interrupt enable ADC ref buffer ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12RDYIE_A {
    #[doc = "0: Interrupt disabled"]
    ADC12RDYIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC12RDYIE_1 = 1,
}
impl From<ADC12RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12RDYIE`"]
pub type ADC12RDYIE_R = crate::R<bool, ADC12RDYIE_A>;
impl ADC12RDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12RDYIE_A {
        match self.bits {
            false => ADC12RDYIE_A::ADC12RDYIE_0,
            true => ADC12RDYIE_A::ADC12RDYIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12RDYIE_0`"]
    #[inline(always)]
    pub fn is_adc12rdyie_0(&self) -> bool {
        *self == ADC12RDYIE_A::ADC12RDYIE_0
    }
    #[doc = "Checks if the value of the field is `ADC12RDYIE_1`"]
    #[inline(always)]
    pub fn is_adc12rdyie_1(&self) -> bool {
        *self == ADC12RDYIE_A::ADC12RDYIE_1
    }
}
#[doc = "Write proxy for field `ADC12RDYIE`"]
pub struct ADC12RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12RDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc12rdyie_0(self) -> &'a mut W {
        self.variant(ADC12RDYIE_A::ADC12RDYIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc12rdyie_1(self) -> &'a mut W {
        self.variant(ADC12RDYIE_A::ADC12RDYIE_1)
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
impl R {
    #[doc = "Bit 1 - 1:1\\]
interrupt enable MEMx between ADC12HI and LO"]
    #[inline(always)]
    pub fn adc12inie(&self) -> ADC12INIE_R {
        ADC12INIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
interrupt enable MEMx ADC12LO"]
    #[inline(always)]
    pub fn adc12loie(&self) -> ADC12LOIE_R {
        ADC12LOIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
interrupt enable MEMx ADC12HI"]
    #[inline(always)]
    pub fn adc12hiie(&self) -> ADC12HIIE_R {
        ADC12HIIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ADC12MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&self) -> ADC12OVIE_R {
        ADC12OVIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&self) -> ADC12TOVIE_R {
        ADC12TOVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
interrupt enable ADC ref buffer ready"]
    #[inline(always)]
    pub fn adc12rdyie(&self) -> ADC12RDYIE_R {
        ADC12RDYIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
interrupt enable MEMx between ADC12HI and LO"]
    #[inline(always)]
    pub fn adc12inie(&mut self) -> ADC12INIE_W {
        ADC12INIE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
interrupt enable MEMx ADC12LO"]
    #[inline(always)]
    pub fn adc12loie(&mut self) -> ADC12LOIE_W {
        ADC12LOIE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
interrupt enable MEMx ADC12HI"]
    #[inline(always)]
    pub fn adc12hiie(&mut self) -> ADC12HIIE_W {
        ADC12HIIE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
ADC12MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&mut self) -> ADC12OVIE_W {
        ADC12OVIE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&mut self) -> ADC12TOVIE_W {
        ADC12TOVIE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
interrupt enable ADC ref buffer ready"]
    #[inline(always)]
    pub fn adc12rdyie(&mut self) -> ADC12RDYIE_W {
        ADC12RDYIE_W { w: self }
    }
}
