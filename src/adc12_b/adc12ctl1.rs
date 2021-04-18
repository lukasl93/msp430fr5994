#[doc = "Reader of register ADC12CTL1"]
pub type R = crate::R<u16, super::ADC12CTL1>;
#[doc = "Writer for register ADC12CTL1"]
pub type W = crate::W<u16, super::ADC12CTL1>;
#[doc = "Register ADC12CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12CTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
ADC busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12BUSY_A {
    #[doc = "0: No operation is active."]
    ADC12BUSY_0 = 0,
    #[doc = "1: A sequence, sample, or conversion is active."]
    ADC12BUSY_1 = 1,
}
impl From<ADC12BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12BUSY`"]
pub type ADC12BUSY_R = crate::R<bool, ADC12BUSY_A>;
impl ADC12BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12BUSY_A {
        match self.bits {
            false => ADC12BUSY_A::ADC12BUSY_0,
            true => ADC12BUSY_A::ADC12BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12BUSY_0`"]
    #[inline(always)]
    pub fn is_adc12busy_0(&self) -> bool {
        *self == ADC12BUSY_A::ADC12BUSY_0
    }
    #[doc = "Checks if the value of the field is `ADC12BUSY_1`"]
    #[inline(always)]
    pub fn is_adc12busy_1(&self) -> bool {
        *self == ADC12BUSY_A::ADC12BUSY_1
    }
}
#[doc = "Write proxy for field `ADC12BUSY`"]
pub struct ADC12BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12BUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12BUSY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation is active."]
    #[inline(always)]
    pub fn adc12busy_0(self) -> &'a mut W {
        self.variant(ADC12BUSY_A::ADC12BUSY_0)
    }
    #[doc = "A sequence, sample, or conversion is active."]
    #[inline(always)]
    pub fn adc12busy_1(self) -> &'a mut W {
        self.variant(ADC12BUSY_A::ADC12BUSY_1)
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
#[doc = "2:1\\]
conversion sequence mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12CONSEQ_A {
    #[doc = "0: Single-channel, single-conversion"]
    ADC12CONSEQ_0 = 0,
    #[doc = "1: Sequence-of-channels"]
    ADC12CONSEQ_1 = 1,
    #[doc = "2: Repeat-single-channel"]
    ADC12CONSEQ_2 = 2,
    #[doc = "3: Repeat-sequence-of-channels"]
    ADC12CONSEQ_3 = 3,
}
impl From<ADC12CONSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12CONSEQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12CONSEQ`"]
pub type ADC12CONSEQ_R = crate::R<u8, ADC12CONSEQ_A>;
impl ADC12CONSEQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12CONSEQ_A {
        match self.bits {
            0 => ADC12CONSEQ_A::ADC12CONSEQ_0,
            1 => ADC12CONSEQ_A::ADC12CONSEQ_1,
            2 => ADC12CONSEQ_A::ADC12CONSEQ_2,
            3 => ADC12CONSEQ_A::ADC12CONSEQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_0`"]
    #[inline(always)]
    pub fn is_adc12conseq_0(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_0
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_1`"]
    #[inline(always)]
    pub fn is_adc12conseq_1(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_1
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_2`"]
    #[inline(always)]
    pub fn is_adc12conseq_2(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_2
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_3`"]
    #[inline(always)]
    pub fn is_adc12conseq_3(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_3
    }
}
#[doc = "Write proxy for field `ADC12CONSEQ`"]
pub struct ADC12CONSEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12CONSEQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12CONSEQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-channel, single-conversion"]
    #[inline(always)]
    pub fn adc12conseq_0(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_0)
    }
    #[doc = "Sequence-of-channels"]
    #[inline(always)]
    pub fn adc12conseq_1(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_1)
    }
    #[doc = "Repeat-single-channel"]
    #[inline(always)]
    pub fn adc12conseq_2(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_2)
    }
    #[doc = "Repeat-sequence-of-channels"]
    #[inline(always)]
    pub fn adc12conseq_3(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u16) & 0x03) << 1);
        self.w
    }
}
#[doc = "4:3\\]
clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SSEL_A {
    #[doc = "0: ADC12OSC (MODOSC)"]
    ADC12SSEL_0 = 0,
    #[doc = "1: ACLK"]
    ADC12SSEL_1 = 1,
    #[doc = "2: MCLK"]
    ADC12SSEL_2 = 2,
    #[doc = "3: SMCLK"]
    ADC12SSEL_3 = 3,
}
impl From<ADC12SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12SSEL`"]
pub type ADC12SSEL_R = crate::R<u8, ADC12SSEL_A>;
impl ADC12SSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SSEL_A {
        match self.bits {
            0 => ADC12SSEL_A::ADC12SSEL_0,
            1 => ADC12SSEL_A::ADC12SSEL_1,
            2 => ADC12SSEL_A::ADC12SSEL_2,
            3 => ADC12SSEL_A::ADC12SSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_0`"]
    #[inline(always)]
    pub fn is_adc12ssel_0(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_1`"]
    #[inline(always)]
    pub fn is_adc12ssel_1(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_2`"]
    #[inline(always)]
    pub fn is_adc12ssel_2(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_2
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_3`"]
    #[inline(always)]
    pub fn is_adc12ssel_3(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_3
    }
}
#[doc = "Write proxy for field `ADC12SSEL`"]
pub struct ADC12SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12OSC (MODOSC)"]
    #[inline(always)]
    pub fn adc12ssel_0(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn adc12ssel_1(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_1)
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn adc12ssel_2(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_2)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn adc12ssel_3(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u16) & 0x03) << 3);
        self.w
    }
}
#[doc = "7:5\\]
clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12DIV_A {
    #[doc = "0: /1"]
    ADC12DIV_0 = 0,
    #[doc = "1: /2"]
    ADC12DIV_1 = 1,
    #[doc = "2: /3"]
    ADC12DIV_2 = 2,
    #[doc = "3: /4"]
    ADC12DIV_3 = 3,
    #[doc = "4: /5"]
    ADC12DIV_4 = 4,
    #[doc = "5: /6"]
    ADC12DIV_5 = 5,
    #[doc = "6: /7"]
    ADC12DIV_6 = 6,
    #[doc = "7: /8"]
    ADC12DIV_7 = 7,
}
impl From<ADC12DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12DIV`"]
pub type ADC12DIV_R = crate::R<u8, ADC12DIV_A>;
impl ADC12DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12DIV_A {
        match self.bits {
            0 => ADC12DIV_A::ADC12DIV_0,
            1 => ADC12DIV_A::ADC12DIV_1,
            2 => ADC12DIV_A::ADC12DIV_2,
            3 => ADC12DIV_A::ADC12DIV_3,
            4 => ADC12DIV_A::ADC12DIV_4,
            5 => ADC12DIV_A::ADC12DIV_5,
            6 => ADC12DIV_A::ADC12DIV_6,
            7 => ADC12DIV_A::ADC12DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_0`"]
    #[inline(always)]
    pub fn is_adc12div_0(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_0
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_1`"]
    #[inline(always)]
    pub fn is_adc12div_1(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_1
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_2`"]
    #[inline(always)]
    pub fn is_adc12div_2(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_2
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_3`"]
    #[inline(always)]
    pub fn is_adc12div_3(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_3
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_4`"]
    #[inline(always)]
    pub fn is_adc12div_4(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_4
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_5`"]
    #[inline(always)]
    pub fn is_adc12div_5(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_5
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_6`"]
    #[inline(always)]
    pub fn is_adc12div_6(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_6
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_7`"]
    #[inline(always)]
    pub fn is_adc12div_7(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_7
    }
}
#[doc = "Write proxy for field `ADC12DIV`"]
pub struct ADC12DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn adc12div_0(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn adc12div_1(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_1)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn adc12div_2(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn adc12div_3(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_3)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn adc12div_4(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_4)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn adc12div_5(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_5)
    }
    #[doc = "/7"]
    #[inline(always)]
    pub fn adc12div_6(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_6)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn adc12div_7(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u16) & 0x07) << 5);
        self.w
    }
}
#[doc = "8:8\\]
invert signal sample-and-hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12ISSH_A {
    #[doc = "0: The sample-input signal is not inverted."]
    ADC12ISSH_0 = 0,
    #[doc = "1: The sample-input signal is inverted."]
    ADC12ISSH_1 = 1,
}
impl From<ADC12ISSH_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12ISSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12ISSH`"]
pub type ADC12ISSH_R = crate::R<bool, ADC12ISSH_A>;
impl ADC12ISSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12ISSH_A {
        match self.bits {
            false => ADC12ISSH_A::ADC12ISSH_0,
            true => ADC12ISSH_A::ADC12ISSH_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12ISSH_0`"]
    #[inline(always)]
    pub fn is_adc12issh_0(&self) -> bool {
        *self == ADC12ISSH_A::ADC12ISSH_0
    }
    #[doc = "Checks if the value of the field is `ADC12ISSH_1`"]
    #[inline(always)]
    pub fn is_adc12issh_1(&self) -> bool {
        *self == ADC12ISSH_A::ADC12ISSH_1
    }
}
#[doc = "Write proxy for field `ADC12ISSH`"]
pub struct ADC12ISSH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ISSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12ISSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The sample-input signal is not inverted."]
    #[inline(always)]
    pub fn adc12issh_0(self) -> &'a mut W {
        self.variant(ADC12ISSH_A::ADC12ISSH_0)
    }
    #[doc = "The sample-input signal is inverted."]
    #[inline(always)]
    pub fn adc12issh_1(self) -> &'a mut W {
        self.variant(ADC12ISSH_A::ADC12ISSH_1)
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
sample-and-hold pulse-mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12SHP_A {
    #[doc = "0: SAMPCON signal is sourced from the sample-input signal."]
    ADC12SHP_0 = 0,
    #[doc = "1: SAMPCON signal is sourced from the sampling timer."]
    ADC12SHP_1 = 1,
}
impl From<ADC12SHP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12SHP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12SHP`"]
pub type ADC12SHP_R = crate::R<bool, ADC12SHP_A>;
impl ADC12SHP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHP_A {
        match self.bits {
            false => ADC12SHP_A::ADC12SHP_0,
            true => ADC12SHP_A::ADC12SHP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHP_0`"]
    #[inline(always)]
    pub fn is_adc12shp_0(&self) -> bool {
        *self == ADC12SHP_A::ADC12SHP_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHP_1`"]
    #[inline(always)]
    pub fn is_adc12shp_1(&self) -> bool {
        *self == ADC12SHP_A::ADC12SHP_1
    }
}
#[doc = "Write proxy for field `ADC12SHP`"]
pub struct ADC12SHP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SHP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SAMPCON signal is sourced from the sample-input signal."]
    #[inline(always)]
    pub fn adc12shp_0(self) -> &'a mut W {
        self.variant(ADC12SHP_A::ADC12SHP_0)
    }
    #[doc = "SAMPCON signal is sourced from the sampling timer."]
    #[inline(always)]
    pub fn adc12shp_1(self) -> &'a mut W {
        self.variant(ADC12SHP_A::ADC12SHP_1)
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
#[doc = "12:10\\]
sample-and-hold source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SHS_A {
    #[doc = "0: ADC12SC bit"]
    ADC12SHS_0 = 0,
    #[doc = "1: see the device-specific data sheet for source"]
    ADC12SHS_1 = 1,
    #[doc = "2: see the device-specific data sheet for source"]
    ADC12SHS_2 = 2,
    #[doc = "3: see the device-specific data sheet for source"]
    ADC12SHS_3 = 3,
    #[doc = "4: see the device-specific data sheet for source"]
    ADC12SHS_4 = 4,
    #[doc = "5: see the device-specific data sheet for source"]
    ADC12SHS_5 = 5,
    #[doc = "6: see the device-specific data sheet for source"]
    ADC12SHS_6 = 6,
    #[doc = "7: see the device-specific data sheet for source"]
    ADC12SHS_7 = 7,
}
impl From<ADC12SHS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SHS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12SHS`"]
pub type ADC12SHS_R = crate::R<u8, ADC12SHS_A>;
impl ADC12SHS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHS_A {
        match self.bits {
            0 => ADC12SHS_A::ADC12SHS_0,
            1 => ADC12SHS_A::ADC12SHS_1,
            2 => ADC12SHS_A::ADC12SHS_2,
            3 => ADC12SHS_A::ADC12SHS_3,
            4 => ADC12SHS_A::ADC12SHS_4,
            5 => ADC12SHS_A::ADC12SHS_5,
            6 => ADC12SHS_A::ADC12SHS_6,
            7 => ADC12SHS_A::ADC12SHS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_0`"]
    #[inline(always)]
    pub fn is_adc12shs_0(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_1`"]
    #[inline(always)]
    pub fn is_adc12shs_1(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_1
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_2`"]
    #[inline(always)]
    pub fn is_adc12shs_2(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_2
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_3`"]
    #[inline(always)]
    pub fn is_adc12shs_3(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_3
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_4`"]
    #[inline(always)]
    pub fn is_adc12shs_4(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_4
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_5`"]
    #[inline(always)]
    pub fn is_adc12shs_5(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_5
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_6`"]
    #[inline(always)]
    pub fn is_adc12shs_6(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_6
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_7`"]
    #[inline(always)]
    pub fn is_adc12shs_7(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_7
    }
}
#[doc = "Write proxy for field `ADC12SHS`"]
pub struct ADC12SHS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SHS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12SC bit"]
    #[inline(always)]
    pub fn adc12shs_0(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_0)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_1(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_1)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_2(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_2)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_3(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_3)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_4(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_4)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_5(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_5)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_6(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_6)
    }
    #[doc = "see the device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc12shs_7(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u16) & 0x07) << 10);
        self.w
    }
}
#[doc = "14:13\\]
predivider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12PDIV_A {
    #[doc = "0: Predivide by 1"]
    _1 = 0,
    #[doc = "1: Predivide by 4"]
    _4 = 1,
    #[doc = "2: Predivide by 32"]
    _32 = 2,
    #[doc = "3: Predivide by 64"]
    _64 = 3,
}
impl From<ADC12PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12PDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12PDIV`"]
pub type ADC12PDIV_R = crate::R<u8, ADC12PDIV_A>;
impl ADC12PDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12PDIV_A {
        match self.bits {
            0 => ADC12PDIV_A::_1,
            1 => ADC12PDIV_A::_4,
            2 => ADC12PDIV_A::_32,
            3 => ADC12PDIV_A::_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC12PDIV_A::_1
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == ADC12PDIV_A::_4
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == ADC12PDIV_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == ADC12PDIV_A::_64
    }
}
#[doc = "Write proxy for field `ADC12PDIV`"]
pub struct ADC12PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12PDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12PDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Predivide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC12PDIV_A::_1)
    }
    #[doc = "Predivide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(ADC12PDIV_A::_4)
    }
    #[doc = "Predivide by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(ADC12PDIV_A::_32)
    }
    #[doc = "Predivide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(ADC12PDIV_A::_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u16) & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
ADC busy"]
    #[inline(always)]
    pub fn adc12busy(&self) -> ADC12BUSY_R {
        ADC12BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
conversion sequence mode select"]
    #[inline(always)]
    pub fn adc12conseq(&self) -> ADC12CONSEQ_R {
        ADC12CONSEQ_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
clock source select"]
    #[inline(always)]
    pub fn adc12ssel(&self) -> ADC12SSEL_R {
        ADC12SSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
clock divider"]
    #[inline(always)]
    pub fn adc12div(&self) -> ADC12DIV_R {
        ADC12DIV_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adc12issh(&self) -> ADC12ISSH_R {
        ADC12ISSH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adc12shp(&self) -> ADC12SHP_R {
        ADC12SHP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - 12:10\\]
sample-and-hold source select"]
    #[inline(always)]
    pub fn adc12shs(&self) -> ADC12SHS_R {
        ADC12SHS_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:14 - 14:13\\]
predivider"]
    #[inline(always)]
    pub fn adc12pdiv(&self) -> ADC12PDIV_R {
        ADC12PDIV_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ADC busy"]
    #[inline(always)]
    pub fn adc12busy(&mut self) -> ADC12BUSY_W {
        ADC12BUSY_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\]
conversion sequence mode select"]
    #[inline(always)]
    pub fn adc12conseq(&mut self) -> ADC12CONSEQ_W {
        ADC12CONSEQ_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\]
clock source select"]
    #[inline(always)]
    pub fn adc12ssel(&mut self) -> ADC12SSEL_W {
        ADC12SSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
clock divider"]
    #[inline(always)]
    pub fn adc12div(&mut self) -> ADC12DIV_W {
        ADC12DIV_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adc12issh(&mut self) -> ADC12ISSH_W {
        ADC12ISSH_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adc12shp(&mut self) -> ADC12SHP_W {
        ADC12SHP_W { w: self }
    }
    #[doc = "Bits 10:12 - 12:10\\]
sample-and-hold source select"]
    #[inline(always)]
    pub fn adc12shs(&mut self) -> ADC12SHS_W {
        ADC12SHS_W { w: self }
    }
    #[doc = "Bits 13:14 - 14:13\\]
predivider"]
    #[inline(always)]
    pub fn adc12pdiv(&mut self) -> ADC12PDIV_W {
        ADC12PDIV_W { w: self }
    }
}
