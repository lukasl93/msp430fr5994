#[doc = "Reader of register DMA1CTL"]
pub type R = crate::R<u16, super::DMA1CTL>;
#[doc = "Writer for register DMA1CTL"]
pub type W = crate::W<u16, super::DMA1CTL>;
#[doc = "Register DMA1CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA1CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAREQ_A {
    #[doc = "0: No DMA start"]
    DMAREQ_0 = 0,
    #[doc = "1: Start DMA"]
    DMAREQ_1 = 1,
}
impl From<DMAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: DMAREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAREQ`"]
pub type DMAREQ_R = crate::R<bool, DMAREQ_A>;
impl DMAREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAREQ_A {
        match self.bits {
            false => DMAREQ_A::DMAREQ_0,
            true => DMAREQ_A::DMAREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAREQ_0`"]
    #[inline(always)]
    pub fn is_dmareq_0(&self) -> bool {
        *self == DMAREQ_A::DMAREQ_0
    }
    #[doc = "Checks if the value of the field is `DMAREQ_1`"]
    #[inline(always)]
    pub fn is_dmareq_1(&self) -> bool {
        *self == DMAREQ_A::DMAREQ_1
    }
}
#[doc = "Write proxy for field `DMAREQ`"]
pub struct DMAREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA start"]
    #[inline(always)]
    pub fn dmareq_0(self) -> &'a mut W {
        self.variant(DMAREQ_A::DMAREQ_0)
    }
    #[doc = "Start DMA"]
    #[inline(always)]
    pub fn dmareq_1(self) -> &'a mut W {
        self.variant(DMAREQ_A::DMAREQ_1)
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
DMA abort\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAABORT_A {
    #[doc = "0: DMA transfer not interrupted"]
    DMAABORT_0 = 0,
    #[doc = "1: DMA transfer interrupted by NMI"]
    DMAABORT_1 = 1,
}
impl From<DMAABORT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAABORT`"]
pub type DMAABORT_R = crate::R<bool, DMAABORT_A>;
impl DMAABORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAABORT_A {
        match self.bits {
            false => DMAABORT_A::DMAABORT_0,
            true => DMAABORT_A::DMAABORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAABORT_0`"]
    #[inline(always)]
    pub fn is_dmaabort_0(&self) -> bool {
        *self == DMAABORT_A::DMAABORT_0
    }
    #[doc = "Checks if the value of the field is `DMAABORT_1`"]
    #[inline(always)]
    pub fn is_dmaabort_1(&self) -> bool {
        *self == DMAABORT_A::DMAABORT_1
    }
}
#[doc = "Write proxy for field `DMAABORT`"]
pub struct DMAABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAABORT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA transfer not interrupted"]
    #[inline(always)]
    pub fn dmaabort_0(self) -> &'a mut W {
        self.variant(DMAABORT_A::DMAABORT_0)
    }
    #[doc = "DMA transfer interrupted by NMI"]
    #[inline(always)]
    pub fn dmaabort_1(self) -> &'a mut W {
        self.variant(DMAABORT_A::DMAABORT_1)
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
#[doc = "2:2\\]
DMA interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAIE_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<DMAIE_A> for bool {
    #[inline(always)]
    fn from(variant: DMAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAIE`"]
pub type DMAIE_R = crate::R<bool, DMAIE_A>;
impl DMAIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAIE_A {
        match self.bits {
            false => DMAIE_A::DISABLE,
            true => DMAIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `DMAIE`"]
pub struct DMAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAIE_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "3:3\\]
DMA interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAIFG_A {
    #[doc = "0: No interrupt pending"]
    DMAIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    DMAIFG_1 = 1,
}
impl From<DMAIFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMAIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAIFG`"]
pub type DMAIFG_R = crate::R<bool, DMAIFG_A>;
impl DMAIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAIFG_A {
        match self.bits {
            false => DMAIFG_A::DMAIFG_0,
            true => DMAIFG_A::DMAIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAIFG_0`"]
    #[inline(always)]
    pub fn is_dmaifg_0(&self) -> bool {
        *self == DMAIFG_A::DMAIFG_0
    }
    #[doc = "Checks if the value of the field is `DMAIFG_1`"]
    #[inline(always)]
    pub fn is_dmaifg_1(&self) -> bool {
        *self == DMAIFG_A::DMAIFG_1
    }
}
#[doc = "Write proxy for field `DMAIFG`"]
pub struct DMAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn dmaifg_0(self) -> &'a mut W {
        self.variant(DMAIFG_A::DMAIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn dmaifg_1(self) -> &'a mut W {
        self.variant(DMAIFG_A::DMAIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "4:4\\]
DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLE,
            true => DMAEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLE)
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
#[doc = "5:5\\]
DMA level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMALEVEL_A {
    #[doc = "0: Edge sensitive (rising edge)"]
    EDGE = 0,
    #[doc = "1: Level sensitive (high level)"]
    LEVEL = 1,
}
impl From<DMALEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: DMALEVEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMALEVEL`"]
pub type DMALEVEL_R = crate::R<bool, DMALEVEL_A>;
impl DMALEVEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMALEVEL_A {
        match self.bits {
            false => DMALEVEL_A::EDGE,
            true => DMALEVEL_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == DMALEVEL_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DMALEVEL_A::LEVEL
    }
}
#[doc = "Write proxy for field `DMALEVEL`"]
pub struct DMALEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMALEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMALEVEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Edge sensitive (rising edge)"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(DMALEVEL_A::EDGE)
    }
    #[doc = "Level sensitive (high level)"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(DMALEVEL_A::LEVEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "6:6\\]
DMA source byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASRCBYTE_A {
    #[doc = "0: Word"]
    WORD = 0,
    #[doc = "1: Byte"]
    BYTE = 1,
}
impl From<DMASRCBYTE_A> for bool {
    #[inline(always)]
    fn from(variant: DMASRCBYTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMASRCBYTE`"]
pub type DMASRCBYTE_R = crate::R<bool, DMASRCBYTE_A>;
impl DMASRCBYTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASRCBYTE_A {
        match self.bits {
            false => DMASRCBYTE_A::WORD,
            true => DMASRCBYTE_A::BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DMASRCBYTE_A::WORD
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DMASRCBYTE_A::BYTE
    }
}
#[doc = "Write proxy for field `DMASRCBYTE`"]
pub struct DMASRCBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASRCBYTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMASRCBYTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DMASRCBYTE_A::WORD)
    }
    #[doc = "Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DMASRCBYTE_A::BYTE)
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
DMA destination byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMADSTBYTE_A {
    #[doc = "0: Word"]
    WORD = 0,
    #[doc = "1: Byte"]
    BYTE = 1,
}
impl From<DMADSTBYTE_A> for bool {
    #[inline(always)]
    fn from(variant: DMADSTBYTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMADSTBYTE`"]
pub type DMADSTBYTE_R = crate::R<bool, DMADSTBYTE_A>;
impl DMADSTBYTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADSTBYTE_A {
        match self.bits {
            false => DMADSTBYTE_A::WORD,
            true => DMADSTBYTE_A::BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DMADSTBYTE_A::WORD
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DMADSTBYTE_A::BYTE
    }
}
#[doc = "Write proxy for field `DMADSTBYTE`"]
pub struct DMADSTBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADSTBYTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADSTBYTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DMADSTBYTE_A::WORD)
    }
    #[doc = "Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DMADSTBYTE_A::BYTE)
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
#[doc = "9:8\\]
DMA source increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMASRCINCR_A {
    #[doc = "0: Source address is unchanged"]
    DMASRCINCR_0 = 0,
    #[doc = "1: Source address is unchanged"]
    DMASRCINCR_1 = 1,
    #[doc = "2: Source address is decremented"]
    DMASRCINCR_2 = 2,
    #[doc = "3: Source address is incremented"]
    DMASRCINCR_3 = 3,
}
impl From<DMASRCINCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASRCINCR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMASRCINCR`"]
pub type DMASRCINCR_R = crate::R<u8, DMASRCINCR_A>;
impl DMASRCINCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASRCINCR_A {
        match self.bits {
            0 => DMASRCINCR_A::DMASRCINCR_0,
            1 => DMASRCINCR_A::DMASRCINCR_1,
            2 => DMASRCINCR_A::DMASRCINCR_2,
            3 => DMASRCINCR_A::DMASRCINCR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_0`"]
    #[inline(always)]
    pub fn is_dmasrcincr_0(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_0
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_1`"]
    #[inline(always)]
    pub fn is_dmasrcincr_1(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_1
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_2`"]
    #[inline(always)]
    pub fn is_dmasrcincr_2(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_2
    }
    #[doc = "Checks if the value of the field is `DMASRCINCR_3`"]
    #[inline(always)]
    pub fn is_dmasrcincr_3(&self) -> bool {
        *self == DMASRCINCR_A::DMASRCINCR_3
    }
}
#[doc = "Write proxy for field `DMASRCINCR`"]
pub struct DMASRCINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASRCINCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMASRCINCR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Source address is unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_0(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_0)
    }
    #[doc = "Source address is unchanged"]
    #[inline(always)]
    pub fn dmasrcincr_1(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_1)
    }
    #[doc = "Source address is decremented"]
    #[inline(always)]
    pub fn dmasrcincr_2(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_2)
    }
    #[doc = "Source address is incremented"]
    #[inline(always)]
    pub fn dmasrcincr_3(self) -> &'a mut W {
        self.variant(DMASRCINCR_A::DMASRCINCR_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "11:10\\]
DMA destination increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMADSTINCR_A {
    #[doc = "0: Destination address is unchanged"]
    DMADSTINCR_0 = 0,
    #[doc = "1: Destination address is unchanged"]
    DMADSTINCR_1 = 1,
    #[doc = "2: Destination address is decremented"]
    DMADSTINCR_2 = 2,
    #[doc = "3: Destination address is incremented"]
    DMADSTINCR_3 = 3,
}
impl From<DMADSTINCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DMADSTINCR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMADSTINCR`"]
pub type DMADSTINCR_R = crate::R<u8, DMADSTINCR_A>;
impl DMADSTINCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADSTINCR_A {
        match self.bits {
            0 => DMADSTINCR_A::DMADSTINCR_0,
            1 => DMADSTINCR_A::DMADSTINCR_1,
            2 => DMADSTINCR_A::DMADSTINCR_2,
            3 => DMADSTINCR_A::DMADSTINCR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_0`"]
    #[inline(always)]
    pub fn is_dmadstincr_0(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_0
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_1`"]
    #[inline(always)]
    pub fn is_dmadstincr_1(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_1
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_2`"]
    #[inline(always)]
    pub fn is_dmadstincr_2(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_2
    }
    #[doc = "Checks if the value of the field is `DMADSTINCR_3`"]
    #[inline(always)]
    pub fn is_dmadstincr_3(&self) -> bool {
        *self == DMADSTINCR_A::DMADSTINCR_3
    }
}
#[doc = "Write proxy for field `DMADSTINCR`"]
pub struct DMADSTINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADSTINCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADSTINCR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Destination address is unchanged"]
    #[inline(always)]
    pub fn dmadstincr_0(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_0)
    }
    #[doc = "Destination address is unchanged"]
    #[inline(always)]
    pub fn dmadstincr_1(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_1)
    }
    #[doc = "Destination address is decremented"]
    #[inline(always)]
    pub fn dmadstincr_2(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_2)
    }
    #[doc = "Destination address is incremented"]
    #[inline(always)]
    pub fn dmadstincr_3(self) -> &'a mut W {
        self.variant(DMADSTINCR_A::DMADSTINCR_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "14:12\\]
DMA transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMADT_A {
    #[doc = "0: Single transfer"]
    DMADT_0 = 0,
    #[doc = "1: Block transfer"]
    DMADT_1 = 1,
    #[doc = "2: Burst-block transfer"]
    DMADT_2 = 2,
    #[doc = "3: Burst-block transfer"]
    DMADT_3 = 3,
    #[doc = "4: Repeated single transfer"]
    DMADT_4 = 4,
    #[doc = "5: Repeated block transfer"]
    DMADT_5 = 5,
    #[doc = "6: Repeated burst-block transfer"]
    DMADT_6 = 6,
    #[doc = "7: Repeated burst-block transfer"]
    DMADT_7 = 7,
}
impl From<DMADT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMADT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMADT`"]
pub type DMADT_R = crate::R<u8, DMADT_A>;
impl DMADT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADT_A {
        match self.bits {
            0 => DMADT_A::DMADT_0,
            1 => DMADT_A::DMADT_1,
            2 => DMADT_A::DMADT_2,
            3 => DMADT_A::DMADT_3,
            4 => DMADT_A::DMADT_4,
            5 => DMADT_A::DMADT_5,
            6 => DMADT_A::DMADT_6,
            7 => DMADT_A::DMADT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMADT_0`"]
    #[inline(always)]
    pub fn is_dmadt_0(&self) -> bool {
        *self == DMADT_A::DMADT_0
    }
    #[doc = "Checks if the value of the field is `DMADT_1`"]
    #[inline(always)]
    pub fn is_dmadt_1(&self) -> bool {
        *self == DMADT_A::DMADT_1
    }
    #[doc = "Checks if the value of the field is `DMADT_2`"]
    #[inline(always)]
    pub fn is_dmadt_2(&self) -> bool {
        *self == DMADT_A::DMADT_2
    }
    #[doc = "Checks if the value of the field is `DMADT_3`"]
    #[inline(always)]
    pub fn is_dmadt_3(&self) -> bool {
        *self == DMADT_A::DMADT_3
    }
    #[doc = "Checks if the value of the field is `DMADT_4`"]
    #[inline(always)]
    pub fn is_dmadt_4(&self) -> bool {
        *self == DMADT_A::DMADT_4
    }
    #[doc = "Checks if the value of the field is `DMADT_5`"]
    #[inline(always)]
    pub fn is_dmadt_5(&self) -> bool {
        *self == DMADT_A::DMADT_5
    }
    #[doc = "Checks if the value of the field is `DMADT_6`"]
    #[inline(always)]
    pub fn is_dmadt_6(&self) -> bool {
        *self == DMADT_A::DMADT_6
    }
    #[doc = "Checks if the value of the field is `DMADT_7`"]
    #[inline(always)]
    pub fn is_dmadt_7(&self) -> bool {
        *self == DMADT_A::DMADT_7
    }
}
#[doc = "Write proxy for field `DMADT`"]
pub struct DMADT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single transfer"]
    #[inline(always)]
    pub fn dmadt_0(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_0)
    }
    #[doc = "Block transfer"]
    #[inline(always)]
    pub fn dmadt_1(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_1)
    }
    #[doc = "Burst-block transfer"]
    #[inline(always)]
    pub fn dmadt_2(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_2)
    }
    #[doc = "Burst-block transfer"]
    #[inline(always)]
    pub fn dmadt_3(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_3)
    }
    #[doc = "Repeated single transfer"]
    #[inline(always)]
    pub fn dmadt_4(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_4)
    }
    #[doc = "Repeated block transfer"]
    #[inline(always)]
    pub fn dmadt_5(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_5)
    }
    #[doc = "Repeated burst-block transfer"]
    #[inline(always)]
    pub fn dmadt_6(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_6)
    }
    #[doc = "Repeated burst-block transfer"]
    #[inline(always)]
    pub fn dmadt_7(self) -> &'a mut W {
        self.variant(DMADT_A::DMADT_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMA request"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DMA abort"]
    #[inline(always)]
    pub fn dmaabort(&self) -> DMAABORT_R {
        DMAABORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
DMA interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&self) -> DMAIE_R {
        DMAIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
DMA interrupt flag"]
    #[inline(always)]
    pub fn dmaifg(&self) -> DMAIFG_R {
        DMAIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
DMA level"]
    #[inline(always)]
    pub fn dmalevel(&self) -> DMALEVEL_R {
        DMALEVEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
DMA source byte"]
    #[inline(always)]
    pub fn dmasrcbyte(&self) -> DMASRCBYTE_R {
        DMASRCBYTE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
DMA destination byte"]
    #[inline(always)]
    pub fn dmadstbyte(&self) -> DMADSTBYTE_R {
        DMADSTBYTE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
DMA source increment"]
    #[inline(always)]
    pub fn dmasrcincr(&self) -> DMASRCINCR_R {
        DMASRCINCR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
DMA destination increment"]
    #[inline(always)]
    pub fn dmadstincr(&self) -> DMADSTINCR_R {
        DMADSTINCR_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
DMA transfer mode"]
    #[inline(always)]
    pub fn dmadt(&self) -> DMADT_R {
        DMADT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMA request"]
    #[inline(always)]
    pub fn dmareq(&mut self) -> DMAREQ_W {
        DMAREQ_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
DMA abort"]
    #[inline(always)]
    pub fn dmaabort(&mut self) -> DMAABORT_W {
        DMAABORT_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
DMA interrupt enable"]
    #[inline(always)]
    pub fn dmaie(&mut self) -> DMAIE_W {
        DMAIE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
DMA interrupt flag"]
    #[inline(always)]
    pub fn dmaifg(&mut self) -> DMAIFG_W {
        DMAIFG_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
DMA level"]
    #[inline(always)]
    pub fn dmalevel(&mut self) -> DMALEVEL_W {
        DMALEVEL_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
DMA source byte"]
    #[inline(always)]
    pub fn dmasrcbyte(&mut self) -> DMASRCBYTE_W {
        DMASRCBYTE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
DMA destination byte"]
    #[inline(always)]
    pub fn dmadstbyte(&mut self) -> DMADSTBYTE_W {
        DMADSTBYTE_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
DMA source increment"]
    #[inline(always)]
    pub fn dmasrcincr(&mut self) -> DMASRCINCR_W {
        DMASRCINCR_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
DMA destination increment"]
    #[inline(always)]
    pub fn dmadstincr(&mut self) -> DMADSTINCR_W {
        DMADSTINCR_W { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\]
DMA transfer mode"]
    #[inline(always)]
    pub fn dmadt(&mut self) -> DMADT_W {
        DMADT_W { w: self }
    }
}
