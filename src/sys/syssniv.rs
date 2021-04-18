#[doc = "Reader of register SYSSNIV"]
pub type R = crate::R<u16, super::SYSSNIV>;
#[doc = "Writer for register SYSSNIV"]
pub type W = crate::W<u16, super::SYSSNIV>;
#[doc = "Register SYSSNIV `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSSNIV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
System NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SYSSNIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Reserved"]
    SYSSNIV_2 = 2,
    #[doc = "4: Uncorrectable FRAM bit error detection"]
    UBDIFG = 4,
    #[doc = "6: FRAM Access Time Error"]
    ACCTEIFG = 6,
    #[doc = "8: MPUSEGPIFG encapsulated IP memory segment violation"]
    MPUSEGPIFG = 8,
    #[doc = "10: MPUSEGIIFG information memory segment violation"]
    MPUSEGIIFG = 10,
    #[doc = "12: MPUSEG1IFG segment 1 memory violation"]
    MPUSEG1IFG = 12,
    #[doc = "14: MPUSEG2IFG segment 2 memory violation"]
    MPUSEG2IFG = 14,
    #[doc = "16: MPUSEG3IFG segment 3 memory violation"]
    MPUSEG3IFG = 16,
    #[doc = "18: VMAIFG Vacant memory access"]
    VMAIFG = 18,
    #[doc = "20: JMBINIFG JTAG mailbox input"]
    JMBINIFG = 20,
    #[doc = "22: JMBOUTIFG JTAG mailbox output"]
    JMBOUTIFG = 22,
    #[doc = "24: Correctable FRAM bit error detection"]
    CBDIFG = 24,
    #[doc = "26: FRAM write protection detection"]
    WPROT = 26,
    #[doc = "28: LEA time-out fault"]
    LEATO = 28,
    #[doc = "30: LEA command fault"]
    LEACMD = 30,
}
impl From<SYSSNIV_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSSNIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSSNIV`"]
pub type SYSSNIV_R = crate::R<u16, SYSSNIV_A>;
impl SYSSNIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SYSSNIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSSNIV_A::NONE),
            2 => Val(SYSSNIV_A::SYSSNIV_2),
            4 => Val(SYSSNIV_A::UBDIFG),
            6 => Val(SYSSNIV_A::ACCTEIFG),
            8 => Val(SYSSNIV_A::MPUSEGPIFG),
            10 => Val(SYSSNIV_A::MPUSEGIIFG),
            12 => Val(SYSSNIV_A::MPUSEG1IFG),
            14 => Val(SYSSNIV_A::MPUSEG2IFG),
            16 => Val(SYSSNIV_A::MPUSEG3IFG),
            18 => Val(SYSSNIV_A::VMAIFG),
            20 => Val(SYSSNIV_A::JMBINIFG),
            22 => Val(SYSSNIV_A::JMBOUTIFG),
            24 => Val(SYSSNIV_A::CBDIFG),
            26 => Val(SYSSNIV_A::WPROT),
            28 => Val(SYSSNIV_A::LEATO),
            30 => Val(SYSSNIV_A::LEACMD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYSSNIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `SYSSNIV_2`"]
    #[inline(always)]
    pub fn is_syssniv_2(&self) -> bool {
        *self == SYSSNIV_A::SYSSNIV_2
    }
    #[doc = "Checks if the value of the field is `UBDIFG`"]
    #[inline(always)]
    pub fn is_ubdifg(&self) -> bool {
        *self == SYSSNIV_A::UBDIFG
    }
    #[doc = "Checks if the value of the field is `ACCTEIFG`"]
    #[inline(always)]
    pub fn is_accteifg(&self) -> bool {
        *self == SYSSNIV_A::ACCTEIFG
    }
    #[doc = "Checks if the value of the field is `MPUSEGPIFG`"]
    #[inline(always)]
    pub fn is_mpusegpifg(&self) -> bool {
        *self == SYSSNIV_A::MPUSEGPIFG
    }
    #[doc = "Checks if the value of the field is `MPUSEGIIFG`"]
    #[inline(always)]
    pub fn is_mpusegiifg(&self) -> bool {
        *self == SYSSNIV_A::MPUSEGIIFG
    }
    #[doc = "Checks if the value of the field is `MPUSEG1IFG`"]
    #[inline(always)]
    pub fn is_mpuseg1ifg(&self) -> bool {
        *self == SYSSNIV_A::MPUSEG1IFG
    }
    #[doc = "Checks if the value of the field is `MPUSEG2IFG`"]
    #[inline(always)]
    pub fn is_mpuseg2ifg(&self) -> bool {
        *self == SYSSNIV_A::MPUSEG2IFG
    }
    #[doc = "Checks if the value of the field is `MPUSEG3IFG`"]
    #[inline(always)]
    pub fn is_mpuseg3ifg(&self) -> bool {
        *self == SYSSNIV_A::MPUSEG3IFG
    }
    #[doc = "Checks if the value of the field is `VMAIFG`"]
    #[inline(always)]
    pub fn is_vmaifg(&self) -> bool {
        *self == SYSSNIV_A::VMAIFG
    }
    #[doc = "Checks if the value of the field is `JMBINIFG`"]
    #[inline(always)]
    pub fn is_jmbinifg(&self) -> bool {
        *self == SYSSNIV_A::JMBINIFG
    }
    #[doc = "Checks if the value of the field is `JMBOUTIFG`"]
    #[inline(always)]
    pub fn is_jmboutifg(&self) -> bool {
        *self == SYSSNIV_A::JMBOUTIFG
    }
    #[doc = "Checks if the value of the field is `CBDIFG`"]
    #[inline(always)]
    pub fn is_cbdifg(&self) -> bool {
        *self == SYSSNIV_A::CBDIFG
    }
    #[doc = "Checks if the value of the field is `WPROT`"]
    #[inline(always)]
    pub fn is_wprot(&self) -> bool {
        *self == SYSSNIV_A::WPROT
    }
    #[doc = "Checks if the value of the field is `LEATO`"]
    #[inline(always)]
    pub fn is_leato(&self) -> bool {
        *self == SYSSNIV_A::LEATO
    }
    #[doc = "Checks if the value of the field is `LEACMD`"]
    #[inline(always)]
    pub fn is_leacmd(&self) -> bool {
        *self == SYSSNIV_A::LEACMD
    }
}
#[doc = "Write proxy for field `SYSSNIV`"]
pub struct SYSSNIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSSNIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSSNIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SYSSNIV_A::NONE)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn syssniv_2(self) -> &'a mut W {
        self.variant(SYSSNIV_A::SYSSNIV_2)
    }
    #[doc = "Uncorrectable FRAM bit error detection"]
    #[inline(always)]
    pub fn ubdifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::UBDIFG)
    }
    #[doc = "FRAM Access Time Error"]
    #[inline(always)]
    pub fn accteifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::ACCTEIFG)
    }
    #[doc = "MPUSEGPIFG encapsulated IP memory segment violation"]
    #[inline(always)]
    pub fn mpusegpifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::MPUSEGPIFG)
    }
    #[doc = "MPUSEGIIFG information memory segment violation"]
    #[inline(always)]
    pub fn mpusegiifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::MPUSEGIIFG)
    }
    #[doc = "MPUSEG1IFG segment 1 memory violation"]
    #[inline(always)]
    pub fn mpuseg1ifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::MPUSEG1IFG)
    }
    #[doc = "MPUSEG2IFG segment 2 memory violation"]
    #[inline(always)]
    pub fn mpuseg2ifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::MPUSEG2IFG)
    }
    #[doc = "MPUSEG3IFG segment 3 memory violation"]
    #[inline(always)]
    pub fn mpuseg3ifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::MPUSEG3IFG)
    }
    #[doc = "VMAIFG Vacant memory access"]
    #[inline(always)]
    pub fn vmaifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::VMAIFG)
    }
    #[doc = "JMBINIFG JTAG mailbox input"]
    #[inline(always)]
    pub fn jmbinifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::JMBINIFG)
    }
    #[doc = "JMBOUTIFG JTAG mailbox output"]
    #[inline(always)]
    pub fn jmboutifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::JMBOUTIFG)
    }
    #[doc = "Correctable FRAM bit error detection"]
    #[inline(always)]
    pub fn cbdifg(self) -> &'a mut W {
        self.variant(SYSSNIV_A::CBDIFG)
    }
    #[doc = "FRAM write protection detection"]
    #[inline(always)]
    pub fn wprot(self) -> &'a mut W {
        self.variant(SYSSNIV_A::WPROT)
    }
    #[doc = "LEA time-out fault"]
    #[inline(always)]
    pub fn leato(self) -> &'a mut W {
        self.variant(SYSSNIV_A::LEATO)
    }
    #[doc = "LEA command fault"]
    #[inline(always)]
    pub fn leacmd(self) -> &'a mut W {
        self.variant(SYSSNIV_A::LEACMD)
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
System NMI vector"]
    #[inline(always)]
    pub fn syssniv(&self) -> SYSSNIV_R {
        SYSSNIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
System NMI vector"]
    #[inline(always)]
    pub fn syssniv(&mut self) -> SYSSNIV_W {
        SYSSNIV_W { w: self }
    }
}
