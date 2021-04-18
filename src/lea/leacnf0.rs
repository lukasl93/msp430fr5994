#[doc = "Reader of register LEACNF0"]
pub type R = crate::R<u32, super::LEACNF0>;
#[doc = "Writer for register LEACNF0"]
pub type W = crate::W<u32, super::LEACNF0>;
#[doc = "Register LEACNF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LEACNF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEASWRST`"]
pub type LEASWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEASWRST`"]
pub struct LEASWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEASWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "1:1\\]
Hold on faults and NMIs for all pending LEA operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAFTHOLD_A {
    #[doc = "0: LEA transfers continue on faults/NMIs"]
    LEAFTHOLD_0 = 0,
    #[doc = "1: LEA transfers enter HOLD on faults/NMIs"]
    LEAFTHOLD_1 = 1,
}
impl From<LEAFTHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: LEAFTHOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAFTHOLD`"]
pub type LEAFTHOLD_R = crate::R<bool, LEAFTHOLD_A>;
impl LEAFTHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAFTHOLD_A {
        match self.bits {
            false => LEAFTHOLD_A::LEAFTHOLD_0,
            true => LEAFTHOLD_A::LEAFTHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAFTHOLD_0`"]
    #[inline(always)]
    pub fn is_leafthold_0(&self) -> bool {
        *self == LEAFTHOLD_A::LEAFTHOLD_0
    }
    #[doc = "Checks if the value of the field is `LEAFTHOLD_1`"]
    #[inline(always)]
    pub fn is_leafthold_1(&self) -> bool {
        *self == LEAFTHOLD_A::LEAFTHOLD_1
    }
}
#[doc = "Write proxy for field `LEAFTHOLD`"]
pub struct LEAFTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAFTHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAFTHOLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LEA transfers continue on faults/NMIs"]
    #[inline(always)]
    pub fn leafthold_0(self) -> &'a mut W {
        self.variant(LEAFTHOLD_A::LEAFTHOLD_0)
    }
    #[doc = "LEA transfers enter HOLD on faults/NMIs"]
    #[inline(always)]
    pub fn leafthold_1(self) -> &'a mut W {
        self.variant(LEAFTHOLD_A::LEAFTHOLD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "8:8\\]
This bit defined if command execution shall be continued in LPM modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEALPR_A {
    #[doc = "0: LEA command execution stops in deep low power modes"]
    LEALPR_0 = 0,
    #[doc = "1: LEA command execution continues in deep low power modes"]
    LEALPR_1 = 1,
}
impl From<LEALPR_A> for bool {
    #[inline(always)]
    fn from(variant: LEALPR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEALPR`"]
pub type LEALPR_R = crate::R<bool, LEALPR_A>;
impl LEALPR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEALPR_A {
        match self.bits {
            false => LEALPR_A::LEALPR_0,
            true => LEALPR_A::LEALPR_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEALPR_0`"]
    #[inline(always)]
    pub fn is_lealpr_0(&self) -> bool {
        *self == LEALPR_A::LEALPR_0
    }
    #[doc = "Checks if the value of the field is `LEALPR_1`"]
    #[inline(always)]
    pub fn is_lealpr_1(&self) -> bool {
        *self == LEALPR_A::LEALPR_1
    }
}
#[doc = "Write proxy for field `LEALPR`"]
pub struct LEALPR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEALPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEALPR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LEA command execution stops in deep low power modes"]
    #[inline(always)]
    pub fn lealpr_0(self) -> &'a mut W {
        self.variant(LEALPR_A::LEALPR_0)
    }
    #[doc = "LEA command execution continues in deep low power modes"]
    #[inline(always)]
    pub fn lealpr_1(self) -> &'a mut W {
        self.variant(LEALPR_A::LEALPR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "10:10\\]
This bit defines if a \"Command done interrupt\" shall be triggered in LPM mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAILPM_A {
    #[doc = "0: Interrupt of LEA is suppressed in LPM mode until AM is entered then the LEA interrupt is triggered as well"]
    LEAILPM_0 = 0,
    #[doc = "1: Interrupt of LEA is always triggered on completion of an LEA command"]
    LEAILPM_1 = 1,
}
impl From<LEAILPM_A> for bool {
    #[inline(always)]
    fn from(variant: LEAILPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAILPM`"]
pub type LEAILPM_R = crate::R<bool, LEAILPM_A>;
impl LEAILPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAILPM_A {
        match self.bits {
            false => LEAILPM_A::LEAILPM_0,
            true => LEAILPM_A::LEAILPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAILPM_0`"]
    #[inline(always)]
    pub fn is_leailpm_0(&self) -> bool {
        *self == LEAILPM_A::LEAILPM_0
    }
    #[doc = "Checks if the value of the field is `LEAILPM_1`"]
    #[inline(always)]
    pub fn is_leailpm_1(&self) -> bool {
        *self == LEAILPM_A::LEAILPM_1
    }
}
#[doc = "Write proxy for field `LEAILPM`"]
pub struct LEAILPM_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAILPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAILPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt of LEA is suppressed in LPM mode until AM is entered then the LEA interrupt is triggered as well"]
    #[inline(always)]
    pub fn leailpm_0(self) -> &'a mut W {
        self.variant(LEAILPM_A::LEAILPM_0)
    }
    #[doc = "Interrupt of LEA is always triggered on completion of an LEA command"]
    #[inline(always)]
    pub fn leailpm_1(self) -> &'a mut W {
        self.variant(LEAILPM_A::LEAILPM_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `LEAILB`"]
pub type LEAILB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEAILB`"]
pub struct LEAILB_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAILB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "13:13\\]
LEA module timer fault enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEATIMFLTE_A {
    #[doc = "0: LEA module timer timeout will not cause a fault indication"]
    LEATIMFLT_0 = 0,
    #[doc = "1: LEA module timer timeout will cause a fault indication. LEA stops operation and enters \"Ready-state\"."]
    LEATIMFLTE_1 = 1,
}
impl From<LEATIMFLTE_A> for bool {
    #[inline(always)]
    fn from(variant: LEATIMFLTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEATIMFLTE`"]
pub type LEATIMFLTE_R = crate::R<bool, LEATIMFLTE_A>;
impl LEATIMFLTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEATIMFLTE_A {
        match self.bits {
            false => LEATIMFLTE_A::LEATIMFLT_0,
            true => LEATIMFLTE_A::LEATIMFLTE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEATIMFLT_0`"]
    #[inline(always)]
    pub fn is_leatimflt_0(&self) -> bool {
        *self == LEATIMFLTE_A::LEATIMFLT_0
    }
    #[doc = "Checks if the value of the field is `LEATIMFLTE_1`"]
    #[inline(always)]
    pub fn is_leatimflte_1(&self) -> bool {
        *self == LEATIMFLTE_A::LEATIMFLTE_1
    }
}
#[doc = "Write proxy for field `LEATIMFLTE`"]
pub struct LEATIMFLTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATIMFLTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEATIMFLTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LEA module timer timeout will not cause a fault indication"]
    #[inline(always)]
    pub fn leatimflt_0(self) -> &'a mut W {
        self.variant(LEATIMFLTE_A::LEATIMFLT_0)
    }
    #[doc = "LEA module timer timeout will cause a fault indication. LEA stops operation and enters \"Ready-state\"."]
    #[inline(always)]
    pub fn leatimflte_1(self) -> &'a mut W {
        self.variant(LEATIMFLTE_A::LEATIMFLTE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "14:14\\]
LEAHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEACFLT_A {
    #[doc = "0: LEAHPCFLT is disabled"]
    LEACFLT_0 = 0,
    #[doc = "1: LEAHPCFLT is enabled"]
    LEACFLT_1 = 1,
}
impl From<LEACFLT_A> for bool {
    #[inline(always)]
    fn from(variant: LEACFLT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEACFLT`"]
pub type LEACFLT_R = crate::R<bool, LEACFLT_A>;
impl LEACFLT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEACFLT_A {
        match self.bits {
            false => LEACFLT_A::LEACFLT_0,
            true => LEACFLT_A::LEACFLT_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEACFLT_0`"]
    #[inline(always)]
    pub fn is_leacflt_0(&self) -> bool {
        *self == LEACFLT_A::LEACFLT_0
    }
    #[doc = "Checks if the value of the field is `LEACFLT_1`"]
    #[inline(always)]
    pub fn is_leacflt_1(&self) -> bool {
        *self == LEACFLT_A::LEACFLT_1
    }
}
#[doc = "Write proxy for field `LEACFLT`"]
pub struct LEACFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACFLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACFLT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LEAHPCFLT is disabled"]
    #[inline(always)]
    pub fn leacflt_0(self) -> &'a mut W {
        self.variant(LEACFLT_A::LEACFLT_0)
    }
    #[doc = "LEAHPCFLT is enabled"]
    #[inline(always)]
    pub fn leacflt_1(self) -> &'a mut W {
        self.variant(LEACFLT_A::LEACFLT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "15:15\\]
Enable bit on memory faults.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAMEMFLTE_A {
    #[doc = "0: LEA memory faults are disabled"]
    LEAMEMFLTE_0 = 0,
    #[doc = "1: LEA memory faults are enabled"]
    LEAMEMFLTE_1 = 1,
}
impl From<LEAMEMFLTE_A> for bool {
    #[inline(always)]
    fn from(variant: LEAMEMFLTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAMEMFLTE`"]
pub type LEAMEMFLTE_R = crate::R<bool, LEAMEMFLTE_A>;
impl LEAMEMFLTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAMEMFLTE_A {
        match self.bits {
            false => LEAMEMFLTE_A::LEAMEMFLTE_0,
            true => LEAMEMFLTE_A::LEAMEMFLTE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAMEMFLTE_0`"]
    #[inline(always)]
    pub fn is_leamemflte_0(&self) -> bool {
        *self == LEAMEMFLTE_A::LEAMEMFLTE_0
    }
    #[doc = "Checks if the value of the field is `LEAMEMFLTE_1`"]
    #[inline(always)]
    pub fn is_leamemflte_1(&self) -> bool {
        *self == LEAMEMFLTE_A::LEAMEMFLTE_1
    }
}
#[doc = "Write proxy for field `LEAMEMFLTE`"]
pub struct LEAMEMFLTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAMEMFLTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAMEMFLTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LEA memory faults are disabled"]
    #[inline(always)]
    pub fn leamemflte_0(self) -> &'a mut W {
        self.variant(LEAMEMFLTE_A::LEAMEMFLTE_0)
    }
    #[doc = "LEA memory faults are enabled"]
    #[inline(always)]
    pub fn leamemflte_1(self) -> &'a mut W {
        self.variant(LEAMEMFLTE_A::LEAMEMFLTE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `LEADONES`"]
pub type LEADONES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEADONES`"]
pub struct LEADONES_W<'a> {
    w: &'a mut W,
}
impl<'a> LEADONES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `LEAFREES`"]
pub type LEAFREES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEAFREES`"]
pub struct LEAFREES_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAFREES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `LEATIMFLTS`"]
pub type LEATIMFLTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEATIMFLTS`"]
pub struct LEATIMFLTS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATIMFLTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "22:22\\]
LEAHPCFLTS when hardware trigger enabled later LEA command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a \"User-NMI\" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEACFLTS_A {
    #[doc = "0: No command fault occurred since this bit was cleared"]
    LEACFLTS_0 = 0,
    #[doc = "1: At least one command fault occurred since this bit was cleared"]
    LEACFLTS_1 = 1,
}
impl From<LEACFLTS_A> for bool {
    #[inline(always)]
    fn from(variant: LEACFLTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEACFLTS`"]
pub type LEACFLTS_R = crate::R<bool, LEACFLTS_A>;
impl LEACFLTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEACFLTS_A {
        match self.bits {
            false => LEACFLTS_A::LEACFLTS_0,
            true => LEACFLTS_A::LEACFLTS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEACFLTS_0`"]
    #[inline(always)]
    pub fn is_leacflts_0(&self) -> bool {
        *self == LEACFLTS_A::LEACFLTS_0
    }
    #[doc = "Checks if the value of the field is `LEACFLTS_1`"]
    #[inline(always)]
    pub fn is_leacflts_1(&self) -> bool {
        *self == LEACFLTS_A::LEACFLTS_1
    }
}
#[doc = "Write proxy for field `LEACFLTS`"]
pub struct LEACFLTS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACFLTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACFLTS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leacflts_0(self) -> &'a mut W {
        self.variant(LEACFLTS_A::LEACFLTS_0)
    }
    #[doc = "At least one command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leacflts_1(self) -> &'a mut W {
        self.variant(LEACFLTS_A::LEACFLTS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "23:23\\]
LEA memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEACNF1. LEAWRSTAT and LEACNF1.LEARDSTAT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAMEMFLTS_A {
    #[doc = "0: No memory fault occurred since this bit was cleared"]
    LEAMEMFLTS_0 = 0,
    #[doc = "1: At least one memory fault since this bit was cleared"]
    LEAMEMFLTS_1 = 1,
}
impl From<LEAMEMFLTS_A> for bool {
    #[inline(always)]
    fn from(variant: LEAMEMFLTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAMEMFLTS`"]
pub type LEAMEMFLTS_R = crate::R<bool, LEAMEMFLTS_A>;
impl LEAMEMFLTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAMEMFLTS_A {
        match self.bits {
            false => LEAMEMFLTS_A::LEAMEMFLTS_0,
            true => LEAMEMFLTS_A::LEAMEMFLTS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAMEMFLTS_0`"]
    #[inline(always)]
    pub fn is_leamemflts_0(&self) -> bool {
        *self == LEAMEMFLTS_A::LEAMEMFLTS_0
    }
    #[doc = "Checks if the value of the field is `LEAMEMFLTS_1`"]
    #[inline(always)]
    pub fn is_leamemflts_1(&self) -> bool {
        *self == LEAMEMFLTS_A::LEAMEMFLTS_1
    }
}
#[doc = "Write proxy for field `LEAMEMFLTS`"]
pub struct LEAMEMFLTS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAMEMFLTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAMEMFLTS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No memory fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leamemflts_0(self) -> &'a mut W {
        self.variant(LEAMEMFLTS_A::LEAMEMFLTS_0)
    }
    #[doc = "At least one memory fault since this bit was cleared"]
    #[inline(always)]
    pub fn leamemflts_1(self) -> &'a mut W {
        self.variant(LEAMEMFLTS_A::LEAMEMFLTS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `LEATRST`"]
pub type LEATRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEATRST`"]
pub struct LEATRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `LEATEN`"]
pub type LEATEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEATEN`"]
pub struct LEATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "31:28\\]
LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEATISEL_A {
    #[doc = "0: Timeout period: 128 x t#sub#CLK#/sub# (16 us at 8 MHz); Interval period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz)"]
    LEATISEL_0 = 0,
    #[doc = "1: Timeout period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz); Interval period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz)"]
    LEATISEL_1 = 1,
    #[doc = "2: Timeout period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz); Interval period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz)"]
    LEATISEL_2 = 2,
    #[doc = "3: Timeout period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz); Interval period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz)"]
    LEATISEL_3 = 3,
    #[doc = "4: Timeout period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz); Interval period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz)"]
    LEATISEL_4 = 4,
    #[doc = "5: Timeout period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz); Interval period: 8192 x t#sub#CLK#/sub# (1ms at 8 MHz)"]
    LEATISEL_5 = 5,
    #[doc = "6: Timeout period: 8192 x t#sub#CLK#/sub# (1 ms at 8 MHz); Interval period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz)"]
    LEATISEL_6 = 6,
    #[doc = "7: Timeout period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz); Interval period: 32768 x t#sub#CLK#/sub# (4 ms at 8 MHz)"]
    LEATISEL_7 = 7,
    #[doc = "8: Timeout period: 32768 x t#sub#CLK#/sub# (4ms at 8 MHz); Interval period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz)"]
    LEATISEL_8 = 8,
    #[doc = "9: Timeout period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz); Interval period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz)"]
    LEATISEL_9 = 9,
    #[doc = "10: Timeout period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz); Interval period: 262144 x t#sub#CLK#/sub# (32 ms at 8 MHz)"]
    LEATISEL_10 = 10,
    #[doc = "11: Timeout period: 524288 x t#sub#CLK#/sub# (65 ms at 8 MHz); Interval period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz)"]
    LEATISEL_11 = 11,
    #[doc = "12: Timeout period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz); Interval period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz)"]
    LEATISEL_12 = 12,
    #[doc = "13: Timeout period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz); Interval period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz)"]
    LEATISEL_13 = 13,
    #[doc = "14: Timeout period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz); Interval period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz)"]
    LEATISEL_14 = 14,
    #[doc = "15: Timeout period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz); Interval period: 16777216 x t#sub#CLK#/sub# (2.1 s at 8 MHz)"]
    LEATISEL_15 = 15,
}
impl From<LEATISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LEATISEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEATISEL`"]
pub type LEATISEL_R = crate::R<u8, LEATISEL_A>;
impl LEATISEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEATISEL_A {
        match self.bits {
            0 => LEATISEL_A::LEATISEL_0,
            1 => LEATISEL_A::LEATISEL_1,
            2 => LEATISEL_A::LEATISEL_2,
            3 => LEATISEL_A::LEATISEL_3,
            4 => LEATISEL_A::LEATISEL_4,
            5 => LEATISEL_A::LEATISEL_5,
            6 => LEATISEL_A::LEATISEL_6,
            7 => LEATISEL_A::LEATISEL_7,
            8 => LEATISEL_A::LEATISEL_8,
            9 => LEATISEL_A::LEATISEL_9,
            10 => LEATISEL_A::LEATISEL_10,
            11 => LEATISEL_A::LEATISEL_11,
            12 => LEATISEL_A::LEATISEL_12,
            13 => LEATISEL_A::LEATISEL_13,
            14 => LEATISEL_A::LEATISEL_14,
            15 => LEATISEL_A::LEATISEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEATISEL_0`"]
    #[inline(always)]
    pub fn is_leatisel_0(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_0
    }
    #[doc = "Checks if the value of the field is `LEATISEL_1`"]
    #[inline(always)]
    pub fn is_leatisel_1(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_1
    }
    #[doc = "Checks if the value of the field is `LEATISEL_2`"]
    #[inline(always)]
    pub fn is_leatisel_2(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_2
    }
    #[doc = "Checks if the value of the field is `LEATISEL_3`"]
    #[inline(always)]
    pub fn is_leatisel_3(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_3
    }
    #[doc = "Checks if the value of the field is `LEATISEL_4`"]
    #[inline(always)]
    pub fn is_leatisel_4(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_4
    }
    #[doc = "Checks if the value of the field is `LEATISEL_5`"]
    #[inline(always)]
    pub fn is_leatisel_5(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_5
    }
    #[doc = "Checks if the value of the field is `LEATISEL_6`"]
    #[inline(always)]
    pub fn is_leatisel_6(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_6
    }
    #[doc = "Checks if the value of the field is `LEATISEL_7`"]
    #[inline(always)]
    pub fn is_leatisel_7(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_7
    }
    #[doc = "Checks if the value of the field is `LEATISEL_8`"]
    #[inline(always)]
    pub fn is_leatisel_8(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_8
    }
    #[doc = "Checks if the value of the field is `LEATISEL_9`"]
    #[inline(always)]
    pub fn is_leatisel_9(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_9
    }
    #[doc = "Checks if the value of the field is `LEATISEL_10`"]
    #[inline(always)]
    pub fn is_leatisel_10(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_10
    }
    #[doc = "Checks if the value of the field is `LEATISEL_11`"]
    #[inline(always)]
    pub fn is_leatisel_11(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_11
    }
    #[doc = "Checks if the value of the field is `LEATISEL_12`"]
    #[inline(always)]
    pub fn is_leatisel_12(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_12
    }
    #[doc = "Checks if the value of the field is `LEATISEL_13`"]
    #[inline(always)]
    pub fn is_leatisel_13(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_13
    }
    #[doc = "Checks if the value of the field is `LEATISEL_14`"]
    #[inline(always)]
    pub fn is_leatisel_14(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_14
    }
    #[doc = "Checks if the value of the field is `LEATISEL_15`"]
    #[inline(always)]
    pub fn is_leatisel_15(&self) -> bool {
        *self == LEATISEL_A::LEATISEL_15
    }
}
#[doc = "Write proxy for field `LEATISEL`"]
pub struct LEATISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEATISEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timeout period: 128 x t#sub#CLK#/sub# (16 us at 8 MHz); Interval period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_0(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_0)
    }
    #[doc = "Timeout period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz); Interval period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_1(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_1)
    }
    #[doc = "Timeout period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz); Interval period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_2(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_2)
    }
    #[doc = "Timeout period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz); Interval period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_3(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_3)
    }
    #[doc = "Timeout period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz); Interval period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_4(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_4)
    }
    #[doc = "Timeout period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz); Interval period: 8192 x t#sub#CLK#/sub# (1ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_5(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_5)
    }
    #[doc = "Timeout period: 8192 x t#sub#CLK#/sub# (1 ms at 8 MHz); Interval period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_6(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_6)
    }
    #[doc = "Timeout period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz); Interval period: 32768 x t#sub#CLK#/sub# (4 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_7(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_7)
    }
    #[doc = "Timeout period: 32768 x t#sub#CLK#/sub# (4ms at 8 MHz); Interval period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_8(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_8)
    }
    #[doc = "Timeout period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz); Interval period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_9(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_9)
    }
    #[doc = "Timeout period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz); Interval period: 262144 x t#sub#CLK#/sub# (32 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_10(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_10)
    }
    #[doc = "Timeout period: 524288 x t#sub#CLK#/sub# (65 ms at 8 MHz); Interval period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_11(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_11)
    }
    #[doc = "Timeout period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz); Interval period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_12(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_12)
    }
    #[doc = "Timeout period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz); Interval period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_13(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_13)
    }
    #[doc = "Timeout period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz); Interval period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_14(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_14)
    }
    #[doc = "Timeout period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz); Interval period: 16777216 x t#sub#CLK#/sub# (2.1 s at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_15(self) -> &'a mut W {
        self.variant(LEATISEL_A::LEATISEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
LEA module software restart. Setting this bit to one restarts the LEA module. As long this bit remains set to one the LEA is held in Restart. (The LEA accessible memory behaves as system RAM)"]
    #[inline(always)]
    pub fn leaswrst(&self) -> LEASWRST_R {
        LEASWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Hold on faults and NMIs for all pending LEA operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)"]
    #[inline(always)]
    pub fn leafthold(&self) -> LEAFTHOLD_R {
        LEAFTHOLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This bit defined if command execution shall be continued in LPM modes"]
    #[inline(always)]
    pub fn lealpr(&self) -> LEALPR_R {
        LEALPR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit defines if a \"Command done interrupt\" shall be triggered in LPM mode"]
    #[inline(always)]
    pub fn leailpm(&self) -> LEAILPM_R {
        LEAILPM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
LEA instruction loop buffer disable. Debugging function for LEA (leave it zero)."]
    #[inline(always)]
    pub fn leailb(&self) -> LEAILB_R {
        LEAILB_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
LEA module timer fault enable."]
    #[inline(always)]
    pub fn leatimflte(&self) -> LEATIMFLTE_R {
        LEATIMFLTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
LEAHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults."]
    #[inline(always)]
    pub fn leacflt(&self) -> LEACFLT_R {
        LEACFLT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable bit on memory faults."]
    #[inline(always)]
    pub fn leamemflte(&self) -> LEAMEMFLTE_R {
        LEAMEMFLTE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
LEA done event indication and set flag. This bit indicated the done event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leadones(&self) -> LEADONES_R {
        LEADONES_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
LEA free event indication and set flag. This bit indicated the free event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leafrees(&self) -> LEAFREES_R {
        LEAFREES_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
LEA timeout fault indication and set flag; This bits indicates that timer timeout occurred. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module (A package option)"]
    #[inline(always)]
    pub fn leatimflts(&self) -> LEATIMFLTS_R {
        LEATIMFLTS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
LEAHPCFLTS when hardware trigger enabled later LEA command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a \"User-NMI\" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module."]
    #[inline(always)]
    pub fn leacflts(&self) -> LEACFLTS_R {
        LEACFLTS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
LEA memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEACNF1. LEAWRSTAT and LEACNF1.LEARDSTAT."]
    #[inline(always)]
    pub fn leamemflts(&self) -> LEAMEMFLTS_R {
        LEAMEMFLTS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
LEA module timer reset. Setting this bit to one clears LEA module timer. This bit is self clearing and will always be read as zero."]
    #[inline(always)]
    pub fn leatrst(&self) -> LEATRST_R {
        LEATRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
LEA timer enable; writing a one to this bit enables LEA timer operations."]
    #[inline(always)]
    pub fn leaten(&self) -> LEATEN_R {
        LEATEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK"]
    #[inline(always)]
    pub fn leatisel(&self) -> LEATISEL_R {
        LEATISEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LEA module software restart. Setting this bit to one restarts the LEA module. As long this bit remains set to one the LEA is held in Restart. (The LEA accessible memory behaves as system RAM)"]
    #[inline(always)]
    pub fn leaswrst(&mut self) -> LEASWRST_W {
        LEASWRST_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Hold on faults and NMIs for all pending LEA operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)"]
    #[inline(always)]
    pub fn leafthold(&mut self) -> LEAFTHOLD_W {
        LEAFTHOLD_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
This bit defined if command execution shall be continued in LPM modes"]
    #[inline(always)]
    pub fn lealpr(&mut self) -> LEALPR_W {
        LEALPR_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
This bit defines if a \"Command done interrupt\" shall be triggered in LPM mode"]
    #[inline(always)]
    pub fn leailpm(&mut self) -> LEAILPM_W {
        LEAILPM_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
LEA instruction loop buffer disable. Debugging function for LEA (leave it zero)."]
    #[inline(always)]
    pub fn leailb(&mut self) -> LEAILB_W {
        LEAILB_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
LEA module timer fault enable."]
    #[inline(always)]
    pub fn leatimflte(&mut self) -> LEATIMFLTE_W {
        LEATIMFLTE_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
LEAHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults."]
    #[inline(always)]
    pub fn leacflt(&mut self) -> LEACFLT_W {
        LEACFLT_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Enable bit on memory faults."]
    #[inline(always)]
    pub fn leamemflte(&mut self) -> LEAMEMFLTE_W {
        LEAMEMFLTE_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
LEA done event indication and set flag. This bit indicated the done event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leadones(&mut self) -> LEADONES_W {
        LEADONES_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
LEA free event indication and set flag. This bit indicated the free event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leafrees(&mut self) -> LEAFREES_W {
        LEAFREES_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
LEA timeout fault indication and set flag; This bits indicates that timer timeout occurred. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module (A package option)"]
    #[inline(always)]
    pub fn leatimflts(&mut self) -> LEATIMFLTS_W {
        LEATIMFLTS_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
LEAHPCFLTS when hardware trigger enabled later LEA command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a \"User-NMI\" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module."]
    #[inline(always)]
    pub fn leacflts(&mut self) -> LEACFLTS_W {
        LEACFLTS_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
LEA memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEACNF1. LEAWRSTAT and LEACNF1.LEARDSTAT."]
    #[inline(always)]
    pub fn leamemflts(&mut self) -> LEAMEMFLTS_W {
        LEAMEMFLTS_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
LEA module timer reset. Setting this bit to one clears LEA module timer. This bit is self clearing and will always be read as zero."]
    #[inline(always)]
    pub fn leatrst(&mut self) -> LEATRST_W {
        LEATRST_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
LEA timer enable; writing a one to this bit enables LEA timer operations."]
    #[inline(always)]
    pub fn leaten(&mut self) -> LEATEN_W {
        LEATEN_W { w: self }
    }
    #[doc = "Bits 28:31 - 31:28\\]
LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK"]
    #[inline(always)]
    pub fn leatisel(&mut self) -> LEATISEL_W {
        LEATISEL_W { w: self }
    }
}
