#[doc = "Reader of register UCA2RXBUF_SPI"]
pub type R = crate::R<u16, super::UCA2RXBUF_SPI>;
#[doc = "Writer for register UCA2RXBUF_SPI"]
pub type W = crate::W<u16, super::UCA2RXBUF_SPI>;
#[doc = "Register UCA2RXBUF_SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA2RXBUF_SPI {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCRXBUF`"]
pub type UCRXBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCRXBUF`"]
pub struct UCRXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UCRXBUF_R {
        UCRXBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&mut self) -> UCRXBUF_W {
        UCRXBUF_W { w: self }
    }
}