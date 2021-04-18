#[doc = "Reader of register CECTL2"]
pub type R = crate::R<u16, super::CECTL2>;
#[doc = "Writer for register CECTL2"]
pub type W = crate::W<u16, super::CECTL2>;
#[doc = "Register CECTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CECTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CEREF0`"]
pub type CEREF0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CEREF0`"]
pub struct CEREF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "5:5\\]
Reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CERSEL_A {
    #[doc = "0: When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    CERSEL_0 = 0,
    #[doc = "1: When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    CERSEL_1 = 1,
}
impl From<CERSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CERSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CERSEL`"]
pub type CERSEL_R = crate::R<bool, CERSEL_A>;
impl CERSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERSEL_A {
        match self.bits {
            false => CERSEL_A::CERSEL_0,
            true => CERSEL_A::CERSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CERSEL_0`"]
    #[inline(always)]
    pub fn is_cersel_0(&self) -> bool {
        *self == CERSEL_A::CERSEL_0
    }
    #[doc = "Checks if the value of the field is `CERSEL_1`"]
    #[inline(always)]
    pub fn is_cersel_1(&self) -> bool {
        *self == CERSEL_A::CERSEL_1
    }
}
#[doc = "Write proxy for field `CERSEL`"]
pub struct CERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CERSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CERSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    #[inline(always)]
    pub fn cersel_0(self) -> &'a mut W {
        self.variant(CERSEL_A::CERSEL_0)
    }
    #[doc = "When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    #[inline(always)]
    pub fn cersel_1(self) -> &'a mut W {
        self.variant(CERSEL_A::CERSEL_1)
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
#[doc = "7:6\\]
Reference source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CERS_A {
    #[doc = "0: No current is drawn by the reference circuitry"]
    CERS_0 = 0,
    #[doc = "1: VCC applied to the resistor ladder"]
    CERS_1 = 1,
    #[doc = "2: Shared reference voltage applied to the resistor ladder"]
    CERS_2 = 2,
    #[doc = "3: Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    CERS_3 = 3,
}
impl From<CERS_A> for u8 {
    #[inline(always)]
    fn from(variant: CERS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CERS`"]
pub type CERS_R = crate::R<u8, CERS_A>;
impl CERS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERS_A {
        match self.bits {
            0 => CERS_A::CERS_0,
            1 => CERS_A::CERS_1,
            2 => CERS_A::CERS_2,
            3 => CERS_A::CERS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CERS_0`"]
    #[inline(always)]
    pub fn is_cers_0(&self) -> bool {
        *self == CERS_A::CERS_0
    }
    #[doc = "Checks if the value of the field is `CERS_1`"]
    #[inline(always)]
    pub fn is_cers_1(&self) -> bool {
        *self == CERS_A::CERS_1
    }
    #[doc = "Checks if the value of the field is `CERS_2`"]
    #[inline(always)]
    pub fn is_cers_2(&self) -> bool {
        *self == CERS_A::CERS_2
    }
    #[doc = "Checks if the value of the field is `CERS_3`"]
    #[inline(always)]
    pub fn is_cers_3(&self) -> bool {
        *self == CERS_A::CERS_3
    }
}
#[doc = "Write proxy for field `CERS`"]
pub struct CERS_W<'a> {
    w: &'a mut W,
}
impl<'a> CERS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CERS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No current is drawn by the reference circuitry"]
    #[inline(always)]
    pub fn cers_0(self) -> &'a mut W {
        self.variant(CERS_A::CERS_0)
    }
    #[doc = "VCC applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_1(self) -> &'a mut W {
        self.variant(CERS_A::CERS_1)
    }
    #[doc = "Shared reference voltage applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_2(self) -> &'a mut W {
        self.variant(CERS_A::CERS_2)
    }
    #[doc = "Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    #[inline(always)]
    pub fn cers_3(self) -> &'a mut W {
        self.variant(CERS_A::CERS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `CEREF1`"]
pub type CEREF1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CEREF1`"]
pub struct CEREF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
#[doc = "14:13\\]
Reference voltage level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEREFL_A {
    #[doc = "0: Reference amplifier is disabled. No reference voltage is requested"]
    OFF = 0,
    #[doc = "1: 1.2 V is selected as shared reference voltage input"]
    _1P2V = 1,
    #[doc = "2: 2.0 V is selected as shared reference voltage input"]
    _2P0V = 2,
    #[doc = "3: 2.5 V is selected as shared reference voltage input"]
    _2P5V = 3,
}
impl From<CEREFL_A> for u8 {
    #[inline(always)]
    fn from(variant: CEREFL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEREFL`"]
pub type CEREFL_R = crate::R<u8, CEREFL_A>;
impl CEREFL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREFL_A {
        match self.bits {
            0 => CEREFL_A::OFF,
            1 => CEREFL_A::_1P2V,
            2 => CEREFL_A::_2P0V,
            3 => CEREFL_A::_2P5V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CEREFL_A::OFF
    }
    #[doc = "Checks if the value of the field is `_1P2V`"]
    #[inline(always)]
    pub fn is_1p2v(&self) -> bool {
        *self == CEREFL_A::_1P2V
    }
    #[doc = "Checks if the value of the field is `_2P0V`"]
    #[inline(always)]
    pub fn is_2p0v(&self) -> bool {
        *self == CEREFL_A::_2P0V
    }
    #[doc = "Checks if the value of the field is `_2P5V`"]
    #[inline(always)]
    pub fn is_2p5v(&self) -> bool {
        *self == CEREFL_A::_2P5V
    }
}
#[doc = "Write proxy for field `CEREFL`"]
pub struct CEREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREFL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEREFL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Reference amplifier is disabled. No reference voltage is requested"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CEREFL_A::OFF)
    }
    #[doc = "1.2 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn _1p2v(self) -> &'a mut W {
        self.variant(CEREFL_A::_1P2V)
    }
    #[doc = "2.0 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn _2p0v(self) -> &'a mut W {
        self.variant(CEREFL_A::_2P0V)
    }
    #[doc = "2.5 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn _2p5v(self) -> &'a mut W {
        self.variant(CEREFL_A::_2P5V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u16) & 0x03) << 13);
        self.w
    }
}
#[doc = "15:15\\]
Reference accuracy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEREFACC_A {
    #[doc = "0: Static mode"]
    STATIC = 0,
    #[doc = "1: Clocked (low power, low accuracy) mode"]
    CLOCKED = 1,
}
impl From<CEREFACC_A> for bool {
    #[inline(always)]
    fn from(variant: CEREFACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEREFACC`"]
pub type CEREFACC_R = crate::R<bool, CEREFACC_A>;
impl CEREFACC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREFACC_A {
        match self.bits {
            false => CEREFACC_A::STATIC,
            true => CEREFACC_A::CLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static_(&self) -> bool {
        *self == CEREFACC_A::STATIC
    }
    #[doc = "Checks if the value of the field is `CLOCKED`"]
    #[inline(always)]
    pub fn is_clocked(&self) -> bool {
        *self == CEREFACC_A::CLOCKED
    }
}
#[doc = "Write proxy for field `CEREFACC`"]
pub struct CEREFACC_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREFACC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEREFACC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(CEREFACC_A::STATIC)
    }
    #[doc = "Clocked (low power, low accuracy) mode"]
    #[inline(always)]
    pub fn clocked(self) -> &'a mut W {
        self.variant(CEREFACC_A::CLOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&self) -> CEREF0_R {
        CEREF0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Reference select"]
    #[inline(always)]
    pub fn cersel(&self) -> CERSEL_R {
        CERSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Reference source"]
    #[inline(always)]
    pub fn cers(&self) -> CERS_R {
        CERS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&self) -> CEREF1_R {
        CEREF1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&self) -> CEREFL_R {
        CEREFL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&self) -> CEREFACC_R {
        CEREFACC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&mut self) -> CEREF0_W {
        CEREF0_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Reference select"]
    #[inline(always)]
    pub fn cersel(&mut self) -> CERSEL_W {
        CERSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Reference source"]
    #[inline(always)]
    pub fn cers(&mut self) -> CERS_W {
        CERS_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&mut self) -> CEREF1_W {
        CEREF1_W { w: self }
    }
    #[doc = "Bits 13:14 - 14:13\\]
Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&mut self) -> CEREFL_W {
        CEREFL_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&mut self) -> CEREFACC_W {
        CEREFACC_W { w: self }
    }
}
