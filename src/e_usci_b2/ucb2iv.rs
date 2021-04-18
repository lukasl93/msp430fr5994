#[doc = "Reader of register UCB2IV"]
pub type R = crate::R<u16, super::UCB2IV>;
#[doc = "Writer for register UCB2IV"]
pub type W = crate::W<u16, super::UCB2IV>;
#[doc = "Register UCB2IV `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB2IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "15:0\\]
eUSCI_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum UCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest"]
    UCALIFG = 2,
    #[doc = "4: Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG"]
    UCNACKIFG = 4,
    #[doc = "6: Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG"]
    UCSTTIFG = 6,
    #[doc = "8: Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG"]
    UCSTPIFG = 8,
    #[doc = "10: Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3"]
    UCRXIFG3 = 10,
    #[doc = "12: Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3"]
    UCTXIFG3 = 12,
    #[doc = "14: Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2"]
    UCRXIFG2 = 14,
    #[doc = "16: Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2"]
    UCTXIFG2 = 16,
    #[doc = "18: Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1"]
    UCRXIFG1 = 18,
    #[doc = "20: Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1"]
    UCTXIFG1 = 20,
    #[doc = "22: Interrupt Source: Data received; Interrupt Flag: UCRXIFG0"]
    UCRXIFG0 = 22,
    #[doc = "24: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0"]
    UCTXIFG0 = 24,
    #[doc = "26: Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG"]
    UCBCNTIFG = 26,
    #[doc = "28: Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG"]
    UCCLTOIFG = 28,
    #[doc = "30: Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest"]
    UCBIT9IFG = 30,
}
impl From<UCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: UCIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UCIV`"]
pub type UCIV_R = crate::R<u16, UCIV_A>;
impl UCIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, UCIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UCIV_A::NONE),
            2 => Val(UCIV_A::UCALIFG),
            4 => Val(UCIV_A::UCNACKIFG),
            6 => Val(UCIV_A::UCSTTIFG),
            8 => Val(UCIV_A::UCSTPIFG),
            10 => Val(UCIV_A::UCRXIFG3),
            12 => Val(UCIV_A::UCTXIFG3),
            14 => Val(UCIV_A::UCRXIFG2),
            16 => Val(UCIV_A::UCTXIFG2),
            18 => Val(UCIV_A::UCRXIFG1),
            20 => Val(UCIV_A::UCTXIFG1),
            22 => Val(UCIV_A::UCRXIFG0),
            24 => Val(UCIV_A::UCTXIFG0),
            26 => Val(UCIV_A::UCBCNTIFG),
            28 => Val(UCIV_A::UCCLTOIFG),
            30 => Val(UCIV_A::UCBIT9IFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `UCALIFG`"]
    #[inline(always)]
    pub fn is_ucalifg(&self) -> bool {
        *self == UCIV_A::UCALIFG
    }
    #[doc = "Checks if the value of the field is `UCNACKIFG`"]
    #[inline(always)]
    pub fn is_ucnackifg(&self) -> bool {
        *self == UCIV_A::UCNACKIFG
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG`"]
    #[inline(always)]
    pub fn is_ucsttifg(&self) -> bool {
        *self == UCIV_A::UCSTTIFG
    }
    #[doc = "Checks if the value of the field is `UCSTPIFG`"]
    #[inline(always)]
    pub fn is_ucstpifg(&self) -> bool {
        *self == UCIV_A::UCSTPIFG
    }
    #[doc = "Checks if the value of the field is `UCRXIFG3`"]
    #[inline(always)]
    pub fn is_ucrxifg3(&self) -> bool {
        *self == UCIV_A::UCRXIFG3
    }
    #[doc = "Checks if the value of the field is `UCTXIFG3`"]
    #[inline(always)]
    pub fn is_uctxifg3(&self) -> bool {
        *self == UCIV_A::UCTXIFG3
    }
    #[doc = "Checks if the value of the field is `UCRXIFG2`"]
    #[inline(always)]
    pub fn is_ucrxifg2(&self) -> bool {
        *self == UCIV_A::UCRXIFG2
    }
    #[doc = "Checks if the value of the field is `UCTXIFG2`"]
    #[inline(always)]
    pub fn is_uctxifg2(&self) -> bool {
        *self == UCIV_A::UCTXIFG2
    }
    #[doc = "Checks if the value of the field is `UCRXIFG1`"]
    #[inline(always)]
    pub fn is_ucrxifg1(&self) -> bool {
        *self == UCIV_A::UCRXIFG1
    }
    #[doc = "Checks if the value of the field is `UCTXIFG1`"]
    #[inline(always)]
    pub fn is_uctxifg1(&self) -> bool {
        *self == UCIV_A::UCTXIFG1
    }
    #[doc = "Checks if the value of the field is `UCRXIFG0`"]
    #[inline(always)]
    pub fn is_ucrxifg0(&self) -> bool {
        *self == UCIV_A::UCRXIFG0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG0`"]
    #[inline(always)]
    pub fn is_uctxifg0(&self) -> bool {
        *self == UCIV_A::UCTXIFG0
    }
    #[doc = "Checks if the value of the field is `UCBCNTIFG`"]
    #[inline(always)]
    pub fn is_ucbcntifg(&self) -> bool {
        *self == UCIV_A::UCBCNTIFG
    }
    #[doc = "Checks if the value of the field is `UCCLTOIFG`"]
    #[inline(always)]
    pub fn is_uccltoifg(&self) -> bool {
        *self == UCIV_A::UCCLTOIFG
    }
    #[doc = "Checks if the value of the field is `UCBIT9IFG`"]
    #[inline(always)]
    pub fn is_ucbit9ifg(&self) -> bool {
        *self == UCIV_A::UCBIT9IFG
    }
}
#[doc = "Write proxy for field `UCIV`"]
pub struct UCIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(UCIV_A::NONE)
    }
    #[doc = "Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn ucalifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCALIFG)
    }
    #[doc = "Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG"]
    #[inline(always)]
    pub fn ucnackifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCNACKIFG)
    }
    #[doc = "Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG"]
    #[inline(always)]
    pub fn ucsttifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCSTTIFG)
    }
    #[doc = "Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG"]
    #[inline(always)]
    pub fn ucstpifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCSTPIFG)
    }
    #[doc = "Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3"]
    #[inline(always)]
    pub fn ucrxifg3(self) -> &'a mut W {
        self.variant(UCIV_A::UCRXIFG3)
    }
    #[doc = "Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3"]
    #[inline(always)]
    pub fn uctxifg3(self) -> &'a mut W {
        self.variant(UCIV_A::UCTXIFG3)
    }
    #[doc = "Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2"]
    #[inline(always)]
    pub fn ucrxifg2(self) -> &'a mut W {
        self.variant(UCIV_A::UCRXIFG2)
    }
    #[doc = "Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2"]
    #[inline(always)]
    pub fn uctxifg2(self) -> &'a mut W {
        self.variant(UCIV_A::UCTXIFG2)
    }
    #[doc = "Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1"]
    #[inline(always)]
    pub fn ucrxifg1(self) -> &'a mut W {
        self.variant(UCIV_A::UCRXIFG1)
    }
    #[doc = "Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1"]
    #[inline(always)]
    pub fn uctxifg1(self) -> &'a mut W {
        self.variant(UCIV_A::UCTXIFG1)
    }
    #[doc = "Interrupt Source: Data received; Interrupt Flag: UCRXIFG0"]
    #[inline(always)]
    pub fn ucrxifg0(self) -> &'a mut W {
        self.variant(UCIV_A::UCRXIFG0)
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0"]
    #[inline(always)]
    pub fn uctxifg0(self) -> &'a mut W {
        self.variant(UCIV_A::UCTXIFG0)
    }
    #[doc = "Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG"]
    #[inline(always)]
    pub fn ucbcntifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCBCNTIFG)
    }
    #[doc = "Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG"]
    #[inline(always)]
    pub fn uccltoifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCCLTOIFG)
    }
    #[doc = "Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest"]
    #[inline(always)]
    pub fn ucbit9ifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCBIT9IFG)
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
eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&mut self) -> UCIV_W {
        UCIV_W { w: self }
    }
}
