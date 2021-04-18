#[doc = "Reader of register ADC12MCTL8"]
pub type R = crate::R<u16, super::ADC12MCTL8>;
#[doc = "Writer for register ADC12MCTL8"]
pub type W = crate::W<u16, super::ADC12MCTL8>;
#[doc = "Register ADC12MCTL8 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12MCTL8 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "4:0\\]
Input channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12INCH_A {
    #[doc = "0: If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    ADC12INCH_0 = 0,
    #[doc = "1: If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    ADC12INCH_1 = 1,
    #[doc = "2: If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    ADC12INCH_2 = 2,
    #[doc = "3: If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    ADC12INCH_3 = 3,
    #[doc = "4: If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    ADC12INCH_4 = 4,
    #[doc = "5: If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    ADC12INCH_5 = 5,
    #[doc = "6: If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    ADC12INCH_6 = 6,
    #[doc = "7: If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    ADC12INCH_7 = 7,
    #[doc = "8: If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    ADC12INCH_8 = 8,
    #[doc = "9: If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    ADC12INCH_9 = 9,
    #[doc = "10: If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    ADC12INCH_10 = 10,
    #[doc = "11: If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    ADC12INCH_11 = 11,
    #[doc = "12: If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    ADC12INCH_12 = 12,
    #[doc = "13: If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    ADC12INCH_13 = 13,
    #[doc = "14: If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    ADC12INCH_14 = 14,
    #[doc = "15: If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    ADC12INCH_15 = 15,
    #[doc = "16: If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    ADC12INCH_16 = 16,
    #[doc = "17: If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    ADC12INCH_17 = 17,
    #[doc = "18: If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    ADC12INCH_18 = 18,
    #[doc = "19: If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    ADC12INCH_19 = 19,
    #[doc = "20: If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    ADC12INCH_20 = 20,
    #[doc = "21: If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    ADC12INCH_21 = 21,
    #[doc = "22: If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    ADC12INCH_22 = 22,
    #[doc = "23: If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    ADC12INCH_23 = 23,
    #[doc = "24: If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    ADC12INCH_24 = 24,
    #[doc = "25: If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    ADC12INCH_25 = 25,
    #[doc = "26: If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27"]
    ADC12INCH_26 = 26,
    #[doc = "27: If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27"]
    ADC12INCH_27 = 27,
    #[doc = "28: If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    ADC12INCH_28 = 28,
    #[doc = "29: If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    ADC12INCH_29 = 29,
    #[doc = "30: If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    ADC12INCH_30 = 30,
    #[doc = "31: If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    ADC12INCH_31 = 31,
}
impl From<ADC12INCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12INCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12INCH`"]
pub type ADC12INCH_R = crate::R<u8, ADC12INCH_A>;
impl ADC12INCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12INCH_A {
        match self.bits {
            0 => ADC12INCH_A::ADC12INCH_0,
            1 => ADC12INCH_A::ADC12INCH_1,
            2 => ADC12INCH_A::ADC12INCH_2,
            3 => ADC12INCH_A::ADC12INCH_3,
            4 => ADC12INCH_A::ADC12INCH_4,
            5 => ADC12INCH_A::ADC12INCH_5,
            6 => ADC12INCH_A::ADC12INCH_6,
            7 => ADC12INCH_A::ADC12INCH_7,
            8 => ADC12INCH_A::ADC12INCH_8,
            9 => ADC12INCH_A::ADC12INCH_9,
            10 => ADC12INCH_A::ADC12INCH_10,
            11 => ADC12INCH_A::ADC12INCH_11,
            12 => ADC12INCH_A::ADC12INCH_12,
            13 => ADC12INCH_A::ADC12INCH_13,
            14 => ADC12INCH_A::ADC12INCH_14,
            15 => ADC12INCH_A::ADC12INCH_15,
            16 => ADC12INCH_A::ADC12INCH_16,
            17 => ADC12INCH_A::ADC12INCH_17,
            18 => ADC12INCH_A::ADC12INCH_18,
            19 => ADC12INCH_A::ADC12INCH_19,
            20 => ADC12INCH_A::ADC12INCH_20,
            21 => ADC12INCH_A::ADC12INCH_21,
            22 => ADC12INCH_A::ADC12INCH_22,
            23 => ADC12INCH_A::ADC12INCH_23,
            24 => ADC12INCH_A::ADC12INCH_24,
            25 => ADC12INCH_A::ADC12INCH_25,
            26 => ADC12INCH_A::ADC12INCH_26,
            27 => ADC12INCH_A::ADC12INCH_27,
            28 => ADC12INCH_A::ADC12INCH_28,
            29 => ADC12INCH_A::ADC12INCH_29,
            30 => ADC12INCH_A::ADC12INCH_30,
            31 => ADC12INCH_A::ADC12INCH_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_0`"]
    #[inline(always)]
    pub fn is_adc12inch_0(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_0
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_1`"]
    #[inline(always)]
    pub fn is_adc12inch_1(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_1
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_2`"]
    #[inline(always)]
    pub fn is_adc12inch_2(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_2
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_3`"]
    #[inline(always)]
    pub fn is_adc12inch_3(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_3
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_4`"]
    #[inline(always)]
    pub fn is_adc12inch_4(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_4
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_5`"]
    #[inline(always)]
    pub fn is_adc12inch_5(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_5
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_6`"]
    #[inline(always)]
    pub fn is_adc12inch_6(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_6
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_7`"]
    #[inline(always)]
    pub fn is_adc12inch_7(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_7
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_8`"]
    #[inline(always)]
    pub fn is_adc12inch_8(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_8
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_9`"]
    #[inline(always)]
    pub fn is_adc12inch_9(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_9
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_10`"]
    #[inline(always)]
    pub fn is_adc12inch_10(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_10
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_11`"]
    #[inline(always)]
    pub fn is_adc12inch_11(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_11
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_12`"]
    #[inline(always)]
    pub fn is_adc12inch_12(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_12
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_13`"]
    #[inline(always)]
    pub fn is_adc12inch_13(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_13
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_14`"]
    #[inline(always)]
    pub fn is_adc12inch_14(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_14
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_15`"]
    #[inline(always)]
    pub fn is_adc12inch_15(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_15
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_16`"]
    #[inline(always)]
    pub fn is_adc12inch_16(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_16
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_17`"]
    #[inline(always)]
    pub fn is_adc12inch_17(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_17
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_18`"]
    #[inline(always)]
    pub fn is_adc12inch_18(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_18
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_19`"]
    #[inline(always)]
    pub fn is_adc12inch_19(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_19
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_20`"]
    #[inline(always)]
    pub fn is_adc12inch_20(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_20
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_21`"]
    #[inline(always)]
    pub fn is_adc12inch_21(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_21
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_22`"]
    #[inline(always)]
    pub fn is_adc12inch_22(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_22
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_23`"]
    #[inline(always)]
    pub fn is_adc12inch_23(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_23
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_24`"]
    #[inline(always)]
    pub fn is_adc12inch_24(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_24
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_25`"]
    #[inline(always)]
    pub fn is_adc12inch_25(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_25
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_26`"]
    #[inline(always)]
    pub fn is_adc12inch_26(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_26
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_27`"]
    #[inline(always)]
    pub fn is_adc12inch_27(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_27
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_28`"]
    #[inline(always)]
    pub fn is_adc12inch_28(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_28
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_29`"]
    #[inline(always)]
    pub fn is_adc12inch_29(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_29
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_30`"]
    #[inline(always)]
    pub fn is_adc12inch_30(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_30
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_31`"]
    #[inline(always)]
    pub fn is_adc12inch_31(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_31
    }
}
#[doc = "Write proxy for field `ADC12INCH`"]
pub struct ADC12INCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12INCH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "If ADC12DIF = 0: A0; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc12inch_0(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_0)
    }
    #[doc = "If ADC12DIF = 0: A1; If ADC12DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc12inch_1(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_1)
    }
    #[doc = "If ADC12DIF = 0: A2; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc12inch_2(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_2)
    }
    #[doc = "If ADC12DIF = 0: A3; If ADC12DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc12inch_3(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_3)
    }
    #[doc = "If ADC12DIF = 0: A4; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc12inch_4(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_4)
    }
    #[doc = "If ADC12DIF = 0: A5; If ADC12DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc12inch_5(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_5)
    }
    #[doc = "If ADC12DIF = 0: A6; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc12inch_6(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_6)
    }
    #[doc = "If ADC12DIF = 0: A7; If ADC12DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc12inch_7(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_7)
    }
    #[doc = "If ADC12DIF = 0: A8; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc12inch_8(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_8)
    }
    #[doc = "If ADC12DIF = 0: A9; If ADC12DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc12inch_9(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_9)
    }
    #[doc = "If ADC12DIF = 0: A10; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc12inch_10(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_10)
    }
    #[doc = "If ADC12DIF = 0: A11; If ADC12DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc12inch_11(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_11)
    }
    #[doc = "If ADC12DIF = 0: A12; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc12inch_12(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_12)
    }
    #[doc = "If ADC12DIF = 0: A13; If ADC12DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc12inch_13(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_13)
    }
    #[doc = "If ADC12DIF = 0: A14; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc12inch_14(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_14)
    }
    #[doc = "If ADC12DIF = 0: A15; If ADC12DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc12inch_15(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_15)
    }
    #[doc = "If ADC12DIF = 0: A16; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc12inch_16(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_16)
    }
    #[doc = "If ADC12DIF = 0: A17; If ADC12DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc12inch_17(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_17)
    }
    #[doc = "If ADC12DIF = 0: A18; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc12inch_18(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_18)
    }
    #[doc = "If ADC12DIF = 0: A19; If ADC12DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc12inch_19(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_19)
    }
    #[doc = "If ADC12DIF = 0: A20; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc12inch_20(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_20)
    }
    #[doc = "If ADC12DIF = 0: A21; If ADC12DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc12inch_21(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_21)
    }
    #[doc = "If ADC12DIF = 0: A22; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc12inch_22(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_22)
    }
    #[doc = "If ADC12DIF = 0: A23; If ADC12DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc12inch_23(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_23)
    }
    #[doc = "If ADC12DIF = 0: A24; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc12inch_24(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_24)
    }
    #[doc = "If ADC12DIF = 0: A25; If ADC12DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc12inch_25(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_25)
    }
    #[doc = "If ADC12DIF = 0: A26; If ADC12DIF = 1: Ain+ = A26, Ain- =A27"]
    #[inline(always)]
    pub fn adc12inch_26(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_26)
    }
    #[doc = "If ADC12DIF = 0: A27; If ADC12DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn adc12inch_27(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_27)
    }
    #[doc = "If ADC12DIF = 0: A28; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc12inch_28(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_28)
    }
    #[doc = "If ADC12DIF = 0: A29; If ADC12DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc12inch_29(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_29)
    }
    #[doc = "If ADC12DIF = 0: A30; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc12inch_30(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_30)
    }
    #[doc = "If ADC12DIF = 0: A31; If ADC12DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc12inch_31(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "7:7\\]
End of sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12EOS_A {
    #[doc = "0: Not end of sequence"]
    ADC12EOS_0 = 0,
    #[doc = "1: End of sequence"]
    ADC12EOS_1 = 1,
}
impl From<ADC12EOS_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12EOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12EOS`"]
pub type ADC12EOS_R = crate::R<bool, ADC12EOS_A>;
impl ADC12EOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12EOS_A {
        match self.bits {
            false => ADC12EOS_A::ADC12EOS_0,
            true => ADC12EOS_A::ADC12EOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12EOS_0`"]
    #[inline(always)]
    pub fn is_adc12eos_0(&self) -> bool {
        *self == ADC12EOS_A::ADC12EOS_0
    }
    #[doc = "Checks if the value of the field is `ADC12EOS_1`"]
    #[inline(always)]
    pub fn is_adc12eos_1(&self) -> bool {
        *self == ADC12EOS_A::ADC12EOS_1
    }
}
#[doc = "Write proxy for field `ADC12EOS`"]
pub struct ADC12EOS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12EOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12EOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not end of sequence"]
    #[inline(always)]
    pub fn adc12eos_0(self) -> &'a mut W {
        self.variant(ADC12EOS_A::ADC12EOS_0)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn adc12eos_1(self) -> &'a mut W {
        self.variant(ADC12EOS_A::ADC12EOS_1)
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
reference selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12VRSEL_A {
    #[doc = "0: VR+ = AVCC, VR- = AVSS"]
    ADC12VRSEL_0 = 0,
    #[doc = "1: VR+ = VREF buffered, VR- = AVSS"]
    ADC12VRSEL_1 = 1,
    #[doc = "2: VR+ = VeREF-, VR- = AVSS"]
    ADC12VRSEL_2 = 2,
    #[doc = "3: VR+ = VeREF+ buffered, VR- = AVSS"]
    ADC12VRSEL_3 = 3,
    #[doc = "4: VR+ = VeREF+, VR- = AVSS"]
    ADC12VRSEL_4 = 4,
    #[doc = "5: VR+ = AVCC, VR- = VeREF+ buffered"]
    ADC12VRSEL_5 = 5,
    #[doc = "6: VR+ = AVCC, VR- = VeREF+"]
    ADC12VRSEL_6 = 6,
    #[doc = "7: VR+ = VREF buffered, VR- = VeREF+"]
    ADC12VRSEL_7 = 7,
    #[doc = "8: Reserved"]
    ADC12VRSEL_8 = 8,
    #[doc = "9: VR+ = AVCC, VR- = VREF buffered"]
    ADC12VRSEL_9 = 9,
    #[doc = "10: Reserved"]
    ADC12VRSEL_10 = 10,
    #[doc = "11: VR+ = VeREF+, VR- = VREF buffered"]
    ADC12VRSEL_11 = 11,
    #[doc = "12: VR+ = AVCC, VR- = VeREF-"]
    ADC12VRSEL_12 = 12,
    #[doc = "13: VR+ = VREF buffered, VR- = VeREF-"]
    ADC12VRSEL_13 = 13,
    #[doc = "14: VR+ = VeREF+, VR- = VeREF-"]
    ADC12VRSEL_14 = 14,
    #[doc = "15: VR+ = VeREF+ buffered, VR- = VeREF-"]
    ADC12VRSEL_15 = 15,
}
impl From<ADC12VRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12VRSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12VRSEL`"]
pub type ADC12VRSEL_R = crate::R<u8, ADC12VRSEL_A>;
impl ADC12VRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12VRSEL_A {
        match self.bits {
            0 => ADC12VRSEL_A::ADC12VRSEL_0,
            1 => ADC12VRSEL_A::ADC12VRSEL_1,
            2 => ADC12VRSEL_A::ADC12VRSEL_2,
            3 => ADC12VRSEL_A::ADC12VRSEL_3,
            4 => ADC12VRSEL_A::ADC12VRSEL_4,
            5 => ADC12VRSEL_A::ADC12VRSEL_5,
            6 => ADC12VRSEL_A::ADC12VRSEL_6,
            7 => ADC12VRSEL_A::ADC12VRSEL_7,
            8 => ADC12VRSEL_A::ADC12VRSEL_8,
            9 => ADC12VRSEL_A::ADC12VRSEL_9,
            10 => ADC12VRSEL_A::ADC12VRSEL_10,
            11 => ADC12VRSEL_A::ADC12VRSEL_11,
            12 => ADC12VRSEL_A::ADC12VRSEL_12,
            13 => ADC12VRSEL_A::ADC12VRSEL_13,
            14 => ADC12VRSEL_A::ADC12VRSEL_14,
            15 => ADC12VRSEL_A::ADC12VRSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_0`"]
    #[inline(always)]
    pub fn is_adc12vrsel_0(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_1`"]
    #[inline(always)]
    pub fn is_adc12vrsel_1(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_2`"]
    #[inline(always)]
    pub fn is_adc12vrsel_2(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_2
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_3`"]
    #[inline(always)]
    pub fn is_adc12vrsel_3(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_3
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_4`"]
    #[inline(always)]
    pub fn is_adc12vrsel_4(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_4
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_5`"]
    #[inline(always)]
    pub fn is_adc12vrsel_5(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_5
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_6`"]
    #[inline(always)]
    pub fn is_adc12vrsel_6(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_6
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_7`"]
    #[inline(always)]
    pub fn is_adc12vrsel_7(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_7
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_8`"]
    #[inline(always)]
    pub fn is_adc12vrsel_8(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_8
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_9`"]
    #[inline(always)]
    pub fn is_adc12vrsel_9(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_9
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_10`"]
    #[inline(always)]
    pub fn is_adc12vrsel_10(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_10
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_11`"]
    #[inline(always)]
    pub fn is_adc12vrsel_11(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_11
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_12`"]
    #[inline(always)]
    pub fn is_adc12vrsel_12(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_12
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_13`"]
    #[inline(always)]
    pub fn is_adc12vrsel_13(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_13
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_14`"]
    #[inline(always)]
    pub fn is_adc12vrsel_14(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_14
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_15`"]
    #[inline(always)]
    pub fn is_adc12vrsel_15(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_15
    }
}
#[doc = "Write proxy for field `ADC12VRSEL`"]
pub struct ADC12VRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12VRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12VRSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "VR+ = AVCC, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_0(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_0)
    }
    #[doc = "VR+ = VREF buffered, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_1(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_1)
    }
    #[doc = "VR+ = VeREF-, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_2(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_2)
    }
    #[doc = "VR+ = VeREF+ buffered, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_3(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_3)
    }
    #[doc = "VR+ = VeREF+, VR- = AVSS"]
    #[inline(always)]
    pub fn adc12vrsel_4(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_4)
    }
    #[doc = "VR+ = AVCC, VR- = VeREF+ buffered"]
    #[inline(always)]
    pub fn adc12vrsel_5(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_5)
    }
    #[doc = "VR+ = AVCC, VR- = VeREF+"]
    #[inline(always)]
    pub fn adc12vrsel_6(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_6)
    }
    #[doc = "VR+ = VREF buffered, VR- = VeREF+"]
    #[inline(always)]
    pub fn adc12vrsel_7(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_7)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12vrsel_8(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_8)
    }
    #[doc = "VR+ = AVCC, VR- = VREF buffered"]
    #[inline(always)]
    pub fn adc12vrsel_9(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_9)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adc12vrsel_10(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_10)
    }
    #[doc = "VR+ = VeREF+, VR- = VREF buffered"]
    #[inline(always)]
    pub fn adc12vrsel_11(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_11)
    }
    #[doc = "VR+ = AVCC, VR- = VeREF-"]
    #[inline(always)]
    pub fn adc12vrsel_12(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_12)
    }
    #[doc = "VR+ = VREF buffered, VR- = VeREF-"]
    #[inline(always)]
    pub fn adc12vrsel_13(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_13)
    }
    #[doc = "VR+ = VeREF+, VR- = VeREF-"]
    #[inline(always)]
    pub fn adc12vrsel_14(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_14)
    }
    #[doc = "VR+ = VeREF+ buffered, VR- = VeREF-"]
    #[inline(always)]
    pub fn adc12vrsel_15(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "13:13\\]
Differential mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12DIF_A {
    #[doc = "0: Single-ended mode enabled"]
    ADC12DIF_0 = 0,
    #[doc = "1: Differential mode enabled"]
    ADC12DIF_1 = 1,
}
impl From<ADC12DIF_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12DIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12DIF`"]
pub type ADC12DIF_R = crate::R<bool, ADC12DIF_A>;
impl ADC12DIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12DIF_A {
        match self.bits {
            false => ADC12DIF_A::ADC12DIF_0,
            true => ADC12DIF_A::ADC12DIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12DIF_0`"]
    #[inline(always)]
    pub fn is_adc12dif_0(&self) -> bool {
        *self == ADC12DIF_A::ADC12DIF_0
    }
    #[doc = "Checks if the value of the field is `ADC12DIF_1`"]
    #[inline(always)]
    pub fn is_adc12dif_1(&self) -> bool {
        *self == ADC12DIF_A::ADC12DIF_1
    }
}
#[doc = "Write proxy for field `ADC12DIF`"]
pub struct ADC12DIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12DIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12DIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single-ended mode enabled"]
    #[inline(always)]
    pub fn adc12dif_0(self) -> &'a mut W {
        self.variant(ADC12DIF_A::ADC12DIF_0)
    }
    #[doc = "Differential mode enabled"]
    #[inline(always)]
    pub fn adc12dif_1(self) -> &'a mut W {
        self.variant(ADC12DIF_A::ADC12DIF_1)
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
Comparator window enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12WINC_A {
    #[doc = "0: Comparator window disabled"]
    ADC12WINC_0 = 0,
    #[doc = "1: Comparator window enabled"]
    ADC12WINC_1 = 1,
}
impl From<ADC12WINC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12WINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12WINC`"]
pub type ADC12WINC_R = crate::R<bool, ADC12WINC_A>;
impl ADC12WINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12WINC_A {
        match self.bits {
            false => ADC12WINC_A::ADC12WINC_0,
            true => ADC12WINC_A::ADC12WINC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC12WINC_0`"]
    #[inline(always)]
    pub fn is_adc12winc_0(&self) -> bool {
        *self == ADC12WINC_A::ADC12WINC_0
    }
    #[doc = "Checks if the value of the field is `ADC12WINC_1`"]
    #[inline(always)]
    pub fn is_adc12winc_1(&self) -> bool {
        *self == ADC12WINC_A::ADC12WINC_1
    }
}
#[doc = "Write proxy for field `ADC12WINC`"]
pub struct ADC12WINC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12WINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12WINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator window disabled"]
    #[inline(always)]
    pub fn adc12winc_0(self) -> &'a mut W {
        self.variant(ADC12WINC_A::ADC12WINC_0)
    }
    #[doc = "Comparator window enabled"]
    #[inline(always)]
    pub fn adc12winc_1(self) -> &'a mut W {
        self.variant(ADC12WINC_A::ADC12WINC_1)
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
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Input channel select"]
    #[inline(always)]
    pub fn adc12inch(&self) -> ADC12INCH_R {
        ADC12INCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
End of sequence"]
    #[inline(always)]
    pub fn adc12eos(&self) -> ADC12EOS_R {
        ADC12EOS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
reference selection"]
    #[inline(always)]
    pub fn adc12vrsel(&self) -> ADC12VRSEL_R {
        ADC12VRSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Differential mode."]
    #[inline(always)]
    pub fn adc12dif(&self) -> ADC12DIF_R {
        ADC12DIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Comparator window enable"]
    #[inline(always)]
    pub fn adc12winc(&self) -> ADC12WINC_R {
        ADC12WINC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Input channel select"]
    #[inline(always)]
    pub fn adc12inch(&mut self) -> ADC12INCH_W {
        ADC12INCH_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
End of sequence"]
    #[inline(always)]
    pub fn adc12eos(&mut self) -> ADC12EOS_W {
        ADC12EOS_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
reference selection"]
    #[inline(always)]
    pub fn adc12vrsel(&mut self) -> ADC12VRSEL_W {
        ADC12VRSEL_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Differential mode."]
    #[inline(always)]
    pub fn adc12dif(&mut self) -> ADC12DIF_W {
        ADC12DIF_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Comparator window enable"]
    #[inline(always)]
    pub fn adc12winc(&mut self) -> ADC12WINC_W {
        ADC12WINC_W { w: self }
    }
}
