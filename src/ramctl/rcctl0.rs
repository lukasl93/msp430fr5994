#[doc = "Reader of register RCCTL0"]
pub type R = crate::R<u16, super::RCCTL0>;
#[doc = "Writer for register RCCTL0"]
pub type W = crate::W<u16, super::RCCTL0>;
#[doc = "Register RCCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "1:0\\]
RAM controller RAM sector 0 off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCRS0OFF_A {
    #[doc = "0: Contents of this RAM sector are retained in LPM3 and LPM4."]
    RCRS0OFF_0 = 0,
    #[doc = "1: Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    RCRS0OFF_1 = 1,
    #[doc = "2: Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    RCRS0OFF_2 = 2,
}
impl From<RCRS0OFF_A> for u8 {
    #[inline(always)]
    fn from(variant: RCRS0OFF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RCRS0OFF`"]
pub type RCRS0OFF_R = crate::R<u8, RCRS0OFF_A>;
impl RCRS0OFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RCRS0OFF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RCRS0OFF_A::RCRS0OFF_0),
            1 => Val(RCRS0OFF_A::RCRS0OFF_1),
            2 => Val(RCRS0OFF_A::RCRS0OFF_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_0`"]
    #[inline(always)]
    pub fn is_rcrs0off_0(&self) -> bool {
        *self == RCRS0OFF_A::RCRS0OFF_0
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_1`"]
    #[inline(always)]
    pub fn is_rcrs0off_1(&self) -> bool {
        *self == RCRS0OFF_A::RCRS0OFF_1
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_2`"]
    #[inline(always)]
    pub fn is_rcrs0off_2(&self) -> bool {
        *self == RCRS0OFF_A::RCRS0OFF_2
    }
}
#[doc = "Write proxy for field `RCRS0OFF`"]
pub struct RCRS0OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS0OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRS0OFF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn rcrs0off_0(self) -> &'a mut W {
        self.variant(RCRS0OFF_A::RCRS0OFF_0)
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_1(self) -> &'a mut W {
        self.variant(RCRS0OFF_A::RCRS0OFF_1)
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_2(self) -> &'a mut W {
        self.variant(RCRS0OFF_A::RCRS0OFF_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `RCKEY`"]
pub type RCKEY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCKEY`"]
pub struct RCKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> RCKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
#[doc = "3:2\\]
RAM controller RAM sector 0 off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCRS1OFF_A {
    #[doc = "0: Contents of this RAM sector are retained in LPM3 and LPM4."]
    RCRS0OFF_0 = 0,
    #[doc = "1: Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    RCRS0OFF_1 = 1,
    #[doc = "2: Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    RCRS0OFF_2 = 2,
}
impl From<RCRS1OFF_A> for u8 {
    #[inline(always)]
    fn from(variant: RCRS1OFF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RCRS1OFF`"]
pub type RCRS1OFF_R = crate::R<u8, RCRS1OFF_A>;
impl RCRS1OFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RCRS1OFF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RCRS1OFF_A::RCRS0OFF_0),
            1 => Val(RCRS1OFF_A::RCRS0OFF_1),
            2 => Val(RCRS1OFF_A::RCRS0OFF_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_0`"]
    #[inline(always)]
    pub fn is_rcrs0off_0(&self) -> bool {
        *self == RCRS1OFF_A::RCRS0OFF_0
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_1`"]
    #[inline(always)]
    pub fn is_rcrs0off_1(&self) -> bool {
        *self == RCRS1OFF_A::RCRS0OFF_1
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_2`"]
    #[inline(always)]
    pub fn is_rcrs0off_2(&self) -> bool {
        *self == RCRS1OFF_A::RCRS0OFF_2
    }
}
#[doc = "Write proxy for field `RCRS1OFF`"]
pub struct RCRS1OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS1OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRS1OFF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn rcrs0off_0(self) -> &'a mut W {
        self.variant(RCRS1OFF_A::RCRS0OFF_0)
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_1(self) -> &'a mut W {
        self.variant(RCRS1OFF_A::RCRS0OFF_1)
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_2(self) -> &'a mut W {
        self.variant(RCRS1OFF_A::RCRS0OFF_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "7:6\\]
RAM controller RAM sector 3 off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCRS3OFF_A {
    #[doc = "0: Contents of this RAM sector are retained in LPM3 and LPM4."]
    RCRS0OFF_0 = 0,
    #[doc = "1: Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    RCRS0OFF_1 = 1,
    #[doc = "2: Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    RCRS0OFF_2 = 2,
}
impl From<RCRS3OFF_A> for u8 {
    #[inline(always)]
    fn from(variant: RCRS3OFF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RCRS3OFF`"]
pub type RCRS3OFF_R = crate::R<u8, RCRS3OFF_A>;
impl RCRS3OFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RCRS3OFF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RCRS3OFF_A::RCRS0OFF_0),
            1 => Val(RCRS3OFF_A::RCRS0OFF_1),
            2 => Val(RCRS3OFF_A::RCRS0OFF_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_0`"]
    #[inline(always)]
    pub fn is_rcrs0off_0(&self) -> bool {
        *self == RCRS3OFF_A::RCRS0OFF_0
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_1`"]
    #[inline(always)]
    pub fn is_rcrs0off_1(&self) -> bool {
        *self == RCRS3OFF_A::RCRS0OFF_1
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_2`"]
    #[inline(always)]
    pub fn is_rcrs0off_2(&self) -> bool {
        *self == RCRS3OFF_A::RCRS0OFF_2
    }
}
#[doc = "Write proxy for field `RCRS3OFF`"]
pub struct RCRS3OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS3OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRS3OFF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn rcrs0off_0(self) -> &'a mut W {
        self.variant(RCRS3OFF_A::RCRS0OFF_0)
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_1(self) -> &'a mut W {
        self.variant(RCRS3OFF_A::RCRS0OFF_1)
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_2(self) -> &'a mut W {
        self.variant(RCRS3OFF_A::RCRS0OFF_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "5:4\\]
RAM controller RAM sector 0 off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCRS2OFF_A {
    #[doc = "0: Contents of this RAM sector are retained in LPM3 and LPM4."]
    RCRS0OFF_0 = 0,
    #[doc = "1: Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    RCRS0OFF_1 = 1,
    #[doc = "2: Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    RCRS0OFF_2 = 2,
}
impl From<RCRS2OFF_A> for u8 {
    #[inline(always)]
    fn from(variant: RCRS2OFF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RCRS2OFF`"]
pub type RCRS2OFF_R = crate::R<u8, RCRS2OFF_A>;
impl RCRS2OFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RCRS2OFF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RCRS2OFF_A::RCRS0OFF_0),
            1 => Val(RCRS2OFF_A::RCRS0OFF_1),
            2 => Val(RCRS2OFF_A::RCRS0OFF_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_0`"]
    #[inline(always)]
    pub fn is_rcrs0off_0(&self) -> bool {
        *self == RCRS2OFF_A::RCRS0OFF_0
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_1`"]
    #[inline(always)]
    pub fn is_rcrs0off_1(&self) -> bool {
        *self == RCRS2OFF_A::RCRS0OFF_1
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_2`"]
    #[inline(always)]
    pub fn is_rcrs0off_2(&self) -> bool {
        *self == RCRS2OFF_A::RCRS0OFF_2
    }
}
#[doc = "Write proxy for field `RCRS2OFF`"]
pub struct RCRS2OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS2OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRS2OFF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Contents of this RAM sector are retained in LPM3 and LPM4."]
    #[inline(always)]
    pub fn rcrs0off_0(self) -> &'a mut W {
        self.variant(RCRS2OFF_A::RCRS0OFF_0)
    }
    #[doc = "Turns off this RAM sector in LPM3 and LPM4, re-activates it on wake-up. All data of this RAM sector is lost after wakeup from LPM3 and LPM4. See the device-specific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_1(self) -> &'a mut W {
        self.variant(RCRS2OFF_A::RCRS0OFF_1)
    }
    #[doc = "Turns off this RAM sector entering LPM3 and LPM4, the RAM sector remains off after wake-up. All data of this RAM sector is lost. See the devicespecific data sheet to find the number of available sectors, the address range, and the size of each RAM sector."]
    #[inline(always)]
    pub fn rcrs0off_2(self) -> &'a mut W {
        self.variant(RCRS2OFF_A::RCRS0OFF_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
RAM controller RAM sector 0 off"]
    #[inline(always)]
    pub fn rcrs0off(&self) -> RCRS0OFF_R {
        RCRS0OFF_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RAM controller key. Always reads as 69h. Must be written as 5Ah; any other write is is ignored."]
    #[inline(always)]
    pub fn rckey(&self) -> RCKEY_R {
        RCKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
RAM controller RAM sector 0 off"]
    #[inline(always)]
    pub fn rcrs1off(&self) -> RCRS1OFF_R {
        RCRS1OFF_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
RAM controller RAM sector 3 off"]
    #[inline(always)]
    pub fn rcrs3off(&self) -> RCRS3OFF_R {
        RCRS3OFF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
RAM controller RAM sector 0 off"]
    #[inline(always)]
    pub fn rcrs2off(&self) -> RCRS2OFF_R {
        RCRS2OFF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
RAM controller RAM sector 0 off"]
    #[inline(always)]
    pub fn rcrs0off(&mut self) -> RCRS0OFF_W {
        RCRS0OFF_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
RAM controller key. Always reads as 69h. Must be written as 5Ah; any other write is is ignored."]
    #[inline(always)]
    pub fn rckey(&mut self) -> RCKEY_W {
        RCKEY_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
RAM controller RAM sector 0 off"]
    #[inline(always)]
    pub fn rcrs1off(&mut self) -> RCRS1OFF_W {
        RCRS1OFF_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
RAM controller RAM sector 3 off"]
    #[inline(always)]
    pub fn rcrs3off(&mut self) -> RCRS3OFF_W {
        RCRS3OFF_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
RAM controller RAM sector 0 off"]
    #[inline(always)]
    pub fn rcrs2off(&mut self) -> RCRS2OFF_W {
        RCRS2OFF_W { w: self }
    }
}
