#[doc = "Reader of register DMAIV"]
pub type R = crate::R<u16, super::DMAIV>;
#[doc = "Writer for register DMAIV"]
pub type W = crate::W<u16, super::DMAIV>;
#[doc = "Register DMAIV `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAIV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
DMA interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum DMAIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: DMA channel 0; Interrupt Flag: DMA0IFG; Interrupt Priority: Highest"]
    DMA0IFG = 2,
    #[doc = "4: Interrupt Source: DMA channel 1; Interrupt Flag: DMA1IFG"]
    DMA1IFG = 4,
    #[doc = "6: Interrupt Source: DMA channel 2; Interrupt Flag: DMA2IFG"]
    DMA2IFG = 6,
    #[doc = "8: Interrupt Source: DMA channel 3; Interrupt Flag: DMA3IFG"]
    DMA3IFG = 8,
    #[doc = "10: Interrupt Source: DMA channel 4; Interrupt Flag: DMA4IFG"]
    DMA4IFG = 10,
    #[doc = "12: Interrupt Source: DMA channel 5; Interrupt Flag: DMA5IFG"]
    DMA5IFG = 12,
    #[doc = "14: Interrupt Source: DMA channel 6; Interrupt Flag: DMA6IFG"]
    DMA6IFG = 14,
    #[doc = "16: Interrupt Source: DMA channel 7; Interrupt Flag: DMA7IFG; Interrupt Priority: Lowest"]
    DMA7IFG = 16,
}
impl From<DMAIV_A> for u16 {
    #[inline(always)]
    fn from(variant: DMAIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMAIV`"]
pub type DMAIV_R = crate::R<u16, DMAIV_A>;
impl DMAIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, DMAIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMAIV_A::NONE),
            2 => Val(DMAIV_A::DMA0IFG),
            4 => Val(DMAIV_A::DMA1IFG),
            6 => Val(DMAIV_A::DMA2IFG),
            8 => Val(DMAIV_A::DMA3IFG),
            10 => Val(DMAIV_A::DMA4IFG),
            12 => Val(DMAIV_A::DMA5IFG),
            14 => Val(DMAIV_A::DMA6IFG),
            16 => Val(DMAIV_A::DMA7IFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMAIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `DMA0IFG`"]
    #[inline(always)]
    pub fn is_dma0ifg(&self) -> bool {
        *self == DMAIV_A::DMA0IFG
    }
    #[doc = "Checks if the value of the field is `DMA1IFG`"]
    #[inline(always)]
    pub fn is_dma1ifg(&self) -> bool {
        *self == DMAIV_A::DMA1IFG
    }
    #[doc = "Checks if the value of the field is `DMA2IFG`"]
    #[inline(always)]
    pub fn is_dma2ifg(&self) -> bool {
        *self == DMAIV_A::DMA2IFG
    }
    #[doc = "Checks if the value of the field is `DMA3IFG`"]
    #[inline(always)]
    pub fn is_dma3ifg(&self) -> bool {
        *self == DMAIV_A::DMA3IFG
    }
    #[doc = "Checks if the value of the field is `DMA4IFG`"]
    #[inline(always)]
    pub fn is_dma4ifg(&self) -> bool {
        *self == DMAIV_A::DMA4IFG
    }
    #[doc = "Checks if the value of the field is `DMA5IFG`"]
    #[inline(always)]
    pub fn is_dma5ifg(&self) -> bool {
        *self == DMAIV_A::DMA5IFG
    }
    #[doc = "Checks if the value of the field is `DMA6IFG`"]
    #[inline(always)]
    pub fn is_dma6ifg(&self) -> bool {
        *self == DMAIV_A::DMA6IFG
    }
    #[doc = "Checks if the value of the field is `DMA7IFG`"]
    #[inline(always)]
    pub fn is_dma7ifg(&self) -> bool {
        *self == DMAIV_A::DMA7IFG
    }
}
#[doc = "Write proxy for field `DMAIV`"]
pub struct DMAIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DMAIV_A::NONE)
    }
    #[doc = "Interrupt Source: DMA channel 0; Interrupt Flag: DMA0IFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn dma0ifg(self) -> &'a mut W {
        self.variant(DMAIV_A::DMA0IFG)
    }
    #[doc = "Interrupt Source: DMA channel 1; Interrupt Flag: DMA1IFG"]
    #[inline(always)]
    pub fn dma1ifg(self) -> &'a mut W {
        self.variant(DMAIV_A::DMA1IFG)
    }
    #[doc = "Interrupt Source: DMA channel 2; Interrupt Flag: DMA2IFG"]
    #[inline(always)]
    pub fn dma2ifg(self) -> &'a mut W {
        self.variant(DMAIV_A::DMA2IFG)
    }
    #[doc = "Interrupt Source: DMA channel 3; Interrupt Flag: DMA3IFG"]
    #[inline(always)]
    pub fn dma3ifg(self) -> &'a mut W {
        self.variant(DMAIV_A::DMA3IFG)
    }
    #[doc = "Interrupt Source: DMA channel 4; Interrupt Flag: DMA4IFG"]
    #[inline(always)]
    pub fn dma4ifg(self) -> &'a mut W {
        self.variant(DMAIV_A::DMA4IFG)
    }
    #[doc = "Interrupt Source: DMA channel 5; Interrupt Flag: DMA5IFG"]
    #[inline(always)]
    pub fn dma5ifg(self) -> &'a mut W {
        self.variant(DMAIV_A::DMA5IFG)
    }
    #[doc = "Interrupt Source: DMA channel 6; Interrupt Flag: DMA6IFG"]
    #[inline(always)]
    pub fn dma6ifg(self) -> &'a mut W {
        self.variant(DMAIV_A::DMA6IFG)
    }
    #[doc = "Interrupt Source: DMA channel 7; Interrupt Flag: DMA7IFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn dma7ifg(self) -> &'a mut W {
        self.variant(DMAIV_A::DMA7IFG)
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
DMA interrupt vector value"]
    #[inline(always)]
    pub fn dmaiv(&self) -> DMAIV_R {
        DMAIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DMA interrupt vector value"]
    #[inline(always)]
    pub fn dmaiv(&mut self) -> DMAIV_W {
        DMAIV_W { w: self }
    }
}
