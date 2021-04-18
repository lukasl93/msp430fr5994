#[doc = "Reader of register LEACAP"]
pub type R = crate::R<u32, super::LEACAP>;
#[doc = "Writer for register LEACAP"]
pub type W = crate::W<u32, super::LEACAP>;
#[doc = "Register LEACAP `reset()`'s with value 0"]
impl crate::ResetValue for super::LEACAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "3:0\\]
LEA Code Memory Size. This register identifies the size of available code RAM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEAMSIZ_A {
    #[doc = "0: no code RAM"]
    LEAMSIZ_0 = 0,
    #[doc = "1: 1KB Code RAM"]
    LEAMSIZ_1 = 1,
}
impl From<LEAMSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: LEAMSIZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEAMSIZ`"]
pub type LEAMSIZ_R = crate::R<u8, LEAMSIZ_A>;
impl LEAMSIZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LEAMSIZ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LEAMSIZ_A::LEAMSIZ_0),
            1 => Val(LEAMSIZ_A::LEAMSIZ_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEAMSIZ_0`"]
    #[inline(always)]
    pub fn is_leamsiz_0(&self) -> bool {
        *self == LEAMSIZ_A::LEAMSIZ_0
    }
    #[doc = "Checks if the value of the field is `LEAMSIZ_1`"]
    #[inline(always)]
    pub fn is_leamsiz_1(&self) -> bool {
        *self == LEAMSIZ_A::LEAMSIZ_1
    }
}
#[doc = "Write proxy for field `LEAMSIZ`"]
pub struct LEAMSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAMSIZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAMSIZ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no code RAM"]
    #[inline(always)]
    pub fn leamsiz_0(self) -> &'a mut W {
        self.variant(LEAMSIZ_A::LEAMSIZ_0)
    }
    #[doc = "1KB Code RAM"]
    #[inline(always)]
    pub fn leamsiz_1(self) -> &'a mut W {
        self.variant(LEAMSIZ_A::LEAMSIZ_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
LEA Code Memory Size. This register identifies the size of available code RAM."]
    #[inline(always)]
    pub fn leamsiz(&self) -> LEAMSIZ_R {
        LEAMSIZ_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
LEA Code Memory Size. This register identifies the size of available code RAM."]
    #[inline(always)]
    pub fn leamsiz(&mut self) -> LEAMSIZ_W {
        LEAMSIZ_W { w: self }
    }
}
