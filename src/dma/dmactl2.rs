#[doc = "Reader of register DMACTL2"]
pub type R = crate::R<u16, super::DMACTL2>;
#[doc = "Writer for register DMACTL2"]
pub type W = crate::W<u16, super::DMACTL2>;
#[doc = "Register DMACTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "4:0\\]
DMA trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA4TSEL_A {
    #[doc = "0: DMA4TRIG0"]
    DMA4TRIG0 = 0,
    #[doc = "1: DMA4TRIG1"]
    DMA4TRIG1 = 1,
    #[doc = "2: DMA4TRIG2"]
    DMA4TRIG2 = 2,
    #[doc = "3: DMA4TRIG3"]
    DMA4TRIG3 = 3,
    #[doc = "4: DMA4TRIG4"]
    DMA4TRIG4 = 4,
    #[doc = "5: DMA4TRIG5"]
    DMA4TRIG5 = 5,
    #[doc = "6: DMA4TRIG6"]
    DMA4TRIG6 = 6,
    #[doc = "7: DMA4TRIG7"]
    DMA4TRIG7 = 7,
    #[doc = "8: DMA4TRIG8"]
    DMA4TRIG8 = 8,
    #[doc = "9: DMA4TRIG9"]
    DMA4TRIG9 = 9,
    #[doc = "10: DMA4TRIG10"]
    DMA4TRIG10 = 10,
    #[doc = "11: DMA4TRIG11"]
    DMA4TRIG11 = 11,
    #[doc = "12: DMA4TRIG12"]
    DMA4TRIG12 = 12,
    #[doc = "13: DMA4TRIG13"]
    DMA4TRIG13 = 13,
    #[doc = "14: DMA4TRIG14"]
    DMA4TRIG14 = 14,
    #[doc = "15: DMA4TRIG15"]
    DMA4TRIG15 = 15,
    #[doc = "16: DMA4TRIG16"]
    DMA4TRIG16 = 16,
    #[doc = "17: DMA4TRIG17"]
    DMA4TRIG17 = 17,
    #[doc = "18: DMA4TRIG18"]
    DMA4TRIG18 = 18,
    #[doc = "19: DMA4TRIG19"]
    DMA4TRIG19 = 19,
    #[doc = "20: DMA4TRIG20"]
    DMA4TRIG20 = 20,
    #[doc = "21: DMA4TRIG21"]
    DMA4TRIG21 = 21,
    #[doc = "22: DMA4TRIG22"]
    DMA4TRIG22 = 22,
    #[doc = "23: DMA4TRIG23"]
    DMA4TRIG23 = 23,
    #[doc = "24: DMA4TRIG24"]
    DMA4TRIG24 = 24,
    #[doc = "25: DMA4TRIG25"]
    DMA4TRIG25 = 25,
    #[doc = "26: DMA4TRIG26"]
    DMA4TRIG26 = 26,
    #[doc = "27: DMA4TRIG27"]
    DMA4TRIG27 = 27,
    #[doc = "28: DMA4TRIG28"]
    DMA4TRIG28 = 28,
    #[doc = "29: DMA4TRIG29"]
    DMA4TRIG29 = 29,
    #[doc = "30: DMA4TRIG30"]
    DMA4TRIG30 = 30,
    #[doc = "31: DMA4TRIG31"]
    DMA4TRIG31 = 31,
}
impl From<DMA4TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA4TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA4TSEL`"]
pub type DMA4TSEL_R = crate::R<u8, DMA4TSEL_A>;
impl DMA4TSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA4TSEL_A {
        match self.bits {
            0 => DMA4TSEL_A::DMA4TRIG0,
            1 => DMA4TSEL_A::DMA4TRIG1,
            2 => DMA4TSEL_A::DMA4TRIG2,
            3 => DMA4TSEL_A::DMA4TRIG3,
            4 => DMA4TSEL_A::DMA4TRIG4,
            5 => DMA4TSEL_A::DMA4TRIG5,
            6 => DMA4TSEL_A::DMA4TRIG6,
            7 => DMA4TSEL_A::DMA4TRIG7,
            8 => DMA4TSEL_A::DMA4TRIG8,
            9 => DMA4TSEL_A::DMA4TRIG9,
            10 => DMA4TSEL_A::DMA4TRIG10,
            11 => DMA4TSEL_A::DMA4TRIG11,
            12 => DMA4TSEL_A::DMA4TRIG12,
            13 => DMA4TSEL_A::DMA4TRIG13,
            14 => DMA4TSEL_A::DMA4TRIG14,
            15 => DMA4TSEL_A::DMA4TRIG15,
            16 => DMA4TSEL_A::DMA4TRIG16,
            17 => DMA4TSEL_A::DMA4TRIG17,
            18 => DMA4TSEL_A::DMA4TRIG18,
            19 => DMA4TSEL_A::DMA4TRIG19,
            20 => DMA4TSEL_A::DMA4TRIG20,
            21 => DMA4TSEL_A::DMA4TRIG21,
            22 => DMA4TSEL_A::DMA4TRIG22,
            23 => DMA4TSEL_A::DMA4TRIG23,
            24 => DMA4TSEL_A::DMA4TRIG24,
            25 => DMA4TSEL_A::DMA4TRIG25,
            26 => DMA4TSEL_A::DMA4TRIG26,
            27 => DMA4TSEL_A::DMA4TRIG27,
            28 => DMA4TSEL_A::DMA4TRIG28,
            29 => DMA4TSEL_A::DMA4TRIG29,
            30 => DMA4TSEL_A::DMA4TRIG30,
            31 => DMA4TSEL_A::DMA4TRIG31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG0`"]
    #[inline(always)]
    pub fn is_dma4trig0(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG0
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG1`"]
    #[inline(always)]
    pub fn is_dma4trig1(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG1
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG2`"]
    #[inline(always)]
    pub fn is_dma4trig2(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG2
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG3`"]
    #[inline(always)]
    pub fn is_dma4trig3(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG3
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG4`"]
    #[inline(always)]
    pub fn is_dma4trig4(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG4
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG5`"]
    #[inline(always)]
    pub fn is_dma4trig5(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG5
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG6`"]
    #[inline(always)]
    pub fn is_dma4trig6(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG6
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG7`"]
    #[inline(always)]
    pub fn is_dma4trig7(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG7
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG8`"]
    #[inline(always)]
    pub fn is_dma4trig8(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG8
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG9`"]
    #[inline(always)]
    pub fn is_dma4trig9(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG9
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG10`"]
    #[inline(always)]
    pub fn is_dma4trig10(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG10
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG11`"]
    #[inline(always)]
    pub fn is_dma4trig11(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG11
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG12`"]
    #[inline(always)]
    pub fn is_dma4trig12(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG12
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG13`"]
    #[inline(always)]
    pub fn is_dma4trig13(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG13
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG14`"]
    #[inline(always)]
    pub fn is_dma4trig14(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG14
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG15`"]
    #[inline(always)]
    pub fn is_dma4trig15(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG15
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG16`"]
    #[inline(always)]
    pub fn is_dma4trig16(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG16
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG17`"]
    #[inline(always)]
    pub fn is_dma4trig17(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG17
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG18`"]
    #[inline(always)]
    pub fn is_dma4trig18(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG18
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG19`"]
    #[inline(always)]
    pub fn is_dma4trig19(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG19
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG20`"]
    #[inline(always)]
    pub fn is_dma4trig20(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG20
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG21`"]
    #[inline(always)]
    pub fn is_dma4trig21(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG21
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG22`"]
    #[inline(always)]
    pub fn is_dma4trig22(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG22
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG23`"]
    #[inline(always)]
    pub fn is_dma4trig23(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG23
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG24`"]
    #[inline(always)]
    pub fn is_dma4trig24(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG24
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG25`"]
    #[inline(always)]
    pub fn is_dma4trig25(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG25
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG26`"]
    #[inline(always)]
    pub fn is_dma4trig26(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG26
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG27`"]
    #[inline(always)]
    pub fn is_dma4trig27(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG27
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG28`"]
    #[inline(always)]
    pub fn is_dma4trig28(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG28
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG29`"]
    #[inline(always)]
    pub fn is_dma4trig29(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG29
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG30`"]
    #[inline(always)]
    pub fn is_dma4trig30(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG30
    }
    #[doc = "Checks if the value of the field is `DMA4TRIG31`"]
    #[inline(always)]
    pub fn is_dma4trig31(&self) -> bool {
        *self == DMA4TSEL_A::DMA4TRIG31
    }
}
#[doc = "Write proxy for field `DMA4TSEL`"]
pub struct DMA4TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA4TSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA4TSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA4TRIG0"]
    #[inline(always)]
    pub fn dma4trig0(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG0)
    }
    #[doc = "DMA4TRIG1"]
    #[inline(always)]
    pub fn dma4trig1(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG1)
    }
    #[doc = "DMA4TRIG2"]
    #[inline(always)]
    pub fn dma4trig2(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG2)
    }
    #[doc = "DMA4TRIG3"]
    #[inline(always)]
    pub fn dma4trig3(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG3)
    }
    #[doc = "DMA4TRIG4"]
    #[inline(always)]
    pub fn dma4trig4(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG4)
    }
    #[doc = "DMA4TRIG5"]
    #[inline(always)]
    pub fn dma4trig5(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG5)
    }
    #[doc = "DMA4TRIG6"]
    #[inline(always)]
    pub fn dma4trig6(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG6)
    }
    #[doc = "DMA4TRIG7"]
    #[inline(always)]
    pub fn dma4trig7(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG7)
    }
    #[doc = "DMA4TRIG8"]
    #[inline(always)]
    pub fn dma4trig8(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG8)
    }
    #[doc = "DMA4TRIG9"]
    #[inline(always)]
    pub fn dma4trig9(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG9)
    }
    #[doc = "DMA4TRIG10"]
    #[inline(always)]
    pub fn dma4trig10(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG10)
    }
    #[doc = "DMA4TRIG11"]
    #[inline(always)]
    pub fn dma4trig11(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG11)
    }
    #[doc = "DMA4TRIG12"]
    #[inline(always)]
    pub fn dma4trig12(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG12)
    }
    #[doc = "DMA4TRIG13"]
    #[inline(always)]
    pub fn dma4trig13(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG13)
    }
    #[doc = "DMA4TRIG14"]
    #[inline(always)]
    pub fn dma4trig14(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG14)
    }
    #[doc = "DMA4TRIG15"]
    #[inline(always)]
    pub fn dma4trig15(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG15)
    }
    #[doc = "DMA4TRIG16"]
    #[inline(always)]
    pub fn dma4trig16(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG16)
    }
    #[doc = "DMA4TRIG17"]
    #[inline(always)]
    pub fn dma4trig17(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG17)
    }
    #[doc = "DMA4TRIG18"]
    #[inline(always)]
    pub fn dma4trig18(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG18)
    }
    #[doc = "DMA4TRIG19"]
    #[inline(always)]
    pub fn dma4trig19(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG19)
    }
    #[doc = "DMA4TRIG20"]
    #[inline(always)]
    pub fn dma4trig20(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG20)
    }
    #[doc = "DMA4TRIG21"]
    #[inline(always)]
    pub fn dma4trig21(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG21)
    }
    #[doc = "DMA4TRIG22"]
    #[inline(always)]
    pub fn dma4trig22(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG22)
    }
    #[doc = "DMA4TRIG23"]
    #[inline(always)]
    pub fn dma4trig23(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG23)
    }
    #[doc = "DMA4TRIG24"]
    #[inline(always)]
    pub fn dma4trig24(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG24)
    }
    #[doc = "DMA4TRIG25"]
    #[inline(always)]
    pub fn dma4trig25(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG25)
    }
    #[doc = "DMA4TRIG26"]
    #[inline(always)]
    pub fn dma4trig26(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG26)
    }
    #[doc = "DMA4TRIG27"]
    #[inline(always)]
    pub fn dma4trig27(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG27)
    }
    #[doc = "DMA4TRIG28"]
    #[inline(always)]
    pub fn dma4trig28(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG28)
    }
    #[doc = "DMA4TRIG29"]
    #[inline(always)]
    pub fn dma4trig29(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG29)
    }
    #[doc = "DMA4TRIG30"]
    #[inline(always)]
    pub fn dma4trig30(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG30)
    }
    #[doc = "DMA4TRIG31"]
    #[inline(always)]
    pub fn dma4trig31(self) -> &'a mut W {
        self.variant(DMA4TSEL_A::DMA4TRIG31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "12:8\\]
DMA trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA5TSEL_A {
    #[doc = "0: DMA5TRIG0"]
    DMA5TRIG0 = 0,
    #[doc = "1: DMA5TRIG1"]
    DMA5TRIG1 = 1,
    #[doc = "2: DMA5TRIG2"]
    DMA5TRIG2 = 2,
    #[doc = "3: DMA5TRIG3"]
    DMA5TRIG3 = 3,
    #[doc = "4: DMA5TRIG4"]
    DMA5TRIG4 = 4,
    #[doc = "5: DMA5TRIG5"]
    DMA5TRIG5 = 5,
    #[doc = "6: DMA5TRIG6"]
    DMA5TRIG6 = 6,
    #[doc = "7: DMA5TRIG7"]
    DMA5TRIG7 = 7,
    #[doc = "8: DMA5TRIG8"]
    DMA5TRIG8 = 8,
    #[doc = "9: DMA5TRIG9"]
    DMA5TRIG9 = 9,
    #[doc = "10: DMA5TRIG10"]
    DMA5TRIG10 = 10,
    #[doc = "11: DMA5TRIG11"]
    DMA5TRIG11 = 11,
    #[doc = "12: DMA5TRIG12"]
    DMA5TRIG12 = 12,
    #[doc = "13: DMA5TRIG13"]
    DMA5TRIG13 = 13,
    #[doc = "14: DMA5TRIG14"]
    DMA5TRIG14 = 14,
    #[doc = "15: DMA5TRIG15"]
    DMA5TRIG15 = 15,
    #[doc = "16: DMA5TRIG16"]
    DMA5TRIG16 = 16,
    #[doc = "17: DMA5TRIG17"]
    DMA5TRIG17 = 17,
    #[doc = "18: DMA5TRIG18"]
    DMA5TRIG18 = 18,
    #[doc = "19: DMA5TRIG19"]
    DMA5TRIG19 = 19,
    #[doc = "20: DMA5TRIG20"]
    DMA5TRIG20 = 20,
    #[doc = "21: DMA5TRIG21"]
    DMA5TRIG21 = 21,
    #[doc = "22: DMA5TRIG22"]
    DMA5TRIG22 = 22,
    #[doc = "23: DMA5TRIG23"]
    DMA5TRIG23 = 23,
    #[doc = "24: DMA5TRIG24"]
    DMA5TRIG24 = 24,
    #[doc = "25: DMA5TRIG25"]
    DMA5TRIG25 = 25,
    #[doc = "26: DMA5TRIG26"]
    DMA5TRIG26 = 26,
    #[doc = "27: DMA5TRIG27"]
    DMA5TRIG27 = 27,
    #[doc = "28: DMA5TRIG28"]
    DMA5TRIG28 = 28,
    #[doc = "29: DMA5TRIG29"]
    DMA5TRIG29 = 29,
    #[doc = "30: DMA5TRIG30"]
    DMA5TRIG30 = 30,
    #[doc = "31: DMA5TRIG31"]
    DMA5TRIG31 = 31,
}
impl From<DMA5TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA5TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA5TSEL`"]
pub type DMA5TSEL_R = crate::R<u8, DMA5TSEL_A>;
impl DMA5TSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA5TSEL_A {
        match self.bits {
            0 => DMA5TSEL_A::DMA5TRIG0,
            1 => DMA5TSEL_A::DMA5TRIG1,
            2 => DMA5TSEL_A::DMA5TRIG2,
            3 => DMA5TSEL_A::DMA5TRIG3,
            4 => DMA5TSEL_A::DMA5TRIG4,
            5 => DMA5TSEL_A::DMA5TRIG5,
            6 => DMA5TSEL_A::DMA5TRIG6,
            7 => DMA5TSEL_A::DMA5TRIG7,
            8 => DMA5TSEL_A::DMA5TRIG8,
            9 => DMA5TSEL_A::DMA5TRIG9,
            10 => DMA5TSEL_A::DMA5TRIG10,
            11 => DMA5TSEL_A::DMA5TRIG11,
            12 => DMA5TSEL_A::DMA5TRIG12,
            13 => DMA5TSEL_A::DMA5TRIG13,
            14 => DMA5TSEL_A::DMA5TRIG14,
            15 => DMA5TSEL_A::DMA5TRIG15,
            16 => DMA5TSEL_A::DMA5TRIG16,
            17 => DMA5TSEL_A::DMA5TRIG17,
            18 => DMA5TSEL_A::DMA5TRIG18,
            19 => DMA5TSEL_A::DMA5TRIG19,
            20 => DMA5TSEL_A::DMA5TRIG20,
            21 => DMA5TSEL_A::DMA5TRIG21,
            22 => DMA5TSEL_A::DMA5TRIG22,
            23 => DMA5TSEL_A::DMA5TRIG23,
            24 => DMA5TSEL_A::DMA5TRIG24,
            25 => DMA5TSEL_A::DMA5TRIG25,
            26 => DMA5TSEL_A::DMA5TRIG26,
            27 => DMA5TSEL_A::DMA5TRIG27,
            28 => DMA5TSEL_A::DMA5TRIG28,
            29 => DMA5TSEL_A::DMA5TRIG29,
            30 => DMA5TSEL_A::DMA5TRIG30,
            31 => DMA5TSEL_A::DMA5TRIG31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG0`"]
    #[inline(always)]
    pub fn is_dma5trig0(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG0
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG1`"]
    #[inline(always)]
    pub fn is_dma5trig1(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG1
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG2`"]
    #[inline(always)]
    pub fn is_dma5trig2(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG2
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG3`"]
    #[inline(always)]
    pub fn is_dma5trig3(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG3
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG4`"]
    #[inline(always)]
    pub fn is_dma5trig4(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG4
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG5`"]
    #[inline(always)]
    pub fn is_dma5trig5(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG5
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG6`"]
    #[inline(always)]
    pub fn is_dma5trig6(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG6
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG7`"]
    #[inline(always)]
    pub fn is_dma5trig7(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG7
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG8`"]
    #[inline(always)]
    pub fn is_dma5trig8(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG8
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG9`"]
    #[inline(always)]
    pub fn is_dma5trig9(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG9
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG10`"]
    #[inline(always)]
    pub fn is_dma5trig10(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG10
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG11`"]
    #[inline(always)]
    pub fn is_dma5trig11(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG11
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG12`"]
    #[inline(always)]
    pub fn is_dma5trig12(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG12
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG13`"]
    #[inline(always)]
    pub fn is_dma5trig13(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG13
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG14`"]
    #[inline(always)]
    pub fn is_dma5trig14(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG14
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG15`"]
    #[inline(always)]
    pub fn is_dma5trig15(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG15
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG16`"]
    #[inline(always)]
    pub fn is_dma5trig16(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG16
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG17`"]
    #[inline(always)]
    pub fn is_dma5trig17(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG17
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG18`"]
    #[inline(always)]
    pub fn is_dma5trig18(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG18
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG19`"]
    #[inline(always)]
    pub fn is_dma5trig19(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG19
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG20`"]
    #[inline(always)]
    pub fn is_dma5trig20(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG20
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG21`"]
    #[inline(always)]
    pub fn is_dma5trig21(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG21
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG22`"]
    #[inline(always)]
    pub fn is_dma5trig22(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG22
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG23`"]
    #[inline(always)]
    pub fn is_dma5trig23(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG23
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG24`"]
    #[inline(always)]
    pub fn is_dma5trig24(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG24
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG25`"]
    #[inline(always)]
    pub fn is_dma5trig25(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG25
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG26`"]
    #[inline(always)]
    pub fn is_dma5trig26(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG26
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG27`"]
    #[inline(always)]
    pub fn is_dma5trig27(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG27
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG28`"]
    #[inline(always)]
    pub fn is_dma5trig28(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG28
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG29`"]
    #[inline(always)]
    pub fn is_dma5trig29(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG29
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG30`"]
    #[inline(always)]
    pub fn is_dma5trig30(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG30
    }
    #[doc = "Checks if the value of the field is `DMA5TRIG31`"]
    #[inline(always)]
    pub fn is_dma5trig31(&self) -> bool {
        *self == DMA5TSEL_A::DMA5TRIG31
    }
}
#[doc = "Write proxy for field `DMA5TSEL`"]
pub struct DMA5TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA5TSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA5TSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA5TRIG0"]
    #[inline(always)]
    pub fn dma5trig0(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG0)
    }
    #[doc = "DMA5TRIG1"]
    #[inline(always)]
    pub fn dma5trig1(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG1)
    }
    #[doc = "DMA5TRIG2"]
    #[inline(always)]
    pub fn dma5trig2(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG2)
    }
    #[doc = "DMA5TRIG3"]
    #[inline(always)]
    pub fn dma5trig3(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG3)
    }
    #[doc = "DMA5TRIG4"]
    #[inline(always)]
    pub fn dma5trig4(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG4)
    }
    #[doc = "DMA5TRIG5"]
    #[inline(always)]
    pub fn dma5trig5(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG5)
    }
    #[doc = "DMA5TRIG6"]
    #[inline(always)]
    pub fn dma5trig6(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG6)
    }
    #[doc = "DMA5TRIG7"]
    #[inline(always)]
    pub fn dma5trig7(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG7)
    }
    #[doc = "DMA5TRIG8"]
    #[inline(always)]
    pub fn dma5trig8(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG8)
    }
    #[doc = "DMA5TRIG9"]
    #[inline(always)]
    pub fn dma5trig9(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG9)
    }
    #[doc = "DMA5TRIG10"]
    #[inline(always)]
    pub fn dma5trig10(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG10)
    }
    #[doc = "DMA5TRIG11"]
    #[inline(always)]
    pub fn dma5trig11(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG11)
    }
    #[doc = "DMA5TRIG12"]
    #[inline(always)]
    pub fn dma5trig12(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG12)
    }
    #[doc = "DMA5TRIG13"]
    #[inline(always)]
    pub fn dma5trig13(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG13)
    }
    #[doc = "DMA5TRIG14"]
    #[inline(always)]
    pub fn dma5trig14(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG14)
    }
    #[doc = "DMA5TRIG15"]
    #[inline(always)]
    pub fn dma5trig15(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG15)
    }
    #[doc = "DMA5TRIG16"]
    #[inline(always)]
    pub fn dma5trig16(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG16)
    }
    #[doc = "DMA5TRIG17"]
    #[inline(always)]
    pub fn dma5trig17(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG17)
    }
    #[doc = "DMA5TRIG18"]
    #[inline(always)]
    pub fn dma5trig18(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG18)
    }
    #[doc = "DMA5TRIG19"]
    #[inline(always)]
    pub fn dma5trig19(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG19)
    }
    #[doc = "DMA5TRIG20"]
    #[inline(always)]
    pub fn dma5trig20(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG20)
    }
    #[doc = "DMA5TRIG21"]
    #[inline(always)]
    pub fn dma5trig21(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG21)
    }
    #[doc = "DMA5TRIG22"]
    #[inline(always)]
    pub fn dma5trig22(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG22)
    }
    #[doc = "DMA5TRIG23"]
    #[inline(always)]
    pub fn dma5trig23(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG23)
    }
    #[doc = "DMA5TRIG24"]
    #[inline(always)]
    pub fn dma5trig24(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG24)
    }
    #[doc = "DMA5TRIG25"]
    #[inline(always)]
    pub fn dma5trig25(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG25)
    }
    #[doc = "DMA5TRIG26"]
    #[inline(always)]
    pub fn dma5trig26(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG26)
    }
    #[doc = "DMA5TRIG27"]
    #[inline(always)]
    pub fn dma5trig27(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG27)
    }
    #[doc = "DMA5TRIG28"]
    #[inline(always)]
    pub fn dma5trig28(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG28)
    }
    #[doc = "DMA5TRIG29"]
    #[inline(always)]
    pub fn dma5trig29(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG29)
    }
    #[doc = "DMA5TRIG30"]
    #[inline(always)]
    pub fn dma5trig30(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG30)
    }
    #[doc = "DMA5TRIG31"]
    #[inline(always)]
    pub fn dma5trig31(self) -> &'a mut W {
        self.variant(DMA5TSEL_A::DMA5TRIG31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma4tsel(&self) -> DMA4TSEL_R {
        DMA4TSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma5tsel(&self) -> DMA5TSEL_R {
        DMA5TSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma4tsel(&mut self) -> DMA4TSEL_W {
        DMA4TSEL_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma5tsel(&mut self) -> DMA5TSEL_W {
        DMA5TSEL_W { w: self }
    }
}
