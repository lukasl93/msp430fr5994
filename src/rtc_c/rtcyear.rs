#[doc = "Reader of register RTCYEAR"]
pub type R = crate::R<u16, super::RTCYEAR>;
#[doc = "Writer for register RTCYEAR"]
pub type W = crate::W<u16, super::RTCYEAR>;
#[doc = "Register RTCYEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCYEAR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `YEARLOWBYTE`"]
pub type YEARLOWBYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YEARLOWBYTE`"]
pub struct YEARLOWBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> YEARLOWBYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `YEARHIGHBYTE`"]
pub type YEARHIGHBYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YEARHIGHBYTE`"]
pub struct YEARHIGHBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> YEARHIGHBYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Year low byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn yearlowbyte(&self) -> YEARLOWBYTE_R {
        YEARLOWBYTE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Year high byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn yearhighbyte(&self) -> YEARHIGHBYTE_R {
        YEARHIGHBYTE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Year low byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn yearlowbyte(&mut self) -> YEARLOWBYTE_W {
        YEARLOWBYTE_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Year high byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn yearhighbyte(&mut self) -> YEARHIGHBYTE_W {
        YEARHIGHBYTE_W { w: self }
    }
}
