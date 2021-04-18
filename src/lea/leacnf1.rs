#[doc = "Reader of register LEACNF1"]
pub type R = crate::R<u32, super::LEACNF1>;
#[doc = "Writer for register LEACNF1"]
pub type W = crate::W<u32, super::LEACNF1>;
#[doc = "Register LEACNF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LEACNF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
This bit indicate if LEA is able to accept new Commands (SUSPEND is always accepted)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEABUSY_A {
    #[doc = "0: LEA is in Ready can accept new commands"]
    READY = 0,
    #[doc = "1: LEA is busy right now and cannot accept any commands"]
    BUSY = 1,
}
impl From<LEABUSY_A> for bool {
    #[inline(always)]
    fn from(variant: LEABUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEABUSY`"]
pub type LEABUSY_R = crate::R<bool, LEABUSY_A>;
impl LEABUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEABUSY_A {
        match self.bits {
            false => LEABUSY_A::READY,
            true => LEABUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LEABUSY_A::READY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == LEABUSY_A::BUSY
    }
}
#[doc = "Write proxy for field `LEABUSY`"]
pub struct LEABUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> LEABUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEABUSY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LEA is in Ready can accept new commands"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(LEABUSY_A::READY)
    }
    #[doc = "LEA is busy right now and cannot accept any commands"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(LEABUSY_A::BUSY)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "7:4\\]
These Bits indicate the actual operation mode LEA is in. Other = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEAMODE_A {
    #[doc = "0: Off (implicit)"]
    OFF = 0,
    #[doc = "1: Ready"]
    READY = 1,
    #[doc = "2: RunS (SUSPEND)"]
    RUNS = 2,
    #[doc = "3: RunR (RESUME)"]
    RUNR = 3,
    #[doc = "4: RunA (regular command operation )"]
    RUNA = 4,
    #[doc = "5: Notify"]
    NOTIFY = 5,
    #[doc = "6: Sleep"]
    SLEEP = 6,
    #[doc = "7: RunL"]
    RUNL = 7,
}
impl From<LEAMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LEAMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEAMODE`"]
pub type LEAMODE_R = crate::R<u8, LEAMODE_A>;
impl LEAMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LEAMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LEAMODE_A::OFF),
            1 => Val(LEAMODE_A::READY),
            2 => Val(LEAMODE_A::RUNS),
            3 => Val(LEAMODE_A::RUNR),
            4 => Val(LEAMODE_A::RUNA),
            5 => Val(LEAMODE_A::NOTIFY),
            6 => Val(LEAMODE_A::SLEEP),
            7 => Val(LEAMODE_A::RUNL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LEAMODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LEAMODE_A::READY
    }
    #[doc = "Checks if the value of the field is `RUNS`"]
    #[inline(always)]
    pub fn is_runs(&self) -> bool {
        *self == LEAMODE_A::RUNS
    }
    #[doc = "Checks if the value of the field is `RUNR`"]
    #[inline(always)]
    pub fn is_runr(&self) -> bool {
        *self == LEAMODE_A::RUNR
    }
    #[doc = "Checks if the value of the field is `RUNA`"]
    #[inline(always)]
    pub fn is_runa(&self) -> bool {
        *self == LEAMODE_A::RUNA
    }
    #[doc = "Checks if the value of the field is `NOTIFY`"]
    #[inline(always)]
    pub fn is_notify(&self) -> bool {
        *self == LEAMODE_A::NOTIFY
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == LEAMODE_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `RUNL`"]
    #[inline(always)]
    pub fn is_runl(&self) -> bool {
        *self == LEAMODE_A::RUNL
    }
}
#[doc = "Write proxy for field `LEAMODE`"]
pub struct LEAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Off (implicit)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LEAMODE_A::OFF)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(LEAMODE_A::READY)
    }
    #[doc = "RunS (SUSPEND)"]
    #[inline(always)]
    pub fn runs(self) -> &'a mut W {
        self.variant(LEAMODE_A::RUNS)
    }
    #[doc = "RunR (RESUME)"]
    #[inline(always)]
    pub fn runr(self) -> &'a mut W {
        self.variant(LEAMODE_A::RUNR)
    }
    #[doc = "RunA (regular command operation )"]
    #[inline(always)]
    pub fn runa(self) -> &'a mut W {
        self.variant(LEAMODE_A::RUNA)
    }
    #[doc = "Notify"]
    #[inline(always)]
    pub fn notify(self) -> &'a mut W {
        self.variant(LEAMODE_A::NOTIFY)
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(LEAMODE_A::SLEEP)
    }
    #[doc = "RunL"]
    #[inline(always)]
    pub fn runl(self) -> &'a mut W {
        self.variant(LEAMODE_A::RUNL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `LEAPWST`"]
pub type LEAPWST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEAPWST`"]
pub struct LEAPWST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAPWST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `LEAASST`"]
pub type LEAASST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEAASST`"]
pub struct LEAASST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAASST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `LEADONEC`"]
pub type LEADONEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEADONEC`"]
pub struct LEADONEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEADONEC_W<'a> {
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
#[doc = "Reader of field `LEAFREEC`"]
pub type LEAFREEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEAFREEC`"]
pub struct LEAFREEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAFREEC_W<'a> {
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
#[doc = "Reader of field `LEATIMFLTC`"]
pub type LEATIMFLTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEATIMFLTC`"]
pub struct LEATIMFLTC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEATIMFLTC_W<'a> {
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
LEA command fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEACFLTC_A {
    #[doc = "0: No command fault occurred since this bit was cleared"]
    LEACFLTC_0 = 0,
    #[doc = "1: At least one command fault occurred since this bit was cleared"]
    LEACFLTC_1 = 1,
}
impl From<LEACFLTC_A> for bool {
    #[inline(always)]
    fn from(variant: LEACFLTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEACFLTC`"]
pub type LEACFLTC_R = crate::R<bool, LEACFLTC_A>;
impl LEACFLTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEACFLTC_A {
        match self.bits {
            false => LEACFLTC_A::LEACFLTC_0,
            true => LEACFLTC_A::LEACFLTC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEACFLTC_0`"]
    #[inline(always)]
    pub fn is_leacfltc_0(&self) -> bool {
        *self == LEACFLTC_A::LEACFLTC_0
    }
    #[doc = "Checks if the value of the field is `LEACFLTC_1`"]
    #[inline(always)]
    pub fn is_leacfltc_1(&self) -> bool {
        *self == LEACFLTC_A::LEACFLTC_1
    }
}
#[doc = "Write proxy for field `LEACFLTC`"]
pub struct LEACFLTC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACFLTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACFLTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leacfltc_0(self) -> &'a mut W {
        self.variant(LEACFLTC_A::LEACFLTC_0)
    }
    #[doc = "At least one command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leacfltc_1(self) -> &'a mut W {
        self.variant(LEACFLTC_A::LEACFLTC_1)
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
LEA memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEAWRSTAT and LEARDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEACNF0.LEAMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAMEMFLTC_A {
    #[doc = "0: No memory fault occurred since this bit was cleared"]
    LEAMEMFLTC_0 = 0,
    #[doc = "1: At least one memory fault since this bit was cleared"]
    LEAMEMFLTC_1 = 1,
}
impl From<LEAMEMFLTC_A> for bool {
    #[inline(always)]
    fn from(variant: LEAMEMFLTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEAMEMFLTC`"]
pub type LEAMEMFLTC_R = crate::R<bool, LEAMEMFLTC_A>;
impl LEAMEMFLTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAMEMFLTC_A {
        match self.bits {
            false => LEAMEMFLTC_A::LEAMEMFLTC_0,
            true => LEAMEMFLTC_A::LEAMEMFLTC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEAMEMFLTC_0`"]
    #[inline(always)]
    pub fn is_leamemfltc_0(&self) -> bool {
        *self == LEAMEMFLTC_A::LEAMEMFLTC_0
    }
    #[doc = "Checks if the value of the field is `LEAMEMFLTC_1`"]
    #[inline(always)]
    pub fn is_leamemfltc_1(&self) -> bool {
        *self == LEAMEMFLTC_A::LEAMEMFLTC_1
    }
}
#[doc = "Write proxy for field `LEAMEMFLTC`"]
pub struct LEAMEMFLTC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAMEMFLTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAMEMFLTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No memory fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leamemfltc_0(self) -> &'a mut W {
        self.variant(LEAMEMFLTC_A::LEAMEMFLTC_0)
    }
    #[doc = "At least one memory fault since this bit was cleared"]
    #[inline(always)]
    pub fn leamemfltc_1(self) -> &'a mut W {
        self.variant(LEAMEMFLTC_A::LEAMEMFLTC_1)
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
#[doc = "Reader of field `LEARDSTAT`"]
pub type LEARDSTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEARDSTAT`"]
pub struct LEARDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LEARDSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `LEAWRSTAT`"]
pub type LEAWRSTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEAWRSTAT`"]
pub struct LEAWRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAWRSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit indicate if LEA is able to accept new Commands (SUSPEND is always accepted)"]
    #[inline(always)]
    pub fn leabusy(&self) -> LEABUSY_R {
        LEABUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
These Bits indicate the actual operation mode LEA is in. Other = Reserved"]
    #[inline(always)]
    pub fn leamode(&self) -> LEAMODE_R {
        LEAMODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
These bits indicate the current power consumption as a relative value. The value zero indicated only static operation (usually clock less). This register might be read out for statistical power estimation of an application. These bits are also reflected in J-STATE when debugging"]
    #[inline(always)]
    pub fn leapwst(&self) -> LEAPWST_R {
        LEAPWST_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
These bits are used to store the internal state of the application specific processor (ASIP) inside the accelerator core. The specific meaning of those bit patterns is not shown in this document."]
    #[inline(always)]
    pub fn leaasst(&self) -> LEAASST_R {
        LEAASST_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
LEA done event indication and clear flag. This bit indicated the done event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leadonec(&self) -> LEADONEC_R {
        LEADONEC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
LEA free event indication and clear flag. This bit indicated the free event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leafreec(&self) -> LEAFREEC_R {
        LEAFREEC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
LEA timeout fault indication and clear flag; This bits indicates that a timer timeout occurred. This fault is cleared by writing a one to it. Writing a zero has no effect.."]
    #[inline(always)]
    pub fn leatimfltc(&self) -> LEATIMFLTC_R {
        LEATIMFLTC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
LEA command fault"]
    #[inline(always)]
    pub fn leacfltc(&self) -> LEACFLTC_R {
        LEACFLTC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
LEA memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEAWRSTAT and LEARDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEACNF0.LEAMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leamemfltc(&self) -> LEAMEMFLTC_R {
        LEAMEMFLTC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Read Status. This bit field keeps the VBUS read status lines from the last bus error condition."]
    #[inline(always)]
    pub fn leardstat(&self) -> LEARDSTAT_R {
        LEARDSTAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Write Status. This bit field keeps the VBUS write status lines from the last bus error condition."]
    #[inline(always)]
    pub fn leawrstat(&self) -> LEAWRSTAT_R {
        LEAWRSTAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit indicate if LEA is able to accept new Commands (SUSPEND is always accepted)"]
    #[inline(always)]
    pub fn leabusy(&mut self) -> LEABUSY_W {
        LEABUSY_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
These Bits indicate the actual operation mode LEA is in. Other = Reserved"]
    #[inline(always)]
    pub fn leamode(&mut self) -> LEAMODE_W {
        LEAMODE_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
These bits indicate the current power consumption as a relative value. The value zero indicated only static operation (usually clock less). This register might be read out for statistical power estimation of an application. These bits are also reflected in J-STATE when debugging"]
    #[inline(always)]
    pub fn leapwst(&mut self) -> LEAPWST_W {
        LEAPWST_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
These bits are used to store the internal state of the application specific processor (ASIP) inside the accelerator core. The specific meaning of those bit patterns is not shown in this document."]
    #[inline(always)]
    pub fn leaasst(&mut self) -> LEAASST_W {
        LEAASST_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
LEA done event indication and clear flag. This bit indicated the done event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leadonec(&mut self) -> LEADONEC_W {
        LEADONEC_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
LEA free event indication and clear flag. This bit indicated the free event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leafreec(&mut self) -> LEAFREEC_W {
        LEAFREEC_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
LEA timeout fault indication and clear flag; This bits indicates that a timer timeout occurred. This fault is cleared by writing a one to it. Writing a zero has no effect.."]
    #[inline(always)]
    pub fn leatimfltc(&mut self) -> LEATIMFLTC_W {
        LEATIMFLTC_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
LEA command fault"]
    #[inline(always)]
    pub fn leacfltc(&mut self) -> LEACFLTC_W {
        LEACFLTC_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
LEA memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEAWRSTAT and LEARDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEACNF0.LEAMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leamemfltc(&mut self) -> LEAMEMFLTC_W {
        LEAMEMFLTC_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Read Status. This bit field keeps the VBUS read status lines from the last bus error condition."]
    #[inline(always)]
    pub fn leardstat(&mut self) -> LEARDSTAT_W {
        LEARDSTAT_W { w: self }
    }
    #[doc = "Bits 28:31 - 31:28\\]
Write Status. This bit field keeps the VBUS write status lines from the last bus error condition."]
    #[inline(always)]
    pub fn leawrstat(&mut self) -> LEAWRSTAT_W {
        LEAWRSTAT_W { w: self }
    }
}
