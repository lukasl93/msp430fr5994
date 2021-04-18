#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uca2ctlw: [u8; 2usize],
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    pub uca2ctlw1: UCA2CTLW1,
    _reserved2: [u8; 2usize],
    _reserved_2_uca2: [u8; 2usize],
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    pub uca2mctlw: UCA2MCTLW,
    _reserved_4_uca2: [u8; 2usize],
    _reserved_5_uca2: [u8; 2usize],
    _reserved_6_uca2: [u8; 2usize],
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub uca2abctl: UCA2ABCTL,
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    pub uca2irctl: UCA2IRCTL,
    _reserved9: [u8; 6usize],
    _reserved_9_uca2: [u8; 2usize],
    _reserved_10_uca2: [u8; 2usize],
    _reserved_11_uca2: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca2ctlw0_spi(&self) -> &UCA2CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA2CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca2ctlw0_spi_mut(&self) -> &mut UCA2CTLW0_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCA2CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca2ctlw0(&self) -> &UCA2CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA2CTLW0) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca2ctlw0_mut(&self) -> &mut UCA2CTLW0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCA2CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca2brw_spi(&self) -> &UCA2BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA2BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca2brw_spi_mut(&self) -> &mut UCA2BRW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA2BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca2brw(&self) -> &UCA2BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA2BRW) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca2brw_mut(&self) -> &mut UCA2BRW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA2BRW) }
    }
    #[doc = "0x0a - UCA2STATW_SPI"]
    #[inline(always)]
    pub fn uca2statw_spi(&self) -> &UCA2STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA2STATW_SPI) }
    }
    #[doc = "0x0a - UCA2STATW_SPI"]
    #[inline(always)]
    pub fn uca2statw_spi_mut(&self) -> &mut UCA2STATW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut UCA2STATW_SPI) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca2statw(&self) -> &UCA2STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA2STATW) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca2statw_mut(&self) -> &mut UCA2STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(10usize) as *mut UCA2STATW) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca2rxbuf_spi(&self) -> &UCA2RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA2RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca2rxbuf_spi_mut(&self) -> &mut UCA2RXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCA2RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca2rxbuf(&self) -> &UCA2RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA2RXBUF) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca2rxbuf_mut(&self) -> &mut UCA2RXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCA2RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca2txbuf_spi(&self) -> &UCA2TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA2TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca2txbuf_spi_mut(&self) -> &mut UCA2TXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCA2TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca2txbuf(&self) -> &UCA2TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA2TXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca2txbuf_mut(&self) -> &mut UCA2TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCA2TXBUF) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca2ie_spi(&self) -> &UCA2IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA2IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca2ie_spi_mut(&self) -> &mut UCA2IE_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut UCA2IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca2ie(&self) -> &UCA2IE {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA2IE) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca2ie_mut(&self) -> &mut UCA2IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(26usize) as *mut UCA2IE) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca2ifg_spi(&self) -> &UCA2IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA2IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca2ifg_spi_mut(&self) -> &mut UCA2IFG_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut UCA2IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca2ifg(&self) -> &UCA2IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA2IFG) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca2ifg_mut(&self) -> &mut UCA2IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut UCA2IFG) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca2iv_spi(&self) -> &UCA2IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA2IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca2iv_spi_mut(&self) -> &mut UCA2IV_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(30usize) as *mut UCA2IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca2iv(&self) -> &UCA2IV {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA2IV) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca2iv_mut(&self) -> &mut UCA2IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(30usize) as *mut UCA2IV) }
    }
}
#[doc = "eUSCI_Ax Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2ctlw0](uca2ctlw0) module"]
pub type UCA2CTLW0 = crate::Reg<u16, _UCA2CTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2CTLW0;
#[doc = "`read()` method returns [uca2ctlw0::R](uca2ctlw0::R) reader structure"]
impl crate::Readable for UCA2CTLW0 {}
#[doc = "`write(|w| ..)` method takes [uca2ctlw0::W](uca2ctlw0::W) writer structure"]
impl crate::Writable for UCA2CTLW0 {}
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca2ctlw0;
#[doc = "eUSCI_Ax Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2ctlw0_spi](uca2ctlw0_spi) module"]
pub type UCA2CTLW0_SPI = crate::Reg<u16, _UCA2CTLW0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2CTLW0_SPI;
#[doc = "`read()` method returns [uca2ctlw0_spi::R](uca2ctlw0_spi::R) reader structure"]
impl crate::Readable for UCA2CTLW0_SPI {}
#[doc = "`write(|w| ..)` method takes [uca2ctlw0_spi::W](uca2ctlw0_spi::W) writer structure"]
impl crate::Writable for UCA2CTLW0_SPI {}
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca2ctlw0_spi;
#[doc = "eUSCI_Ax Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2ctlw1](uca2ctlw1) module"]
pub type UCA2CTLW1 = crate::Reg<u16, _UCA2CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2CTLW1;
#[doc = "`read()` method returns [uca2ctlw1::R](uca2ctlw1::R) reader structure"]
impl crate::Readable for UCA2CTLW1 {}
#[doc = "`write(|w| ..)` method takes [uca2ctlw1::W](uca2ctlw1::W) writer structure"]
impl crate::Writable for UCA2CTLW1 {}
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod uca2ctlw1;
#[doc = "eUSCI_Ax Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2brw](uca2brw) module"]
pub type UCA2BRW = crate::Reg<u16, _UCA2BRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2BRW;
#[doc = "`read()` method returns [uca2brw::R](uca2brw::R) reader structure"]
impl crate::Readable for UCA2BRW {}
#[doc = "`write(|w| ..)` method takes [uca2brw::W](uca2brw::W) writer structure"]
impl crate::Writable for UCA2BRW {}
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca2brw;
#[doc = "eUSCI_Ax Bit Rate Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2brw_spi](uca2brw_spi) module"]
pub type UCA2BRW_SPI = crate::Reg<u16, _UCA2BRW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2BRW_SPI;
#[doc = "`read()` method returns [uca2brw_spi::R](uca2brw_spi::R) reader structure"]
impl crate::Readable for UCA2BRW_SPI {}
#[doc = "`write(|w| ..)` method takes [uca2brw_spi::W](uca2brw_spi::W) writer structure"]
impl crate::Writable for UCA2BRW_SPI {}
#[doc = "eUSCI_Ax Bit Rate Control Register 1"]
pub mod uca2brw_spi;
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2mctlw](uca2mctlw) module"]
pub type UCA2MCTLW = crate::Reg<u16, _UCA2MCTLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2MCTLW;
#[doc = "`read()` method returns [uca2mctlw::R](uca2mctlw::R) reader structure"]
impl crate::Readable for UCA2MCTLW {}
#[doc = "`write(|w| ..)` method takes [uca2mctlw::W](uca2mctlw::W) writer structure"]
impl crate::Writable for UCA2MCTLW {}
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca2mctlw;
#[doc = "eUSCI_Ax Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2statw](uca2statw) module"]
pub type UCA2STATW = crate::Reg<u16, _UCA2STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2STATW;
#[doc = "`read()` method returns [uca2statw::R](uca2statw::R) reader structure"]
impl crate::Readable for UCA2STATW {}
#[doc = "`write(|w| ..)` method takes [uca2statw::W](uca2statw::W) writer structure"]
impl crate::Writable for UCA2STATW {}
#[doc = "eUSCI_Ax Status Register"]
pub mod uca2statw;
#[doc = "UCA2STATW_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2statw_spi](uca2statw_spi) module"]
pub type UCA2STATW_SPI = crate::Reg<u16, _UCA2STATW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2STATW_SPI;
#[doc = "`read()` method returns [uca2statw_spi::R](uca2statw_spi::R) reader structure"]
impl crate::Readable for UCA2STATW_SPI {}
#[doc = "`write(|w| ..)` method takes [uca2statw_spi::W](uca2statw_spi::W) writer structure"]
impl crate::Writable for UCA2STATW_SPI {}
#[doc = "UCA2STATW_SPI"]
pub mod uca2statw_spi;
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2rxbuf](uca2rxbuf) module"]
pub type UCA2RXBUF = crate::Reg<u16, _UCA2RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2RXBUF;
#[doc = "`read()` method returns [uca2rxbuf::R](uca2rxbuf::R) reader structure"]
impl crate::Readable for UCA2RXBUF {}
#[doc = "`write(|w| ..)` method takes [uca2rxbuf::W](uca2rxbuf::W) writer structure"]
impl crate::Writable for UCA2RXBUF {}
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca2rxbuf;
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2rxbuf_spi](uca2rxbuf_spi) module"]
pub type UCA2RXBUF_SPI = crate::Reg<u16, _UCA2RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2RXBUF_SPI;
#[doc = "`read()` method returns [uca2rxbuf_spi::R](uca2rxbuf_spi::R) reader structure"]
impl crate::Readable for UCA2RXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca2rxbuf_spi::W](uca2rxbuf_spi::W) writer structure"]
impl crate::Writable for UCA2RXBUF_SPI {}
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca2rxbuf_spi;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2txbuf](uca2txbuf) module"]
pub type UCA2TXBUF = crate::Reg<u16, _UCA2TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2TXBUF;
#[doc = "`read()` method returns [uca2txbuf::R](uca2txbuf::R) reader structure"]
impl crate::Readable for UCA2TXBUF {}
#[doc = "`write(|w| ..)` method takes [uca2txbuf::W](uca2txbuf::W) writer structure"]
impl crate::Writable for UCA2TXBUF {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca2txbuf;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2txbuf_spi](uca2txbuf_spi) module"]
pub type UCA2TXBUF_SPI = crate::Reg<u16, _UCA2TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2TXBUF_SPI;
#[doc = "`read()` method returns [uca2txbuf_spi::R](uca2txbuf_spi::R) reader structure"]
impl crate::Readable for UCA2TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [uca2txbuf_spi::W](uca2txbuf_spi::W) writer structure"]
impl crate::Writable for UCA2TXBUF_SPI {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca2txbuf_spi;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2abctl](uca2abctl) module"]
pub type UCA2ABCTL = crate::Reg<u16, _UCA2ABCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2ABCTL;
#[doc = "`read()` method returns [uca2abctl::R](uca2abctl::R) reader structure"]
impl crate::Readable for UCA2ABCTL {}
#[doc = "`write(|w| ..)` method takes [uca2abctl::W](uca2abctl::W) writer structure"]
impl crate::Writable for UCA2ABCTL {}
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca2abctl;
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2irctl](uca2irctl) module"]
pub type UCA2IRCTL = crate::Reg<u16, _UCA2IRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2IRCTL;
#[doc = "`read()` method returns [uca2irctl::R](uca2irctl::R) reader structure"]
impl crate::Readable for UCA2IRCTL {}
#[doc = "`write(|w| ..)` method takes [uca2irctl::W](uca2irctl::W) writer structure"]
impl crate::Writable for UCA2IRCTL {}
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca2irctl;
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2ie](uca2ie) module"]
pub type UCA2IE = crate::Reg<u16, _UCA2IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2IE;
#[doc = "`read()` method returns [uca2ie::R](uca2ie::R) reader structure"]
impl crate::Readable for UCA2IE {}
#[doc = "`write(|w| ..)` method takes [uca2ie::W](uca2ie::W) writer structure"]
impl crate::Writable for UCA2IE {}
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca2ie;
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2ie_spi](uca2ie_spi) module"]
pub type UCA2IE_SPI = crate::Reg<u16, _UCA2IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2IE_SPI;
#[doc = "`read()` method returns [uca2ie_spi::R](uca2ie_spi::R) reader structure"]
impl crate::Readable for UCA2IE_SPI {}
#[doc = "`write(|w| ..)` method takes [uca2ie_spi::W](uca2ie_spi::W) writer structure"]
impl crate::Writable for UCA2IE_SPI {}
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca2ie_spi;
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2ifg](uca2ifg) module"]
pub type UCA2IFG = crate::Reg<u16, _UCA2IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2IFG;
#[doc = "`read()` method returns [uca2ifg::R](uca2ifg::R) reader structure"]
impl crate::Readable for UCA2IFG {}
#[doc = "`write(|w| ..)` method takes [uca2ifg::W](uca2ifg::W) writer structure"]
impl crate::Writable for UCA2IFG {}
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca2ifg;
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2ifg_spi](uca2ifg_spi) module"]
pub type UCA2IFG_SPI = crate::Reg<u16, _UCA2IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2IFG_SPI;
#[doc = "`read()` method returns [uca2ifg_spi::R](uca2ifg_spi::R) reader structure"]
impl crate::Readable for UCA2IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [uca2ifg_spi::W](uca2ifg_spi::W) writer structure"]
impl crate::Writable for UCA2IFG_SPI {}
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca2ifg_spi;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2iv](uca2iv) module"]
pub type UCA2IV = crate::Reg<u16, _UCA2IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2IV;
#[doc = "`read()` method returns [uca2iv::R](uca2iv::R) reader structure"]
impl crate::Readable for UCA2IV {}
#[doc = "`write(|w| ..)` method takes [uca2iv::W](uca2iv::W) writer structure"]
impl crate::Writable for UCA2IV {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca2iv;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2iv_spi](uca2iv_spi) module"]
pub type UCA2IV_SPI = crate::Reg<u16, _UCA2IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA2IV_SPI;
#[doc = "`read()` method returns [uca2iv_spi::R](uca2iv_spi::R) reader structure"]
impl crate::Readable for UCA2IV_SPI {}
#[doc = "`write(|w| ..)` method takes [uca2iv_spi::W](uca2iv_spi::W) writer structure"]
impl crate::Writable for UCA2IV_SPI {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca2iv_spi;
