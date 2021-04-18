#[doc = "Reader of register DMACTL0"]
pub type R = crate::R<u16, super::DMACTL0>;
#[doc = "Writer for register DMACTL0"]
pub type W = crate::W<u16, super::DMACTL0>;
#[doc = "Register DMACTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACTL0 {
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
pub enum DMA0TSEL_A {
    #[doc = "0: DMA0TRIG0"]
    DMA0TRIG0 = 0,
    #[doc = "1: DMA0TRIG1"]
    DMA0TRIG1 = 1,
    #[doc = "2: DMA0TRIG2"]
    DMA0TRIG2 = 2,
    #[doc = "3: DMA0TRIG3"]
    DMA0TRIG3 = 3,
    #[doc = "4: DMA0TRIG4"]
    DMA0TRIG4 = 4,
    #[doc = "5: DMA0TRIG5"]
    DMA0TRIG5 = 5,
    #[doc = "6: DMA0TRIG6"]
    DMA0TRIG6 = 6,
    #[doc = "7: DMA0TRIG7"]
    DMA0TRIG7 = 7,
    #[doc = "8: DMA0TRIG8"]
    DMA0TRIG8 = 8,
    #[doc = "9: DMA0TRIG9"]
    DMA0TRIG9 = 9,
    #[doc = "10: DMA0TRIG10"]
    DMA0TRIG10 = 10,
    #[doc = "11: DMA0TRIG11"]
    DMA0TRIG11 = 11,
    #[doc = "12: DMA0TRIG12"]
    DMA0TRIG12 = 12,
    #[doc = "13: DMA0TRIG13"]
    DMA0TRIG13 = 13,
    #[doc = "14: DMA0TRIG14"]
    DMA0TRIG14 = 14,
    #[doc = "15: DMA0TRIG15"]
    DMA0TRIG15 = 15,
    #[doc = "16: DMA0TRIG16"]
    DMA0TRIG16 = 16,
    #[doc = "17: DMA0TRIG17"]
    DMA0TRIG17 = 17,
    #[doc = "18: DMA0TRIG18"]
    DMA0TRIG18 = 18,
    #[doc = "19: DMA0TRIG19"]
    DMA0TRIG19 = 19,
    #[doc = "20: DMA0TRIG20"]
    DMA0TRIG20 = 20,
    #[doc = "21: DMA0TRIG21"]
    DMA0TRIG21 = 21,
    #[doc = "22: DMA0TRIG22"]
    DMA0TRIG22 = 22,
    #[doc = "23: DMA0TRIG23"]
    DMA0TRIG23 = 23,
    #[doc = "24: DMA0TRIG24"]
    DMA0TRIG24 = 24,
    #[doc = "25: DMA0TRIG25"]
    DMA0TRIG25 = 25,
    #[doc = "26: DMA0TRIG26"]
    DMA0TRIG26 = 26,
    #[doc = "27: DMA0TRIG27"]
    DMA0TRIG27 = 27,
    #[doc = "28: DMA0TRIG28"]
    DMA0TRIG28 = 28,
    #[doc = "29: DMA0TRIG29"]
    DMA0TRIG29 = 29,
    #[doc = "30: DMA0TRIG30"]
    DMA0TRIG30 = 30,
    #[doc = "31: DMA0TRIG31"]
    DMA0TRIG31 = 31,
}
impl From<DMA0TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA0TSEL`"]
pub type DMA0TSEL_R = crate::R<u8, DMA0TSEL_A>;
impl DMA0TSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0TSEL_A {
        match self.bits {
            0 => DMA0TSEL_A::DMA0TRIG0,
            1 => DMA0TSEL_A::DMA0TRIG1,
            2 => DMA0TSEL_A::DMA0TRIG2,
            3 => DMA0TSEL_A::DMA0TRIG3,
            4 => DMA0TSEL_A::DMA0TRIG4,
            5 => DMA0TSEL_A::DMA0TRIG5,
            6 => DMA0TSEL_A::DMA0TRIG6,
            7 => DMA0TSEL_A::DMA0TRIG7,
            8 => DMA0TSEL_A::DMA0TRIG8,
            9 => DMA0TSEL_A::DMA0TRIG9,
            10 => DMA0TSEL_A::DMA0TRIG10,
            11 => DMA0TSEL_A::DMA0TRIG11,
            12 => DMA0TSEL_A::DMA0TRIG12,
            13 => DMA0TSEL_A::DMA0TRIG13,
            14 => DMA0TSEL_A::DMA0TRIG14,
            15 => DMA0TSEL_A::DMA0TRIG15,
            16 => DMA0TSEL_A::DMA0TRIG16,
            17 => DMA0TSEL_A::DMA0TRIG17,
            18 => DMA0TSEL_A::DMA0TRIG18,
            19 => DMA0TSEL_A::DMA0TRIG19,
            20 => DMA0TSEL_A::DMA0TRIG20,
            21 => DMA0TSEL_A::DMA0TRIG21,
            22 => DMA0TSEL_A::DMA0TRIG22,
            23 => DMA0TSEL_A::DMA0TRIG23,
            24 => DMA0TSEL_A::DMA0TRIG24,
            25 => DMA0TSEL_A::DMA0TRIG25,
            26 => DMA0TSEL_A::DMA0TRIG26,
            27 => DMA0TSEL_A::DMA0TRIG27,
            28 => DMA0TSEL_A::DMA0TRIG28,
            29 => DMA0TSEL_A::DMA0TRIG29,
            30 => DMA0TSEL_A::DMA0TRIG30,
            31 => DMA0TSEL_A::DMA0TRIG31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG0`"]
    #[inline(always)]
    pub fn is_dma0trig0(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG0
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG1`"]
    #[inline(always)]
    pub fn is_dma0trig1(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG1
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG2`"]
    #[inline(always)]
    pub fn is_dma0trig2(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG2
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG3`"]
    #[inline(always)]
    pub fn is_dma0trig3(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG3
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG4`"]
    #[inline(always)]
    pub fn is_dma0trig4(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG4
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG5`"]
    #[inline(always)]
    pub fn is_dma0trig5(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG5
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG6`"]
    #[inline(always)]
    pub fn is_dma0trig6(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG6
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG7`"]
    #[inline(always)]
    pub fn is_dma0trig7(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG7
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG8`"]
    #[inline(always)]
    pub fn is_dma0trig8(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG8
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG9`"]
    #[inline(always)]
    pub fn is_dma0trig9(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG9
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG10`"]
    #[inline(always)]
    pub fn is_dma0trig10(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG10
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG11`"]
    #[inline(always)]
    pub fn is_dma0trig11(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG11
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG12`"]
    #[inline(always)]
    pub fn is_dma0trig12(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG12
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG13`"]
    #[inline(always)]
    pub fn is_dma0trig13(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG13
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG14`"]
    #[inline(always)]
    pub fn is_dma0trig14(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG14
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG15`"]
    #[inline(always)]
    pub fn is_dma0trig15(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG15
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG16`"]
    #[inline(always)]
    pub fn is_dma0trig16(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG16
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG17`"]
    #[inline(always)]
    pub fn is_dma0trig17(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG17
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG18`"]
    #[inline(always)]
    pub fn is_dma0trig18(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG18
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG19`"]
    #[inline(always)]
    pub fn is_dma0trig19(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG19
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG20`"]
    #[inline(always)]
    pub fn is_dma0trig20(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG20
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG21`"]
    #[inline(always)]
    pub fn is_dma0trig21(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG21
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG22`"]
    #[inline(always)]
    pub fn is_dma0trig22(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG22
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG23`"]
    #[inline(always)]
    pub fn is_dma0trig23(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG23
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG24`"]
    #[inline(always)]
    pub fn is_dma0trig24(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG24
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG25`"]
    #[inline(always)]
    pub fn is_dma0trig25(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG25
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG26`"]
    #[inline(always)]
    pub fn is_dma0trig26(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG26
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG27`"]
    #[inline(always)]
    pub fn is_dma0trig27(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG27
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG28`"]
    #[inline(always)]
    pub fn is_dma0trig28(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG28
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG29`"]
    #[inline(always)]
    pub fn is_dma0trig29(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG29
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG30`"]
    #[inline(always)]
    pub fn is_dma0trig30(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG30
    }
    #[doc = "Checks if the value of the field is `DMA0TRIG31`"]
    #[inline(always)]
    pub fn is_dma0trig31(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TRIG31
    }
}
#[doc = "Write proxy for field `DMA0TSEL`"]
pub struct DMA0TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0TSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0TSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA0TRIG0"]
    #[inline(always)]
    pub fn dma0trig0(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG0)
    }
    #[doc = "DMA0TRIG1"]
    #[inline(always)]
    pub fn dma0trig1(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG1)
    }
    #[doc = "DMA0TRIG2"]
    #[inline(always)]
    pub fn dma0trig2(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG2)
    }
    #[doc = "DMA0TRIG3"]
    #[inline(always)]
    pub fn dma0trig3(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG3)
    }
    #[doc = "DMA0TRIG4"]
    #[inline(always)]
    pub fn dma0trig4(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG4)
    }
    #[doc = "DMA0TRIG5"]
    #[inline(always)]
    pub fn dma0trig5(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG5)
    }
    #[doc = "DMA0TRIG6"]
    #[inline(always)]
    pub fn dma0trig6(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG6)
    }
    #[doc = "DMA0TRIG7"]
    #[inline(always)]
    pub fn dma0trig7(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG7)
    }
    #[doc = "DMA0TRIG8"]
    #[inline(always)]
    pub fn dma0trig8(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG8)
    }
    #[doc = "DMA0TRIG9"]
    #[inline(always)]
    pub fn dma0trig9(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG9)
    }
    #[doc = "DMA0TRIG10"]
    #[inline(always)]
    pub fn dma0trig10(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG10)
    }
    #[doc = "DMA0TRIG11"]
    #[inline(always)]
    pub fn dma0trig11(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG11)
    }
    #[doc = "DMA0TRIG12"]
    #[inline(always)]
    pub fn dma0trig12(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG12)
    }
    #[doc = "DMA0TRIG13"]
    #[inline(always)]
    pub fn dma0trig13(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG13)
    }
    #[doc = "DMA0TRIG14"]
    #[inline(always)]
    pub fn dma0trig14(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG14)
    }
    #[doc = "DMA0TRIG15"]
    #[inline(always)]
    pub fn dma0trig15(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG15)
    }
    #[doc = "DMA0TRIG16"]
    #[inline(always)]
    pub fn dma0trig16(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG16)
    }
    #[doc = "DMA0TRIG17"]
    #[inline(always)]
    pub fn dma0trig17(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG17)
    }
    #[doc = "DMA0TRIG18"]
    #[inline(always)]
    pub fn dma0trig18(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG18)
    }
    #[doc = "DMA0TRIG19"]
    #[inline(always)]
    pub fn dma0trig19(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG19)
    }
    #[doc = "DMA0TRIG20"]
    #[inline(always)]
    pub fn dma0trig20(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG20)
    }
    #[doc = "DMA0TRIG21"]
    #[inline(always)]
    pub fn dma0trig21(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG21)
    }
    #[doc = "DMA0TRIG22"]
    #[inline(always)]
    pub fn dma0trig22(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG22)
    }
    #[doc = "DMA0TRIG23"]
    #[inline(always)]
    pub fn dma0trig23(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG23)
    }
    #[doc = "DMA0TRIG24"]
    #[inline(always)]
    pub fn dma0trig24(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG24)
    }
    #[doc = "DMA0TRIG25"]
    #[inline(always)]
    pub fn dma0trig25(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG25)
    }
    #[doc = "DMA0TRIG26"]
    #[inline(always)]
    pub fn dma0trig26(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG26)
    }
    #[doc = "DMA0TRIG27"]
    #[inline(always)]
    pub fn dma0trig27(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG27)
    }
    #[doc = "DMA0TRIG28"]
    #[inline(always)]
    pub fn dma0trig28(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG28)
    }
    #[doc = "DMA0TRIG29"]
    #[inline(always)]
    pub fn dma0trig29(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG29)
    }
    #[doc = "DMA0TRIG30"]
    #[inline(always)]
    pub fn dma0trig30(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG30)
    }
    #[doc = "DMA0TRIG31"]
    #[inline(always)]
    pub fn dma0trig31(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TRIG31)
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
pub enum DMA1TSEL_A {
    #[doc = "0: DMA1TRIG0"]
    DMA1TRIG0 = 0,
    #[doc = "1: DMA1TRIG1"]
    DMA1TRIG1 = 1,
    #[doc = "2: DMA1TRIG2"]
    DMA1TRIG2 = 2,
    #[doc = "3: DMA1TRIG3"]
    DMA1TRIG3 = 3,
    #[doc = "4: DMA1TRIG4"]
    DMA1TRIG4 = 4,
    #[doc = "5: DMA1TRIG5"]
    DMA1TRIG5 = 5,
    #[doc = "6: DMA1TRIG6"]
    DMA1TRIG6 = 6,
    #[doc = "7: DMA1TRIG7"]
    DMA1TRIG7 = 7,
    #[doc = "8: DMA1TRIG8"]
    DMA1TRIG8 = 8,
    #[doc = "9: DMA1TRIG9"]
    DMA1TRIG9 = 9,
    #[doc = "10: DMA1TRIG10"]
    DMA1TRIG10 = 10,
    #[doc = "11: DMA1TRIG11"]
    DMA1TRIG11 = 11,
    #[doc = "12: DMA1TRIG12"]
    DMA1TRIG12 = 12,
    #[doc = "13: DMA1TRIG13"]
    DMA1TRIG13 = 13,
    #[doc = "14: DMA1TRIG14"]
    DMA1TRIG14 = 14,
    #[doc = "15: DMA1TRIG15"]
    DMA1TRIG15 = 15,
    #[doc = "16: DMA1TRIG16"]
    DMA1TRIG16 = 16,
    #[doc = "17: DMA1TRIG17"]
    DMA1TRIG17 = 17,
    #[doc = "18: DMA1TRIG18"]
    DMA1TRIG18 = 18,
    #[doc = "19: DMA1TRIG19"]
    DMA1TRIG19 = 19,
    #[doc = "20: DMA1TRIG20"]
    DMA1TRIG20 = 20,
    #[doc = "21: DMA1TRIG21"]
    DMA1TRIG21 = 21,
    #[doc = "22: DMA1TRIG22"]
    DMA1TRIG22 = 22,
    #[doc = "23: DMA1TRIG23"]
    DMA1TRIG23 = 23,
    #[doc = "24: DMA1TRIG24"]
    DMA1TRIG24 = 24,
    #[doc = "25: DMA1TRIG25"]
    DMA1TRIG25 = 25,
    #[doc = "26: DMA1TRIG26"]
    DMA1TRIG26 = 26,
    #[doc = "27: DMA1TRIG27"]
    DMA1TRIG27 = 27,
    #[doc = "28: DMA1TRIG28"]
    DMA1TRIG28 = 28,
    #[doc = "29: DMA1TRIG29"]
    DMA1TRIG29 = 29,
    #[doc = "30: DMA1TRIG30"]
    DMA1TRIG30 = 30,
    #[doc = "31: DMA1TRIG31"]
    DMA1TRIG31 = 31,
}
impl From<DMA1TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA1TSEL`"]
pub type DMA1TSEL_R = crate::R<u8, DMA1TSEL_A>;
impl DMA1TSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1TSEL_A {
        match self.bits {
            0 => DMA1TSEL_A::DMA1TRIG0,
            1 => DMA1TSEL_A::DMA1TRIG1,
            2 => DMA1TSEL_A::DMA1TRIG2,
            3 => DMA1TSEL_A::DMA1TRIG3,
            4 => DMA1TSEL_A::DMA1TRIG4,
            5 => DMA1TSEL_A::DMA1TRIG5,
            6 => DMA1TSEL_A::DMA1TRIG6,
            7 => DMA1TSEL_A::DMA1TRIG7,
            8 => DMA1TSEL_A::DMA1TRIG8,
            9 => DMA1TSEL_A::DMA1TRIG9,
            10 => DMA1TSEL_A::DMA1TRIG10,
            11 => DMA1TSEL_A::DMA1TRIG11,
            12 => DMA1TSEL_A::DMA1TRIG12,
            13 => DMA1TSEL_A::DMA1TRIG13,
            14 => DMA1TSEL_A::DMA1TRIG14,
            15 => DMA1TSEL_A::DMA1TRIG15,
            16 => DMA1TSEL_A::DMA1TRIG16,
            17 => DMA1TSEL_A::DMA1TRIG17,
            18 => DMA1TSEL_A::DMA1TRIG18,
            19 => DMA1TSEL_A::DMA1TRIG19,
            20 => DMA1TSEL_A::DMA1TRIG20,
            21 => DMA1TSEL_A::DMA1TRIG21,
            22 => DMA1TSEL_A::DMA1TRIG22,
            23 => DMA1TSEL_A::DMA1TRIG23,
            24 => DMA1TSEL_A::DMA1TRIG24,
            25 => DMA1TSEL_A::DMA1TRIG25,
            26 => DMA1TSEL_A::DMA1TRIG26,
            27 => DMA1TSEL_A::DMA1TRIG27,
            28 => DMA1TSEL_A::DMA1TRIG28,
            29 => DMA1TSEL_A::DMA1TRIG29,
            30 => DMA1TSEL_A::DMA1TRIG30,
            31 => DMA1TSEL_A::DMA1TRIG31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG0`"]
    #[inline(always)]
    pub fn is_dma1trig0(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG0
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG1`"]
    #[inline(always)]
    pub fn is_dma1trig1(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG1
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG2`"]
    #[inline(always)]
    pub fn is_dma1trig2(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG2
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG3`"]
    #[inline(always)]
    pub fn is_dma1trig3(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG3
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG4`"]
    #[inline(always)]
    pub fn is_dma1trig4(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG4
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG5`"]
    #[inline(always)]
    pub fn is_dma1trig5(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG5
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG6`"]
    #[inline(always)]
    pub fn is_dma1trig6(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG6
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG7`"]
    #[inline(always)]
    pub fn is_dma1trig7(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG7
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG8`"]
    #[inline(always)]
    pub fn is_dma1trig8(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG8
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG9`"]
    #[inline(always)]
    pub fn is_dma1trig9(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG9
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG10`"]
    #[inline(always)]
    pub fn is_dma1trig10(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG10
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG11`"]
    #[inline(always)]
    pub fn is_dma1trig11(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG11
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG12`"]
    #[inline(always)]
    pub fn is_dma1trig12(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG12
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG13`"]
    #[inline(always)]
    pub fn is_dma1trig13(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG13
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG14`"]
    #[inline(always)]
    pub fn is_dma1trig14(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG14
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG15`"]
    #[inline(always)]
    pub fn is_dma1trig15(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG15
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG16`"]
    #[inline(always)]
    pub fn is_dma1trig16(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG16
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG17`"]
    #[inline(always)]
    pub fn is_dma1trig17(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG17
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG18`"]
    #[inline(always)]
    pub fn is_dma1trig18(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG18
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG19`"]
    #[inline(always)]
    pub fn is_dma1trig19(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG19
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG20`"]
    #[inline(always)]
    pub fn is_dma1trig20(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG20
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG21`"]
    #[inline(always)]
    pub fn is_dma1trig21(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG21
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG22`"]
    #[inline(always)]
    pub fn is_dma1trig22(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG22
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG23`"]
    #[inline(always)]
    pub fn is_dma1trig23(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG23
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG24`"]
    #[inline(always)]
    pub fn is_dma1trig24(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG24
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG25`"]
    #[inline(always)]
    pub fn is_dma1trig25(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG25
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG26`"]
    #[inline(always)]
    pub fn is_dma1trig26(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG26
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG27`"]
    #[inline(always)]
    pub fn is_dma1trig27(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG27
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG28`"]
    #[inline(always)]
    pub fn is_dma1trig28(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG28
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG29`"]
    #[inline(always)]
    pub fn is_dma1trig29(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG29
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG30`"]
    #[inline(always)]
    pub fn is_dma1trig30(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG30
    }
    #[doc = "Checks if the value of the field is `DMA1TRIG31`"]
    #[inline(always)]
    pub fn is_dma1trig31(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TRIG31
    }
}
#[doc = "Write proxy for field `DMA1TSEL`"]
pub struct DMA1TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1TSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1TSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA1TRIG0"]
    #[inline(always)]
    pub fn dma1trig0(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG0)
    }
    #[doc = "DMA1TRIG1"]
    #[inline(always)]
    pub fn dma1trig1(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG1)
    }
    #[doc = "DMA1TRIG2"]
    #[inline(always)]
    pub fn dma1trig2(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG2)
    }
    #[doc = "DMA1TRIG3"]
    #[inline(always)]
    pub fn dma1trig3(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG3)
    }
    #[doc = "DMA1TRIG4"]
    #[inline(always)]
    pub fn dma1trig4(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG4)
    }
    #[doc = "DMA1TRIG5"]
    #[inline(always)]
    pub fn dma1trig5(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG5)
    }
    #[doc = "DMA1TRIG6"]
    #[inline(always)]
    pub fn dma1trig6(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG6)
    }
    #[doc = "DMA1TRIG7"]
    #[inline(always)]
    pub fn dma1trig7(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG7)
    }
    #[doc = "DMA1TRIG8"]
    #[inline(always)]
    pub fn dma1trig8(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG8)
    }
    #[doc = "DMA1TRIG9"]
    #[inline(always)]
    pub fn dma1trig9(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG9)
    }
    #[doc = "DMA1TRIG10"]
    #[inline(always)]
    pub fn dma1trig10(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG10)
    }
    #[doc = "DMA1TRIG11"]
    #[inline(always)]
    pub fn dma1trig11(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG11)
    }
    #[doc = "DMA1TRIG12"]
    #[inline(always)]
    pub fn dma1trig12(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG12)
    }
    #[doc = "DMA1TRIG13"]
    #[inline(always)]
    pub fn dma1trig13(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG13)
    }
    #[doc = "DMA1TRIG14"]
    #[inline(always)]
    pub fn dma1trig14(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG14)
    }
    #[doc = "DMA1TRIG15"]
    #[inline(always)]
    pub fn dma1trig15(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG15)
    }
    #[doc = "DMA1TRIG16"]
    #[inline(always)]
    pub fn dma1trig16(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG16)
    }
    #[doc = "DMA1TRIG17"]
    #[inline(always)]
    pub fn dma1trig17(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG17)
    }
    #[doc = "DMA1TRIG18"]
    #[inline(always)]
    pub fn dma1trig18(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG18)
    }
    #[doc = "DMA1TRIG19"]
    #[inline(always)]
    pub fn dma1trig19(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG19)
    }
    #[doc = "DMA1TRIG20"]
    #[inline(always)]
    pub fn dma1trig20(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG20)
    }
    #[doc = "DMA1TRIG21"]
    #[inline(always)]
    pub fn dma1trig21(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG21)
    }
    #[doc = "DMA1TRIG22"]
    #[inline(always)]
    pub fn dma1trig22(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG22)
    }
    #[doc = "DMA1TRIG23"]
    #[inline(always)]
    pub fn dma1trig23(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG23)
    }
    #[doc = "DMA1TRIG24"]
    #[inline(always)]
    pub fn dma1trig24(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG24)
    }
    #[doc = "DMA1TRIG25"]
    #[inline(always)]
    pub fn dma1trig25(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG25)
    }
    #[doc = "DMA1TRIG26"]
    #[inline(always)]
    pub fn dma1trig26(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG26)
    }
    #[doc = "DMA1TRIG27"]
    #[inline(always)]
    pub fn dma1trig27(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG27)
    }
    #[doc = "DMA1TRIG28"]
    #[inline(always)]
    pub fn dma1trig28(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG28)
    }
    #[doc = "DMA1TRIG29"]
    #[inline(always)]
    pub fn dma1trig29(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG29)
    }
    #[doc = "DMA1TRIG30"]
    #[inline(always)]
    pub fn dma1trig30(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG30)
    }
    #[doc = "DMA1TRIG31"]
    #[inline(always)]
    pub fn dma1trig31(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TRIG31)
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
    pub fn dma0tsel(&self) -> DMA0TSEL_R {
        DMA0TSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma1tsel(&self) -> DMA1TSEL_R {
        DMA1TSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma0tsel(&mut self) -> DMA0TSEL_W {
        DMA0TSEL_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma1tsel(&mut self) -> DMA1TSEL_W {
        DMA1TSEL_W { w: self }
    }
}
