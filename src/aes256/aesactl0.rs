#[doc = "Reader of register AESACTL0"]
pub type R = crate::R<u16, super::AESACTL0>;
#[doc = "Writer for register AESACTL0"]
pub type W = crate::W<u16, super::AESACTL0>;
#[doc = "Register AESACTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::AESACTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "1:0\\]
AES operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESOP_A {
    #[doc = "0: Encryption"]
    AESOP_0 = 0,
    #[doc = "1: Decryption. The provided key is the same key used for encryption"]
    AESOP_1 = 1,
    #[doc = "2: Generate first round key required for decryption"]
    AESOP_2 = 2,
    #[doc = "3: Decryption. The provided key is the first round key required for decryption"]
    AESOP_3 = 3,
}
impl From<AESOP_A> for u8 {
    #[inline(always)]
    fn from(variant: AESOP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESOP`"]
pub type AESOP_R = crate::R<u8, AESOP_A>;
impl AESOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESOP_A {
        match self.bits {
            0 => AESOP_A::AESOP_0,
            1 => AESOP_A::AESOP_1,
            2 => AESOP_A::AESOP_2,
            3 => AESOP_A::AESOP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AESOP_0`"]
    #[inline(always)]
    pub fn is_aesop_0(&self) -> bool {
        *self == AESOP_A::AESOP_0
    }
    #[doc = "Checks if the value of the field is `AESOP_1`"]
    #[inline(always)]
    pub fn is_aesop_1(&self) -> bool {
        *self == AESOP_A::AESOP_1
    }
    #[doc = "Checks if the value of the field is `AESOP_2`"]
    #[inline(always)]
    pub fn is_aesop_2(&self) -> bool {
        *self == AESOP_A::AESOP_2
    }
    #[doc = "Checks if the value of the field is `AESOP_3`"]
    #[inline(always)]
    pub fn is_aesop_3(&self) -> bool {
        *self == AESOP_A::AESOP_3
    }
}
#[doc = "Write proxy for field `AESOP`"]
pub struct AESOP_W<'a> {
    w: &'a mut W,
}
impl<'a> AESOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESOP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn aesop_0(self) -> &'a mut W {
        self.variant(AESOP_A::AESOP_0)
    }
    #[doc = "Decryption. The provided key is the same key used for encryption"]
    #[inline(always)]
    pub fn aesop_1(self) -> &'a mut W {
        self.variant(AESOP_A::AESOP_1)
    }
    #[doc = "Generate first round key required for decryption"]
    #[inline(always)]
    pub fn aesop_2(self) -> &'a mut W {
        self.variant(AESOP_A::AESOP_2)
    }
    #[doc = "Decryption. The provided key is the first round key required for decryption"]
    #[inline(always)]
    pub fn aesop_3(self) -> &'a mut W {
        self.variant(AESOP_A::AESOP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "3:2\\]
AES key length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESKL_A {
    #[doc = "0: AES128. The key size is 128 bit"]
    _128 = 0,
    #[doc = "1: AES192. The key size is 192 bit."]
    _192 = 1,
    #[doc = "2: AES256. The key size is 256 bit"]
    _256 = 2,
}
impl From<AESKL_A> for u8 {
    #[inline(always)]
    fn from(variant: AESKL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESKL`"]
pub type AESKL_R = crate::R<u8, AESKL_A>;
impl AESKL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AESKL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESKL_A::_128),
            1 => Val(AESKL_A::_192),
            2 => Val(AESKL_A::_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == AESKL_A::_128
    }
    #[doc = "Checks if the value of the field is `_192`"]
    #[inline(always)]
    pub fn is_192(&self) -> bool {
        *self == AESKL_A::_192
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == AESKL_A::_256
    }
}
#[doc = "Write proxy for field `AESKL`"]
pub struct AESKL_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AES128. The key size is 128 bit"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(AESKL_A::_128)
    }
    #[doc = "AES192. The key size is 192 bit."]
    #[inline(always)]
    pub fn _192(self) -> &'a mut W {
        self.variant(AESKL_A::_192)
    }
    #[doc = "AES256. The key size is 256 bit"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(AESKL_A::_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "6:5\\]
AES cipher mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESCM_A {
    #[doc = "0: ECB"]
    ECB = 0,
    #[doc = "1: CBC"]
    CBC = 1,
    #[doc = "2: OFB"]
    OFB = 2,
    #[doc = "3: CFB"]
    CFB = 3,
}
impl From<AESCM_A> for u8 {
    #[inline(always)]
    fn from(variant: AESCM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESCM`"]
pub type AESCM_R = crate::R<u8, AESCM_A>;
impl AESCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESCM_A {
        match self.bits {
            0 => AESCM_A::ECB,
            1 => AESCM_A::CBC,
            2 => AESCM_A::OFB,
            3 => AESCM_A::CFB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == AESCM_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == AESCM_A::CBC
    }
    #[doc = "Checks if the value of the field is `OFB`"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == AESCM_A::OFB
    }
    #[doc = "Checks if the value of the field is `CFB`"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == AESCM_A::CFB
    }
}
#[doc = "Write proxy for field `AESCM`"]
pub struct AESCM_W<'a> {
    w: &'a mut W,
}
impl<'a> AESCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESCM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ECB"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(AESCM_A::ECB)
    }
    #[doc = "CBC"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(AESCM_A::CBC)
    }
    #[doc = "OFB"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut W {
        self.variant(AESCM_A::OFB)
    }
    #[doc = "CFB"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut W {
        self.variant(AESCM_A::CFB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u16) & 0x03) << 5);
        self.w
    }
}
#[doc = "7:7\\]
AES software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESSWRST_A {
    #[doc = "0: No reset"]
    AESSWRST_0 = 0,
    #[doc = "1: Reset AES accelerator module"]
    RESET = 1,
}
impl From<AESSWRST_A> for bool {
    #[inline(always)]
    fn from(variant: AESSWRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESSWRST`"]
pub type AESSWRST_R = crate::R<bool, AESSWRST_A>;
impl AESSWRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESSWRST_A {
        match self.bits {
            false => AESSWRST_A::AESSWRST_0,
            true => AESSWRST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `AESSWRST_0`"]
    #[inline(always)]
    pub fn is_aesswrst_0(&self) -> bool {
        *self == AESSWRST_A::AESSWRST_0
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AESSWRST_A::RESET
    }
}
#[doc = "Write proxy for field `AESSWRST`"]
pub struct AESSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESSWRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset"]
    #[inline(always)]
    pub fn aesswrst_0(self) -> &'a mut W {
        self.variant(AESSWRST_A::AESSWRST_0)
    }
    #[doc = "Reset AES accelerator module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(AESSWRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "8:8\\]
AES ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESRDYIFG_A {
    #[doc = "0: No interrupt pending"]
    AESRDYIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    AESRDYIFG_1 = 1,
}
impl From<AESRDYIFG_A> for bool {
    #[inline(always)]
    fn from(variant: AESRDYIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESRDYIFG`"]
pub type AESRDYIFG_R = crate::R<bool, AESRDYIFG_A>;
impl AESRDYIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESRDYIFG_A {
        match self.bits {
            false => AESRDYIFG_A::AESRDYIFG_0,
            true => AESRDYIFG_A::AESRDYIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESRDYIFG_0`"]
    #[inline(always)]
    pub fn is_aesrdyifg_0(&self) -> bool {
        *self == AESRDYIFG_A::AESRDYIFG_0
    }
    #[doc = "Checks if the value of the field is `AESRDYIFG_1`"]
    #[inline(always)]
    pub fn is_aesrdyifg_1(&self) -> bool {
        *self == AESRDYIFG_A::AESRDYIFG_1
    }
}
#[doc = "Write proxy for field `AESRDYIFG`"]
pub struct AESRDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> AESRDYIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESRDYIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_0(self) -> &'a mut W {
        self.variant(AESRDYIFG_A::AESRDYIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_1(self) -> &'a mut W {
        self.variant(AESRDYIFG_A::AESRDYIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "11:11\\]
AES error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESERRFG_A {
    #[doc = "0: No error"]
    AESERRFG_0 = 0,
    #[doc = "1: Error occurred"]
    AESERRFG_1 = 1,
}
impl From<AESERRFG_A> for bool {
    #[inline(always)]
    fn from(variant: AESERRFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESERRFG`"]
pub type AESERRFG_R = crate::R<bool, AESERRFG_A>;
impl AESERRFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESERRFG_A {
        match self.bits {
            false => AESERRFG_A::AESERRFG_0,
            true => AESERRFG_A::AESERRFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESERRFG_0`"]
    #[inline(always)]
    pub fn is_aeserrfg_0(&self) -> bool {
        *self == AESERRFG_A::AESERRFG_0
    }
    #[doc = "Checks if the value of the field is `AESERRFG_1`"]
    #[inline(always)]
    pub fn is_aeserrfg_1(&self) -> bool {
        *self == AESERRFG_A::AESERRFG_1
    }
}
#[doc = "Write proxy for field `AESERRFG`"]
pub struct AESERRFG_W<'a> {
    w: &'a mut W,
}
impl<'a> AESERRFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESERRFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn aeserrfg_0(self) -> &'a mut W {
        self.variant(AESERRFG_A::AESERRFG_0)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn aeserrfg_1(self) -> &'a mut W {
        self.variant(AESERRFG_A::AESERRFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "12:12\\]
AES ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLE = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLE = 1,
}
impl From<AESRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: AESRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESRDYIE`"]
pub type AESRDYIE_R = crate::R<bool, AESRDYIE_A>;
impl AESRDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESRDYIE_A {
        match self.bits {
            false => AESRDYIE_A::DISABLE,
            true => AESRDYIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AESRDYIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AESRDYIE_A::ENABLE
    }
}
#[doc = "Write proxy for field `AESRDYIE`"]
pub struct AESRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AESRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AESRDYIE_A::DISABLE)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AESRDYIE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "15:15\\]
AES cipher mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESCMEN_A {
    #[doc = "0: No DMA triggers are generated"]
    DISABLE = 0,
    #[doc = "1: DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    ENABLE = 1,
}
impl From<AESCMEN_A> for bool {
    #[inline(always)]
    fn from(variant: AESCMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESCMEN`"]
pub type AESCMEN_R = crate::R<bool, AESCMEN_A>;
impl AESCMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESCMEN_A {
        match self.bits {
            false => AESCMEN_A::DISABLE,
            true => AESCMEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AESCMEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AESCMEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `AESCMEN`"]
pub struct AESCMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AESCMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESCMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA triggers are generated"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AESCMEN_A::DISABLE)
    }
    #[doc = "DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AESCMEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
AES operation"]
    #[inline(always)]
    pub fn aesop(&self) -> AESOP_R {
        AESOP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
AES key length"]
    #[inline(always)]
    pub fn aeskl(&self) -> AESKL_R {
        AESKL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
AES cipher mode select"]
    #[inline(always)]
    pub fn aescm(&self) -> AESCM_R {
        AESCM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&self) -> AESSWRST_R {
        AESSWRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&self) -> AESRDYIFG_R {
        AESRDYIFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&self) -> AESERRFG_R {
        AESERRFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&self) -> AESRDYIE_R {
        AESRDYIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&self) -> AESCMEN_R {
        AESCMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
AES operation"]
    #[inline(always)]
    pub fn aesop(&mut self) -> AESOP_W {
        AESOP_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
AES key length"]
    #[inline(always)]
    pub fn aeskl(&mut self) -> AESKL_W {
        AESKL_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\]
AES cipher mode select"]
    #[inline(always)]
    pub fn aescm(&mut self) -> AESCM_W {
        AESCM_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&mut self) -> AESSWRST_W {
        AESSWRST_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&mut self) -> AESRDYIFG_W {
        AESRDYIFG_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&mut self) -> AESERRFG_W {
        AESERRFG_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&mut self) -> AESRDYIE_W {
        AESRDYIE_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&mut self) -> AESCMEN_W {
        AESCMEN_W { w: self }
    }
}
