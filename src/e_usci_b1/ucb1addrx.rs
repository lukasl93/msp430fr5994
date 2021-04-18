#[doc = "Reader of register UCB1ADDRX"]
pub type R = crate::R<u16, super::UCB1ADDRX>;
#[doc = "Writer for register UCB1ADDRX"]
pub type W = crate::W<u16, super::UCB1ADDRX>;
#[doc = "Register UCB1ADDRX `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB1ADDRX {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRX`"]
pub type ADDRX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDRX`"]
pub struct ADDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> ADDRX_R {
        ADDRX_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Received Address Register"]
    #[inline(always)]
    pub fn addrx(&mut self) -> ADDRX_W {
        ADDRX_W { w: self }
    }
}
