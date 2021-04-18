#[doc = "Reader of register LEAMT"]
pub type R = crate::R<u32, super::LEAMT>;
#[doc = "Writer for register LEAMT"]
pub type W = crate::W<u32, super::LEAMT>;
#[doc = "Register LEAMT `reset()`'s with value 0"]
impl crate::ResetValue for super::LEAMT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEAMT`"]
pub type LEAMT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEAMT`"]
pub struct LEAMT_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
LEA memory top address boundary in byte address units"]
    #[inline(always)]
    pub fn leamt(&self) -> LEAMT_R {
        LEAMT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
LEA memory top address boundary in byte address units"]
    #[inline(always)]
    pub fn leamt(&mut self) -> LEAMT_W {
        LEAMT_W { w: self }
    }
}
