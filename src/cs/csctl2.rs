#[doc = "Reader of register CSCTL2"]
pub type R = crate::R<u16, super::CSCTL2>;
#[doc = "Writer for register CSCTL2"]
pub type W = crate::W<u16, super::CSCTL2>;
#[doc = "Register CSCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "2:0\\]
Selects the MCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELM_A {
    #[doc = "0: LFXTCLK when LFXT available, otherwise VLOCLK"]
    LFXTCLK = 0,
    #[doc = "1: VLOCLK"]
    VLOCLK = 1,
    #[doc = "2: LFMODCLK"]
    LFMODCLK = 2,
    #[doc = "3: DCOCLK"]
    DCOCLK = 3,
    #[doc = "4: MODCLK"]
    MODCLK = 4,
    #[doc = "5: HFXTCLK when HFXT available, otherwise DCOCLK"]
    HFXTCLK = 5,
}
impl From<SELM_A> for u8 {
    #[inline(always)]
    fn from(variant: SELM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELM`"]
pub type SELM_R = crate::R<u8, SELM_A>;
impl SELM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SELM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SELM_A::LFXTCLK),
            1 => Val(SELM_A::VLOCLK),
            2 => Val(SELM_A::LFMODCLK),
            3 => Val(SELM_A::DCOCLK),
            4 => Val(SELM_A::MODCLK),
            5 => Val(SELM_A::HFXTCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LFXTCLK`"]
    #[inline(always)]
    pub fn is_lfxtclk(&self) -> bool {
        *self == SELM_A::LFXTCLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == SELM_A::VLOCLK
    }
    #[doc = "Checks if the value of the field is `LFMODCLK`"]
    #[inline(always)]
    pub fn is_lfmodclk(&self) -> bool {
        *self == SELM_A::LFMODCLK
    }
    #[doc = "Checks if the value of the field is `DCOCLK`"]
    #[inline(always)]
    pub fn is_dcoclk(&self) -> bool {
        *self == SELM_A::DCOCLK
    }
    #[doc = "Checks if the value of the field is `MODCLK`"]
    #[inline(always)]
    pub fn is_modclk(&self) -> bool {
        *self == SELM_A::MODCLK
    }
    #[doc = "Checks if the value of the field is `HFXTCLK`"]
    #[inline(always)]
    pub fn is_hfxtclk(&self) -> bool {
        *self == SELM_A::HFXTCLK
    }
}
#[doc = "Write proxy for field `SELM`"]
pub struct SELM_W<'a> {
    w: &'a mut W,
}
impl<'a> SELM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFXTCLK when LFXT available, otherwise VLOCLK"]
    #[inline(always)]
    pub fn lfxtclk(self) -> &'a mut W {
        self.variant(SELM_A::LFXTCLK)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(SELM_A::VLOCLK)
    }
    #[doc = "LFMODCLK"]
    #[inline(always)]
    pub fn lfmodclk(self) -> &'a mut W {
        self.variant(SELM_A::LFMODCLK)
    }
    #[doc = "DCOCLK"]
    #[inline(always)]
    pub fn dcoclk(self) -> &'a mut W {
        self.variant(SELM_A::DCOCLK)
    }
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn modclk(self) -> &'a mut W {
        self.variant(SELM_A::MODCLK)
    }
    #[doc = "HFXTCLK when HFXT available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn hfxtclk(self) -> &'a mut W {
        self.variant(SELM_A::HFXTCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "6:4\\]
Selects the SMCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELS_A {
    #[doc = "0: LFXTCLK when LFXT available, otherwise VLOCLK."]
    LFXTCLK = 0,
    #[doc = "1: VLOCLK"]
    VLOCLK = 1,
    #[doc = "2: LFMODCLK"]
    LFMODCLK = 2,
    #[doc = "3: DCOCLK"]
    DCOCLK = 3,
    #[doc = "4: MODCLK"]
    MODCLK = 4,
    #[doc = "5: HFXTCLK when HFXT available, otherwise DCOCLK."]
    HFXTCLK = 5,
}
impl From<SELS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELS`"]
pub type SELS_R = crate::R<u8, SELS_A>;
impl SELS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SELS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SELS_A::LFXTCLK),
            1 => Val(SELS_A::VLOCLK),
            2 => Val(SELS_A::LFMODCLK),
            3 => Val(SELS_A::DCOCLK),
            4 => Val(SELS_A::MODCLK),
            5 => Val(SELS_A::HFXTCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LFXTCLK`"]
    #[inline(always)]
    pub fn is_lfxtclk(&self) -> bool {
        *self == SELS_A::LFXTCLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == SELS_A::VLOCLK
    }
    #[doc = "Checks if the value of the field is `LFMODCLK`"]
    #[inline(always)]
    pub fn is_lfmodclk(&self) -> bool {
        *self == SELS_A::LFMODCLK
    }
    #[doc = "Checks if the value of the field is `DCOCLK`"]
    #[inline(always)]
    pub fn is_dcoclk(&self) -> bool {
        *self == SELS_A::DCOCLK
    }
    #[doc = "Checks if the value of the field is `MODCLK`"]
    #[inline(always)]
    pub fn is_modclk(&self) -> bool {
        *self == SELS_A::MODCLK
    }
    #[doc = "Checks if the value of the field is `HFXTCLK`"]
    #[inline(always)]
    pub fn is_hfxtclk(&self) -> bool {
        *self == SELS_A::HFXTCLK
    }
}
#[doc = "Write proxy for field `SELS`"]
pub struct SELS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFXTCLK when LFXT available, otherwise VLOCLK."]
    #[inline(always)]
    pub fn lfxtclk(self) -> &'a mut W {
        self.variant(SELS_A::LFXTCLK)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(SELS_A::VLOCLK)
    }
    #[doc = "LFMODCLK"]
    #[inline(always)]
    pub fn lfmodclk(self) -> &'a mut W {
        self.variant(SELS_A::LFMODCLK)
    }
    #[doc = "DCOCLK"]
    #[inline(always)]
    pub fn dcoclk(self) -> &'a mut W {
        self.variant(SELS_A::DCOCLK)
    }
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn modclk(self) -> &'a mut W {
        self.variant(SELS_A::MODCLK)
    }
    #[doc = "HFXTCLK when HFXT available, otherwise DCOCLK."]
    #[inline(always)]
    pub fn hfxtclk(self) -> &'a mut W {
        self.variant(SELS_A::HFXTCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "10:8\\]
Selects the ACLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELA_A {
    #[doc = "0: LFXTCLK when LFXT available, otherwise VLOCLK."]
    LFXTCLK = 0,
    #[doc = "1: VLOCLK"]
    VLOCLK = 1,
    #[doc = "2: LFMODCLK"]
    LFMODCLK = 2,
}
impl From<SELA_A> for u8 {
    #[inline(always)]
    fn from(variant: SELA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELA`"]
pub type SELA_R = crate::R<u8, SELA_A>;
impl SELA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SELA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SELA_A::LFXTCLK),
            1 => Val(SELA_A::VLOCLK),
            2 => Val(SELA_A::LFMODCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LFXTCLK`"]
    #[inline(always)]
    pub fn is_lfxtclk(&self) -> bool {
        *self == SELA_A::LFXTCLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == SELA_A::VLOCLK
    }
    #[doc = "Checks if the value of the field is `LFMODCLK`"]
    #[inline(always)]
    pub fn is_lfmodclk(&self) -> bool {
        *self == SELA_A::LFMODCLK
    }
}
#[doc = "Write proxy for field `SELA`"]
pub struct SELA_W<'a> {
    w: &'a mut W,
}
impl<'a> SELA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFXTCLK when LFXT available, otherwise VLOCLK."]
    #[inline(always)]
    pub fn lfxtclk(self) -> &'a mut W {
        self.variant(SELA_A::LFXTCLK)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(SELA_A::VLOCLK)
    }
    #[doc = "LFMODCLK"]
    #[inline(always)]
    pub fn lfmodclk(self) -> &'a mut W {
        self.variant(SELA_A::LFMODCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the MCLK source"]
    #[inline(always)]
    pub fn selm(&self) -> SELM_R {
        SELM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Selects the SMCLK source"]
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&self) -> SELA_R {
        SELA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the MCLK source"]
    #[inline(always)]
    pub fn selm(&mut self) -> SELM_W {
        SELM_W { w: self }
    }
    #[doc = "Bits 4:6 - 6:4\\]
Selects the SMCLK source"]
    #[inline(always)]
    pub fn sels(&mut self) -> SELS_W {
        SELS_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&mut self) -> SELA_W {
        SELA_W { w: self }
    }
}
