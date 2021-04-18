#[doc = "Reader of register DMACTL4"]
pub type R = crate::R<u16, super::DMACTL4>;
#[doc = "Writer for register DMACTL4"]
pub type W = crate::W<u16, super::DMACTL4>;
#[doc = "Register DMACTL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACTL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
Enable NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENNMI_A {
    #[doc = "0: NMI does not interrupt DMA transfer"]
    ENNMI_0 = 0,
    #[doc = "1: NMI interrupts a DMA transfer"]
    ENNMI_1 = 1,
}
impl From<ENNMI_A> for bool {
    #[inline(always)]
    fn from(variant: ENNMI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENNMI`"]
pub type ENNMI_R = crate::R<bool, ENNMI_A>;
impl ENNMI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENNMI_A {
        match self.bits {
            false => ENNMI_A::ENNMI_0,
            true => ENNMI_A::ENNMI_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENNMI_0`"]
    #[inline(always)]
    pub fn is_ennmi_0(&self) -> bool {
        *self == ENNMI_A::ENNMI_0
    }
    #[doc = "Checks if the value of the field is `ENNMI_1`"]
    #[inline(always)]
    pub fn is_ennmi_1(&self) -> bool {
        *self == ENNMI_A::ENNMI_1
    }
}
#[doc = "Write proxy for field `ENNMI`"]
pub struct ENNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> ENNMI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENNMI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NMI does not interrupt DMA transfer"]
    #[inline(always)]
    pub fn ennmi_0(self) -> &'a mut W {
        self.variant(ENNMI_A::ENNMI_0)
    }
    #[doc = "NMI interrupts a DMA transfer"]
    #[inline(always)]
    pub fn ennmi_1(self) -> &'a mut W {
        self.variant(ENNMI_A::ENNMI_1)
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
Round robin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROUNDROBIN_A {
    #[doc = "0: DMA channel priority is DMA0-DMA1-DMA2 - ... - DMA7"]
    ROUNDROBIN_0 = 0,
    #[doc = "1: DMA channel priority changes with each transfer"]
    ROUNDROBIN_1 = 1,
}
impl From<ROUNDROBIN_A> for bool {
    #[inline(always)]
    fn from(variant: ROUNDROBIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROUNDROBIN`"]
pub type ROUNDROBIN_R = crate::R<bool, ROUNDROBIN_A>;
impl ROUNDROBIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROUNDROBIN_A {
        match self.bits {
            false => ROUNDROBIN_A::ROUNDROBIN_0,
            true => ROUNDROBIN_A::ROUNDROBIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROUNDROBIN_0`"]
    #[inline(always)]
    pub fn is_roundrobin_0(&self) -> bool {
        *self == ROUNDROBIN_A::ROUNDROBIN_0
    }
    #[doc = "Checks if the value of the field is `ROUNDROBIN_1`"]
    #[inline(always)]
    pub fn is_roundrobin_1(&self) -> bool {
        *self == ROUNDROBIN_A::ROUNDROBIN_1
    }
}
#[doc = "Write proxy for field `ROUNDROBIN`"]
pub struct ROUNDROBIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUNDROBIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROUNDROBIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA channel priority is DMA0-DMA1-DMA2 - ... - DMA7"]
    #[inline(always)]
    pub fn roundrobin_0(self) -> &'a mut W {
        self.variant(ROUNDROBIN_A::ROUNDROBIN_0)
    }
    #[doc = "DMA channel priority changes with each transfer"]
    #[inline(always)]
    pub fn roundrobin_1(self) -> &'a mut W {
        self.variant(ROUNDROBIN_A::ROUNDROBIN_1)
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
Read-modify-write disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARMWDIS_A {
    #[doc = "0: DMA transfers can occur during read-modify-write CPU operations"]
    DMARMWDIS_0 = 0,
    #[doc = "1: DMA transfers inhibited during read-modify-write CPU operations"]
    DMARMWDIS_1 = 1,
}
impl From<DMARMWDIS_A> for bool {
    #[inline(always)]
    fn from(variant: DMARMWDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMARMWDIS`"]
pub type DMARMWDIS_R = crate::R<bool, DMARMWDIS_A>;
impl DMARMWDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMARMWDIS_A {
        match self.bits {
            false => DMARMWDIS_A::DMARMWDIS_0,
            true => DMARMWDIS_A::DMARMWDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMARMWDIS_0`"]
    #[inline(always)]
    pub fn is_dmarmwdis_0(&self) -> bool {
        *self == DMARMWDIS_A::DMARMWDIS_0
    }
    #[doc = "Checks if the value of the field is `DMARMWDIS_1`"]
    #[inline(always)]
    pub fn is_dmarmwdis_1(&self) -> bool {
        *self == DMARMWDIS_A::DMARMWDIS_1
    }
}
#[doc = "Write proxy for field `DMARMWDIS`"]
pub struct DMARMWDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARMWDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMARMWDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA transfers can occur during read-modify-write CPU operations"]
    #[inline(always)]
    pub fn dmarmwdis_0(self) -> &'a mut W {
        self.variant(DMARMWDIS_A::DMARMWDIS_0)
    }
    #[doc = "DMA transfers inhibited during read-modify-write CPU operations"]
    #[inline(always)]
    pub fn dmarmwdis_1(self) -> &'a mut W {
        self.variant(DMARMWDIS_A::DMARMWDIS_1)
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
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable NMI"]
    #[inline(always)]
    pub fn ennmi(&self) -> ENNMI_R {
        ENNMI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Round robin"]
    #[inline(always)]
    pub fn roundrobin(&self) -> ROUNDROBIN_R {
        ROUNDROBIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Read-modify-write disable"]
    #[inline(always)]
    pub fn dmarmwdis(&self) -> DMARMWDIS_R {
        DMARMWDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable NMI"]
    #[inline(always)]
    pub fn ennmi(&mut self) -> ENNMI_W {
        ENNMI_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Round robin"]
    #[inline(always)]
    pub fn roundrobin(&mut self) -> ROUNDROBIN_W {
        ROUNDROBIN_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Read-modify-write disable"]
    #[inline(always)]
    pub fn dmarmwdis(&mut self) -> DMARMWDIS_W {
        DMARMWDIS_W { w: self }
    }
}
