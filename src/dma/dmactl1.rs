#[doc = "Reader of register DMACTL1"]
pub type R = crate::R<u16, super::DMACTL1>;
#[doc = "Writer for register DMACTL1"]
pub type W = crate::W<u16, super::DMACTL1>;
#[doc = "Register DMACTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACTL1 {
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
pub enum DMA2TSEL_A {
    #[doc = "0: DMA2TRIG0"]
    DMA2TRIG0 = 0,
    #[doc = "1: DMA2TRIG1"]
    DMA2TRIG1 = 1,
    #[doc = "2: DMA2TRIG2"]
    DMA2TRIG2 = 2,
    #[doc = "3: DMA2TRIG3"]
    DMA2TRIG3 = 3,
    #[doc = "4: DMA2TRIG4"]
    DMA2TRIG4 = 4,
    #[doc = "5: DMA2TRIG5"]
    DMA2TRIG5 = 5,
    #[doc = "6: DMA2TRIG6"]
    DMA2TRIG6 = 6,
    #[doc = "7: DMA2TRIG7"]
    DMA2TRIG7 = 7,
    #[doc = "8: DMA2TRIG8"]
    DMA2TRIG8 = 8,
    #[doc = "9: DMA2TRIG9"]
    DMA2TRIG9 = 9,
    #[doc = "10: DMA2TRIG10"]
    DMA2TRIG10 = 10,
    #[doc = "11: DMA2TRIG11"]
    DMA2TRIG11 = 11,
    #[doc = "12: DMA2TRIG12"]
    DMA2TRIG12 = 12,
    #[doc = "13: DMA2TRIG13"]
    DMA2TRIG13 = 13,
    #[doc = "14: DMA2TRIG14"]
    DMA2TRIG14 = 14,
    #[doc = "15: DMA2TRIG15"]
    DMA2TRIG15 = 15,
    #[doc = "16: DMA2TRIG16"]
    DMA2TRIG16 = 16,
    #[doc = "17: DMA2TRIG17"]
    DMA2TRIG17 = 17,
    #[doc = "18: DMA2TRIG18"]
    DMA2TRIG18 = 18,
    #[doc = "19: DMA2TRIG19"]
    DMA2TRIG19 = 19,
    #[doc = "20: DMA2TRIG20"]
    DMA2TRIG20 = 20,
    #[doc = "21: DMA2TRIG21"]
    DMA2TRIG21 = 21,
    #[doc = "22: DMA2TRIG22"]
    DMA2TRIG22 = 22,
    #[doc = "23: DMA2TRIG23"]
    DMA2TRIG23 = 23,
    #[doc = "24: DMA2TRIG24"]
    DMA2TRIG24 = 24,
    #[doc = "25: DMA2TRIG25"]
    DMA2TRIG25 = 25,
    #[doc = "26: DMA2TRIG26"]
    DMA2TRIG26 = 26,
    #[doc = "27: DMA2TRIG27"]
    DMA2TRIG27 = 27,
    #[doc = "28: DMA2TRIG28"]
    DMA2TRIG28 = 28,
    #[doc = "29: DMA2TRIG29"]
    DMA2TRIG29 = 29,
    #[doc = "30: DMA2TRIG30"]
    DMA2TRIG30 = 30,
    #[doc = "31: DMA2TRIG31"]
    DMA2TRIG31 = 31,
}
impl From<DMA2TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA2TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA2TSEL`"]
pub type DMA2TSEL_R = crate::R<u8, DMA2TSEL_A>;
impl DMA2TSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2TSEL_A {
        match self.bits {
            0 => DMA2TSEL_A::DMA2TRIG0,
            1 => DMA2TSEL_A::DMA2TRIG1,
            2 => DMA2TSEL_A::DMA2TRIG2,
            3 => DMA2TSEL_A::DMA2TRIG3,
            4 => DMA2TSEL_A::DMA2TRIG4,
            5 => DMA2TSEL_A::DMA2TRIG5,
            6 => DMA2TSEL_A::DMA2TRIG6,
            7 => DMA2TSEL_A::DMA2TRIG7,
            8 => DMA2TSEL_A::DMA2TRIG8,
            9 => DMA2TSEL_A::DMA2TRIG9,
            10 => DMA2TSEL_A::DMA2TRIG10,
            11 => DMA2TSEL_A::DMA2TRIG11,
            12 => DMA2TSEL_A::DMA2TRIG12,
            13 => DMA2TSEL_A::DMA2TRIG13,
            14 => DMA2TSEL_A::DMA2TRIG14,
            15 => DMA2TSEL_A::DMA2TRIG15,
            16 => DMA2TSEL_A::DMA2TRIG16,
            17 => DMA2TSEL_A::DMA2TRIG17,
            18 => DMA2TSEL_A::DMA2TRIG18,
            19 => DMA2TSEL_A::DMA2TRIG19,
            20 => DMA2TSEL_A::DMA2TRIG20,
            21 => DMA2TSEL_A::DMA2TRIG21,
            22 => DMA2TSEL_A::DMA2TRIG22,
            23 => DMA2TSEL_A::DMA2TRIG23,
            24 => DMA2TSEL_A::DMA2TRIG24,
            25 => DMA2TSEL_A::DMA2TRIG25,
            26 => DMA2TSEL_A::DMA2TRIG26,
            27 => DMA2TSEL_A::DMA2TRIG27,
            28 => DMA2TSEL_A::DMA2TRIG28,
            29 => DMA2TSEL_A::DMA2TRIG29,
            30 => DMA2TSEL_A::DMA2TRIG30,
            31 => DMA2TSEL_A::DMA2TRIG31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG0`"]
    #[inline(always)]
    pub fn is_dma2trig0(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG0
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG1`"]
    #[inline(always)]
    pub fn is_dma2trig1(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG1
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG2`"]
    #[inline(always)]
    pub fn is_dma2trig2(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG2
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG3`"]
    #[inline(always)]
    pub fn is_dma2trig3(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG3
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG4`"]
    #[inline(always)]
    pub fn is_dma2trig4(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG4
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG5`"]
    #[inline(always)]
    pub fn is_dma2trig5(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG5
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG6`"]
    #[inline(always)]
    pub fn is_dma2trig6(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG6
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG7`"]
    #[inline(always)]
    pub fn is_dma2trig7(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG7
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG8`"]
    #[inline(always)]
    pub fn is_dma2trig8(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG8
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG9`"]
    #[inline(always)]
    pub fn is_dma2trig9(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG9
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG10`"]
    #[inline(always)]
    pub fn is_dma2trig10(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG10
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG11`"]
    #[inline(always)]
    pub fn is_dma2trig11(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG11
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG12`"]
    #[inline(always)]
    pub fn is_dma2trig12(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG12
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG13`"]
    #[inline(always)]
    pub fn is_dma2trig13(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG13
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG14`"]
    #[inline(always)]
    pub fn is_dma2trig14(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG14
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG15`"]
    #[inline(always)]
    pub fn is_dma2trig15(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG15
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG16`"]
    #[inline(always)]
    pub fn is_dma2trig16(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG16
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG17`"]
    #[inline(always)]
    pub fn is_dma2trig17(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG17
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG18`"]
    #[inline(always)]
    pub fn is_dma2trig18(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG18
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG19`"]
    #[inline(always)]
    pub fn is_dma2trig19(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG19
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG20`"]
    #[inline(always)]
    pub fn is_dma2trig20(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG20
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG21`"]
    #[inline(always)]
    pub fn is_dma2trig21(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG21
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG22`"]
    #[inline(always)]
    pub fn is_dma2trig22(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG22
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG23`"]
    #[inline(always)]
    pub fn is_dma2trig23(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG23
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG24`"]
    #[inline(always)]
    pub fn is_dma2trig24(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG24
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG25`"]
    #[inline(always)]
    pub fn is_dma2trig25(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG25
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG26`"]
    #[inline(always)]
    pub fn is_dma2trig26(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG26
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG27`"]
    #[inline(always)]
    pub fn is_dma2trig27(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG27
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG28`"]
    #[inline(always)]
    pub fn is_dma2trig28(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG28
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG29`"]
    #[inline(always)]
    pub fn is_dma2trig29(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG29
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG30`"]
    #[inline(always)]
    pub fn is_dma2trig30(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG30
    }
    #[doc = "Checks if the value of the field is `DMA2TRIG31`"]
    #[inline(always)]
    pub fn is_dma2trig31(&self) -> bool {
        *self == DMA2TSEL_A::DMA2TRIG31
    }
}
#[doc = "Write proxy for field `DMA2TSEL`"]
pub struct DMA2TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2TSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2TSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA2TRIG0"]
    #[inline(always)]
    pub fn dma2trig0(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG0)
    }
    #[doc = "DMA2TRIG1"]
    #[inline(always)]
    pub fn dma2trig1(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG1)
    }
    #[doc = "DMA2TRIG2"]
    #[inline(always)]
    pub fn dma2trig2(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG2)
    }
    #[doc = "DMA2TRIG3"]
    #[inline(always)]
    pub fn dma2trig3(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG3)
    }
    #[doc = "DMA2TRIG4"]
    #[inline(always)]
    pub fn dma2trig4(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG4)
    }
    #[doc = "DMA2TRIG5"]
    #[inline(always)]
    pub fn dma2trig5(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG5)
    }
    #[doc = "DMA2TRIG6"]
    #[inline(always)]
    pub fn dma2trig6(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG6)
    }
    #[doc = "DMA2TRIG7"]
    #[inline(always)]
    pub fn dma2trig7(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG7)
    }
    #[doc = "DMA2TRIG8"]
    #[inline(always)]
    pub fn dma2trig8(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG8)
    }
    #[doc = "DMA2TRIG9"]
    #[inline(always)]
    pub fn dma2trig9(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG9)
    }
    #[doc = "DMA2TRIG10"]
    #[inline(always)]
    pub fn dma2trig10(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG10)
    }
    #[doc = "DMA2TRIG11"]
    #[inline(always)]
    pub fn dma2trig11(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG11)
    }
    #[doc = "DMA2TRIG12"]
    #[inline(always)]
    pub fn dma2trig12(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG12)
    }
    #[doc = "DMA2TRIG13"]
    #[inline(always)]
    pub fn dma2trig13(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG13)
    }
    #[doc = "DMA2TRIG14"]
    #[inline(always)]
    pub fn dma2trig14(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG14)
    }
    #[doc = "DMA2TRIG15"]
    #[inline(always)]
    pub fn dma2trig15(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG15)
    }
    #[doc = "DMA2TRIG16"]
    #[inline(always)]
    pub fn dma2trig16(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG16)
    }
    #[doc = "DMA2TRIG17"]
    #[inline(always)]
    pub fn dma2trig17(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG17)
    }
    #[doc = "DMA2TRIG18"]
    #[inline(always)]
    pub fn dma2trig18(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG18)
    }
    #[doc = "DMA2TRIG19"]
    #[inline(always)]
    pub fn dma2trig19(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG19)
    }
    #[doc = "DMA2TRIG20"]
    #[inline(always)]
    pub fn dma2trig20(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG20)
    }
    #[doc = "DMA2TRIG21"]
    #[inline(always)]
    pub fn dma2trig21(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG21)
    }
    #[doc = "DMA2TRIG22"]
    #[inline(always)]
    pub fn dma2trig22(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG22)
    }
    #[doc = "DMA2TRIG23"]
    #[inline(always)]
    pub fn dma2trig23(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG23)
    }
    #[doc = "DMA2TRIG24"]
    #[inline(always)]
    pub fn dma2trig24(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG24)
    }
    #[doc = "DMA2TRIG25"]
    #[inline(always)]
    pub fn dma2trig25(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG25)
    }
    #[doc = "DMA2TRIG26"]
    #[inline(always)]
    pub fn dma2trig26(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG26)
    }
    #[doc = "DMA2TRIG27"]
    #[inline(always)]
    pub fn dma2trig27(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG27)
    }
    #[doc = "DMA2TRIG28"]
    #[inline(always)]
    pub fn dma2trig28(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG28)
    }
    #[doc = "DMA2TRIG29"]
    #[inline(always)]
    pub fn dma2trig29(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG29)
    }
    #[doc = "DMA2TRIG30"]
    #[inline(always)]
    pub fn dma2trig30(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG30)
    }
    #[doc = "DMA2TRIG31"]
    #[inline(always)]
    pub fn dma2trig31(self) -> &'a mut W {
        self.variant(DMA2TSEL_A::DMA2TRIG31)
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
pub enum DMA3TSEL_A {
    #[doc = "0: DMA3TRIG0"]
    DMA3TRIG0 = 0,
    #[doc = "1: DMA3TRIG1"]
    DMA3TRIG1 = 1,
    #[doc = "2: DMA3TRIG2"]
    DMA3TRIG2 = 2,
    #[doc = "3: DMA3TRIG3"]
    DMA3TRIG3 = 3,
    #[doc = "4: DMA3TRIG4"]
    DMA3TRIG4 = 4,
    #[doc = "5: DMA3TRIG5"]
    DMA3TRIG5 = 5,
    #[doc = "6: DMA3TRIG6"]
    DMA3TRIG6 = 6,
    #[doc = "7: DMA3TRIG7"]
    DMA3TRIG7 = 7,
    #[doc = "8: DMA3TRIG8"]
    DMA3TRIG8 = 8,
    #[doc = "9: DMA3TRIG9"]
    DMA3TRIG9 = 9,
    #[doc = "10: DMA3TRIG10"]
    DMA3TRIG10 = 10,
    #[doc = "11: DMA3TRIG11"]
    DMA3TRIG11 = 11,
    #[doc = "12: DMA3TRIG12"]
    DMA3TRIG12 = 12,
    #[doc = "13: DMA3TRIG13"]
    DMA3TRIG13 = 13,
    #[doc = "14: DMA3TRIG14"]
    DMA3TRIG14 = 14,
    #[doc = "15: DMA3TRIG15"]
    DMA3TRIG15 = 15,
    #[doc = "16: DMA3TRIG16"]
    DMA3TRIG16 = 16,
    #[doc = "17: DMA3TRIG17"]
    DMA3TRIG17 = 17,
    #[doc = "18: DMA3TRIG18"]
    DMA3TRIG18 = 18,
    #[doc = "19: DMA3TRIG19"]
    DMA3TRIG19 = 19,
    #[doc = "20: DMA3TRIG20"]
    DMA3TRIG20 = 20,
    #[doc = "21: DMA3TRIG21"]
    DMA3TRIG21 = 21,
    #[doc = "22: DMA3TRIG22"]
    DMA3TRIG22 = 22,
    #[doc = "23: DMA3TRIG23"]
    DMA3TRIG23 = 23,
    #[doc = "24: DMA3TRIG24"]
    DMA3TRIG24 = 24,
    #[doc = "25: DMA3TRIG25"]
    DMA3TRIG25 = 25,
    #[doc = "26: DMA3TRIG26"]
    DMA3TRIG26 = 26,
    #[doc = "27: DMA3TRIG27"]
    DMA3TRIG27 = 27,
    #[doc = "28: DMA3TRIG28"]
    DMA3TRIG28 = 28,
    #[doc = "29: DMA3TRIG29"]
    DMA3TRIG29 = 29,
    #[doc = "30: DMA3TRIG30"]
    DMA3TRIG30 = 30,
    #[doc = "31: DMA3TRIG31"]
    DMA3TRIG31 = 31,
}
impl From<DMA3TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA3TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA3TSEL`"]
pub type DMA3TSEL_R = crate::R<u8, DMA3TSEL_A>;
impl DMA3TSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA3TSEL_A {
        match self.bits {
            0 => DMA3TSEL_A::DMA3TRIG0,
            1 => DMA3TSEL_A::DMA3TRIG1,
            2 => DMA3TSEL_A::DMA3TRIG2,
            3 => DMA3TSEL_A::DMA3TRIG3,
            4 => DMA3TSEL_A::DMA3TRIG4,
            5 => DMA3TSEL_A::DMA3TRIG5,
            6 => DMA3TSEL_A::DMA3TRIG6,
            7 => DMA3TSEL_A::DMA3TRIG7,
            8 => DMA3TSEL_A::DMA3TRIG8,
            9 => DMA3TSEL_A::DMA3TRIG9,
            10 => DMA3TSEL_A::DMA3TRIG10,
            11 => DMA3TSEL_A::DMA3TRIG11,
            12 => DMA3TSEL_A::DMA3TRIG12,
            13 => DMA3TSEL_A::DMA3TRIG13,
            14 => DMA3TSEL_A::DMA3TRIG14,
            15 => DMA3TSEL_A::DMA3TRIG15,
            16 => DMA3TSEL_A::DMA3TRIG16,
            17 => DMA3TSEL_A::DMA3TRIG17,
            18 => DMA3TSEL_A::DMA3TRIG18,
            19 => DMA3TSEL_A::DMA3TRIG19,
            20 => DMA3TSEL_A::DMA3TRIG20,
            21 => DMA3TSEL_A::DMA3TRIG21,
            22 => DMA3TSEL_A::DMA3TRIG22,
            23 => DMA3TSEL_A::DMA3TRIG23,
            24 => DMA3TSEL_A::DMA3TRIG24,
            25 => DMA3TSEL_A::DMA3TRIG25,
            26 => DMA3TSEL_A::DMA3TRIG26,
            27 => DMA3TSEL_A::DMA3TRIG27,
            28 => DMA3TSEL_A::DMA3TRIG28,
            29 => DMA3TSEL_A::DMA3TRIG29,
            30 => DMA3TSEL_A::DMA3TRIG30,
            31 => DMA3TSEL_A::DMA3TRIG31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG0`"]
    #[inline(always)]
    pub fn is_dma3trig0(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG0
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG1`"]
    #[inline(always)]
    pub fn is_dma3trig1(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG1
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG2`"]
    #[inline(always)]
    pub fn is_dma3trig2(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG2
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG3`"]
    #[inline(always)]
    pub fn is_dma3trig3(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG3
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG4`"]
    #[inline(always)]
    pub fn is_dma3trig4(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG4
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG5`"]
    #[inline(always)]
    pub fn is_dma3trig5(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG5
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG6`"]
    #[inline(always)]
    pub fn is_dma3trig6(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG6
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG7`"]
    #[inline(always)]
    pub fn is_dma3trig7(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG7
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG8`"]
    #[inline(always)]
    pub fn is_dma3trig8(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG8
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG9`"]
    #[inline(always)]
    pub fn is_dma3trig9(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG9
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG10`"]
    #[inline(always)]
    pub fn is_dma3trig10(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG10
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG11`"]
    #[inline(always)]
    pub fn is_dma3trig11(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG11
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG12`"]
    #[inline(always)]
    pub fn is_dma3trig12(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG12
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG13`"]
    #[inline(always)]
    pub fn is_dma3trig13(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG13
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG14`"]
    #[inline(always)]
    pub fn is_dma3trig14(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG14
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG15`"]
    #[inline(always)]
    pub fn is_dma3trig15(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG15
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG16`"]
    #[inline(always)]
    pub fn is_dma3trig16(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG16
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG17`"]
    #[inline(always)]
    pub fn is_dma3trig17(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG17
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG18`"]
    #[inline(always)]
    pub fn is_dma3trig18(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG18
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG19`"]
    #[inline(always)]
    pub fn is_dma3trig19(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG19
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG20`"]
    #[inline(always)]
    pub fn is_dma3trig20(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG20
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG21`"]
    #[inline(always)]
    pub fn is_dma3trig21(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG21
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG22`"]
    #[inline(always)]
    pub fn is_dma3trig22(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG22
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG23`"]
    #[inline(always)]
    pub fn is_dma3trig23(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG23
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG24`"]
    #[inline(always)]
    pub fn is_dma3trig24(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG24
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG25`"]
    #[inline(always)]
    pub fn is_dma3trig25(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG25
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG26`"]
    #[inline(always)]
    pub fn is_dma3trig26(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG26
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG27`"]
    #[inline(always)]
    pub fn is_dma3trig27(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG27
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG28`"]
    #[inline(always)]
    pub fn is_dma3trig28(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG28
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG29`"]
    #[inline(always)]
    pub fn is_dma3trig29(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG29
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG30`"]
    #[inline(always)]
    pub fn is_dma3trig30(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG30
    }
    #[doc = "Checks if the value of the field is `DMA3TRIG31`"]
    #[inline(always)]
    pub fn is_dma3trig31(&self) -> bool {
        *self == DMA3TSEL_A::DMA3TRIG31
    }
}
#[doc = "Write proxy for field `DMA3TSEL`"]
pub struct DMA3TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA3TSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA3TSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA3TRIG0"]
    #[inline(always)]
    pub fn dma3trig0(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG0)
    }
    #[doc = "DMA3TRIG1"]
    #[inline(always)]
    pub fn dma3trig1(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG1)
    }
    #[doc = "DMA3TRIG2"]
    #[inline(always)]
    pub fn dma3trig2(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG2)
    }
    #[doc = "DMA3TRIG3"]
    #[inline(always)]
    pub fn dma3trig3(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG3)
    }
    #[doc = "DMA3TRIG4"]
    #[inline(always)]
    pub fn dma3trig4(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG4)
    }
    #[doc = "DMA3TRIG5"]
    #[inline(always)]
    pub fn dma3trig5(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG5)
    }
    #[doc = "DMA3TRIG6"]
    #[inline(always)]
    pub fn dma3trig6(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG6)
    }
    #[doc = "DMA3TRIG7"]
    #[inline(always)]
    pub fn dma3trig7(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG7)
    }
    #[doc = "DMA3TRIG8"]
    #[inline(always)]
    pub fn dma3trig8(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG8)
    }
    #[doc = "DMA3TRIG9"]
    #[inline(always)]
    pub fn dma3trig9(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG9)
    }
    #[doc = "DMA3TRIG10"]
    #[inline(always)]
    pub fn dma3trig10(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG10)
    }
    #[doc = "DMA3TRIG11"]
    #[inline(always)]
    pub fn dma3trig11(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG11)
    }
    #[doc = "DMA3TRIG12"]
    #[inline(always)]
    pub fn dma3trig12(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG12)
    }
    #[doc = "DMA3TRIG13"]
    #[inline(always)]
    pub fn dma3trig13(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG13)
    }
    #[doc = "DMA3TRIG14"]
    #[inline(always)]
    pub fn dma3trig14(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG14)
    }
    #[doc = "DMA3TRIG15"]
    #[inline(always)]
    pub fn dma3trig15(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG15)
    }
    #[doc = "DMA3TRIG16"]
    #[inline(always)]
    pub fn dma3trig16(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG16)
    }
    #[doc = "DMA3TRIG17"]
    #[inline(always)]
    pub fn dma3trig17(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG17)
    }
    #[doc = "DMA3TRIG18"]
    #[inline(always)]
    pub fn dma3trig18(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG18)
    }
    #[doc = "DMA3TRIG19"]
    #[inline(always)]
    pub fn dma3trig19(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG19)
    }
    #[doc = "DMA3TRIG20"]
    #[inline(always)]
    pub fn dma3trig20(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG20)
    }
    #[doc = "DMA3TRIG21"]
    #[inline(always)]
    pub fn dma3trig21(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG21)
    }
    #[doc = "DMA3TRIG22"]
    #[inline(always)]
    pub fn dma3trig22(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG22)
    }
    #[doc = "DMA3TRIG23"]
    #[inline(always)]
    pub fn dma3trig23(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG23)
    }
    #[doc = "DMA3TRIG24"]
    #[inline(always)]
    pub fn dma3trig24(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG24)
    }
    #[doc = "DMA3TRIG25"]
    #[inline(always)]
    pub fn dma3trig25(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG25)
    }
    #[doc = "DMA3TRIG26"]
    #[inline(always)]
    pub fn dma3trig26(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG26)
    }
    #[doc = "DMA3TRIG27"]
    #[inline(always)]
    pub fn dma3trig27(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG27)
    }
    #[doc = "DMA3TRIG28"]
    #[inline(always)]
    pub fn dma3trig28(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG28)
    }
    #[doc = "DMA3TRIG29"]
    #[inline(always)]
    pub fn dma3trig29(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG29)
    }
    #[doc = "DMA3TRIG30"]
    #[inline(always)]
    pub fn dma3trig30(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG30)
    }
    #[doc = "DMA3TRIG31"]
    #[inline(always)]
    pub fn dma3trig31(self) -> &'a mut W {
        self.variant(DMA3TSEL_A::DMA3TRIG31)
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
    pub fn dma2tsel(&self) -> DMA2TSEL_R {
        DMA2TSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma3tsel(&self) -> DMA3TSEL_R {
        DMA3TSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma2tsel(&mut self) -> DMA2TSEL_W {
        DMA2TSEL_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
DMA trigger select"]
    #[inline(always)]
    pub fn dma3tsel(&mut self) -> DMA3TSEL_W {
        DMA3TSEL_W { w: self }
    }
}
