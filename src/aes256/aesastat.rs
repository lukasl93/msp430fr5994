#[doc = "Reader of register AESASTAT"]
pub type R = crate::R<u16, super::AESASTAT>;
#[doc = "Writer for register AESASTAT"]
pub type W = crate::W<u16, super::AESASTAT>;
#[doc = "Register AESASTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::AESASTAT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
AES accelerator module busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESBUSY_A {
    #[doc = "0: Not busy"]
    IDLE = 0,
    #[doc = "1: Busy"]
    BUSY = 1,
}
impl From<AESBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: AESBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESBUSY`"]
pub type AESBUSY_R = crate::R<bool, AESBUSY_A>;
impl AESBUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESBUSY_A {
        match self.bits {
            false => AESBUSY_A::IDLE,
            true => AESBUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == AESBUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == AESBUSY_A::BUSY
    }
}
#[doc = "Write proxy for field `AESBUSY`"]
pub struct AESBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESBUSY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not busy"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(AESBUSY_A::IDLE)
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(AESBUSY_A::BUSY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "1:1\\]
All 16 bytes written to AESAKEY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESKEYWR_A {
    #[doc = "0: Not all bytes written"]
    AESKEYWR_0 = 0,
    #[doc = "1: All bytes written"]
    AESKEYWR_1 = 1,
}
impl From<AESKEYWR_A> for bool {
    #[inline(always)]
    fn from(variant: AESKEYWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESKEYWR`"]
pub type AESKEYWR_R = crate::R<bool, AESKEYWR_A>;
impl AESKEYWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESKEYWR_A {
        match self.bits {
            false => AESKEYWR_A::AESKEYWR_0,
            true => AESKEYWR_A::AESKEYWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESKEYWR_0`"]
    #[inline(always)]
    pub fn is_aeskeywr_0(&self) -> bool {
        *self == AESKEYWR_A::AESKEYWR_0
    }
    #[doc = "Checks if the value of the field is `AESKEYWR_1`"]
    #[inline(always)]
    pub fn is_aeskeywr_1(&self) -> bool {
        *self == AESKEYWR_A::AESKEYWR_1
    }
}
#[doc = "Write proxy for field `AESKEYWR`"]
pub struct AESKEYWR_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEYWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKEYWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn aeskeywr_0(self) -> &'a mut W {
        self.variant(AESKEYWR_A::AESKEYWR_0)
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn aeskeywr_1(self) -> &'a mut W {
        self.variant(AESKEYWR_A::AESKEYWR_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "2:2\\]
All 16 bytes written to AESADIN, AESAXDIN or AESAXIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESDINWR_A {
    #[doc = "0: Not all bytes written"]
    AESDINWR_0 = 0,
    #[doc = "1: All bytes written"]
    AESDINWR_1 = 1,
}
impl From<AESDINWR_A> for bool {
    #[inline(always)]
    fn from(variant: AESDINWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESDINWR`"]
pub type AESDINWR_R = crate::R<bool, AESDINWR_A>;
impl AESDINWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESDINWR_A {
        match self.bits {
            false => AESDINWR_A::AESDINWR_0,
            true => AESDINWR_A::AESDINWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESDINWR_0`"]
    #[inline(always)]
    pub fn is_aesdinwr_0(&self) -> bool {
        *self == AESDINWR_A::AESDINWR_0
    }
    #[doc = "Checks if the value of the field is `AESDINWR_1`"]
    #[inline(always)]
    pub fn is_aesdinwr_1(&self) -> bool {
        *self == AESDINWR_A::AESDINWR_1
    }
}
#[doc = "Write proxy for field `AESDINWR`"]
pub struct AESDINWR_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDINWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESDINWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn aesdinwr_0(self) -> &'a mut W {
        self.variant(AESDINWR_A::AESDINWR_0)
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn aesdinwr_1(self) -> &'a mut W {
        self.variant(AESDINWR_A::AESDINWR_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "3:3\\]
All 16 bytes read from AESADOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESDOUTRD_A {
    #[doc = "0: Not all bytes read"]
    AESDOUTRD_0 = 0,
    #[doc = "1: All bytes read"]
    AESDOUTRD_1 = 1,
}
impl From<AESDOUTRD_A> for bool {
    #[inline(always)]
    fn from(variant: AESDOUTRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESDOUTRD`"]
pub type AESDOUTRD_R = crate::R<bool, AESDOUTRD_A>;
impl AESDOUTRD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESDOUTRD_A {
        match self.bits {
            false => AESDOUTRD_A::AESDOUTRD_0,
            true => AESDOUTRD_A::AESDOUTRD_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESDOUTRD_0`"]
    #[inline(always)]
    pub fn is_aesdoutrd_0(&self) -> bool {
        *self == AESDOUTRD_A::AESDOUTRD_0
    }
    #[doc = "Checks if the value of the field is `AESDOUTRD_1`"]
    #[inline(always)]
    pub fn is_aesdoutrd_1(&self) -> bool {
        *self == AESDOUTRD_A::AESDOUTRD_1
    }
}
#[doc = "Write proxy for field `AESDOUTRD`"]
pub struct AESDOUTRD_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUTRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESDOUTRD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not all bytes read"]
    #[inline(always)]
    pub fn aesdoutrd_0(self) -> &'a mut W {
        self.variant(AESDOUTRD_A::AESDOUTRD_0)
    }
    #[doc = "All bytes read"]
    #[inline(always)]
    pub fn aesdoutrd_1(self) -> &'a mut W {
        self.variant(AESDOUTRD_A::AESDOUTRD_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `AESKEYCNT`"]
pub type AESKEYCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESKEYCNT`"]
pub struct AESKEYCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEYCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `AESDINCNT`"]
pub type AESDINCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESDINCNT`"]
pub struct AESDINCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDINCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AESDOUTCNT`"]
pub type AESDOUTCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESDOUTCNT`"]
pub struct AESDOUTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
AES accelerator module busy"]
    #[inline(always)]
    pub fn aesbusy(&self) -> AESBUSY_R {
        AESBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&self) -> AESKEYWR_R {
        AESKEYWR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdinwr(&self) -> AESDINWR_R {
        AESDINWR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
All 16 bytes read from AESADOUT"]
    #[inline(always)]
    pub fn aesdoutrd(&self) -> AESDOUTRD_R {
        AESDOUTRD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Bytes written via AESAKEY for AESKL=00, half-words written via AESAKEY"]
    #[inline(always)]
    pub fn aeskeycnt(&self) -> AESKEYCNT_R {
        AESKEYCNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Bytes written via AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdincnt(&self) -> AESDINCNT_R {
        AESDINCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Bytes read via AESADOUT"]
    #[inline(always)]
    pub fn aesdoutcnt(&self) -> AESDOUTCNT_R {
        AESDOUTCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
AES accelerator module busy"]
    #[inline(always)]
    pub fn aesbusy(&mut self) -> AESBUSY_W {
        AESBUSY_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&mut self) -> AESKEYWR_W {
        AESKEYWR_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdinwr(&mut self) -> AESDINWR_W {
        AESDINWR_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
All 16 bytes read from AESADOUT"]
    #[inline(always)]
    pub fn aesdoutrd(&mut self) -> AESDOUTRD_W {
        AESDOUTRD_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Bytes written via AESAKEY for AESKL=00, half-words written via AESAKEY"]
    #[inline(always)]
    pub fn aeskeycnt(&mut self) -> AESKEYCNT_W {
        AESKEYCNT_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Bytes written via AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdincnt(&mut self) -> AESDINCNT_W {
        AESDINCNT_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Bytes read via AESADOUT"]
    #[inline(always)]
    pub fn aesdoutcnt(&mut self) -> AESDOUTCNT_W {
        AESDOUTCNT_W { w: self }
    }
}
