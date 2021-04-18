#[doc = "Reader of register LEAMB"]
pub type R = crate::R<u32, super::LEAMB>;
#[doc = "Writer for register LEAMB"]
pub type W = crate::W<u32, super::LEAMB>;
#[doc = "Register LEAMB `reset()`'s with value 0"]
impl crate::ResetValue for super::LEAMB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEAMB`"]
pub type LEAMB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEAMB`"]
pub struct LEAMB_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
LEA memory bottom address boundary in byte address units"]
    #[inline(always)]
    pub fn leamb(&self) -> LEAMB_R {
        LEAMB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
LEA memory bottom address boundary in byte address units"]
    #[inline(always)]
    pub fn leamb(&mut self) -> LEAMB_W {
        LEAMB_W { w: self }
    }
}
