#[doc = "Reader of register LEACNF2"]
pub type R = crate::R<u32, super::LEACNF2>;
#[doc = "Writer for register LEACNF2"]
pub type W = crate::W<u32, super::LEACNF2>;
#[doc = "Register LEACNF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LEACNF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEASPTR`"]
pub type LEASPTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEASPTR`"]
pub struct LEASPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEASPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
LEA stack pointer value (byte units). 64 lower kB is physical limit for LEA application complexity"]
    #[inline(always)]
    pub fn leasptr(&self) -> LEASPTR_R {
        LEASPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
LEA stack pointer value (byte units). 64 lower kB is physical limit for LEA application complexity"]
    #[inline(always)]
    pub fn leasptr(&mut self) -> LEASPTR_W {
        LEASPTR_W { w: self }
    }
}
