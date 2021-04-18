#[doc = "Reader of register ADC12IV"]
pub type R = crate::R<u16, super::ADC12IV>;
#[doc = "Writer for register ADC12IV"]
pub type W = crate::W<u16, super::ADC12IV>;
#[doc = "Register ADC12IV `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADC12IV_A {
    #[doc = "0: Interrupt Source: No interrupt pending, Interrupt Flag: None"]
    NONE = 0,
    #[doc = "2: Interrupt Source: ADC12MEMx overflow, Interrupt Flag: ADC12OVIFG, Interrupt Priority: Highest"]
    ADC12OVIFG = 2,
    #[doc = "4: Interrupt Source: Conversion time overflow, Interrupt Flag: ADC12TOVIFG"]
    ADC12TOVIFG = 4,
    #[doc = "6: Interrupt Source: ADC12 window high interrupt flag, Interrupt Flag: ADC12HIIFG"]
    ADC12HIIFG = 6,
    #[doc = "8: Interrupt Source: ADC12 window low interrupt flag, Interrupt Flag: ADC12LOIFG"]
    ADC12LOIFG = 8,
    #[doc = "10: Interrupt Source: ADC12 in-window interrupt flag, Interrupt Flag: ADC12INIFG"]
    ADC12INIFG = 10,
    #[doc = "12: Interrupt Source: ADC12MEM0 interrupt flag, Interrupt Flag: ADC12IFG0"]
    ADC12IFG0 = 12,
    #[doc = "14: Interrupt Source: ADC12MEM1 interrupt flag, Interrupt Flag: ADC12IFG1"]
    ADC12IFG1 = 14,
    #[doc = "16: Interrupt Source: ADC12MEM2 interrupt flag, Interrupt Flag: ADC12IFG2"]
    ADC12IFG2 = 16,
    #[doc = "18: Interrupt Source: ADC12MEM3 interrupt flag, Interrupt Flag: ADC12IFG3"]
    ADC12IFG3 = 18,
    #[doc = "20: Interrupt Source: ADC12MEM4 interrupt flag, Interrupt Flag: ADC12IFG4"]
    ADC12IFG4 = 20,
    #[doc = "22: Interrupt Source: ADC12MEM5 interrupt flag, Interrupt Flag: ADC12IFG5"]
    ADC12IFG5 = 22,
    #[doc = "24: Interrupt Source: ADC12MEM6 interrupt flag, Interrupt Flag: ADC12IFG6"]
    ADC12IFG6 = 24,
    #[doc = "26: Interrupt Source: ADC12MEM7 interrupt flag, Interrupt Flag: ADC12IFG7"]
    ADC12IFG7 = 26,
    #[doc = "28: Interrupt Source: ADC12MEM8 interrupt flag, Interrupt Flag: ADC12IFG8"]
    ADC12IFG8 = 28,
    #[doc = "30: Interrupt Source: ADC12MEM9 interrupt flag, Interrupt Flag: ADC12IFG9"]
    ADC12IFG9 = 30,
    #[doc = "32: Interrupt Source: ADC12MEM10 interrupt flag, Interrupt Flag: ADC12IFG10"]
    ADC12IFG10 = 32,
    #[doc = "34: Interrupt Source: ADC12MEM11 interrupt flag, Interrupt Flag: ADC12IFG11"]
    ADC12IFG11 = 34,
    #[doc = "36: Interrupt Source: ADC12MEM12 interrupt flag, Interrupt Flag: ADC12IFG12"]
    ADC12IFG12 = 36,
    #[doc = "38: Interrupt Source: ADC12MEM13 interrupt flag, Interrupt Flag: ADC12IFG13"]
    ADC12IFG13 = 38,
    #[doc = "40: Interrupt Source: ADC12MEM14 interrupt flag, Interrupt Flag: ADC12IFG14"]
    ADC12IFG14 = 40,
    #[doc = "42: Interrupt Source: ADC12MEM15 interrupt flag, Interrupt Flag: ADC12IFG15"]
    ADC12IFG15 = 42,
    #[doc = "44: Interrupt Source: ADC12MEM16 interrupt flag, Interrupt Flag: ADC12IFG16"]
    ADC12IFG16 = 44,
    #[doc = "46: Interrupt Source: ADC12MEM17 interrupt flag, Interrupt Flag: ADC12IFG17"]
    ADC12IFG17 = 46,
    #[doc = "48: Interrupt Source: ADC12MEM18 interrupt flag, Interrupt Flag: ADC12IFG18"]
    ADC12IFG18 = 48,
    #[doc = "50: Interrupt Source: ADC12MEM19 interrupt flag, Interrupt Flag: ADC12IFG19"]
    ADC12IFG19 = 50,
    #[doc = "52: Interrupt Source: ADC12MEM20 interrupt flag, Interrupt Flag: ADC12IFG20"]
    ADC12IFG20 = 52,
    #[doc = "54: Interrupt Source: ADC12MEM21 interrupt flag, Interrupt Flag: ADC12IFG21"]
    ADC12IFG21 = 54,
    #[doc = "56: Interrupt Source: ADC12MEM22 interrupt flag, Interrupt Flag: ADC12IFG22"]
    ADC12IFG22 = 56,
    #[doc = "58: Interrupt Source: ADC12MEM23 interrupt flag, Interrupt Flag: ADC12IFG23"]
    ADC12IFG23 = 58,
    #[doc = "60: Interrupt Source: ADC12MEM24 interrupt flag, Interrupt Flag: ADC12IFG24"]
    ADC12IFG24 = 60,
    #[doc = "62: Interrupt Source: ADC12MEM25 interrupt flag, Interrupt Flag: ADC12IFG25"]
    ADC12IFG25 = 62,
    #[doc = "64: Interrupt Source: ADC12MEM26 interrupt flag, Interrupt Flag: ADC12IFG26"]
    ADC12IFG26 = 64,
    #[doc = "66: Interrupt Source: ADC12MEM27 interrupt flag, Interrupt Flag: ADC12IFG27"]
    ADC12IFG27 = 66,
    #[doc = "68: Interrupt Source: ADC12MEM28 interrupt flag, Interrupt Flag: ADC12IFG28"]
    ADC12IFG28 = 68,
    #[doc = "70: Interrupt Source: ADC12MEM29 interrupt flag, Interrupt Flag: ADC12IFG29"]
    ADC12IFG29 = 70,
    #[doc = "72: Interrupt Source: ADC12MEM30 interrupt flag, Interrupt Flag: ADC12IFG30"]
    ADC12IFG30 = 72,
    #[doc = "74: Interrupt Source: ADC12MEM31 interrupt flag, Interrupt Flag: ADC12IFG31"]
    ADC12IFG31 = 74,
    #[doc = "76: Interrupt Source: ADC12RDYIFG interrupt flag, Interrupt Flag: ADC12RDYIFG"]
    ADC12RDYIFG = 76,
}
impl From<ADC12IV_A> for u16 {
    #[inline(always)]
    fn from(variant: ADC12IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12IV`"]
pub type ADC12IV_R = crate::R<u16, ADC12IV_A>;
impl ADC12IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ADC12IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC12IV_A::NONE),
            2 => Val(ADC12IV_A::ADC12OVIFG),
            4 => Val(ADC12IV_A::ADC12TOVIFG),
            6 => Val(ADC12IV_A::ADC12HIIFG),
            8 => Val(ADC12IV_A::ADC12LOIFG),
            10 => Val(ADC12IV_A::ADC12INIFG),
            12 => Val(ADC12IV_A::ADC12IFG0),
            14 => Val(ADC12IV_A::ADC12IFG1),
            16 => Val(ADC12IV_A::ADC12IFG2),
            18 => Val(ADC12IV_A::ADC12IFG3),
            20 => Val(ADC12IV_A::ADC12IFG4),
            22 => Val(ADC12IV_A::ADC12IFG5),
            24 => Val(ADC12IV_A::ADC12IFG6),
            26 => Val(ADC12IV_A::ADC12IFG7),
            28 => Val(ADC12IV_A::ADC12IFG8),
            30 => Val(ADC12IV_A::ADC12IFG9),
            32 => Val(ADC12IV_A::ADC12IFG10),
            34 => Val(ADC12IV_A::ADC12IFG11),
            36 => Val(ADC12IV_A::ADC12IFG12),
            38 => Val(ADC12IV_A::ADC12IFG13),
            40 => Val(ADC12IV_A::ADC12IFG14),
            42 => Val(ADC12IV_A::ADC12IFG15),
            44 => Val(ADC12IV_A::ADC12IFG16),
            46 => Val(ADC12IV_A::ADC12IFG17),
            48 => Val(ADC12IV_A::ADC12IFG18),
            50 => Val(ADC12IV_A::ADC12IFG19),
            52 => Val(ADC12IV_A::ADC12IFG20),
            54 => Val(ADC12IV_A::ADC12IFG21),
            56 => Val(ADC12IV_A::ADC12IFG22),
            58 => Val(ADC12IV_A::ADC12IFG23),
            60 => Val(ADC12IV_A::ADC12IFG24),
            62 => Val(ADC12IV_A::ADC12IFG25),
            64 => Val(ADC12IV_A::ADC12IFG26),
            66 => Val(ADC12IV_A::ADC12IFG27),
            68 => Val(ADC12IV_A::ADC12IFG28),
            70 => Val(ADC12IV_A::ADC12IFG29),
            72 => Val(ADC12IV_A::ADC12IFG30),
            74 => Val(ADC12IV_A::ADC12IFG31),
            76 => Val(ADC12IV_A::ADC12RDYIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ADC12IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `ADC12OVIFG`"]
    #[inline(always)]
    pub fn is_adc12ovifg(&self) -> bool {
        *self == ADC12IV_A::ADC12OVIFG
    }
    #[doc = "Checks if the value of the field is `ADC12TOVIFG`"]
    #[inline(always)]
    pub fn is_adc12tovifg(&self) -> bool {
        *self == ADC12IV_A::ADC12TOVIFG
    }
    #[doc = "Checks if the value of the field is `ADC12HIIFG`"]
    #[inline(always)]
    pub fn is_adc12hiifg(&self) -> bool {
        *self == ADC12IV_A::ADC12HIIFG
    }
    #[doc = "Checks if the value of the field is `ADC12LOIFG`"]
    #[inline(always)]
    pub fn is_adc12loifg(&self) -> bool {
        *self == ADC12IV_A::ADC12LOIFG
    }
    #[doc = "Checks if the value of the field is `ADC12INIFG`"]
    #[inline(always)]
    pub fn is_adc12inifg(&self) -> bool {
        *self == ADC12IV_A::ADC12INIFG
    }
    #[doc = "Checks if the value of the field is `ADC12IFG0`"]
    #[inline(always)]
    pub fn is_adc12ifg0(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG0
    }
    #[doc = "Checks if the value of the field is `ADC12IFG1`"]
    #[inline(always)]
    pub fn is_adc12ifg1(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG1
    }
    #[doc = "Checks if the value of the field is `ADC12IFG2`"]
    #[inline(always)]
    pub fn is_adc12ifg2(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG2
    }
    #[doc = "Checks if the value of the field is `ADC12IFG3`"]
    #[inline(always)]
    pub fn is_adc12ifg3(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG3
    }
    #[doc = "Checks if the value of the field is `ADC12IFG4`"]
    #[inline(always)]
    pub fn is_adc12ifg4(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG4
    }
    #[doc = "Checks if the value of the field is `ADC12IFG5`"]
    #[inline(always)]
    pub fn is_adc12ifg5(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG5
    }
    #[doc = "Checks if the value of the field is `ADC12IFG6`"]
    #[inline(always)]
    pub fn is_adc12ifg6(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG6
    }
    #[doc = "Checks if the value of the field is `ADC12IFG7`"]
    #[inline(always)]
    pub fn is_adc12ifg7(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG7
    }
    #[doc = "Checks if the value of the field is `ADC12IFG8`"]
    #[inline(always)]
    pub fn is_adc12ifg8(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG8
    }
    #[doc = "Checks if the value of the field is `ADC12IFG9`"]
    #[inline(always)]
    pub fn is_adc12ifg9(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG9
    }
    #[doc = "Checks if the value of the field is `ADC12IFG10`"]
    #[inline(always)]
    pub fn is_adc12ifg10(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG10
    }
    #[doc = "Checks if the value of the field is `ADC12IFG11`"]
    #[inline(always)]
    pub fn is_adc12ifg11(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG11
    }
    #[doc = "Checks if the value of the field is `ADC12IFG12`"]
    #[inline(always)]
    pub fn is_adc12ifg12(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG12
    }
    #[doc = "Checks if the value of the field is `ADC12IFG13`"]
    #[inline(always)]
    pub fn is_adc12ifg13(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG13
    }
    #[doc = "Checks if the value of the field is `ADC12IFG14`"]
    #[inline(always)]
    pub fn is_adc12ifg14(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG14
    }
    #[doc = "Checks if the value of the field is `ADC12IFG15`"]
    #[inline(always)]
    pub fn is_adc12ifg15(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG15
    }
    #[doc = "Checks if the value of the field is `ADC12IFG16`"]
    #[inline(always)]
    pub fn is_adc12ifg16(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG16
    }
    #[doc = "Checks if the value of the field is `ADC12IFG17`"]
    #[inline(always)]
    pub fn is_adc12ifg17(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG17
    }
    #[doc = "Checks if the value of the field is `ADC12IFG18`"]
    #[inline(always)]
    pub fn is_adc12ifg18(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG18
    }
    #[doc = "Checks if the value of the field is `ADC12IFG19`"]
    #[inline(always)]
    pub fn is_adc12ifg19(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG19
    }
    #[doc = "Checks if the value of the field is `ADC12IFG20`"]
    #[inline(always)]
    pub fn is_adc12ifg20(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG20
    }
    #[doc = "Checks if the value of the field is `ADC12IFG21`"]
    #[inline(always)]
    pub fn is_adc12ifg21(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG21
    }
    #[doc = "Checks if the value of the field is `ADC12IFG22`"]
    #[inline(always)]
    pub fn is_adc12ifg22(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG22
    }
    #[doc = "Checks if the value of the field is `ADC12IFG23`"]
    #[inline(always)]
    pub fn is_adc12ifg23(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG23
    }
    #[doc = "Checks if the value of the field is `ADC12IFG24`"]
    #[inline(always)]
    pub fn is_adc12ifg24(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG24
    }
    #[doc = "Checks if the value of the field is `ADC12IFG25`"]
    #[inline(always)]
    pub fn is_adc12ifg25(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG25
    }
    #[doc = "Checks if the value of the field is `ADC12IFG26`"]
    #[inline(always)]
    pub fn is_adc12ifg26(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG26
    }
    #[doc = "Checks if the value of the field is `ADC12IFG27`"]
    #[inline(always)]
    pub fn is_adc12ifg27(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG27
    }
    #[doc = "Checks if the value of the field is `ADC12IFG28`"]
    #[inline(always)]
    pub fn is_adc12ifg28(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG28
    }
    #[doc = "Checks if the value of the field is `ADC12IFG29`"]
    #[inline(always)]
    pub fn is_adc12ifg29(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG29
    }
    #[doc = "Checks if the value of the field is `ADC12IFG30`"]
    #[inline(always)]
    pub fn is_adc12ifg30(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG30
    }
    #[doc = "Checks if the value of the field is `ADC12IFG31`"]
    #[inline(always)]
    pub fn is_adc12ifg31(&self) -> bool {
        *self == ADC12IV_A::ADC12IFG31
    }
    #[doc = "Checks if the value of the field is `ADC12RDYIFG`"]
    #[inline(always)]
    pub fn is_adc12rdyifg(&self) -> bool {
        *self == ADC12IV_A::ADC12RDYIFG
    }
}
#[doc = "Write proxy for field `ADC12IV`"]
pub struct ADC12IV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12IV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt Source: No interrupt pending, Interrupt Flag: None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ADC12IV_A::NONE)
    }
    #[doc = "Interrupt Source: ADC12MEMx overflow, Interrupt Flag: ADC12OVIFG, Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn adc12ovifg(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12OVIFG)
    }
    #[doc = "Interrupt Source: Conversion time overflow, Interrupt Flag: ADC12TOVIFG"]
    #[inline(always)]
    pub fn adc12tovifg(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12TOVIFG)
    }
    #[doc = "Interrupt Source: ADC12 window high interrupt flag, Interrupt Flag: ADC12HIIFG"]
    #[inline(always)]
    pub fn adc12hiifg(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12HIIFG)
    }
    #[doc = "Interrupt Source: ADC12 window low interrupt flag, Interrupt Flag: ADC12LOIFG"]
    #[inline(always)]
    pub fn adc12loifg(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12LOIFG)
    }
    #[doc = "Interrupt Source: ADC12 in-window interrupt flag, Interrupt Flag: ADC12INIFG"]
    #[inline(always)]
    pub fn adc12inifg(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12INIFG)
    }
    #[doc = "Interrupt Source: ADC12MEM0 interrupt flag, Interrupt Flag: ADC12IFG0"]
    #[inline(always)]
    pub fn adc12ifg0(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG0)
    }
    #[doc = "Interrupt Source: ADC12MEM1 interrupt flag, Interrupt Flag: ADC12IFG1"]
    #[inline(always)]
    pub fn adc12ifg1(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG1)
    }
    #[doc = "Interrupt Source: ADC12MEM2 interrupt flag, Interrupt Flag: ADC12IFG2"]
    #[inline(always)]
    pub fn adc12ifg2(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG2)
    }
    #[doc = "Interrupt Source: ADC12MEM3 interrupt flag, Interrupt Flag: ADC12IFG3"]
    #[inline(always)]
    pub fn adc12ifg3(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG3)
    }
    #[doc = "Interrupt Source: ADC12MEM4 interrupt flag, Interrupt Flag: ADC12IFG4"]
    #[inline(always)]
    pub fn adc12ifg4(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG4)
    }
    #[doc = "Interrupt Source: ADC12MEM5 interrupt flag, Interrupt Flag: ADC12IFG5"]
    #[inline(always)]
    pub fn adc12ifg5(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG5)
    }
    #[doc = "Interrupt Source: ADC12MEM6 interrupt flag, Interrupt Flag: ADC12IFG6"]
    #[inline(always)]
    pub fn adc12ifg6(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG6)
    }
    #[doc = "Interrupt Source: ADC12MEM7 interrupt flag, Interrupt Flag: ADC12IFG7"]
    #[inline(always)]
    pub fn adc12ifg7(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG7)
    }
    #[doc = "Interrupt Source: ADC12MEM8 interrupt flag, Interrupt Flag: ADC12IFG8"]
    #[inline(always)]
    pub fn adc12ifg8(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG8)
    }
    #[doc = "Interrupt Source: ADC12MEM9 interrupt flag, Interrupt Flag: ADC12IFG9"]
    #[inline(always)]
    pub fn adc12ifg9(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG9)
    }
    #[doc = "Interrupt Source: ADC12MEM10 interrupt flag, Interrupt Flag: ADC12IFG10"]
    #[inline(always)]
    pub fn adc12ifg10(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG10)
    }
    #[doc = "Interrupt Source: ADC12MEM11 interrupt flag, Interrupt Flag: ADC12IFG11"]
    #[inline(always)]
    pub fn adc12ifg11(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG11)
    }
    #[doc = "Interrupt Source: ADC12MEM12 interrupt flag, Interrupt Flag: ADC12IFG12"]
    #[inline(always)]
    pub fn adc12ifg12(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG12)
    }
    #[doc = "Interrupt Source: ADC12MEM13 interrupt flag, Interrupt Flag: ADC12IFG13"]
    #[inline(always)]
    pub fn adc12ifg13(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG13)
    }
    #[doc = "Interrupt Source: ADC12MEM14 interrupt flag, Interrupt Flag: ADC12IFG14"]
    #[inline(always)]
    pub fn adc12ifg14(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG14)
    }
    #[doc = "Interrupt Source: ADC12MEM15 interrupt flag, Interrupt Flag: ADC12IFG15"]
    #[inline(always)]
    pub fn adc12ifg15(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG15)
    }
    #[doc = "Interrupt Source: ADC12MEM16 interrupt flag, Interrupt Flag: ADC12IFG16"]
    #[inline(always)]
    pub fn adc12ifg16(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG16)
    }
    #[doc = "Interrupt Source: ADC12MEM17 interrupt flag, Interrupt Flag: ADC12IFG17"]
    #[inline(always)]
    pub fn adc12ifg17(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG17)
    }
    #[doc = "Interrupt Source: ADC12MEM18 interrupt flag, Interrupt Flag: ADC12IFG18"]
    #[inline(always)]
    pub fn adc12ifg18(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG18)
    }
    #[doc = "Interrupt Source: ADC12MEM19 interrupt flag, Interrupt Flag: ADC12IFG19"]
    #[inline(always)]
    pub fn adc12ifg19(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG19)
    }
    #[doc = "Interrupt Source: ADC12MEM20 interrupt flag, Interrupt Flag: ADC12IFG20"]
    #[inline(always)]
    pub fn adc12ifg20(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG20)
    }
    #[doc = "Interrupt Source: ADC12MEM21 interrupt flag, Interrupt Flag: ADC12IFG21"]
    #[inline(always)]
    pub fn adc12ifg21(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG21)
    }
    #[doc = "Interrupt Source: ADC12MEM22 interrupt flag, Interrupt Flag: ADC12IFG22"]
    #[inline(always)]
    pub fn adc12ifg22(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG22)
    }
    #[doc = "Interrupt Source: ADC12MEM23 interrupt flag, Interrupt Flag: ADC12IFG23"]
    #[inline(always)]
    pub fn adc12ifg23(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG23)
    }
    #[doc = "Interrupt Source: ADC12MEM24 interrupt flag, Interrupt Flag: ADC12IFG24"]
    #[inline(always)]
    pub fn adc12ifg24(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG24)
    }
    #[doc = "Interrupt Source: ADC12MEM25 interrupt flag, Interrupt Flag: ADC12IFG25"]
    #[inline(always)]
    pub fn adc12ifg25(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG25)
    }
    #[doc = "Interrupt Source: ADC12MEM26 interrupt flag, Interrupt Flag: ADC12IFG26"]
    #[inline(always)]
    pub fn adc12ifg26(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG26)
    }
    #[doc = "Interrupt Source: ADC12MEM27 interrupt flag, Interrupt Flag: ADC12IFG27"]
    #[inline(always)]
    pub fn adc12ifg27(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG27)
    }
    #[doc = "Interrupt Source: ADC12MEM28 interrupt flag, Interrupt Flag: ADC12IFG28"]
    #[inline(always)]
    pub fn adc12ifg28(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG28)
    }
    #[doc = "Interrupt Source: ADC12MEM29 interrupt flag, Interrupt Flag: ADC12IFG29"]
    #[inline(always)]
    pub fn adc12ifg29(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG29)
    }
    #[doc = "Interrupt Source: ADC12MEM30 interrupt flag, Interrupt Flag: ADC12IFG30"]
    #[inline(always)]
    pub fn adc12ifg30(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG30)
    }
    #[doc = "Interrupt Source: ADC12MEM31 interrupt flag, Interrupt Flag: ADC12IFG31"]
    #[inline(always)]
    pub fn adc12ifg31(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12IFG31)
    }
    #[doc = "Interrupt Source: ADC12RDYIFG interrupt flag, Interrupt Flag: ADC12RDYIFG"]
    #[inline(always)]
    pub fn adc12rdyifg(self) -> &'a mut W {
        self.variant(ADC12IV_A::ADC12RDYIFG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
interrupt vector value"]
    #[inline(always)]
    pub fn adc12iv(&self) -> ADC12IV_R {
        ADC12IV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
interrupt vector value"]
    #[inline(always)]
    pub fn adc12iv(&mut self) -> ADC12IV_W {
        ADC12IV_W { w: self }
    }
}
