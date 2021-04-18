#[doc = "Reader of register RTCTIM1_BCD"]
pub type R = crate::R<u16, super::RTCTIM1_BCD>;
#[doc = "Writer for register RTCTIM1_BCD"]
pub type W = crate::W<u16, super::RTCTIM1_BCD>;
#[doc = "Register RTCTIM1_BCD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCTIM1_BCD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOURSLOWDIGIT`"]
pub type HOURSLOWDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOURSLOWDIGIT`"]
pub struct HOURSLOWDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURSLOWDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `HOURSHIGHDIGIT`"]
pub type HOURSHIGHDIGIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOURSHIGHDIGIT`"]
pub struct HOURSHIGHDIGIT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURSHIGHDIGIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
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
    #[doc = "Bits 0:3 - 3:0\\]
Hours low digit (0 to 9)"]
    #[inline(always)]
    pub fn hourslowdigit(&self) -> HOURSLOWDIGIT_R {
        HOURSLOWDIGIT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Hours high digit (0 to 2)"]
    #[inline(always)]
    pub fn hourshighdigit(&self) -> HOURSHIGHDIGIT_R {
        HOURSHIGHDIGIT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayofweek(&self) -> DAYOFWEEK_R {
        DAYOFWEEK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Hours low digit (0 to 9)"]
    #[inline(always)]
    pub fn hourslowdigit(&mut self) -> HOURSLOWDIGIT_W {
        HOURSLOWDIGIT_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Hours high digit (0 to 2)"]
    #[inline(always)]
    pub fn hourshighdigit(&mut self) -> HOURSHIGHDIGIT_W {
        HOURSHIGHDIGIT_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayofweek(&mut self) -> DAYOFWEEK_W {
        DAYOFWEEK_W { w: self }
    }
}
