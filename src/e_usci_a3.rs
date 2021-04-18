#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uca3ctlw: [u8; 2usize],
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    pub uca3ctlw1: UCA3CTLW1,
    _reserved2: [u8; 2usize],
    _reserved_2_uca3: [u8; 2usize],
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    pub uca3mctlw: UCA3MCTLW,
    _reserved_4_uca3: [u8; 2usize],
    _reserved_5_uca3: [u8; 2usize],
    _reserved_6_uca3: [u8; 2usize],
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub uca3abctl: UCA3ABCTL,
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    pub uca3irctl: UCA3IRCTL,
    _reserved9: [u8; 6usize],
    _reserved_9_uca3: [u8; 2usize],
    _reserved_10_uca3: [u8; 2usize],
    _reserved_11_uca3: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca3ctlw0_spi(&self) -> &UCA3CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA3CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca3ctlw0_spi_mut(&self) -> &mut UCA3CTLW0_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCA3CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca3ctlw0(&self) -> &UCA3CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA3CTLW0) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca3ctlw0_mut(&self) -> &mut UCA3CTLW0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCA3CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca3brw_spi(&self) -> &UCA3BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA3BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca3brw_spi_mut(&self) -> &mut UCA3BRW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA3BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca3brw(&self) -> &UCA3BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA3BRW) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca3brw_mut(&self) -> &mut UCA3BRW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA3BRW) }
    }
    #[doc = "0x0a - UCA3STATW_SPI"]
    #[inline(always)]
    pub fn uca3statw_spi(&self) -> &UCA3STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA3STATW_SPI) }
    }
    #[doc = "0x0a - UCA3STATW_SPI"]
    #[inline(always)]
    pub fn uca3statw_spi_mut(&self) -> &mut UCA3STATW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut UCA3STATW_SPI) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca3statw(&self) -> &UCA3STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA3STATW) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca3statw_mut(&self) -> &mut UCA3STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut UCA3STATW) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca3rxbuf_spi(&self) -> &UCA3RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA3RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca3rxbuf_spi_mut(&self) -> &mut UCA3RXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCA3RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca3rxbuf(&self) -> &UCA3RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA3RXBUF) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca3rxbuf_mut(&self) -> &mut UCA3RXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCA3RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca3txbuf_spi(&self) -> &UCA3TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA3TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca3txbuf_spi_mut(&self) -> &mut UCA3TXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCA3TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca3txbuf(&self) -> &UCA3TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA3TXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca3txbuf_mut(&self) -> &mut UCA3TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCA3TXBUF) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca3ie_spi(&self) -> &UCA3IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA3IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca3ie_spi_mut(&self) -> &mut UCA3IE_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut UCA3IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca3ie(&self) -> &UCA3IE {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA3IE) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca3ie_mut(&self) -> &mut UCA3IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut UCA3IE) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca3ifg_spi(&self) -> &UCA3IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA3IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca3ifg_spi_mut(&self) -> &mut UCA3IFG_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut UCA3IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca3ifg(&self) -> &UCA3IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA3IFG) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca3ifg_mut(&self) -> &mut UCA3IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut UCA3IFG) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca3iv_spi(&self) -> &UCA3IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA3IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca3iv_spi_mut(&self) -> &mut UCA3IV_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(30usize) as *mut UCA3IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca3iv(&self) -> &UCA3IV {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA3IV) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca3iv_mut(&self) -> &mut UCA3IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(30usize) as *mut UCA3IV) }
    }
}
#[doc = "eUSCI_Ax Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3ctlw0](uca3ctlw0) module"]
pub type UCA3CTLW0 = crate::Reg<u16, _UCA3CTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3CTLW0;
#[doc = "`read()` method returns [uca3ctlw0::R](uca3ctlw0::R) reader structure"]
impl crate::Readable for UCA3CTLW0 {}
#[doc = "`write(|w| ..)` method takes [uca3ctlw0::W](uca3ctlw0::W) writer structure"]
impl crate::Writable for UCA3CTLW0 {}
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca3ctlw0;
#[doc = "eUSCI_Ax Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3ctlw0_spi](uca3ctlw0_spi) module"]
pub type UCA3CTLW0_SPI = crate::Reg<u16, _UCA3CTLW0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3CTLW0_SPI;
#[doc = "`read()` method returns [uca3ctlw0_spi::R](uca3ctlw0_spi::R) reader structure"]
impl crate::Readable for UCA3CTLW0_SPI {}
#[doc = "`write(|w| ..)` method takes [uca3ctlw0_spi::W](uca3ctlw0_spi::W) writer structure"]
impl crate::Writable for UCA3CTLW0_SPI {}
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca3ctlw0_spi;
#[doc = "eUSCI_Ax Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3ctlw1](uca3ctlw1) module"]
pub type UCA3CTLW1 = crate::Reg<u16, _UCA3CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3CTLW1;
#[doc = "`read()` method returns [uca3ctlw1::R](uca3ctlw1::R) reader structure"]
impl crate::Readable for UCA3CTLW1 {}
#[doc = "`write(|w| ..)` method takes [uca3ctlw1::W](uca3ctlw1::W) writer structure"]
impl crate::Writable for UCA3CTLW1 {}
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod uca3ctlw1;
#[doc = "eUSCI_Ax Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3brw](uca3brw) module"]
pub type UCA3BRW = crate::Reg<u16, _UCA3BRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3BRW;
#[doc = "`read()` method returns [uca3brw::R](uca3brw::R) reader structure"]
impl crate::Readable for UCA3BRW {}
#[doc = "`write(|w| ..)` method takes [uca3brw::W](uca3brw::W) writer structure"]
impl crate::Writable for UCA3BRW {}
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca3brw;
#[doc = "eUSCI_Ax Bit Rate Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3brw_spi](uca3brw_spi) module"]
pub type UCA3BRW_SPI = crate::Reg<u16, _UCA3BRW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3BRW_SPI;
#[doc = "`read()` method returns [uca3brw_spi::R](uca3brw_spi::R) reader structure"]
impl crate::Readable for UCA3BRW_SPI {}
#[doc = "`write(|w| ..)` method takes [uca3brw_spi::W](uca3brw_spi::W) writer structure"]
impl crate::Writable for UCA3BRW_SPI {}
#[doc = "eUSCI_Ax Bit Rate Control Register 1"]
pub mod uca3brw_spi;
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3mctlw](uca3mctlw) module"]
pub type UCA3MCTLW = crate::Reg<u16, _UCA3MCTLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3MCTLW;
#[doc = "`read()` method returns [uca3mctlw::R](uca3mctlw::R) reader structure"]
impl crate::Readable for UCA3MCTLW {}
#[doc = "`write(|w| ..)` method takes [uca3mctlw::W](uca3mctlw::W) writer structure"]
impl crate::Writable for UCA3MCTLW {}
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca3mctlw;
#[doc = "eUSCI_Ax Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3statw](uca3statw) module"]
pub type UCA3STATW = crate::Reg<u16, _UCA3STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3STATW;
#[doc = "`read()` method returns [uca3statw::R](uca3statw::R) reader structure"]
impl crate::Readable for UCA3STATW {}
#[doc = "`write(|w| ..)` method takes [uca3statw::W](uca3statw::W) writer structure"]
impl crate::Writable for UCA3STATW {}
#[doc = "eUSCI_Ax Status Register"]
pub mod uca3statw;
#[doc = "UCA3STATW_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3statw_spi](uca3statw_spi) module"]
pub type UCA3STATW_SPI = crate::Reg<u16, _UCA3STATW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3STATW_SPI;
#[doc = "`read()` method returns [uca3statw_spi::R](uca3statw_spi::R) reader structure"]
impl crate::Readable for UCA3STATW_SPI {}
#[doc = "`write(|w| ..)` method takes [uca3statw_spi::W](uca3statw_spi::W) writer structure"]
impl crate::Writable for UCA3STATW_SPI {}
#[doc = "UCA3STATW_SPI"]
pub mod uca3statw_spi;
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3rxbuf](uca3rxbuf) module"]
pub type UCA3RXBUF = crate::Reg<u16, _UCA3RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3RXBUF;
#[doc = "`read()` method returns [uca3rxbuf::R](uca3rxbuf::R) reader structure"]
impl crate::Readable for UCA3RXBUF {}
#[doc = "`write(|w| ..)` method takes [uca3rxbuf::W](uca3rxbuf::W) writer structure"]
impl crate::Writable for UCA3RXBUF {}
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca3rxbuf;
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3rxbuf_spi](uca3rxbuf_spi) module"]
pub type UCA3RXBUF_SPI = crate::Reg<u16, _UCA3RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3RXBUF_SPI;
#[doc = "`read()` method returns [uca3rxbuf_spi::R](uca3rxbuf_spi::R) reader structure"]
impl crate::Readable for UCA3RXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca3rxbuf_spi::W](uca3rxbuf_spi::W) writer structure"]
impl crate::Writable for UCA3RXBUF_SPI {}
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca3rxbuf_spi;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3txbuf](uca3txbuf) module"]
pub type UCA3TXBUF = crate::Reg<u16, _UCA3TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3TXBUF;
#[doc = "`read()` method returns [uca3txbuf::R](uca3txbuf::R) reader structure"]
impl crate::Readable for UCA3TXBUF {}
#[doc = "`write(|w| ..)` method takes [uca3txbuf::W](uca3txbuf::W) writer structure"]
impl crate::Writable for UCA3TXBUF {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca3txbuf;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3txbuf_spi](uca3txbuf_spi) module"]
pub type UCA3TXBUF_SPI = crate::Reg<u16, _UCA3TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3TXBUF_SPI;
#[doc = "`read()` method returns [uca3txbuf_spi::R](uca3txbuf_spi::R) reader structure"]
impl crate::Readable for UCA3TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca3txbuf_spi::W](uca3txbuf_spi::W) writer structure"]
impl crate::Writable for UCA3TXBUF_SPI {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca3txbuf_spi;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3abctl](uca3abctl) module"]
pub type UCA3ABCTL = crate::Reg<u16, _UCA3ABCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3ABCTL;
#[doc = "`read()` method returns [uca3abctl::R](uca3abctl::R) reader structure"]
impl crate::Readable for UCA3ABCTL {}
#[doc = "`write(|w| ..)` method takes [uca3abctl::W](uca3abctl::W) writer structure"]
impl crate::Writable for UCA3ABCTL {}
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca3abctl;
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3irctl](uca3irctl) module"]
pub type UCA3IRCTL = crate::Reg<u16, _UCA3IRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3IRCTL;
#[doc = "`read()` method returns [uca3irctl::R](uca3irctl::R) reader structure"]
impl crate::Readable for UCA3IRCTL {}
#[doc = "`write(|w| ..)` method takes [uca3irctl::W](uca3irctl::W) writer structure"]
impl crate::Writable for UCA3IRCTL {}
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca3irctl;
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3ie](uca3ie) module"]
pub type UCA3IE = crate::Reg<u16, _UCA3IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3IE;
#[doc = "`read()` method returns [uca3ie::R](uca3ie::R) reader structure"]
impl crate::Readable for UCA3IE {}
#[doc = "`write(|w| ..)` method takes [uca3ie::W](uca3ie::W) writer structure"]
impl crate::Writable for UCA3IE {}
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca3ie;
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3ie_spi](uca3ie_spi) module"]
pub type UCA3IE_SPI = crate::Reg<u16, _UCA3IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3IE_SPI;
#[doc = "`read()` method returns [uca3ie_spi::R](uca3ie_spi::R) reader structure"]
impl crate::Readable for UCA3IE_SPI {}
#[doc = "`write(|w| ..)` method takes [uca3ie_spi::W](uca3ie_spi::W) writer structure"]
impl crate::Writable for UCA3IE_SPI {}
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca3ie_spi;
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3ifg](uca3ifg) module"]
pub type UCA3IFG = crate::Reg<u16, _UCA3IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3IFG;
#[doc = "`read()` method returns [uca3ifg::R](uca3ifg::R) reader structure"]
impl crate::Readable for UCA3IFG {}
#[doc = "`write(|w| ..)` method takes [uca3ifg::W](uca3ifg::W) writer structure"]
impl crate::Writable for UCA3IFG {}
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca3ifg;
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3ifg_spi](uca3ifg_spi) module"]
pub type UCA3IFG_SPI = crate::Reg<u16, _UCA3IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3IFG_SPI;
#[doc = "`read()` method returns [uca3ifg_spi::R](uca3ifg_spi::R) reader structure"]
impl crate::Readable for UCA3IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [uca3ifg_spi::W](uca3ifg_spi::W) writer structure"]
impl crate::Writable for UCA3IFG_SPI {}
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca3ifg_spi;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3iv](uca3iv) module"]
pub type UCA3IV = crate::Reg<u16, _UCA3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3IV;
#[doc = "`read()` method returns [uca3iv::R](uca3iv::R) reader structure"]
impl crate::Readable for UCA3IV {}
#[doc = "`write(|w| ..)` method takes [uca3iv::W](uca3iv::W) writer structure"]
impl crate::Writable for UCA3IV {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca3iv;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca3iv_spi](uca3iv_spi) module"]
pub type UCA3IV_SPI = crate::Reg<u16, _UCA3IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA3IV_SPI;
#[doc = "`read()` method returns [uca3iv_spi::R](uca3iv_spi::R) reader structure"]
impl crate::Readable for UCA3IV_SPI {}
#[doc = "`write(|w| ..)` method takes [uca3iv_spi::W](uca3iv_spi::W) writer structure"]
impl crate::Writable for UCA3IV_SPI {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca3iv_spi;
