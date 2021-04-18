#[doc = "Reader of register ADC12CTL2"]
pub type R = crate::R<u16, super::ADC12CTL2>;
#[doc = "Writer for register ADC12CTL2"]
pub type W = crate::W<u16, super::ADC12CTL2>;
#[doc = "Register ADC12CTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12CTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12PWRMD_A {
    #[doc = "0: Regular power mode where sample rate is not restricted"]
    ADC12PWRMD_0 = 0,
    #[doc = "1: Low power mode enable, ADC12CLK can not be greater than 1/4 the device-specific data sheet specified maximum for ADC12PWRMD = 0"]
    ADC12PWRMD_1 = 1,
}
impl From<ADC12PWRMD_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12PWRMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12PWRMD`"]
pub type ADC12PWRMD_R = crate::R<bool, ADC12PWRMD_A>;
impl ADC12PWRMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12PWRMD_A {
        match self.bits {
            false => ADC12PWRMD_A::ADC12PWRMD_0,
            true => ADC12PWRMD_A::ADC12PWRMD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12PWRMD_0`"]
    #[inline(always)]
    pub fn is_adc12pwrmd_0(&self) -> bool {
        *self == ADC12PWRMD_A::ADC12PWRMD_0
    }
    #[doc = "Checks if the value of the field is `ADC12PWRMD_1`"]
    #[inline(always)]
    pub fn is_adc12pwrmd_1(&self) -> bool {
        *self == ADC12PWRMD_A::ADC12PWRMD_1
    }
}
#[doc = "Write proxy for field `ADC12PWRMD`"]
pub struct ADC12PWRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12PWRMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12PWRMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Regular power mode where sample rate is not restricted"]
    #[inline(always)]
    pub fn adc12pwrmd_0(self) -> &'a mut W {
        self.variant(ADC12PWRMD_A::ADC12PWRMD_0)
    }
    #[doc = "Low power mode enable, ADC12CLK can not be greater than 1/4 the device-specific data sheet specified maximum for ADC12PWRMD = 0"]
    #[inline(always)]
    pub fn adc12pwrmd_1(self) -> &'a mut W {
        self.variant(ADC12PWRMD_A::ADC12PWRMD_1)
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
#[doc = "3:3\\]
data read-back format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12DF_A {
    #[doc = "0: Binary unsigned. Theoretically for ADC12DIF = 0 and 12-bit mode the analog input voltage  VREF results in 0000h, the analog input voltage + VREF results in 0FFFh."]
    ADC12DF_0 = 0,
    #[doc = "1: Signed binary (2s complement), left aligned. Theoretically, for ADC12DIF = 0 and 12-bit mode, the analog input voltage  VREF results in 8000h, the analog input voltage + VREF results in 7FF0h."]
    ADC12DF_1 = 1,
}
impl From<ADC12DF_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12DF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12DF`"]
pub type ADC12DF_R = crate::R<bool, ADC12DF_A>;
impl ADC12DF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12DF_A {
        match self.bits {
            false => ADC12DF_A::ADC12DF_0,
            true => ADC12DF_A::ADC12DF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12DF_0`"]
    #[inline(always)]
    pub fn is_adc12df_0(&self) -> bool {
        *self == ADC12DF_A::ADC12DF_0
    }
    #[doc = "Checks if the value of the field is `ADC12DF_1`"]
    #[inline(always)]
    pub fn is_adc12df_1(&self) -> bool {
        *self == ADC12DF_A::ADC12DF_1
    }
}
#[doc = "Write proxy for field `ADC12DF`"]
pub struct ADC12DF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12DF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12DF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Binary unsigned. Theoretically for ADC12DIF = 0 and 12-bit mode the analog input voltage VREF results in 0000h, the analog input voltage + VREF results in 0FFFh."]
    #[inline(always)]
    pub fn adc12df_0(self) -> &'a mut W {
        self.variant(ADC12DF_A::ADC12DF_0)
    }
    #[doc = "Signed binary (2s complement), left aligned. Theoretically, for ADC12DIF = 0 and 12-bit mode, the analog input voltage VREF results in 8000h, the analog input voltage + VREF results in 7FF0h."]
    #[inline(always)]
    pub fn adc12df_1(self) -> &'a mut W {
        self.variant(ADC12DF_A::ADC12DF_1)
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
resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12RES_A {
    #[doc = "0: 8 bit (10 clock cycle conversion time)"]
    _8BIT = 0,
    #[doc = "1: 10 bit (12 clock cycle conversion time)"]
    _10BIT = 1,
    #[doc = "2: 12 bit (14 clock cycle conversion time)"]
    _12BIT = 2,
    #[doc = "3: Reserved"]
    ADC12RES_3 = 3,
}
impl From<ADC12RES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12RES`"]
pub type ADC12RES_R = crate::R<u8, ADC12RES_A>;
impl ADC12RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12RES_A {
        match self.bits {
            0 => ADC12RES_A::_8BIT,
            1 => ADC12RES_A::_10BIT,
            2 => ADC12RES_A::_12BIT,
            3 => ADC12RES_A::ADC12RES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == ADC12RES_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == ADC12RES_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == ADC12RES_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `ADC12RES_3`"]
    #[inline(always)]
    pub fn is_adc12res_3(&self) -> bool {
        *self == ADC12RES_A::ADC12RES_3
    }
}
#[doc = "Write proxy for field `ADC12RES`"]
pub struct ADC12RES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8 bit (10 clock cycle conversion time)"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(ADC12RES_A::_8BIT)
    }
    #[doc = "10 bit (12 clock cycle conversion time)"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(ADC12RES_A::_10BIT)
    }
    #[doc = "12 bit (14 clock cycle conversion time)"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(ADC12RES_A::_12BIT)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12res_3(self) -> &'a mut W {
        self.variant(ADC12RES_A::ADC12RES_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
low-power mode"]
    #[inline(always)]
    pub fn adc12pwrmd(&self) -> ADC12PWRMD_R {
        ADC12PWRMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
data read-back format"]
    #[inline(always)]
    pub fn adc12df(&self) -> ADC12DF_R {
        ADC12DF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
resolution"]
    #[inline(always)]
    pub fn adc12res(&self) -> ADC12RES_R {
        ADC12RES_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
low-power mode"]
    #[inline(always)]
    pub fn adc12pwrmd(&mut self) -> ADC12PWRMD_W {
        ADC12PWRMD_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
data read-back format"]
    #[inline(always)]
    pub fn adc12df(&mut self) -> ADC12DF_W {
        ADC12DF_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
resolution"]
    #[inline(always)]
    pub fn adc12res(&mut self) -> ADC12RES_W {
        ADC12RES_W { w: self }
    }
}
