#[doc = "Reader of register ADC12CTL0"]
pub type R = crate::R<u16, super::ADC12CTL0>;
#[doc = "Writer for register ADC12CTL0"]
pub type W = crate::W<u16, super::ADC12CTL0>;
#[doc = "Register ADC12CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
start conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12SC_A {
    #[doc = "0: No sample-and-conversion-start"]
    ADC12SC_0 = 0,
    #[doc = "1: Start sample-and-conversion"]
    ADC12SC_1 = 1,
}
impl From<ADC12SC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12SC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12SC`"]
pub type ADC12SC_R = crate::R<bool, ADC12SC_A>;
impl ADC12SC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SC_A {
        match self.bits {
            false => ADC12SC_A::ADC12SC_0,
            true => ADC12SC_A::ADC12SC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SC_0`"]
    #[inline(always)]
    pub fn is_adc12sc_0(&self) -> bool {
        *self == ADC12SC_A::ADC12SC_0
    }
    #[doc = "Checks if the value of the field is `ADC12SC_1`"]
    #[inline(always)]
    pub fn is_adc12sc_1(&self) -> bool {
        *self == ADC12SC_A::ADC12SC_1
    }
}
#[doc = "Write proxy for field `ADC12SC`"]
pub struct ADC12SC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No sample-and-conversion-start"]
    #[inline(always)]
    pub fn adc12sc_0(self) -> &'a mut W {
        self.variant(ADC12SC_A::ADC12SC_0)
    }
    #[doc = "Start sample-and-conversion"]
    #[inline(always)]
    pub fn adc12sc_1(self) -> &'a mut W {
        self.variant(ADC12SC_A::ADC12SC_1)
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
enable conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12ENC_A {
    #[doc = "0: ADC12_B disabled"]
    ADC12ENC_0 = 0,
    #[doc = "1: ADC12_B enabled"]
    ADC12ENC_1 = 1,
}
impl From<ADC12ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12ENC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12ENC`"]
pub type ADC12ENC_R = crate::R<bool, ADC12ENC_A>;
impl ADC12ENC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12ENC_A {
        match self.bits {
            false => ADC12ENC_A::ADC12ENC_0,
            true => ADC12ENC_A::ADC12ENC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12ENC_0`"]
    #[inline(always)]
    pub fn is_adc12enc_0(&self) -> bool {
        *self == ADC12ENC_A::ADC12ENC_0
    }
    #[doc = "Checks if the value of the field is `ADC12ENC_1`"]
    #[inline(always)]
    pub fn is_adc12enc_1(&self) -> bool {
        *self == ADC12ENC_A::ADC12ENC_1
    }
}
#[doc = "Write proxy for field `ADC12ENC`"]
pub struct ADC12ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ENC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12ENC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC12_B disabled"]
    #[inline(always)]
    pub fn adc12enc_0(self) -> &'a mut W {
        self.variant(ADC12ENC_A::ADC12ENC_0)
    }
    #[doc = "ADC12_B enabled"]
    #[inline(always)]
    pub fn adc12enc_1(self) -> &'a mut W {
        self.variant(ADC12ENC_A::ADC12ENC_1)
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
#[doc = "4:4\\]
ADC on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12ON_A {
    #[doc = "0: ADC12_B off"]
    ADC12ON_0 = 0,
    #[doc = "1: ADC12_B on"]
    ADC12ON_1 = 1,
}
impl From<ADC12ON_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12ON`"]
pub type ADC12ON_R = crate::R<bool, ADC12ON_A>;
impl ADC12ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12ON_A {
        match self.bits {
            false => ADC12ON_A::ADC12ON_0,
            true => ADC12ON_A::ADC12ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12ON_0`"]
    #[inline(always)]
    pub fn is_adc12on_0(&self) -> bool {
        *self == ADC12ON_A::ADC12ON_0
    }
    #[doc = "Checks if the value of the field is `ADC12ON_1`"]
    #[inline(always)]
    pub fn is_adc12on_1(&self) -> bool {
        *self == ADC12ON_A::ADC12ON_1
    }
}
#[doc = "Write proxy for field `ADC12ON`"]
pub struct ADC12ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC12_B off"]
    #[inline(always)]
    pub fn adc12on_0(self) -> &'a mut W {
        self.variant(ADC12ON_A::ADC12ON_0)
    }
    #[doc = "ADC12_B on"]
    #[inline(always)]
    pub fn adc12on_1(self) -> &'a mut W {
        self.variant(ADC12ON_A::ADC12ON_1)
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
#[doc = "7:7\\]
sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12MSC_A {
    #[doc = "0: The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    ADC12MSC_0 = 0,
    #[doc = "1: The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    ADC12MSC_1 = 1,
}
impl From<ADC12MSC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12MSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12MSC`"]
pub type ADC12MSC_R = crate::R<bool, ADC12MSC_A>;
impl ADC12MSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12MSC_A {
        match self.bits {
            false => ADC12MSC_A::ADC12MSC_0,
            true => ADC12MSC_A::ADC12MSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12MSC_0`"]
    #[inline(always)]
    pub fn is_adc12msc_0(&self) -> bool {
        *self == ADC12MSC_A::ADC12MSC_0
    }
    #[doc = "Checks if the value of the field is `ADC12MSC_1`"]
    #[inline(always)]
    pub fn is_adc12msc_1(&self) -> bool {
        *self == ADC12MSC_A::ADC12MSC_1
    }
}
#[doc = "Write proxy for field `ADC12MSC`"]
pub struct ADC12MSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12MSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12MSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    #[inline(always)]
    pub fn adc12msc_0(self) -> &'a mut W {
        self.variant(ADC12MSC_A::ADC12MSC_0)
    }
    #[doc = "The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    #[inline(always)]
    pub fn adc12msc_1(self) -> &'a mut W {
        self.variant(ADC12MSC_A::ADC12MSC_1)
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
#[doc = "11:8\\]
sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SHT0_A {
    #[doc = "0: 4 ADC12CLK cycles"]
    ADC12SHT0_0 = 0,
    #[doc = "1: 8 ADC12CLK cycles"]
    ADC12SHT0_1 = 1,
    #[doc = "2: 16 ADC12CLK cycles"]
    ADC12SHT0_2 = 2,
    #[doc = "3: 32 ADC12CLK cycles"]
    ADC12SHT0_3 = 3,
    #[doc = "4: 64 ADC12CLK cycles"]
    ADC12SHT0_4 = 4,
    #[doc = "5: 96 ADC12CLK cycles"]
    ADC12SHT0_5 = 5,
    #[doc = "6: 128 ADC12CLK cycles"]
    ADC12SHT0_6 = 6,
    #[doc = "7: 192 ADC12CLK cycles"]
    ADC12SHT0_7 = 7,
    #[doc = "8: 256 ADC12CLK cycles"]
    ADC12SHT0_8 = 8,
    #[doc = "9: 384 ADC12CLK cycles"]
    ADC12SHT0_9 = 9,
    #[doc = "10: 512 ADC12CLK cycles"]
    ADC12SHT0_10 = 10,
    #[doc = "11: Reserved"]
    ADC12SHT0_11 = 11,
    #[doc = "12: Reserved"]
    ADC12SHT0_12 = 12,
    #[doc = "13: Reserved"]
    ADC12SHT0_13 = 13,
    #[doc = "14: Reserved"]
    ADC12SHT0_14 = 14,
    #[doc = "15: Reserved"]
    ADC12SHT0_15 = 15,
}
impl From<ADC12SHT0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SHT0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12SHT0`"]
pub type ADC12SHT0_R = crate::R<u8, ADC12SHT0_A>;
impl ADC12SHT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHT0_A {
        match self.bits {
            0 => ADC12SHT0_A::ADC12SHT0_0,
            1 => ADC12SHT0_A::ADC12SHT0_1,
            2 => ADC12SHT0_A::ADC12SHT0_2,
            3 => ADC12SHT0_A::ADC12SHT0_3,
            4 => ADC12SHT0_A::ADC12SHT0_4,
            5 => ADC12SHT0_A::ADC12SHT0_5,
            6 => ADC12SHT0_A::ADC12SHT0_6,
            7 => ADC12SHT0_A::ADC12SHT0_7,
            8 => ADC12SHT0_A::ADC12SHT0_8,
            9 => ADC12SHT0_A::ADC12SHT0_9,
            10 => ADC12SHT0_A::ADC12SHT0_10,
            11 => ADC12SHT0_A::ADC12SHT0_11,
            12 => ADC12SHT0_A::ADC12SHT0_12,
            13 => ADC12SHT0_A::ADC12SHT0_13,
            14 => ADC12SHT0_A::ADC12SHT0_14,
            15 => ADC12SHT0_A::ADC12SHT0_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_0`"]
    #[inline(always)]
    pub fn is_adc12sht0_0(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_1`"]
    #[inline(always)]
    pub fn is_adc12sht0_1(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_1
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_2`"]
    #[inline(always)]
    pub fn is_adc12sht0_2(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_2
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_3`"]
    #[inline(always)]
    pub fn is_adc12sht0_3(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_3
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_4`"]
    #[inline(always)]
    pub fn is_adc12sht0_4(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_4
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_5`"]
    #[inline(always)]
    pub fn is_adc12sht0_5(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_5
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_6`"]
    #[inline(always)]
    pub fn is_adc12sht0_6(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_6
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_7`"]
    #[inline(always)]
    pub fn is_adc12sht0_7(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_7
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_8`"]
    #[inline(always)]
    pub fn is_adc12sht0_8(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_8
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_9`"]
    #[inline(always)]
    pub fn is_adc12sht0_9(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_9
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_10`"]
    #[inline(always)]
    pub fn is_adc12sht0_10(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_10
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_11`"]
    #[inline(always)]
    pub fn is_adc12sht0_11(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_11
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_12`"]
    #[inline(always)]
    pub fn is_adc12sht0_12(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_12
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_13`"]
    #[inline(always)]
    pub fn is_adc12sht0_13(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_13
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_14`"]
    #[inline(always)]
    pub fn is_adc12sht0_14(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_14
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_15`"]
    #[inline(always)]
    pub fn is_adc12sht0_15(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_15
    }
}
#[doc = "Write proxy for field `ADC12SHT0`"]
pub struct ADC12SHT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SHT0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_0(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_0)
    }
    #[doc = "8 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_1(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_1)
    }
    #[doc = "16 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_2(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_2)
    }
    #[doc = "32 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_3(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_3)
    }
    #[doc = "64 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_4(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_4)
    }
    #[doc = "96 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_5(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_5)
    }
    #[doc = "128 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_6(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_6)
    }
    #[doc = "192 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_7(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_7)
    }
    #[doc = "256 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_8(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_8)
    }
    #[doc = "384 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_9(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_9)
    }
    #[doc = "512 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht0_10(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_10)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_11(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_11)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_12(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_12)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_13(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_13)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_14(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_14)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht0_15(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "15:12\\]
sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SHT1_A {
    #[doc = "0: 4 ADC12CLK cycles"]
    ADC12SHT1_0 = 0,
    #[doc = "1: 8 ADC12CLK cycles"]
    ADC12SHT1_1 = 1,
    #[doc = "2: 16 ADC12CLK cycles"]
    ADC12SHT1_2 = 2,
    #[doc = "3: 32 ADC12CLK cycles"]
    ADC12SHT1_3 = 3,
    #[doc = "4: 64 ADC12CLK cycles"]
    ADC12SHT1_4 = 4,
    #[doc = "5: 96 ADC12CLK cycles"]
    ADC12SHT1_5 = 5,
    #[doc = "6: 128 ADC12CLK cycles"]
    ADC12SHT1_6 = 6,
    #[doc = "7: 192 ADC12CLK cycles"]
    ADC12SHT1_7 = 7,
    #[doc = "8: 256 ADC12CLK cycles"]
    ADC12SHT1_8 = 8,
    #[doc = "9: 384 ADC12CLK cycles"]
    ADC12SHT1_9 = 9,
    #[doc = "10: 512 ADC12CLK cycles"]
    ADC12SHT1_10 = 10,
    #[doc = "11: Reserved"]
    ADC12SHT1_11 = 11,
    #[doc = "12: Reserved"]
    ADC12SHT1_12 = 12,
    #[doc = "13: Reserved"]
    ADC12SHT1_13 = 13,
    #[doc = "14: Reserved"]
    ADC12SHT1_14 = 14,
    #[doc = "15: Reserved"]
    ADC12SHT1_15 = 15,
}
impl From<ADC12SHT1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SHT1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12SHT1`"]
pub type ADC12SHT1_R = crate::R<u8, ADC12SHT1_A>;
impl ADC12SHT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHT1_A {
        match self.bits {
            0 => ADC12SHT1_A::ADC12SHT1_0,
            1 => ADC12SHT1_A::ADC12SHT1_1,
            2 => ADC12SHT1_A::ADC12SHT1_2,
            3 => ADC12SHT1_A::ADC12SHT1_3,
            4 => ADC12SHT1_A::ADC12SHT1_4,
            5 => ADC12SHT1_A::ADC12SHT1_5,
            6 => ADC12SHT1_A::ADC12SHT1_6,
            7 => ADC12SHT1_A::ADC12SHT1_7,
            8 => ADC12SHT1_A::ADC12SHT1_8,
            9 => ADC12SHT1_A::ADC12SHT1_9,
            10 => ADC12SHT1_A::ADC12SHT1_10,
            11 => ADC12SHT1_A::ADC12SHT1_11,
            12 => ADC12SHT1_A::ADC12SHT1_12,
            13 => ADC12SHT1_A::ADC12SHT1_13,
            14 => ADC12SHT1_A::ADC12SHT1_14,
            15 => ADC12SHT1_A::ADC12SHT1_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_0`"]
    #[inline(always)]
    pub fn is_adc12sht1_0(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_1`"]
    #[inline(always)]
    pub fn is_adc12sht1_1(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_1
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_2`"]
    #[inline(always)]
    pub fn is_adc12sht1_2(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_2
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_3`"]
    #[inline(always)]
    pub fn is_adc12sht1_3(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_3
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_4`"]
    #[inline(always)]
    pub fn is_adc12sht1_4(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_4
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_5`"]
    #[inline(always)]
    pub fn is_adc12sht1_5(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_5
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_6`"]
    #[inline(always)]
    pub fn is_adc12sht1_6(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_6
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_7`"]
    #[inline(always)]
    pub fn is_adc12sht1_7(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_7
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_8`"]
    #[inline(always)]
    pub fn is_adc12sht1_8(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_8
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_9`"]
    #[inline(always)]
    pub fn is_adc12sht1_9(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_9
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_10`"]
    #[inline(always)]
    pub fn is_adc12sht1_10(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_10
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_11`"]
    #[inline(always)]
    pub fn is_adc12sht1_11(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_11
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_12`"]
    #[inline(always)]
    pub fn is_adc12sht1_12(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_12
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_13`"]
    #[inline(always)]
    pub fn is_adc12sht1_13(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_13
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_14`"]
    #[inline(always)]
    pub fn is_adc12sht1_14(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_14
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_15`"]
    #[inline(always)]
    pub fn is_adc12sht1_15(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_15
    }
}
#[doc = "Write proxy for field `ADC12SHT1`"]
pub struct ADC12SHT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SHT1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_0(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_0)
    }
    #[doc = "8 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_1(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_1)
    }
    #[doc = "16 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_2(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_2)
    }
    #[doc = "32 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_3(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_3)
    }
    #[doc = "64 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_4(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_4)
    }
    #[doc = "96 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_5(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_5)
    }
    #[doc = "128 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_6(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_6)
    }
    #[doc = "192 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_7(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_7)
    }
    #[doc = "256 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_8(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_8)
    }
    #[doc = "384 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_9(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_9)
    }
    #[doc = "512 ADC12CLK cycles"]
    #[inline(always)]
    pub fn adc12sht1_10(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_10)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_11(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_11)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_12(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_12)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_13(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_13)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_14(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_14)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12sht1_15(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
start conversion"]
    #[inline(always)]
    pub fn adc12sc(&self) -> ADC12SC_R {
        ADC12SC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
enable conversion"]
    #[inline(always)]
    pub fn adc12enc(&self) -> ADC12ENC_R {
        ADC12ENC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ADC on"]
    #[inline(always)]
    pub fn adc12on(&self) -> ADC12ON_R {
        ADC12ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
sample-and-hold time."]
    #[inline(always)]
    pub fn adc12msc(&self) -> ADC12MSC_R {
        ADC12MSC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
sample-and-hold time."]
    #[inline(always)]
    pub fn adc12sht0(&self) -> ADC12SHT0_R {
        ADC12SHT0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
sample-and-hold time."]
    #[inline(always)]
    pub fn adc12sht1(&self) -> ADC12SHT1_R {
        ADC12SHT1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
start conversion"]
    #[inline(always)]
    pub fn adc12sc(&mut self) -> ADC12SC_W {
        ADC12SC_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
enable conversion"]
    #[inline(always)]
    pub fn adc12enc(&mut self) -> ADC12ENC_W {
        ADC12ENC_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
ADC on"]
    #[inline(always)]
    pub fn adc12on(&mut self) -> ADC12ON_W {
        ADC12ON_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
sample-and-hold time."]
    #[inline(always)]
    pub fn adc12msc(&mut self) -> ADC12MSC_W {
        ADC12MSC_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
sample-and-hold time."]
    #[inline(always)]
    pub fn adc12sht0(&mut self) -> ADC12SHT0_W {
        ADC12SHT0_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
sample-and-hold time."]
    #[inline(always)]
    pub fn adc12sht1(&mut self) -> ADC12SHT1_W {
        ADC12SHT1_W { w: self }
    }
}
