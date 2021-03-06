#[doc = "Reader of register P2IV"]
pub type R = crate::R<u16, super::P2IV>;
#[doc = "Writer for register P2IV"]
pub type W = crate::W<u16, super::P2IV>;
#[doc = "Register P2IV `reset()`'s with value 0"]
impl crate::ResetValue for super::P2IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "4:0\\]
Port 2 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 2.0 interrupt; Interrupt Flag: P2IFG0; Interrupt Priority: Highest"]
    P2IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 2.1 interrupt; Interrupt Flag: P2IFG1"]
    P2IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 2.2 interrupt; Interrupt Flag: P2IFG2"]
    P2IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 2.3 interrupt; Interrupt Flag: P2IFG3"]
    P2IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 2.4 interrupt; Interrupt Flag: P2IFG4"]
    P2IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 2.5 interrupt; Interrupt Flag: P2IFG5"]
    P2IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 2.6 interrupt; Interrupt Flag: P2IFG6"]
    P2IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 2.7 interrupt; Interrupt Flag: P2IFG7; Interrupt Priority: Lowest"]
    P2IFG7 = 16,
}
impl From<P2IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P2IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2IV`"]
pub type P2IV_R = crate::R<u8, P2IV_A>;
impl P2IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P2IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P2IV_A::NONE),
            2 => Val(P2IV_A::P2IFG0),
            4 => Val(P2IV_A::P2IFG1),
            6 => Val(P2IV_A::P2IFG2),
            8 => Val(P2IV_A::P2IFG3),
            10 => Val(P2IV_A::P2IFG4),
            12 => Val(P2IV_A::P2IFG5),
            14 => Val(P2IV_A::P2IFG6),
            16 => Val(P2IV_A::P2IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P2IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P2IFG0`"]
    #[inline(always)]
    pub fn is_p2ifg0(&self) -> bool {
        *self == P2IV_A::P2IFG0
    }
    #[doc = "Checks if the value of the field is `P2IFG1`"]
    #[inline(always)]
    pub fn is_p2ifg1(&self) -> bool {
        *self == P2IV_A::P2IFG1
    }
    #[doc = "Checks if the value of the field is `P2IFG2`"]
    #[inline(always)]
    pub fn is_p2ifg2(&self) -> bool {
        *self == P2IV_A::P2IFG2
    }
    #[doc = "Checks if the value of the field is `P2IFG3`"]
    #[inline(always)]
    pub fn is_p2ifg3(&self) -> bool {
        *self == P2IV_A::P2IFG3
    }
    #[doc = "Checks if the value of the field is `P2IFG4`"]
    #[inline(always)]
    pub fn is_p2ifg4(&self) -> bool {
        *self == P2IV_A::P2IFG4
    }
    #[doc = "Checks if the value of the field is `P2IFG5`"]
    #[inline(always)]
    pub fn is_p2ifg5(&self) -> bool {
        *self == P2IV_A::P2IFG5
    }
    #[doc = "Checks if the value of the field is `P2IFG6`"]
    #[inline(always)]
    pub fn is_p2ifg6(&self) -> bool {
        *self == P2IV_A::P2IFG6
    }
    #[doc = "Checks if the value of the field is `P2IFG7`"]
    #[inline(always)]
    pub fn is_p2ifg7(&self) -> bool {
        *self == P2IV_A::P2IFG7
    }
}
#[doc = "Write proxy for field `P2IV`"]
pub struct P2IV_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2IV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(P2IV_A::NONE)
    }
    #[doc = "Interrupt Source: Port 2.0 interrupt; Interrupt Flag: P2IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn p2ifg0(self) -> &'a mut W {
        self.variant(P2IV_A::P2IFG0)
    }
    #[doc = "Interrupt Source: Port 2.1 interrupt; Interrupt Flag: P2IFG1"]
    #[inline(always)]
    pub fn p2ifg1(self) -> &'a mut W {
        self.variant(P2IV_A::P2IFG1)
    }
    #[doc = "Interrupt Source: Port 2.2 interrupt; Interrupt Flag: P2IFG2"]
    #[inline(always)]
    pub fn p2ifg2(self) -> &'a mut W {
        self.variant(P2IV_A::P2IFG2)
    }
    #[doc = "Interrupt Source: Port 2.3 interrupt; Interrupt Flag: P2IFG3"]
    #[inline(always)]
    pub fn p2ifg3(self) -> &'a mut W {
        self.variant(P2IV_A::P2IFG3)
    }
    #[doc = "Interrupt Source: Port 2.4 interrupt; Interrupt Flag: P2IFG4"]
    #[inline(always)]
    pub fn p2ifg4(self) -> &'a mut W {
        self.variant(P2IV_A::P2IFG4)
    }
    #[doc = "Interrupt Source: Port 2.5 interrupt; Interrupt Flag: P2IFG5"]
    #[inline(always)]
    pub fn p2ifg5(self) -> &'a mut W {
        self.variant(P2IV_A::P2IFG5)
    }
    #[doc = "Interrupt Source: Port 2.6 interrupt; Interrupt Flag: P2IFG6"]
    #[inline(always)]
    pub fn p2ifg6(self) -> &'a mut W {
        self.variant(P2IV_A::P2IFG6)
    }
    #[doc = "Interrupt Source: Port 2.7 interrupt; Interrupt Flag: P2IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn p2ifg7(self) -> &'a mut W {
        self.variant(P2IV_A::P2IFG7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Port 2 interrupt vector value"]
    #[inline(always)]
    pub fn p2iv(&self) -> P2IV_R {
        P2IV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Port 2 interrupt vector value"]
    #[inline(always)]
    pub fn p2iv(&mut self) -> P2IV_W {
        P2IV_W { w: self }
    }
}
