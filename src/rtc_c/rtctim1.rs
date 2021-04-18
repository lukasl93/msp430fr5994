#[doc = "Reader of register RTCTIM1"]
pub type R = crate::R<u16, super::RTCTIM1>;
#[doc = "Writer for register RTCTIM1"]
pub type W = crate::W<u16, super::RTCTIM1>;
#[doc = "Register RTCTIM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCTIM1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOURS`"]
pub type HOURS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOURS`"]
pub struct HOURS_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `DAYOFWEEK`"]
pub type DAYOFWEEK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYOFWEEK`"]
pub struct DAYOFWEEK_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYOFWEEK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayofweek(&self) -> DAYOFWEEK_R {
        DAYOFWEEK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&mut self) -> HOURS_W {
        HOURS_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayofweek(&mut self) -> DAYOFWEEK_W {
        DAYOFWEEK_W { w: self }
    }
}
