#[doc = "Reader of register TA1CTL"]
pub type R = crate::R<u16, super::TA1CTL>;
#[doc = "Writer for register TA1CTL"]
pub type W = crate::W<u16, super::TA1CTL>;
#[doc = "Register TA1CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TA1CTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
TimerA interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIFG_A {
    #[doc = "0: No interrupt pending"]
    TAIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    TAIFG_1 = 1,
}
impl From<TAIFG_A> for bool {
    #[inline(always)]
    fn from(variant: TAIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAIFG`"]
pub type TAIFG_R = crate::R<bool, TAIFG_A>;
impl TAIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIFG_A {
        match self.bits {
            false => TAIFG_A::TAIFG_0,
            true => TAIFG_A::TAIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIFG_0`"]
    #[inline(always)]
    pub fn is_taifg_0(&self) -> bool {
        *self == TAIFG_A::TAIFG_0
    }
    #[doc = "Checks if the value of the field is `TAIFG_1`"]
    #[inline(always)]
    pub fn is_taifg_1(&self) -> bool {
        *self == TAIFG_A::TAIFG_1
    }
}
#[doc = "Write proxy for field `TAIFG`"]
pub struct TAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn taifg_0(self) -> &'a mut W {
        self.variant(TAIFG_A::TAIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn taifg_1(self) -> &'a mut W {
        self.variant(TAIFG_A::TAIFG_1)
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
TimerA interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIE_A {
    #[doc = "0: Interrupt disabled"]
    TAIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    TAIE_1 = 1,
}
impl From<TAIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAIE`"]
pub type TAIE_R = crate::R<bool, TAIE_A>;
impl TAIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIE_A {
        match self.bits {
            false => TAIE_A::TAIE_0,
            true => TAIE_A::TAIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIE_0`"]
    #[inline(always)]
    pub fn is_taie_0(&self) -> bool {
        *self == TAIE_A::TAIE_0
    }
    #[doc = "Checks if the value of the field is `TAIE_1`"]
    #[inline(always)]
    pub fn is_taie_1(&self) -> bool {
        *self == TAIE_A::TAIE_1
    }
}
#[doc = "Write proxy for field `TAIE`"]
pub struct TAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn taie_0(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn taie_1(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_1)
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
#[doc = "Reader of field `TACLR`"]
pub type TACLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TACLR`"]
pub struct TACLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TACLR_W<'a> {
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
#[doc = "5:4\\]
Mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: Stop mode: Timer is halted"]
    STOP = 0,
    #[doc = "1: Up mode: Timer counts up to TAxCCR0"]
    UP = 1,
    #[doc = "2: Continuous mode: Timer counts up to 0FFFFh"]
    CONTINUOUS = 2,
    #[doc = "3: Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    UPDOWN = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MC`"]
pub type MC_R = crate::R<u8, MC_A>;
impl MC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::STOP,
            1 => MC_A::UP,
            2 => MC_A::CONTINUOUS,
            3 => MC_A::UPDOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MC_A::STOP
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == MC_A::UP
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == MC_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == MC_A::UPDOWN
    }
}
#[doc = "Write proxy for field `MC`"]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MC_A::STOP)
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(MC_A::UP)
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(MC_A::CONTINUOUS)
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut W {
        self.variant(MC_A::UPDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "7:6\\]
Input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /2"]
    _2 = 1,
    #[doc = "2: /4"]
    _4 = 2,
    #[doc = "3: /8"]
    _8 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, ID_A>;
impl ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::_1,
            1 => ID_A::_2,
            2 => ID_A::_4,
            3 => ID_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ID_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == ID_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == ID_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == ID_A::_8
    }
}
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ID_A::_1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(ID_A::_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(ID_A::_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(ID_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "9:8\\]
TimerA clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TASSEL_A {
    #[doc = "0: TAxCLK"]
    TACLK = 0,
    #[doc = "1: ACLK"]
    ACLK = 1,
    #[doc = "2: SMCLK"]
    SMCLK = 2,
    #[doc = "3: INCLK"]
    INCLK = 3,
}
impl From<TASSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TASSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TASSEL`"]
pub type TASSEL_R = crate::R<u8, TASSEL_A>;
impl TASSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TASSEL_A {
        match self.bits {
            0 => TASSEL_A::TACLK,
            1 => TASSEL_A::ACLK,
            2 => TASSEL_A::SMCLK,
            3 => TASSEL_A::INCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TACLK`"]
    #[inline(always)]
    pub fn is_taclk(&self) -> bool {
        *self == TASSEL_A::TACLK
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == TASSEL_A::ACLK
    }
    #[doc = "Checks if the value of the field is `SMCLK`"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == TASSEL_A::SMCLK
    }
    #[doc = "Checks if the value of the field is `INCLK`"]
    #[inline(always)]
    pub fn is_inclk(&self) -> bool {
        *self == TASSEL_A::INCLK
    }
}
#[doc = "Write proxy for field `TASSEL`"]
pub struct TASSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TASSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn taclk(self) -> &'a mut W {
        self.variant(TASSEL_A::TACLK)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(TASSEL_A::ACLK)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut W {
        self.variant(TASSEL_A::SMCLK)
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn inclk(self) -> &'a mut W {
        self.variant(TASSEL_A::INCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TAIFG_R {
        TAIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TimerA clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TACLR_R {
        TACLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Mode control"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Input divider"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&self) -> TASSEL_R {
        TASSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&mut self) -> TAIFG_W {
        TAIFG_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TAIE_W {
        TAIE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
TimerA clear"]
    #[inline(always)]
    pub fn taclr(&mut self) -> TACLR_W {
        TACLR_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Mode control"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Input divider"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&mut self) -> TASSEL_W {
        TASSEL_W { w: self }
    }
}
