#[doc = "Reader of register ADC12IFGR2"]
pub type R = crate::R<u16, super::ADC12IFGR2>;
#[doc = "Writer for register ADC12IFGR2"]
pub type W = crate::W<u16, super::ADC12IFGR2>;
#[doc = "Register ADC12IFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IFGR2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "1:1\\]
Interrupt flag for ADC12MEMx between ADC12HI and ADC12LO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12INIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC12INIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12INIFG_1 = 1,
}
impl From<ADC12INIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12INIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12INIFG`"]
pub type ADC12INIFG_R = crate::R<bool, ADC12INIFG_A>;
impl ADC12INIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12INIFG_A {
        match self.bits {
            false => ADC12INIFG_A::ADC12INIFG_0,
            true => ADC12INIFG_A::ADC12INIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12INIFG_0`"]
    #[inline(always)]
    pub fn is_adc12inifg_0(&self) -> bool {
        *self == ADC12INIFG_A::ADC12INIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC12INIFG_1`"]
    #[inline(always)]
    pub fn is_adc12inifg_1(&self) -> bool {
        *self == ADC12INIFG_A::ADC12INIFG_1
    }
}
#[doc = "Write proxy for field `ADC12INIFG`"]
pub struct ADC12INIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12INIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12inifg_0(self) -> &'a mut W {
        self.variant(ADC12INIFG_A::ADC12INIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12inifg_1(self) -> &'a mut W {
        self.variant(ADC12INIFG_A::ADC12INIFG_1)
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
Interrupt flag for ADC12MEMx ADC12LO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12LOIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC12LOIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12LOIFG_1 = 1,
}
impl From<ADC12LOIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12LOIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12LOIFG`"]
pub type ADC12LOIFG_R = crate::R<bool, ADC12LOIFG_A>;
impl ADC12LOIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12LOIFG_A {
        match self.bits {
            false => ADC12LOIFG_A::ADC12LOIFG_0,
            true => ADC12LOIFG_A::ADC12LOIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12LOIFG_0`"]
    #[inline(always)]
    pub fn is_adc12loifg_0(&self) -> bool {
        *self == ADC12LOIFG_A::ADC12LOIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC12LOIFG_1`"]
    #[inline(always)]
    pub fn is_adc12loifg_1(&self) -> bool {
        *self == ADC12LOIFG_A::ADC12LOIFG_1
    }
}
#[doc = "Write proxy for field `ADC12LOIFG`"]
pub struct ADC12LOIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LOIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12LOIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12loifg_0(self) -> &'a mut W {
        self.variant(ADC12LOIFG_A::ADC12LOIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12loifg_1(self) -> &'a mut W {
        self.variant(ADC12LOIFG_A::ADC12LOIFG_1)
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
Interrupt flag for ADC12MEMx ADC12HI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12HIIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC12HIIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12HIIFG_1 = 1,
}
impl From<ADC12HIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12HIIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12HIIFG`"]
pub type ADC12HIIFG_R = crate::R<bool, ADC12HIIFG_A>;
impl ADC12HIIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12HIIFG_A {
        match self.bits {
            false => ADC12HIIFG_A::ADC12HIIFG_0,
            true => ADC12HIIFG_A::ADC12HIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12HIIFG_0`"]
    #[inline(always)]
    pub fn is_adc12hiifg_0(&self) -> bool {
        *self == ADC12HIIFG_A::ADC12HIIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC12HIIFG_1`"]
    #[inline(always)]
    pub fn is_adc12hiifg_1(&self) -> bool {
        *self == ADC12HIIFG_A::ADC12HIIFG_1
    }
}
#[doc = "Write proxy for field `ADC12HIIFG`"]
pub struct ADC12HIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12HIIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12HIIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12hiifg_0(self) -> &'a mut W {
        self.variant(ADC12HIIFG_A::ADC12HIIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12hiifg_1(self) -> &'a mut W {
        self.variant(ADC12HIIFG_A::ADC12HIIFG_1)
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
ADC12MEMx overflow-interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12OVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC12OVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12OVIFG_1 = 1,
}
impl From<ADC12OVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12OVIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12OVIFG`"]
pub type ADC12OVIFG_R = crate::R<bool, ADC12OVIFG_A>;
impl ADC12OVIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12OVIFG_A {
        match self.bits {
            false => ADC12OVIFG_A::ADC12OVIFG_0,
            true => ADC12OVIFG_A::ADC12OVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12OVIFG_0`"]
    #[inline(always)]
    pub fn is_adc12ovifg_0(&self) -> bool {
        *self == ADC12OVIFG_A::ADC12OVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC12OVIFG_1`"]
    #[inline(always)]
    pub fn is_adc12ovifg_1(&self) -> bool {
        *self == ADC12OVIFG_A::ADC12OVIFG_1
    }
}
#[doc = "Write proxy for field `ADC12OVIFG`"]
pub struct ADC12OVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12OVIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12OVIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12ovifg_0(self) -> &'a mut W {
        self.variant(ADC12OVIFG_A::ADC12OVIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12ovifg_1(self) -> &'a mut W {
        self.variant(ADC12OVIFG_A::ADC12OVIFG_1)
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
conversion-time-overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12TOVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC12TOVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12TOVIFG_1 = 1,
}
impl From<ADC12TOVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12TOVIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12TOVIFG`"]
pub type ADC12TOVIFG_R = crate::R<bool, ADC12TOVIFG_A>;
impl ADC12TOVIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12TOVIFG_A {
        match self.bits {
            false => ADC12TOVIFG_A::ADC12TOVIFG_0,
            true => ADC12TOVIFG_A::ADC12TOVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12TOVIFG_0`"]
    #[inline(always)]
    pub fn is_adc12tovifg_0(&self) -> bool {
        *self == ADC12TOVIFG_A::ADC12TOVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC12TOVIFG_1`"]
    #[inline(always)]
    pub fn is_adc12tovifg_1(&self) -> bool {
        *self == ADC12TOVIFG_A::ADC12TOVIFG_1
    }
}
#[doc = "Write proxy for field `ADC12TOVIFG`"]
pub struct ADC12TOVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TOVIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12TOVIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12tovifg_0(self) -> &'a mut W {
        self.variant(ADC12TOVIFG_A::ADC12TOVIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12tovifg_1(self) -> &'a mut W {
        self.variant(ADC12TOVIFG_A::ADC12TOVIFG_1)
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
reference buffer ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12RDYIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC12RDYIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC12RDYIFG_1 = 1,
}
impl From<ADC12RDYIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12RDYIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12RDYIFG`"]
pub type ADC12RDYIFG_R = crate::R<bool, ADC12RDYIFG_A>;
impl ADC12RDYIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12RDYIFG_A {
        match self.bits {
            false => ADC12RDYIFG_A::ADC12RDYIFG_0,
            true => ADC12RDYIFG_A::ADC12RDYIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12RDYIFG_0`"]
    #[inline(always)]
    pub fn is_adc12rdyifg_0(&self) -> bool {
        *self == ADC12RDYIFG_A::ADC12RDYIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC12RDYIFG_1`"]
    #[inline(always)]
    pub fn is_adc12rdyifg_1(&self) -> bool {
        *self == ADC12RDYIFG_A::ADC12RDYIFG_1
    }
}
#[doc = "Write proxy for field `ADC12RDYIFG`"]
pub struct ADC12RDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RDYIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12RDYIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc12rdyifg_0(self) -> &'a mut W {
        self.variant(ADC12RDYIFG_A::ADC12RDYIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adc12rdyifg_1(self) -> &'a mut W {
        self.variant(ADC12RDYIFG_A::ADC12RDYIFG_1)
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
Interrupt flag for ADC12MEMx between ADC12HI and ADC12LO"]
    #[inline(always)]
    pub fn adc12inifg(&self) -> ADC12INIFG_R {
        ADC12INIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt flag for ADC12MEMx ADC12LO"]
    #[inline(always)]
    pub fn adc12loifg(&self) -> ADC12LOIFG_R {
        ADC12LOIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt flag for ADC12MEMx ADC12HI"]
    #[inline(always)]
    pub fn adc12hiifg(&self) -> ADC12HIIFG_R {
        ADC12HIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ADC12MEMx overflow-interrupt flag."]
    #[inline(always)]
    pub fn adc12ovifg(&self) -> ADC12OVIFG_R {
        ADC12OVIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
conversion-time-overflow interrupt flag"]
    #[inline(always)]
    pub fn adc12tovifg(&self) -> ADC12TOVIFG_R {
        ADC12TOVIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
reference buffer ready interrupt flag"]
    #[inline(always)]
    pub fn adc12rdyifg(&self) -> ADC12RDYIFG_R {
        ADC12RDYIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Interrupt flag for ADC12MEMx between ADC12HI and ADC12LO"]
    #[inline(always)]
    pub fn adc12inifg(&mut self) -> ADC12INIFG_W {
        ADC12INIFG_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt flag for ADC12MEMx ADC12LO"]
    #[inline(always)]
    pub fn adc12loifg(&mut self) -> ADC12LOIFG_W {
        ADC12LOIFG_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt flag for ADC12MEMx ADC12HI"]
    #[inline(always)]
    pub fn adc12hiifg(&mut self) -> ADC12HIIFG_W {
        ADC12HIIFG_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
ADC12MEMx overflow-interrupt flag."]
    #[inline(always)]
    pub fn adc12ovifg(&mut self) -> ADC12OVIFG_W {
        ADC12OVIFG_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
conversion-time-overflow interrupt flag"]
    #[inline(always)]
    pub fn adc12tovifg(&mut self) -> ADC12TOVIFG_W {
        ADC12TOVIFG_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
reference buffer ready interrupt flag"]
    #[inline(always)]
    pub fn adc12rdyifg(&mut self) -> ADC12RDYIFG_W {
        ADC12RDYIFG_W { w: self }
    }
}
