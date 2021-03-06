#[doc = "Reader of register P3IV"]
pub type R = crate::R<u16, super::P3IV>;
#[doc = "Writer for register P3IV"]
pub type W = crate::W<u16, super::P3IV>;
#[doc = "Register P3IV `reset()`'s with value 0"]
impl crate::ResetValue for super::P3IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "4:0\\]
Port 3 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P3IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest"]
    P3IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1"]
    P3IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2"]
    P3IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3"]
    P3IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4"]
    P3IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5"]
    P3IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6"]
    P3IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest"]
    P3IFG7 = 16,
}
impl From<P3IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P3IV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P3IV`"]
pub type P3IV_R = crate::R<u8, P3IV_A>;
impl P3IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P3IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P3IV_A::NONE),
            2 => Val(P3IV_A::P3IFG0),
            4 => Val(P3IV_A::P3IFG1),
            6 => Val(P3IV_A::P3IFG2),
            8 => Val(P3IV_A::P3IFG3),
            10 => Val(P3IV_A::P3IFG4),
            12 => Val(P3IV_A::P3IFG5),
            14 => Val(P3IV_A::P3IFG6),
            16 => Val(P3IV_A::P3IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P3IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P3IFG0`"]
    #[inline(always)]
    pub fn is_p3ifg0(&self) -> bool {
        *self == P3IV_A::P3IFG0
    }
    #[doc = "Checks if the value of the field is `P3IFG1`"]
    #[inline(always)]
    pub fn is_p3ifg1(&self) -> bool {
        *self == P3IV_A::P3IFG1
    }
    #[doc = "Checks if the value of the field is `P3IFG2`"]
    #[inline(always)]
    pub fn is_p3ifg2(&self) -> bool {
        *self == P3IV_A::P3IFG2
    }
    #[doc = "Checks if the value of the field is `P3IFG3`"]
    #[inline(always)]
    pub fn is_p3ifg3(&self) -> bool {
        *self == P3IV_A::P3IFG3
    }
    #[doc = "Checks if the value of the field is `P3IFG4`"]
    #[inline(always)]
    pub fn is_p3ifg4(&self) -> bool {
        *self == P3IV_A::P3IFG4
    }
    #[doc = "Checks if the value of the field is `P3IFG5`"]
    #[inline(always)]
    pub fn is_p3ifg5(&self) -> bool {
        *self == P3IV_A::P3IFG5
    }
    #[doc = "Checks if the value of the field is `P3IFG6`"]
    #[inline(always)]
    pub fn is_p3ifg6(&self) -> bool {
        *self == P3IV_A::P3IFG6
    }
    #[doc = "Checks if the value of the field is `P3IFG7`"]
    #[inline(always)]
    pub fn is_p3ifg7(&self) -> bool {
        *self == P3IV_A::P3IFG7
    }
}
#[doc = "Write proxy for field `P3IV`"]
pub struct P3IV_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3IV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(P3IV_A::NONE)
    }
    #[doc = "Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn p3ifg0(self) -> &'a mut W {
        self.variant(P3IV_A::P3IFG0)
    }
    #[doc = "Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1"]
    #[inline(always)]
    pub fn p3ifg1(self) -> &'a mut W {
        self.variant(P3IV_A::P3IFG1)
    }
    #[doc = "Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2"]
    #[inline(always)]
    pub fn p3ifg2(self) -> &'a mut W {
        self.variant(P3IV_A::P3IFG2)
    }
    #[doc = "Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3"]
    #[inline(always)]
    pub fn p3ifg3(self) -> &'a mut W {
        self.variant(P3IV_A::P3IFG3)
    }
    #[doc = "Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4"]
    #[inline(always)]
    pub fn p3ifg4(self) -> &'a mut W {
        self.variant(P3IV_A::P3IFG4)
    }
    #[doc = "Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5"]
    #[inline(always)]
    pub fn p3ifg5(self) -> &'a mut W {
        self.variant(P3IV_A::P3IFG5)
    }
    #[doc = "Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6"]
    #[inline(always)]
    pub fn p3ifg6(self) -> &'a mut W {
        self.variant(P3IV_A::P3IFG6)
    }
    #[doc = "Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn p3ifg7(self) -> &'a mut W {
        self.variant(P3IV_A::P3IFG7)
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
Port 3 interrupt vector value"]
    #[inline(always)]
    pub fn p3iv(&self) -> P3IV_R {
        P3IV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Port 3 interrupt vector value"]
    #[inline(always)]
    pub fn p3iv(&mut self) -> P3IV_W {
        P3IV_W { w: self }
    }
}
