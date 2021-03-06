#[doc = "Reader of register CAPTIO1CTL"]
pub type R = crate::R<u16, super::CAPTIO1CTL>;
#[doc = "Writer for register CAPTIO1CTL"]
pub type W = crate::W<u16, super::CAPTIO1CTL>;
#[doc = "Register CAPTIO1CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CAPTIO1CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "3:1\\]
Capacitive Touch IO pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTIOPISEL1_A {
    #[doc = "0: Px.0"]
    CAPTIOPISEL_0 = 0,
    #[doc = "1: Px.1"]
    CAPTIOPISEL_1 = 1,
    #[doc = "2: Px.2"]
    CAPTIOPISEL_2 = 2,
    #[doc = "3: Px.3"]
    CAPTIOPISEL_3 = 3,
    #[doc = "4: Px.4"]
    CAPTIOPISEL_4 = 4,
    #[doc = "5: Px.5"]
    CAPTIOPISEL_5 = 5,
    #[doc = "6: Px.6"]
    CAPTIOPISEL_6 = 6,
    #[doc = "7: Px.7"]
    CAPTIOPISEL_7 = 7,
}
impl From<CAPTIOPISEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTIOPISEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CAPTIOPISEL1`"]
pub type CAPTIOPISEL1_R = crate::R<u8, CAPTIOPISEL1_A>;
impl CAPTIOPISEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTIOPISEL1_A {
        match self.bits {
            0 => CAPTIOPISEL1_A::CAPTIOPISEL_0,
            1 => CAPTIOPISEL1_A::CAPTIOPISEL_1,
            2 => CAPTIOPISEL1_A::CAPTIOPISEL_2,
            3 => CAPTIOPISEL1_A::CAPTIOPISEL_3,
            4 => CAPTIOPISEL1_A::CAPTIOPISEL_4,
            5 => CAPTIOPISEL1_A::CAPTIOPISEL_5,
            6 => CAPTIOPISEL1_A::CAPTIOPISEL_6,
            7 => CAPTIOPISEL1_A::CAPTIOPISEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISEL_0`"]
    #[inline(always)]
    pub fn is_captiopisel_0(&self) -> bool {
        *self == CAPTIOPISEL1_A::CAPTIOPISEL_0
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISEL_1`"]
    #[inline(always)]
    pub fn is_captiopisel_1(&self) -> bool {
        *self == CAPTIOPISEL1_A::CAPTIOPISEL_1
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISEL_2`"]
    #[inline(always)]
    pub fn is_captiopisel_2(&self) -> bool {
        *self == CAPTIOPISEL1_A::CAPTIOPISEL_2
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISEL_3`"]
    #[inline(always)]
    pub fn is_captiopisel_3(&self) -> bool {
        *self == CAPTIOPISEL1_A::CAPTIOPISEL_3
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISEL_4`"]
    #[inline(always)]
    pub fn is_captiopisel_4(&self) -> bool {
        *self == CAPTIOPISEL1_A::CAPTIOPISEL_4
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISEL_5`"]
    #[inline(always)]
    pub fn is_captiopisel_5(&self) -> bool {
        *self == CAPTIOPISEL1_A::CAPTIOPISEL_5
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISEL_6`"]
    #[inline(always)]
    pub fn is_captiopisel_6(&self) -> bool {
        *self == CAPTIOPISEL1_A::CAPTIOPISEL_6
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISEL_7`"]
    #[inline(always)]
    pub fn is_captiopisel_7(&self) -> bool {
        *self == CAPTIOPISEL1_A::CAPTIOPISEL_7
    }
}
#[doc = "Write proxy for field `CAPTIOPISEL1`"]
pub struct CAPTIOPISEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPISEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTIOPISEL1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Px.0"]
    #[inline(always)]
    pub fn captiopisel_0(self) -> &'a mut W {
        self.variant(CAPTIOPISEL1_A::CAPTIOPISEL_0)
    }
    #[doc = "Px.1"]
    #[inline(always)]
    pub fn captiopisel_1(self) -> &'a mut W {
        self.variant(CAPTIOPISEL1_A::CAPTIOPISEL_1)
    }
    #[doc = "Px.2"]
    #[inline(always)]
    pub fn captiopisel_2(self) -> &'a mut W {
        self.variant(CAPTIOPISEL1_A::CAPTIOPISEL_2)
    }
    #[doc = "Px.3"]
    #[inline(always)]
    pub fn captiopisel_3(self) -> &'a mut W {
        self.variant(CAPTIOPISEL1_A::CAPTIOPISEL_3)
    }
    #[doc = "Px.4"]
    #[inline(always)]
    pub fn captiopisel_4(self) -> &'a mut W {
        self.variant(CAPTIOPISEL1_A::CAPTIOPISEL_4)
    }
    #[doc = "Px.5"]
    #[inline(always)]
    pub fn captiopisel_5(self) -> &'a mut W {
        self.variant(CAPTIOPISEL1_A::CAPTIOPISEL_5)
    }
    #[doc = "Px.6"]
    #[inline(always)]
    pub fn captiopisel_6(self) -> &'a mut W {
        self.variant(CAPTIOPISEL1_A::CAPTIOPISEL_6)
    }
    #[doc = "Px.7"]
    #[inline(always)]
    pub fn captiopisel_7(self) -> &'a mut W {
        self.variant(CAPTIOPISEL1_A::CAPTIOPISEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u16) & 0x07) << 1);
        self.w
    }
}
#[doc = "7:4\\]
Capacitive Touch IO port select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTIOPOSEL1_A {
    #[doc = "0: Px = PJ"]
    PJ = 0,
    #[doc = "1: Px = P1"]
    P1 = 1,
    #[doc = "2: Px = P2"]
    P2 = 2,
    #[doc = "3: Px = P3"]
    P3 = 3,
    #[doc = "4: Px = P4"]
    P4 = 4,
    #[doc = "5: Px = P5"]
    P5 = 5,
    #[doc = "6: Px = P6"]
    P6 = 6,
    #[doc = "7: Px = P7"]
    P7 = 7,
    #[doc = "8: Px = P8"]
    P8 = 8,
    #[doc = "9: Px = P9"]
    P9 = 9,
    #[doc = "10: Px = P10"]
    P10 = 10,
    #[doc = "11: Px = P11"]
    P11 = 11,
    #[doc = "12: Px = P12"]
    P12 = 12,
    #[doc = "13: Px = P13"]
    P13 = 13,
    #[doc = "14: Px = P14"]
    P14 = 14,
    #[doc = "15: Px = P15"]
    P15 = 15,
}
impl From<CAPTIOPOSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTIOPOSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CAPTIOPOSEL1`"]
pub type CAPTIOPOSEL1_R = crate::R<u8, CAPTIOPOSEL1_A>;
impl CAPTIOPOSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTIOPOSEL1_A {
        match self.bits {
            0 => CAPTIOPOSEL1_A::PJ,
            1 => CAPTIOPOSEL1_A::P1,
            2 => CAPTIOPOSEL1_A::P2,
            3 => CAPTIOPOSEL1_A::P3,
            4 => CAPTIOPOSEL1_A::P4,
            5 => CAPTIOPOSEL1_A::P5,
            6 => CAPTIOPOSEL1_A::P6,
            7 => CAPTIOPOSEL1_A::P7,
            8 => CAPTIOPOSEL1_A::P8,
            9 => CAPTIOPOSEL1_A::P9,
            10 => CAPTIOPOSEL1_A::P10,
            11 => CAPTIOPOSEL1_A::P11,
            12 => CAPTIOPOSEL1_A::P12,
            13 => CAPTIOPOSEL1_A::P13,
            14 => CAPTIOPOSEL1_A::P14,
            15 => CAPTIOPOSEL1_A::P15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PJ`"]
    #[inline(always)]
    pub fn is_pj(&self) -> bool {
        *self == CAPTIOPOSEL1_A::PJ
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P1
    }
    #[doc = "Checks if the value of the field is `P2`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P2
    }
    #[doc = "Checks if the value of the field is `P3`"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P3
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P4
    }
    #[doc = "Checks if the value of the field is `P5`"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P5
    }
    #[doc = "Checks if the value of the field is `P6`"]
    #[inline(always)]
    pub fn is_p6(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P6
    }
    #[doc = "Checks if the value of the field is `P7`"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P7
    }
    #[doc = "Checks if the value of the field is `P8`"]
    #[inline(always)]
    pub fn is_p8(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P8
    }
    #[doc = "Checks if the value of the field is `P9`"]
    #[inline(always)]
    pub fn is_p9(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P9
    }
    #[doc = "Checks if the value of the field is `P10`"]
    #[inline(always)]
    pub fn is_p10(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P10
    }
    #[doc = "Checks if the value of the field is `P11`"]
    #[inline(always)]
    pub fn is_p11(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P11
    }
    #[doc = "Checks if the value of the field is `P12`"]
    #[inline(always)]
    pub fn is_p12(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P12
    }
    #[doc = "Checks if the value of the field is `P13`"]
    #[inline(always)]
    pub fn is_p13(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P13
    }
    #[doc = "Checks if the value of the field is `P14`"]
    #[inline(always)]
    pub fn is_p14(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P14
    }
    #[doc = "Checks if the value of the field is `P15`"]
    #[inline(always)]
    pub fn is_p15(&self) -> bool {
        *self == CAPTIOPOSEL1_A::P15
    }
}
#[doc = "Write proxy for field `CAPTIOPOSEL1`"]
pub struct CAPTIOPOSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPOSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTIOPOSEL1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Px = PJ"]
    #[inline(always)]
    pub fn pj(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::PJ)
    }
    #[doc = "Px = P1"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P1)
    }
    #[doc = "Px = P2"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P2)
    }
    #[doc = "Px = P3"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P3)
    }
    #[doc = "Px = P4"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P4)
    }
    #[doc = "Px = P5"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P5)
    }
    #[doc = "Px = P6"]
    #[inline(always)]
    pub fn p6(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P6)
    }
    #[doc = "Px = P7"]
    #[inline(always)]
    pub fn p7(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P7)
    }
    #[doc = "Px = P8"]
    #[inline(always)]
    pub fn p8(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P8)
    }
    #[doc = "Px = P9"]
    #[inline(always)]
    pub fn p9(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P9)
    }
    #[doc = "Px = P10"]
    #[inline(always)]
    pub fn p10(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P10)
    }
    #[doc = "Px = P11"]
    #[inline(always)]
    pub fn p11(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P11)
    }
    #[doc = "Px = P12"]
    #[inline(always)]
    pub fn p12(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P12)
    }
    #[doc = "Px = P13"]
    #[inline(always)]
    pub fn p13(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P13)
    }
    #[doc = "Px = P14"]
    #[inline(always)]
    pub fn p14(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P14)
    }
    #[doc = "Px = P15"]
    #[inline(always)]
    pub fn p15(self) -> &'a mut W {
        self.variant(CAPTIOPOSEL1_A::P15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "8:8\\]
Capacitive Touch IO enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTIOEN_A {
    #[doc = "0: All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    OFF = 0,
    #[doc = "1: Selected Capacitive Touch IO is enabled"]
    ON = 1,
}
impl From<CAPTIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTIOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTIOEN`"]
pub type CAPTIOEN_R = crate::R<bool, CAPTIOEN_A>;
impl CAPTIOEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTIOEN_A {
        match self.bits {
            false => CAPTIOEN_A::OFF,
            true => CAPTIOEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CAPTIOEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CAPTIOEN_A::ON
    }
}
#[doc = "Write proxy for field `CAPTIOEN`"]
pub struct CAPTIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTIOEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CAPTIOEN_A::OFF)
    }
    #[doc = "Selected Capacitive Touch IO is enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CAPTIOEN_A::ON)
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
Capacitive Touch IO state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTIO_A {
    #[doc = "0: Curent state 0 or Capacitive Touch IO is disabled"]
    CAPTIO_0 = 0,
    #[doc = "1: Current state 1"]
    CAPTIO_1 = 1,
}
impl From<CAPTIO_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTIO`"]
pub type CAPTIO_R = crate::R<bool, CAPTIO_A>;
impl CAPTIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTIO_A {
        match self.bits {
            false => CAPTIO_A::CAPTIO_0,
            true => CAPTIO_A::CAPTIO_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAPTIO_0`"]
    #[inline(always)]
    pub fn is_captio_0(&self) -> bool {
        *self == CAPTIO_A::CAPTIO_0
    }
    #[doc = "Checks if the value of the field is `CAPTIO_1`"]
    #[inline(always)]
    pub fn is_captio_1(&self) -> bool {
        *self == CAPTIO_A::CAPTIO_1
    }
}
#[doc = "Write proxy for field `CAPTIO`"]
pub struct CAPTIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Curent state 0 or Capacitive Touch IO is disabled"]
    #[inline(always)]
    pub fn captio_0(self) -> &'a mut W {
        self.variant(CAPTIO_A::CAPTIO_0)
    }
    #[doc = "Current state 1"]
    #[inline(always)]
    pub fn captio_1(self) -> &'a mut W {
        self.variant(CAPTIO_A::CAPTIO_1)
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
impl R {
    #[doc = "Bits 1:3 - 3:1\\]
Capacitive Touch IO pin select"]
    #[inline(always)]
    pub fn captiopisel1(&self) -> CAPTIOPISEL1_R {
        CAPTIOPISEL1_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Capacitive Touch IO port select"]
    #[inline(always)]
    pub fn captioposel1(&self) -> CAPTIOPOSEL1_R {
        CAPTIOPOSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Capacitive Touch IO enable"]
    #[inline(always)]
    pub fn captioen(&self) -> CAPTIOEN_R {
        CAPTIOEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Capacitive Touch IO state"]
    #[inline(always)]
    pub fn captio(&self) -> CAPTIO_R {
        CAPTIO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - 3:1\\]
Capacitive Touch IO pin select"]
    #[inline(always)]
    pub fn captiopisel1(&mut self) -> CAPTIOPISEL1_W {
        CAPTIOPISEL1_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Capacitive Touch IO port select"]
    #[inline(always)]
    pub fn captioposel1(&mut self) -> CAPTIOPOSEL1_W {
        CAPTIOPOSEL1_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Capacitive Touch IO enable"]
    #[inline(always)]
    pub fn captioen(&mut self) -> CAPTIOEN_W {
        CAPTIOEN_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Capacitive Touch IO state"]
    #[inline(always)]
    pub fn captio(&mut self) -> CAPTIO_W {
        CAPTIO_W { w: self }
    }
}
