#[doc = "Reader of register LEACMCTL"]
pub type R = crate::R<u32, super::LEACMCTL>;
#[doc = "Writer for register LEACMCTL"]
pub type W = crate::W<u32, super::LEACMCTL>;
#[doc = "Register LEACMCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LEACMCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "0:0\\]
This bit controls access to LEA code memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEACMAE_A {
    #[doc = "0: Code memory access disabled. Accesses to LEA code memory are not possible. LEA does accept commands for execution. Reads to LEA code memory will return zeroes and writes are ignored."]
    LEACMAE_0 = 0,
    #[doc = "1: Code memory access enabled. Accesses to LEA code memory are possible. LEA does not accept commands during this mode (command is ignored). Coprocessor interface accesses by the CPU cause a Coprocessor not available indication."]
    LEACMAE_1 = 1,
}
impl From<LEACMAE_A> for bool {
    #[inline(always)]
    fn from(variant: LEACMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEACMAE`"]
pub type LEACMAE_R = crate::R<bool, LEACMAE_A>;
impl LEACMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEACMAE_A {
        match self.bits {
            false => LEACMAE_A::LEACMAE_0,
            true => LEACMAE_A::LEACMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LEACMAE_0`"]
    #[inline(always)]
    pub fn is_leacmae_0(&self) -> bool {
        *self == LEACMAE_A::LEACMAE_0
    }
    #[doc = "Checks if the value of the field is `LEACMAE_1`"]
    #[inline(always)]
    pub fn is_leacmae_1(&self) -> bool {
        *self == LEACMAE_A::LEACMAE_1
    }
}
#[doc = "Write proxy for field `LEACMAE`"]
pub struct LEACMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Code memory access disabled. Accesses to LEA code memory are not possible. LEA does accept commands for execution. Reads to LEA code memory will return zeroes and writes are ignored."]
    #[inline(always)]
    pub fn leacmae_0(self) -> &'a mut W {
        self.variant(LEACMAE_A::LEACMAE_0)
    }
    #[doc = "Code memory access enabled. Accesses to LEA code memory are possible. LEA does not accept commands during this mode (command is ignored). Coprocessor interface accesses by the CPU cause a Coprocessor not available indication."]
    #[inline(always)]
    pub fn leacmae_1(self) -> &'a mut W {
        self.variant(LEACMAE_A::LEACMAE_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `LEAINC`"]
pub type LEAINC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEAINC`"]
pub struct LEAINC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAINC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `LEADEC`"]
pub type LEADEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEADEC`"]
pub struct LEADEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEADEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "5:4\\]
Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEACROFF_A {
    #[doc = "0: Contents of LEA code RAM are retained in LPM3/LPM4."]
    LEACROFF_0 = 0,
    #[doc = "1: Turns off the LEA code RAM in LPM3/LPM4, re-activates it on wake-up. All data of the code RAM is lost after wakeup from LPM3/LPM4. See the device specific data sheet for presence and size of Code RAM."]
    LEACROFF_1 = 1,
    #[doc = "2: Turns off the code RAM entering LPM3/LPM4, the code RAM sector remains off after wake-up. All data of the code RAM is lost. See the device-specific data sheet for presence and size of Code RAM."]
    LEACROFF_2 = 2,
    #[doc = "3: Reserved (Future: Turns off the code RAM immediately. All data of the Code RAM is lost. See the device-specific data sheet for presence and size of Code RAM.)"]
    LEACROFF_3 = 3,
}
impl From<LEACROFF_A> for u8 {
    #[inline(always)]
    fn from(variant: LEACROFF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEACROFF`"]
pub type LEACROFF_R = crate::R<u8, LEACROFF_A>;
impl LEACROFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEACROFF_A {
        match self.bits {
            0 => LEACROFF_A::LEACROFF_0,
            1 => LEACROFF_A::LEACROFF_1,
            2 => LEACROFF_A::LEACROFF_2,
            3 => LEACROFF_A::LEACROFF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEACROFF_0`"]
    #[inline(always)]
    pub fn is_leacroff_0(&self) -> bool {
        *self == LEACROFF_A::LEACROFF_0
    }
    #[doc = "Checks if the value of the field is `LEACROFF_1`"]
    #[inline(always)]
    pub fn is_leacroff_1(&self) -> bool {
        *self == LEACROFF_A::LEACROFF_1
    }
    #[doc = "Checks if the value of the field is `LEACROFF_2`"]
    #[inline(always)]
    pub fn is_leacroff_2(&self) -> bool {
        *self == LEACROFF_A::LEACROFF_2
    }
    #[doc = "Checks if the value of the field is `LEACROFF_3`"]
    #[inline(always)]
    pub fn is_leacroff_3(&self) -> bool {
        *self == LEACROFF_A::LEACROFF_3
    }
}
#[doc = "Write proxy for field `LEACROFF`"]
pub struct LEACROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACROFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEACROFF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Contents of LEA code RAM are retained in LPM3/LPM4."]
    #[inline(always)]
    pub fn leacroff_0(self) -> &'a mut W {
        self.variant(LEACROFF_A::LEACROFF_0)
    }
    #[doc = "Turns off the LEA code RAM in LPM3/LPM4, re-activates it on wake-up. All data of the code RAM is lost after wakeup from LPM3/LPM4. See the device specific data sheet for presence and size of Code RAM."]
    #[inline(always)]
    pub fn leacroff_1(self) -> &'a mut W {
        self.variant(LEACROFF_A::LEACROFF_1)
    }
    #[doc = "Turns off the code RAM entering LPM3/LPM4, the code RAM sector remains off after wake-up. All data of the code RAM is lost. See the device-specific data sheet for presence and size of Code RAM."]
    #[inline(always)]
    pub fn leacroff_2(self) -> &'a mut W {
        self.variant(LEACROFF_A::LEACROFF_2)
    }
    #[doc = "Reserved (Future: Turns off the code RAM immediately. All data of the Code RAM is lost. See the device-specific data sheet for presence and size of Code RAM.)"]
    #[inline(always)]
    pub fn leacroff_3(self) -> &'a mut W {
        self.variant(LEACROFF_A::LEACROFF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LEACRACTION`"]
pub type LEACRACTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEACRACTION`"]
pub struct LEACRACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACRACTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LEACMAP`"]
pub type LEACMAP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEACMAP`"]
pub struct LEACMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> LEACMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit controls access to LEA code memory."]
    #[inline(always)]
    pub fn leacmae(&self) -> LEACMAE_R {
        LEACMAE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit when set causes the code memory address port field LEACMAP to increment each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double increment is performed when read modify write accesses are done on LEACMDP."]
    #[inline(always)]
    pub fn leainc(&self) -> LEAINC_R {
        LEAINC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit when set causes the code memory address port field LEACMAP to decrement each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double decrement is performed when read modify write accesses are done on LEACMDP."]
    #[inline(always)]
    pub fn leadec(&self) -> LEADEC_R {
        LEADEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0"]
    #[inline(always)]
    pub fn leacroff(&self) -> LEACROFF_R {
        LEACROFF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Code RAM action"]
    #[inline(always)]
    pub fn leacraction(&self) -> LEACRACTION_R {
        LEACRACTION_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
LEA code memory address port. This bit field points to the memory location that is accessible via LEACMDP"]
    #[inline(always)]
    pub fn leacmap(&self) -> LEACMAP_R {
        LEACMAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit controls access to LEA code memory."]
    #[inline(always)]
    pub fn leacmae(&mut self) -> LEACMAE_W {
        LEACMAE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
This bit when set causes the code memory address port field LEACMAP to increment each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double increment is performed when read modify write accesses are done on LEACMDP."]
    #[inline(always)]
    pub fn leainc(&mut self) -> LEAINC_W {
        LEAINC_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
This bit when set causes the code memory address port field LEACMAP to decrement each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double decrement is performed when read modify write accesses are done on LEACMDP."]
    #[inline(always)]
    pub fn leadec(&mut self) -> LEADEC_W {
        LEADEC_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0"]
    #[inline(always)]
    pub fn leacroff(&mut self) -> LEACROFF_W {
        LEACROFF_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Code RAM action"]
    #[inline(always)]
    pub fn leacraction(&mut self) -> LEACRACTION_W {
        LEACRACTION_W { w: self }
    }
    #[doc = "Bits 16:31 - 31:16\\]
LEA code memory address port. This bit field points to the memory location that is accessible via LEACMDP"]
    #[inline(always)]
    pub fn leacmap(&mut self) -> LEACMAP_W {
        LEACMAP_W { w: self }
    }
}
