#[doc = "Reader of register SYSRSTIV"]
pub type R = crate::R<u16, super::SYSRSTIV>;
#[doc = "Writer for register SYSRSTIV"]
pub type W = crate::W<u16, super::SYSRSTIV>;
#[doc = "Register SYSRSTIV `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSRSTIV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
Reset interrupt vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SYSRSTIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Brownout"]
    BOR = 2,
    #[doc = "4: RSTIFG RST/NMI"]
    RSTNMI = 4,
    #[doc = "6: PMMSWBOR software BOR"]
    PMMSWBOR = 6,
    #[doc = "8: LPMx.5 wakeup"]
    LPM5WU = 8,
    #[doc = "10: Security violation"]
    SECYV = 10,
    #[doc = "12: Reserved"]
    SYSRSTIV_12 = 12,
    #[doc = "14: SVSHIFG SVSH event"]
    SVSHIFG = 14,
    #[doc = "16: Reserved"]
    SYSRSTIV_16 = 16,
    #[doc = "18: Reserved"]
    SYSRSTIV_18 = 18,
    #[doc = "20: PMMSWPOR software POR"]
    PMMSWPOR = 20,
    #[doc = "22: WDTIFG watchdog timeout"]
    WDTIFG = 22,
    #[doc = "24: WDTPW watchdog password violation"]
    WDTPW = 24,
    #[doc = "26: FRCTLPW password violation"]
    FRCTLPW = 26,
    #[doc = "28: Uncorrectable FRAM bit error detection"]
    UBDIFG = 28,
    #[doc = "30: Peripheral area fetch"]
    PERF = 30,
    #[doc = "32: PMM password violation"]
    PMMPW = 32,
    #[doc = "34: MPU password violation"]
    MPUPW = 34,
    #[doc = "36: CS password violation"]
    CSPW = 36,
    #[doc = "38: MPUSEGPIFG encapsulated IP memory segment violation"]
    MPUSEGPIFG = 38,
    #[doc = "40: MPUSEGIIFG information memory segment violation"]
    MPUSEGIIFG = 40,
    #[doc = "42: MPUSEG1IFG segment 1 memory violation"]
    MPUSEG1IFG = 42,
    #[doc = "44: MPUSEG2IFG segment 2 memory violation"]
    MPUSEG2IFG = 44,
    #[doc = "46: MPUSEG3IFG segment 3 memory violation"]
    MPUSEG3IFG = 46,
}
impl From<SYSRSTIV_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSRSTIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSRSTIV`"]
pub type SYSRSTIV_R = crate::R<u16, SYSRSTIV_A>;
impl SYSRSTIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SYSRSTIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSRSTIV_A::NONE),
            2 => Val(SYSRSTIV_A::BOR),
            4 => Val(SYSRSTIV_A::RSTNMI),
            6 => Val(SYSRSTIV_A::PMMSWBOR),
            8 => Val(SYSRSTIV_A::LPM5WU),
            10 => Val(SYSRSTIV_A::SECYV),
            12 => Val(SYSRSTIV_A::SYSRSTIV_12),
            14 => Val(SYSRSTIV_A::SVSHIFG),
            16 => Val(SYSRSTIV_A::SYSRSTIV_16),
            18 => Val(SYSRSTIV_A::SYSRSTIV_18),
            20 => Val(SYSRSTIV_A::PMMSWPOR),
            22 => Val(SYSRSTIV_A::WDTIFG),
            24 => Val(SYSRSTIV_A::WDTPW),
            26 => Val(SYSRSTIV_A::FRCTLPW),
            28 => Val(SYSRSTIV_A::UBDIFG),
            30 => Val(SYSRSTIV_A::PERF),
            32 => Val(SYSRSTIV_A::PMMPW),
            34 => Val(SYSRSTIV_A::MPUPW),
            36 => Val(SYSRSTIV_A::CSPW),
            38 => Val(SYSRSTIV_A::MPUSEGPIFG),
            40 => Val(SYSRSTIV_A::MPUSEGIIFG),
            42 => Val(SYSRSTIV_A::MPUSEG1IFG),
            44 => Val(SYSRSTIV_A::MPUSEG2IFG),
            46 => Val(SYSRSTIV_A::MPUSEG3IFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYSRSTIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `BOR`"]
    #[inline(always)]
    pub fn is_bor(&self) -> bool {
        *self == SYSRSTIV_A::BOR
    }
    #[doc = "Checks if the value of the field is `RSTNMI`"]
    #[inline(always)]
    pub fn is_rstnmi(&self) -> bool {
        *self == SYSRSTIV_A::RSTNMI
    }
    #[doc = "Checks if the value of the field is `PMMSWBOR`"]
    #[inline(always)]
    pub fn is_pmmswbor(&self) -> bool {
        *self == SYSRSTIV_A::PMMSWBOR
    }
    #[doc = "Checks if the value of the field is `LPM5WU`"]
    #[inline(always)]
    pub fn is_lpm5wu(&self) -> bool {
        *self == SYSRSTIV_A::LPM5WU
    }
    #[doc = "Checks if the value of the field is `SECYV`"]
    #[inline(always)]
    pub fn is_secyv(&self) -> bool {
        *self == SYSRSTIV_A::SECYV
    }
    #[doc = "Checks if the value of the field is `SYSRSTIV_12`"]
    #[inline(always)]
    pub fn is_sysrstiv_12(&self) -> bool {
        *self == SYSRSTIV_A::SYSRSTIV_12
    }
    #[doc = "Checks if the value of the field is `SVSHIFG`"]
    #[inline(always)]
    pub fn is_svshifg(&self) -> bool {
        *self == SYSRSTIV_A::SVSHIFG
    }
    #[doc = "Checks if the value of the field is `SYSRSTIV_16`"]
    #[inline(always)]
    pub fn is_sysrstiv_16(&self) -> bool {
        *self == SYSRSTIV_A::SYSRSTIV_16
    }
    #[doc = "Checks if the value of the field is `SYSRSTIV_18`"]
    #[inline(always)]
    pub fn is_sysrstiv_18(&self) -> bool {
        *self == SYSRSTIV_A::SYSRSTIV_18
    }
    #[doc = "Checks if the value of the field is `PMMSWPOR`"]
    #[inline(always)]
    pub fn is_pmmswpor(&self) -> bool {
        *self == SYSRSTIV_A::PMMSWPOR
    }
    #[doc = "Checks if the value of the field is `WDTIFG`"]
    #[inline(always)]
    pub fn is_wdtifg(&self) -> bool {
        *self == SYSRSTIV_A::WDTIFG
    }
    #[doc = "Checks if the value of the field is `WDTPW`"]
    #[inline(always)]
    pub fn is_wdtpw(&self) -> bool {
        *self == SYSRSTIV_A::WDTPW
    }
    #[doc = "Checks if the value of the field is `FRCTLPW`"]
    #[inline(always)]
    pub fn is_frctlpw(&self) -> bool {
        *self == SYSRSTIV_A::FRCTLPW
    }
    #[doc = "Checks if the value of the field is `UBDIFG`"]
    #[inline(always)]
    pub fn is_ubdifg(&self) -> bool {
        *self == SYSRSTIV_A::UBDIFG
    }
    #[doc = "Checks if the value of the field is `PERF`"]
    #[inline(always)]
    pub fn is_perf(&self) -> bool {
        *self == SYSRSTIV_A::PERF
    }
    #[doc = "Checks if the value of the field is `PMMPW`"]
    #[inline(always)]
    pub fn is_pmmpw(&self) -> bool {
        *self == SYSRSTIV_A::PMMPW
    }
    #[doc = "Checks if the value of the field is `MPUPW`"]
    #[inline(always)]
    pub fn is_mpupw(&self) -> bool {
        *self == SYSRSTIV_A::MPUPW
    }
    #[doc = "Checks if the value of the field is `CSPW`"]
    #[inline(always)]
    pub fn is_cspw(&self) -> bool {
        *self == SYSRSTIV_A::CSPW
    }
    #[doc = "Checks if the value of the field is `MPUSEGPIFG`"]
    #[inline(always)]
    pub fn is_mpusegpifg(&self) -> bool {
        *self == SYSRSTIV_A::MPUSEGPIFG
    }
    #[doc = "Checks if the value of the field is `MPUSEGIIFG`"]
    #[inline(always)]
    pub fn is_mpusegiifg(&self) -> bool {
        *self == SYSRSTIV_A::MPUSEGIIFG
    }
    #[doc = "Checks if the value of the field is `MPUSEG1IFG`"]
    #[inline(always)]
    pub fn is_mpuseg1ifg(&self) -> bool {
        *self == SYSRSTIV_A::MPUSEG1IFG
    }
    #[doc = "Checks if the value of the field is `MPUSEG2IFG`"]
    #[inline(always)]
    pub fn is_mpuseg2ifg(&self) -> bool {
        *self == SYSRSTIV_A::MPUSEG2IFG
    }
    #[doc = "Checks if the value of the field is `MPUSEG3IFG`"]
    #[inline(always)]
    pub fn is_mpuseg3ifg(&self) -> bool {
        *self == SYSRSTIV_A::MPUSEG3IFG
    }
}
#[doc = "Write proxy for field `SYSRSTIV`"]
pub struct SYSRSTIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRSTIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::NONE)
    }
    #[doc = "Brownout"]
    #[inline(always)]
    pub fn bor(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::BOR)
    }
    #[doc = "RSTIFG RST/NMI"]
    #[inline(always)]
    pub fn rstnmi(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::RSTNMI)
    }
    #[doc = "PMMSWBOR software BOR"]
    #[inline(always)]
    pub fn pmmswbor(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::PMMSWBOR)
    }
    #[doc = "LPMx.5 wakeup"]
    #[inline(always)]
    pub fn lpm5wu(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::LPM5WU)
    }
    #[doc = "Security violation"]
    #[inline(always)]
    pub fn secyv(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::SECYV)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn sysrstiv_12(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::SYSRSTIV_12)
    }
    #[doc = "SVSHIFG SVSH event"]
    #[inline(always)]
    pub fn svshifg(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::SVSHIFG)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn sysrstiv_16(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::SYSRSTIV_16)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn sysrstiv_18(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::SYSRSTIV_18)
    }
    #[doc = "PMMSWPOR software POR"]
    #[inline(always)]
    pub fn pmmswpor(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::PMMSWPOR)
    }
    #[doc = "WDTIFG watchdog timeout"]
    #[inline(always)]
    pub fn wdtifg(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::WDTIFG)
    }
    #[doc = "WDTPW watchdog password violation"]
    #[inline(always)]
    pub fn wdtpw(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::WDTPW)
    }
    #[doc = "FRCTLPW password violation"]
    #[inline(always)]
    pub fn frctlpw(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::FRCTLPW)
    }
    #[doc = "Uncorrectable FRAM bit error detection"]
    #[inline(always)]
    pub fn ubdifg(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::UBDIFG)
    }
    #[doc = "Peripheral area fetch"]
    #[inline(always)]
    pub fn perf(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::PERF)
    }
    #[doc = "PMM password violation"]
    #[inline(always)]
    pub fn pmmpw(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::PMMPW)
    }
    #[doc = "MPU password violation"]
    #[inline(always)]
    pub fn mpupw(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::MPUPW)
    }
    #[doc = "CS password violation"]
    #[inline(always)]
    pub fn cspw(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::CSPW)
    }
    #[doc = "MPUSEGPIFG encapsulated IP memory segment violation"]
    #[inline(always)]
    pub fn mpusegpifg(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::MPUSEGPIFG)
    }
    #[doc = "MPUSEGIIFG information memory segment violation"]
    #[inline(always)]
    pub fn mpusegiifg(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::MPUSEGIIFG)
    }
    #[doc = "MPUSEG1IFG segment 1 memory violation"]
    #[inline(always)]
    pub fn mpuseg1ifg(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::MPUSEG1IFG)
    }
    #[doc = "MPUSEG2IFG segment 2 memory violation"]
    #[inline(always)]
    pub fn mpuseg2ifg(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::MPUSEG2IFG)
    }
    #[doc = "MPUSEG3IFG segment 3 memory violation"]
    #[inline(always)]
    pub fn mpuseg3ifg(self) -> &'a mut W {
        self.variant(SYSRSTIV_A::MPUSEG3IFG)
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
Reset interrupt vector"]
    #[inline(always)]
    pub fn sysrstiv(&self) -> SYSRSTIV_R {
        SYSRSTIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Reset interrupt vector"]
    #[inline(always)]
    pub fn sysrstiv(&mut self) -> SYSRSTIV_W {
        SYSRSTIV_W { w: self }
    }
}
