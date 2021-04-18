#[doc = "Reader of register ADC12CTL3"]
pub type R = crate::R<u16, super::ADC12CTL3>;
#[doc = "Writer for register ADC12CTL3"]
pub type W = crate::W<u16, super::ADC12CTL3>;
#[doc = "Register ADC12CTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12CTL3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "4:0\\]
conversion start address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12CSTARTADD_A {
    #[doc = "0: Conversion start address ADC12MEM0"]
    ADC12MEM0 = 0,
    #[doc = "1: Conversion start address ADC12MEM1"]
    ADC12MEM1 = 1,
    #[doc = "2: Conversion start address ADC12MEM2"]
    ADC12MEM2 = 2,
    #[doc = "3: Conversion start address ADC12MEM3"]
    ADC12MEM3 = 3,
    #[doc = "4: Conversion start address ADC12MEM4"]
    ADC12MEM4 = 4,
    #[doc = "5: Conversion start address ADC12MEM5"]
    ADC12MEM5 = 5,
    #[doc = "6: Conversion start address ADC12MEM6"]
    ADC12MEM6 = 6,
    #[doc = "7: Conversion start address ADC12MEM7"]
    ADC12MEM7 = 7,
    #[doc = "8: Conversion start address ADC12MEM8"]
    ADC12MEM8 = 8,
    #[doc = "9: Conversion start address ADC12MEM9"]
    ADC12MEM9 = 9,
    #[doc = "10: Conversion start address ADC12MEM10"]
    ADC12MEM10 = 10,
    #[doc = "11: Conversion start address ADC12MEM10"]
    ADC12MEM11 = 11,
    #[doc = "12: Conversion start address ADC12MEM12"]
    ADC12MEM12 = 12,
    #[doc = "13: Conversion start address ADC12MEM13"]
    ADC12MEM13 = 13,
    #[doc = "14: Conversion start address ADC12MEM14"]
    ADC12MEM14 = 14,
    #[doc = "15: Conversion start address ADC12MEM15"]
    ADC12MEM15 = 15,
    #[doc = "16: Conversion start address ADC12MEM16"]
    ADC12MEM16 = 16,
    #[doc = "17: Conversion start address ADC12MEM17"]
    ADC12MEM17 = 17,
    #[doc = "18: Conversion start address ADC12MEM18"]
    ADC12MEM18 = 18,
    #[doc = "19: Conversion start address ADC12MEM19"]
    ADC12MEM19 = 19,
    #[doc = "20: Conversion start address ADC12MEM20"]
    ADC12MEM20 = 20,
    #[doc = "21: Conversion start address ADC12MEM21"]
    ADC12MEM21 = 21,
    #[doc = "22: Conversion start address ADC12MEM22"]
    ADC12MEM22 = 22,
    #[doc = "23: Conversion start address ADC12MEM23"]
    ADC12MEM23 = 23,
    #[doc = "24: Conversion start address ADC12MEM24"]
    ADC12MEM24 = 24,
    #[doc = "25: Conversion start address ADC12MEM25"]
    ADC12MEM25 = 25,
    #[doc = "26: Conversion start address ADC12MEM26"]
    ADC12MEM26 = 26,
    #[doc = "27: Conversion start address ADC12MEM27"]
    ADC12MEM27 = 27,
    #[doc = "28: Conversion start address ADC12MEM28"]
    ADC12MEM28 = 28,
    #[doc = "29: Conversion start address ADC12MEM29"]
    ADC12MEM29 = 29,
    #[doc = "30: Conversion start address ADC12MEM30"]
    ADC12MEM30 = 30,
    #[doc = "31: Conversion start address ADC12MEM31"]
    ADC12MEM31 = 31,
}
impl From<ADC12CSTARTADD_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12CSTARTADD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12CSTARTADD`"]
pub type ADC12CSTARTADD_R = crate::R<u8, ADC12CSTARTADD_A>;
impl ADC12CSTARTADD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12CSTARTADD_A {
        match self.bits {
            0 => ADC12CSTARTADD_A::ADC12MEM0,
            1 => ADC12CSTARTADD_A::ADC12MEM1,
            2 => ADC12CSTARTADD_A::ADC12MEM2,
            3 => ADC12CSTARTADD_A::ADC12MEM3,
            4 => ADC12CSTARTADD_A::ADC12MEM4,
            5 => ADC12CSTARTADD_A::ADC12MEM5,
            6 => ADC12CSTARTADD_A::ADC12MEM6,
            7 => ADC12CSTARTADD_A::ADC12MEM7,
            8 => ADC12CSTARTADD_A::ADC12MEM8,
            9 => ADC12CSTARTADD_A::ADC12MEM9,
            10 => ADC12CSTARTADD_A::ADC12MEM10,
            11 => ADC12CSTARTADD_A::ADC12MEM11,
            12 => ADC12CSTARTADD_A::ADC12MEM12,
            13 => ADC12CSTARTADD_A::ADC12MEM13,
            14 => ADC12CSTARTADD_A::ADC12MEM14,
            15 => ADC12CSTARTADD_A::ADC12MEM15,
            16 => ADC12CSTARTADD_A::ADC12MEM16,
            17 => ADC12CSTARTADD_A::ADC12MEM17,
            18 => ADC12CSTARTADD_A::ADC12MEM18,
            19 => ADC12CSTARTADD_A::ADC12MEM19,
            20 => ADC12CSTARTADD_A::ADC12MEM20,
            21 => ADC12CSTARTADD_A::ADC12MEM21,
            22 => ADC12CSTARTADD_A::ADC12MEM22,
            23 => ADC12CSTARTADD_A::ADC12MEM23,
            24 => ADC12CSTARTADD_A::ADC12MEM24,
            25 => ADC12CSTARTADD_A::ADC12MEM25,
            26 => ADC12CSTARTADD_A::ADC12MEM26,
            27 => ADC12CSTARTADD_A::ADC12MEM27,
            28 => ADC12CSTARTADD_A::ADC12MEM28,
            29 => ADC12CSTARTADD_A::ADC12MEM29,
            30 => ADC12CSTARTADD_A::ADC12MEM30,
            31 => ADC12CSTARTADD_A::ADC12MEM31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12MEM0`"]
    #[inline(always)]
    pub fn is_adc12mem0(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM0
    }
    #[doc = "Checks if the value of the field is `ADC12MEM1`"]
    #[inline(always)]
    pub fn is_adc12mem1(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM1
    }
    #[doc = "Checks if the value of the field is `ADC12MEM2`"]
    #[inline(always)]
    pub fn is_adc12mem2(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM2
    }
    #[doc = "Checks if the value of the field is `ADC12MEM3`"]
    #[inline(always)]
    pub fn is_adc12mem3(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM3
    }
    #[doc = "Checks if the value of the field is `ADC12MEM4`"]
    #[inline(always)]
    pub fn is_adc12mem4(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM4
    }
    #[doc = "Checks if the value of the field is `ADC12MEM5`"]
    #[inline(always)]
    pub fn is_adc12mem5(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM5
    }
    #[doc = "Checks if the value of the field is `ADC12MEM6`"]
    #[inline(always)]
    pub fn is_adc12mem6(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM6
    }
    #[doc = "Checks if the value of the field is `ADC12MEM7`"]
    #[inline(always)]
    pub fn is_adc12mem7(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM7
    }
    #[doc = "Checks if the value of the field is `ADC12MEM8`"]
    #[inline(always)]
    pub fn is_adc12mem8(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM8
    }
    #[doc = "Checks if the value of the field is `ADC12MEM9`"]
    #[inline(always)]
    pub fn is_adc12mem9(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM9
    }
    #[doc = "Checks if the value of the field is `ADC12MEM10`"]
    #[inline(always)]
    pub fn is_adc12mem10(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM10
    }
    #[doc = "Checks if the value of the field is `ADC12MEM11`"]
    #[inline(always)]
    pub fn is_adc12mem11(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM11
    }
    #[doc = "Checks if the value of the field is `ADC12MEM12`"]
    #[inline(always)]
    pub fn is_adc12mem12(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM12
    }
    #[doc = "Checks if the value of the field is `ADC12MEM13`"]
    #[inline(always)]
    pub fn is_adc12mem13(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM13
    }
    #[doc = "Checks if the value of the field is `ADC12MEM14`"]
    #[inline(always)]
    pub fn is_adc12mem14(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM14
    }
    #[doc = "Checks if the value of the field is `ADC12MEM15`"]
    #[inline(always)]
    pub fn is_adc12mem15(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM15
    }
    #[doc = "Checks if the value of the field is `ADC12MEM16`"]
    #[inline(always)]
    pub fn is_adc12mem16(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM16
    }
    #[doc = "Checks if the value of the field is `ADC12MEM17`"]
    #[inline(always)]
    pub fn is_adc12mem17(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM17
    }
    #[doc = "Checks if the value of the field is `ADC12MEM18`"]
    #[inline(always)]
    pub fn is_adc12mem18(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM18
    }
    #[doc = "Checks if the value of the field is `ADC12MEM19`"]
    #[inline(always)]
    pub fn is_adc12mem19(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM19
    }
    #[doc = "Checks if the value of the field is `ADC12MEM20`"]
    #[inline(always)]
    pub fn is_adc12mem20(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM20
    }
    #[doc = "Checks if the value of the field is `ADC12MEM21`"]
    #[inline(always)]
    pub fn is_adc12mem21(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM21
    }
    #[doc = "Checks if the value of the field is `ADC12MEM22`"]
    #[inline(always)]
    pub fn is_adc12mem22(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM22
    }
    #[doc = "Checks if the value of the field is `ADC12MEM23`"]
    #[inline(always)]
    pub fn is_adc12mem23(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM23
    }
    #[doc = "Checks if the value of the field is `ADC12MEM24`"]
    #[inline(always)]
    pub fn is_adc12mem24(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM24
    }
    #[doc = "Checks if the value of the field is `ADC12MEM25`"]
    #[inline(always)]
    pub fn is_adc12mem25(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM25
    }
    #[doc = "Checks if the value of the field is `ADC12MEM26`"]
    #[inline(always)]
    pub fn is_adc12mem26(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM26
    }
    #[doc = "Checks if the value of the field is `ADC12MEM27`"]
    #[inline(always)]
    pub fn is_adc12mem27(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM27
    }
    #[doc = "Checks if the value of the field is `ADC12MEM28`"]
    #[inline(always)]
    pub fn is_adc12mem28(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM28
    }
    #[doc = "Checks if the value of the field is `ADC12MEM29`"]
    #[inline(always)]
    pub fn is_adc12mem29(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM29
    }
    #[doc = "Checks if the value of the field is `ADC12MEM30`"]
    #[inline(always)]
    pub fn is_adc12mem30(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM30
    }
    #[doc = "Checks if the value of the field is `ADC12MEM31`"]
    #[inline(always)]
    pub fn is_adc12mem31(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12MEM31
    }
}
#[doc = "Write proxy for field `ADC12CSTARTADD`"]
pub struct ADC12CSTARTADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12CSTARTADD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12CSTARTADD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Conversion start address ADC12MEM0"]
    #[inline(always)]
    pub fn adc12mem0(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM0)
    }
    #[doc = "Conversion start address ADC12MEM1"]
    #[inline(always)]
    pub fn adc12mem1(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM1)
    }
    #[doc = "Conversion start address ADC12MEM2"]
    #[inline(always)]
    pub fn adc12mem2(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM2)
    }
    #[doc = "Conversion start address ADC12MEM3"]
    #[inline(always)]
    pub fn adc12mem3(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM3)
    }
    #[doc = "Conversion start address ADC12MEM4"]
    #[inline(always)]
    pub fn adc12mem4(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM4)
    }
    #[doc = "Conversion start address ADC12MEM5"]
    #[inline(always)]
    pub fn adc12mem5(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM5)
    }
    #[doc = "Conversion start address ADC12MEM6"]
    #[inline(always)]
    pub fn adc12mem6(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM6)
    }
    #[doc = "Conversion start address ADC12MEM7"]
    #[inline(always)]
    pub fn adc12mem7(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM7)
    }
    #[doc = "Conversion start address ADC12MEM8"]
    #[inline(always)]
    pub fn adc12mem8(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM8)
    }
    #[doc = "Conversion start address ADC12MEM9"]
    #[inline(always)]
    pub fn adc12mem9(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM9)
    }
    #[doc = "Conversion start address ADC12MEM10"]
    #[inline(always)]
    pub fn adc12mem10(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM10)
    }
    #[doc = "Conversion start address ADC12MEM10"]
    #[inline(always)]
    pub fn adc12mem11(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM11)
    }
    #[doc = "Conversion start address ADC12MEM12"]
    #[inline(always)]
    pub fn adc12mem12(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM12)
    }
    #[doc = "Conversion start address ADC12MEM13"]
    #[inline(always)]
    pub fn adc12mem13(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM13)
    }
    #[doc = "Conversion start address ADC12MEM14"]
    #[inline(always)]
    pub fn adc12mem14(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM14)
    }
    #[doc = "Conversion start address ADC12MEM15"]
    #[inline(always)]
    pub fn adc12mem15(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM15)
    }
    #[doc = "Conversion start address ADC12MEM16"]
    #[inline(always)]
    pub fn adc12mem16(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM16)
    }
    #[doc = "Conversion start address ADC12MEM17"]
    #[inline(always)]
    pub fn adc12mem17(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM17)
    }
    #[doc = "Conversion start address ADC12MEM18"]
    #[inline(always)]
    pub fn adc12mem18(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM18)
    }
    #[doc = "Conversion start address ADC12MEM19"]
    #[inline(always)]
    pub fn adc12mem19(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM19)
    }
    #[doc = "Conversion start address ADC12MEM20"]
    #[inline(always)]
    pub fn adc12mem20(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM20)
    }
    #[doc = "Conversion start address ADC12MEM21"]
    #[inline(always)]
    pub fn adc12mem21(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM21)
    }
    #[doc = "Conversion start address ADC12MEM22"]
    #[inline(always)]
    pub fn adc12mem22(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM22)
    }
    #[doc = "Conversion start address ADC12MEM23"]
    #[inline(always)]
    pub fn adc12mem23(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM23)
    }
    #[doc = "Conversion start address ADC12MEM24"]
    #[inline(always)]
    pub fn adc12mem24(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM24)
    }
    #[doc = "Conversion start address ADC12MEM25"]
    #[inline(always)]
    pub fn adc12mem25(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM25)
    }
    #[doc = "Conversion start address ADC12MEM26"]
    #[inline(always)]
    pub fn adc12mem26(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM26)
    }
    #[doc = "Conversion start address ADC12MEM27"]
    #[inline(always)]
    pub fn adc12mem27(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM27)
    }
    #[doc = "Conversion start address ADC12MEM28"]
    #[inline(always)]
    pub fn adc12mem28(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM28)
    }
    #[doc = "Conversion start address ADC12MEM29"]
    #[inline(always)]
    pub fn adc12mem29(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM29)
    }
    #[doc = "Conversion start address ADC12MEM30"]
    #[inline(always)]
    pub fn adc12mem30(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM30)
    }
    #[doc = "Conversion start address ADC12MEM31"]
    #[inline(always)]
    pub fn adc12mem31(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12MEM31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "6:6\\]
1/2 AVCC ADC input channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12BATMAP_A {
    #[doc = "0: external pin is selected for ADC input channel A31"]
    ADC12BATMAP_0 = 0,
    #[doc = "1: ADC internal 1/2 x AVCC channel is selected for ADC input channel A31"]
    ADC12BATMAP_1 = 1,
}
impl From<ADC12BATMAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12BATMAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12BATMAP`"]
pub type ADC12BATMAP_R = crate::R<bool, ADC12BATMAP_A>;
impl ADC12BATMAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12BATMAP_A {
        match self.bits {
            false => ADC12BATMAP_A::ADC12BATMAP_0,
            true => ADC12BATMAP_A::ADC12BATMAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12BATMAP_0`"]
    #[inline(always)]
    pub fn is_adc12batmap_0(&self) -> bool {
        *self == ADC12BATMAP_A::ADC12BATMAP_0
    }
    #[doc = "Checks if the value of the field is `ADC12BATMAP_1`"]
    #[inline(always)]
    pub fn is_adc12batmap_1(&self) -> bool {
        *self == ADC12BATMAP_A::ADC12BATMAP_1
    }
}
#[doc = "Write proxy for field `ADC12BATMAP`"]
pub struct ADC12BATMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12BATMAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12BATMAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "external pin is selected for ADC input channel A31"]
    #[inline(always)]
    pub fn adc12batmap_0(self) -> &'a mut W {
        self.variant(ADC12BATMAP_A::ADC12BATMAP_0)
    }
    #[doc = "ADC internal 1/2 x AVCC channel is selected for ADC input channel A31"]
    #[inline(always)]
    pub fn adc12batmap_1(self) -> &'a mut W {
        self.variant(ADC12BATMAP_A::ADC12BATMAP_1)
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
temperature sensor ADC input channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12TCMAP_A {
    #[doc = "0: external pin is selected for ADC input channel A30"]
    ADC12TCMAP_0 = 0,
    #[doc = "1: ADC internal temperature sensor channel is selected for ADC input channel A30"]
    ADC12TCMAP_1 = 1,
}
impl From<ADC12TCMAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12TCMAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12TCMAP`"]
pub type ADC12TCMAP_R = crate::R<bool, ADC12TCMAP_A>;
impl ADC12TCMAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12TCMAP_A {
        match self.bits {
            false => ADC12TCMAP_A::ADC12TCMAP_0,
            true => ADC12TCMAP_A::ADC12TCMAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12TCMAP_0`"]
    #[inline(always)]
    pub fn is_adc12tcmap_0(&self) -> bool {
        *self == ADC12TCMAP_A::ADC12TCMAP_0
    }
    #[doc = "Checks if the value of the field is `ADC12TCMAP_1`"]
    #[inline(always)]
    pub fn is_adc12tcmap_1(&self) -> bool {
        *self == ADC12TCMAP_A::ADC12TCMAP_1
    }
}
#[doc = "Write proxy for field `ADC12TCMAP`"]
pub struct ADC12TCMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TCMAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12TCMAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "external pin is selected for ADC input channel A30"]
    #[inline(always)]
    pub fn adc12tcmap_0(self) -> &'a mut W {
        self.variant(ADC12TCMAP_A::ADC12TCMAP_0)
    }
    #[doc = "ADC internal temperature sensor channel is selected for ADC input channel A30"]
    #[inline(always)]
    pub fn adc12tcmap_1(self) -> &'a mut W {
        self.variant(ADC12TCMAP_A::ADC12TCMAP_1)
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
int ch 0 sel to ADC in ch A29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12ICH0MAP_A {
    #[doc = "0: external pin is selected for ADC input channel A29"]
    ADC12ICH0MAP_0 = 0,
    #[doc = "1: ADC input channel internal 0 is selected for ADC input channel A29, see device-specific data sheet for availability"]
    ADC12ICH0MAP_1 = 1,
}
impl From<ADC12ICH0MAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12ICH0MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12ICH0MAP`"]
pub type ADC12ICH0MAP_R = crate::R<bool, ADC12ICH0MAP_A>;
impl ADC12ICH0MAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12ICH0MAP_A {
        match self.bits {
            false => ADC12ICH0MAP_A::ADC12ICH0MAP_0,
            true => ADC12ICH0MAP_A::ADC12ICH0MAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12ICH0MAP_0`"]
    #[inline(always)]
    pub fn is_adc12ich0map_0(&self) -> bool {
        *self == ADC12ICH0MAP_A::ADC12ICH0MAP_0
    }
    #[doc = "Checks if the value of the field is `ADC12ICH0MAP_1`"]
    #[inline(always)]
    pub fn is_adc12ich0map_1(&self) -> bool {
        *self == ADC12ICH0MAP_A::ADC12ICH0MAP_1
    }
}
#[doc = "Write proxy for field `ADC12ICH0MAP`"]
pub struct ADC12ICH0MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ICH0MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12ICH0MAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "external pin is selected for ADC input channel A29"]
    #[inline(always)]
    pub fn adc12ich0map_0(self) -> &'a mut W {
        self.variant(ADC12ICH0MAP_A::ADC12ICH0MAP_0)
    }
    #[doc = "ADC input channel internal 0 is selected for ADC input channel A29, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn adc12ich0map_1(self) -> &'a mut W {
        self.variant(ADC12ICH0MAP_A::ADC12ICH0MAP_1)
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
int ch 1 sel to ADC in ch A28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12ICH1MAP_A {
    #[doc = "0: external pin is selected for ADC input channel A28"]
    ADC12ICH1MAP_0 = 0,
    #[doc = "1: ADC input channel internal 1 is selected for ADC input channel A28, see device-specific data sheet for availability"]
    ADC12ICH1MAP_1 = 1,
}
impl From<ADC12ICH1MAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12ICH1MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12ICH1MAP`"]
pub type ADC12ICH1MAP_R = crate::R<bool, ADC12ICH1MAP_A>;
impl ADC12ICH1MAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12ICH1MAP_A {
        match self.bits {
            false => ADC12ICH1MAP_A::ADC12ICH1MAP_0,
            true => ADC12ICH1MAP_A::ADC12ICH1MAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12ICH1MAP_0`"]
    #[inline(always)]
    pub fn is_adc12ich1map_0(&self) -> bool {
        *self == ADC12ICH1MAP_A::ADC12ICH1MAP_0
    }
    #[doc = "Checks if the value of the field is `ADC12ICH1MAP_1`"]
    #[inline(always)]
    pub fn is_adc12ich1map_1(&self) -> bool {
        *self == ADC12ICH1MAP_A::ADC12ICH1MAP_1
    }
}
#[doc = "Write proxy for field `ADC12ICH1MAP`"]
pub struct ADC12ICH1MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ICH1MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12ICH1MAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "external pin is selected for ADC input channel A28"]
    #[inline(always)]
    pub fn adc12ich1map_0(self) -> &'a mut W {
        self.variant(ADC12ICH1MAP_A::ADC12ICH1MAP_0)
    }
    #[doc = "ADC input channel internal 1 is selected for ADC input channel A28, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn adc12ich1map_1(self) -> &'a mut W {
        self.variant(ADC12ICH1MAP_A::ADC12ICH1MAP_1)
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
int ch 2 sel to ADC in ch A27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12ICH2MAP_A {
    #[doc = "0: external pin is selected for ADC input channel A27"]
    ADC12ICH2MAP_0 = 0,
    #[doc = "1: ADC input channel internal 2 is selected for ADC input channel A27, see device-specific data sheet for availability"]
    ADC12ICH2MAP_1 = 1,
}
impl From<ADC12ICH2MAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12ICH2MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12ICH2MAP`"]
pub type ADC12ICH2MAP_R = crate::R<bool, ADC12ICH2MAP_A>;
impl ADC12ICH2MAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12ICH2MAP_A {
        match self.bits {
            false => ADC12ICH2MAP_A::ADC12ICH2MAP_0,
            true => ADC12ICH2MAP_A::ADC12ICH2MAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12ICH2MAP_0`"]
    #[inline(always)]
    pub fn is_adc12ich2map_0(&self) -> bool {
        *self == ADC12ICH2MAP_A::ADC12ICH2MAP_0
    }
    #[doc = "Checks if the value of the field is `ADC12ICH2MAP_1`"]
    #[inline(always)]
    pub fn is_adc12ich2map_1(&self) -> bool {
        *self == ADC12ICH2MAP_A::ADC12ICH2MAP_1
    }
}
#[doc = "Write proxy for field `ADC12ICH2MAP`"]
pub struct ADC12ICH2MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ICH2MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12ICH2MAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "external pin is selected for ADC input channel A27"]
    #[inline(always)]
    pub fn adc12ich2map_0(self) -> &'a mut W {
        self.variant(ADC12ICH2MAP_A::ADC12ICH2MAP_0)
    }
    #[doc = "ADC input channel internal 2 is selected for ADC input channel A27, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn adc12ich2map_1(self) -> &'a mut W {
        self.variant(ADC12ICH2MAP_A::ADC12ICH2MAP_1)
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
int ch 3 sel to ADC in ch A26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12ICH3MAP_A {
    #[doc = "0: external pin is selected for ADC input channel A26"]
    ADC12ICH3MAP_0 = 0,
    #[doc = "1: ADC input channel internal 3 is selected for ADC input channel A26, see device-specific data sheet for availability"]
    ADC12ICH3MAP_1 = 1,
}
impl From<ADC12ICH3MAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12ICH3MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12ICH3MAP`"]
pub type ADC12ICH3MAP_R = crate::R<bool, ADC12ICH3MAP_A>;
impl ADC12ICH3MAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12ICH3MAP_A {
        match self.bits {
            false => ADC12ICH3MAP_A::ADC12ICH3MAP_0,
            true => ADC12ICH3MAP_A::ADC12ICH3MAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12ICH3MAP_0`"]
    #[inline(always)]
    pub fn is_adc12ich3map_0(&self) -> bool {
        *self == ADC12ICH3MAP_A::ADC12ICH3MAP_0
    }
    #[doc = "Checks if the value of the field is `ADC12ICH3MAP_1`"]
    #[inline(always)]
    pub fn is_adc12ich3map_1(&self) -> bool {
        *self == ADC12ICH3MAP_A::ADC12ICH3MAP_1
    }
}
#[doc = "Write proxy for field `ADC12ICH3MAP`"]
pub struct ADC12ICH3MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ICH3MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12ICH3MAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "external pin is selected for ADC input channel A26"]
    #[inline(always)]
    pub fn adc12ich3map_0(self) -> &'a mut W {
        self.variant(ADC12ICH3MAP_A::ADC12ICH3MAP_0)
    }
    #[doc = "ADC input channel internal 3 is selected for ADC input channel A26, see device-specific data sheet for availability"]
    #[inline(always)]
    pub fn adc12ich3map_1(self) -> &'a mut W {
        self.variant(ADC12ICH3MAP_A::ADC12ICH3MAP_1)
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
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
conversion start address"]
    #[inline(always)]
    pub fn adc12cstartadd(&self) -> ADC12CSTARTADD_R {
        ADC12CSTARTADD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
1/2 AVCC ADC input channel selection"]
    #[inline(always)]
    pub fn adc12batmap(&self) -> ADC12BATMAP_R {
        ADC12BATMAP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
temperature sensor ADC input channel selection"]
    #[inline(always)]
    pub fn adc12tcmap(&self) -> ADC12TCMAP_R {
        ADC12TCMAP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
int ch 0 sel to ADC in ch A29"]
    #[inline(always)]
    pub fn adc12ich0map(&self) -> ADC12ICH0MAP_R {
        ADC12ICH0MAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
int ch 1 sel to ADC in ch A28"]
    #[inline(always)]
    pub fn adc12ich1map(&self) -> ADC12ICH1MAP_R {
        ADC12ICH1MAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
int ch 2 sel to ADC in ch A27"]
    #[inline(always)]
    pub fn adc12ich2map(&self) -> ADC12ICH2MAP_R {
        ADC12ICH2MAP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
int ch 3 sel to ADC in ch A26"]
    #[inline(always)]
    pub fn adc12ich3map(&self) -> ADC12ICH3MAP_R {
        ADC12ICH3MAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
conversion start address"]
    #[inline(always)]
    pub fn adc12cstartadd(&mut self) -> ADC12CSTARTADD_W {
        ADC12CSTARTADD_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
1/2 AVCC ADC input channel selection"]
    #[inline(always)]
    pub fn adc12batmap(&mut self) -> ADC12BATMAP_W {
        ADC12BATMAP_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
temperature sensor ADC input channel selection"]
    #[inline(always)]
    pub fn adc12tcmap(&mut self) -> ADC12TCMAP_W {
        ADC12TCMAP_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
int ch 0 sel to ADC in ch A29"]
    #[inline(always)]
    pub fn adc12ich0map(&mut self) -> ADC12ICH0MAP_W {
        ADC12ICH0MAP_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
int ch 1 sel to ADC in ch A28"]
    #[inline(always)]
    pub fn adc12ich1map(&mut self) -> ADC12ICH1MAP_W {
        ADC12ICH1MAP_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
int ch 2 sel to ADC in ch A27"]
    #[inline(always)]
    pub fn adc12ich2map(&mut self) -> ADC12ICH2MAP_W {
        ADC12ICH2MAP_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
int ch 3 sel to ADC in ch A26"]
    #[inline(always)]
    pub fn adc12ich3map(&mut self) -> ADC12ICH3MAP_W {
        ADC12ICH3MAP_W { w: self }
    }
}
