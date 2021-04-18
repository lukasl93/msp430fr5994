#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ucb2ctlw: [u8; 2usize],
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    pub ucb2ctlw1: UCB2CTLW1,
    _reserved2: [u8; 2usize],
    _reserved_2_ucb2: [u8; 2usize],
    _reserved_3_ucb2: [u8; 2usize],
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    pub ucb2tbcnt: UCB2TBCNT,
    _reserved_5_ucb2: [u8; 2usize],
    _reserved_6_ucb2: [u8; 2usize],
    _reserved7: [u8; 4usize],
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    pub ucb2i2coa0: UCB2I2COA0,
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    pub ucb2i2coa1: UCB2I2COA1,
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    pub ucb2i2coa2: UCB2I2COA2,
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    pub ucb2i2coa3: UCB2I2COA3,
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    pub ucb2addrx: UCB2ADDRX,
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    pub ucb2addmask: UCB2ADDMASK,
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    pub ucb2i2csa: UCB2I2CSA,
    _reserved14: [u8; 8usize],
    _reserved_14_ucb2: [u8; 2usize],
    _reserved_15_ucb2: [u8; 2usize],
    _reserved_16_ucb2: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb2ctlw0_spi(&self) -> &UCB2CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB2CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb2ctlw0_spi_mut(&self) -> &mut UCB2CTLW0_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB2CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb2ctlw0(&self) -> &UCB2CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB2CTLW0) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb2ctlw0_mut(&self) -> &mut UCB2CTLW0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB2CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb2brw_spi(&self) -> &UCB2BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB2BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb2brw_spi_mut(&self) -> &mut UCB2BRW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB2BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb2brw(&self) -> &UCB2BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB2BRW) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb2brw_mut(&self) -> &mut UCB2BRW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB2BRW) }
    }
    #[doc = "0x08 - UCB2STATW_SPI"]
    #[inline(always)]
    pub fn ucb2statw_spi(&self) -> &UCB2STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB2STATW_SPI) }
    }
    #[doc = "0x08 - UCB2STATW_SPI"]
    #[inline(always)]
    pub fn ucb2statw_spi_mut(&self) -> &mut UCB2STATW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut UCB2STATW_SPI) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb2statw(&self) -> &UCB2STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB2STATW) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb2statw_mut(&self) -> &mut UCB2STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut UCB2STATW) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb2rxbuf_spi(&self) -> &UCB2RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB2RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb2rxbuf_spi_mut(&self) -> &mut UCB2RXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCB2RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb2rxbuf(&self) -> &UCB2RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB2RXBUF) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb2rxbuf_mut(&self) -> &mut UCB2RXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCB2RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb2txbuf_spi(&self) -> &UCB2TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB2TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb2txbuf_spi_mut(&self) -> &mut UCB2TXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCB2TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb2txbuf(&self) -> &UCB2TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB2TXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb2txbuf_mut(&self) -> &mut UCB2TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCB2TXBUF) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb2ie_spi(&self) -> &UCB2IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB2IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb2ie_spi_mut(&self) -> &mut UCB2IE_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB2IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb2ie(&self) -> &UCB2IE {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB2IE) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb2ie_mut(&self) -> &mut UCB2IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB2IE) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb2ifg_spi(&self) -> &UCB2IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB2IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb2ifg_spi_mut(&self) -> &mut UCB2IFG_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB2IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb2ifg(&self) -> &UCB2IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB2IFG) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb2ifg_mut(&self) -> &mut UCB2IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB2IFG) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb2iv_spi(&self) -> &UCB2IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB2IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb2iv_spi_mut(&self) -> &mut UCB2IV_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(46usize) as *mut UCB2IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb2iv(&self) -> &UCB2IV {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB2IV) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb2iv_mut(&self) -> &mut UCB2IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(46usize) as *mut UCB2IV) }
    }
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2ctlw0](ucb2ctlw0) module"]
pub type UCB2CTLW0 = crate::Reg<u16, _UCB2CTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2CTLW0;
#[doc = "`read()` method returns [ucb2ctlw0::R](ucb2ctlw0::R) reader structure"]
impl crate::Readable for UCB2CTLW0 {}
#[doc = "`write(|w| ..)` method takes [ucb2ctlw0::W](ucb2ctlw0::W) writer structure"]
impl crate::Writable for UCB2CTLW0 {}
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb2ctlw0;
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2ctlw0_spi](ucb2ctlw0_spi) module"]
pub type UCB2CTLW0_SPI = crate::Reg<u16, _UCB2CTLW0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2CTLW0_SPI;
#[doc = "`read()` method returns [ucb2ctlw0_spi::R](ucb2ctlw0_spi::R) reader structure"]
impl crate::Readable for UCB2CTLW0_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb2ctlw0_spi::W](ucb2ctlw0_spi::W) writer structure"]
impl crate::Writable for UCB2CTLW0_SPI {}
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb2ctlw0_spi;
#[doc = "eUSCI_Bx Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2ctlw1](ucb2ctlw1) module"]
pub type UCB2CTLW1 = crate::Reg<u16, _UCB2CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2CTLW1;
#[doc = "`read()` method returns [ucb2ctlw1::R](ucb2ctlw1::R) reader structure"]
impl crate::Readable for UCB2CTLW1 {}
#[doc = "`write(|w| ..)` method takes [ucb2ctlw1::W](ucb2ctlw1::W) writer structure"]
impl crate::Writable for UCB2CTLW1 {}
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb2ctlw1;
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2brw](ucb2brw) module"]
pub type UCB2BRW = crate::Reg<u16, _UCB2BRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2BRW;
#[doc = "`read()` method returns [ucb2brw::R](ucb2brw::R) reader structure"]
impl crate::Readable for UCB2BRW {}
#[doc = "`write(|w| ..)` method takes [ucb2brw::W](ucb2brw::W) writer structure"]
impl crate::Writable for UCB2BRW {}
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb2brw;
#[doc = "eUSCI_Bx Bit Rate Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2brw_spi](ucb2brw_spi) module"]
pub type UCB2BRW_SPI = crate::Reg<u16, _UCB2BRW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2BRW_SPI;
#[doc = "`read()` method returns [ucb2brw_spi::R](ucb2brw_spi::R) reader structure"]
impl crate::Readable for UCB2BRW_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb2brw_spi::W](ucb2brw_spi::W) writer structure"]
impl crate::Writable for UCB2BRW_SPI {}
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb2brw_spi;
#[doc = "eUSCI_Bx Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2statw](ucb2statw) module"]
pub type UCB2STATW = crate::Reg<u16, _UCB2STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2STATW;
#[doc = "`read()` method returns [ucb2statw::R](ucb2statw::R) reader structure"]
impl crate::Readable for UCB2STATW {}
#[doc = "`write(|w| ..)` method takes [ucb2statw::W](ucb2statw::W) writer structure"]
impl crate::Writable for UCB2STATW {}
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb2statw;
#[doc = "UCB2STATW_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2statw_spi](ucb2statw_spi) module"]
pub type UCB2STATW_SPI = crate::Reg<u16, _UCB2STATW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2STATW_SPI;
#[doc = "`read()` method returns [ucb2statw_spi::R](ucb2statw_spi::R) reader structure"]
impl crate::Readable for UCB2STATW_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb2statw_spi::W](ucb2statw_spi::W) writer structure"]
impl crate::Writable for UCB2STATW_SPI {}
#[doc = "UCB2STATW_SPI"]
pub mod ucb2statw_spi;
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2tbcnt](ucb2tbcnt) module"]
pub type UCB2TBCNT = crate::Reg<u16, _UCB2TBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2TBCNT;
#[doc = "`read()` method returns [ucb2tbcnt::R](ucb2tbcnt::R) reader structure"]
impl crate::Readable for UCB2TBCNT {}
#[doc = "`write(|w| ..)` method takes [ucb2tbcnt::W](ucb2tbcnt::W) writer structure"]
impl crate::Writable for UCB2TBCNT {}
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb2tbcnt;
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2rxbuf](ucb2rxbuf) module"]
pub type UCB2RXBUF = crate::Reg<u16, _UCB2RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2RXBUF;
#[doc = "`read()` method returns [ucb2rxbuf::R](ucb2rxbuf::R) reader structure"]
impl crate::Readable for UCB2RXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb2rxbuf::W](ucb2rxbuf::W) writer structure"]
impl crate::Writable for UCB2RXBUF {}
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb2rxbuf;
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2rxbuf_spi](ucb2rxbuf_spi) module"]
pub type UCB2RXBUF_SPI = crate::Reg<u16, _UCB2RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2RXBUF_SPI;
#[doc = "`read()` method returns [ucb2rxbuf_spi::R](ucb2rxbuf_spi::R) reader structure"]
impl crate::Readable for UCB2RXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb2rxbuf_spi::W](ucb2rxbuf_spi::W) writer structure"]
impl crate::Writable for UCB2RXBUF_SPI {}
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb2rxbuf_spi;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2txbuf](ucb2txbuf) module"]
pub type UCB2TXBUF = crate::Reg<u16, _UCB2TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2TXBUF;
#[doc = "`read()` method returns [ucb2txbuf::R](ucb2txbuf::R) reader structure"]
impl crate::Readable for UCB2TXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb2txbuf::W](ucb2txbuf::W) writer structure"]
impl crate::Writable for UCB2TXBUF {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb2txbuf;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2txbuf_spi](ucb2txbuf_spi) module"]
pub type UCB2TXBUF_SPI = crate::Reg<u16, _UCB2TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2TXBUF_SPI;
#[doc = "`read()` method returns [ucb2txbuf_spi::R](ucb2txbuf_spi::R) reader structure"]
impl crate::Readable for UCB2TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb2txbuf_spi::W](ucb2txbuf_spi::W) writer structure"]
impl crate::Writable for UCB2TXBUF_SPI {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb2txbuf_spi;
#[doc = "eUSCI_Bx I2C Own Address 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2i2coa0](ucb2i2coa0) module"]
pub type UCB2I2COA0 = crate::Reg<u16, _UCB2I2COA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2I2COA0;
#[doc = "`read()` method returns [ucb2i2coa0::R](ucb2i2coa0::R) reader structure"]
impl crate::Readable for UCB2I2COA0 {}
#[doc = "`write(|w| ..)` method takes [ucb2i2coa0::W](ucb2i2coa0::W) writer structure"]
impl crate::Writable for UCB2I2COA0 {}
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb2i2coa0;
#[doc = "eUSCI_Bx I2C Own Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2i2coa1](ucb2i2coa1) module"]
pub type UCB2I2COA1 = crate::Reg<u16, _UCB2I2COA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2I2COA1;
#[doc = "`read()` method returns [ucb2i2coa1::R](ucb2i2coa1::R) reader structure"]
impl crate::Readable for UCB2I2COA1 {}
#[doc = "`write(|w| ..)` method takes [ucb2i2coa1::W](ucb2i2coa1::W) writer structure"]
impl crate::Writable for UCB2I2COA1 {}
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb2i2coa1;
#[doc = "eUSCI_Bx I2C Own Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2i2coa2](ucb2i2coa2) module"]
pub type UCB2I2COA2 = crate::Reg<u16, _UCB2I2COA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2I2COA2;
#[doc = "`read()` method returns [ucb2i2coa2::R](ucb2i2coa2::R) reader structure"]
impl crate::Readable for UCB2I2COA2 {}
#[doc = "`write(|w| ..)` method takes [ucb2i2coa2::W](ucb2i2coa2::W) writer structure"]
impl crate::Writable for UCB2I2COA2 {}
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb2i2coa2;
#[doc = "eUSCI_Bx I2C Own Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2i2coa3](ucb2i2coa3) module"]
pub type UCB2I2COA3 = crate::Reg<u16, _UCB2I2COA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2I2COA3;
#[doc = "`read()` method returns [ucb2i2coa3::R](ucb2i2coa3::R) reader structure"]
impl crate::Readable for UCB2I2COA3 {}
#[doc = "`write(|w| ..)` method takes [ucb2i2coa3::W](ucb2i2coa3::W) writer structure"]
impl crate::Writable for UCB2I2COA3 {}
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb2i2coa3;
#[doc = "eUSCI_Bx I2C Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2addrx](ucb2addrx) module"]
pub type UCB2ADDRX = crate::Reg<u16, _UCB2ADDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2ADDRX;
#[doc = "`read()` method returns [ucb2addrx::R](ucb2addrx::R) reader structure"]
impl crate::Readable for UCB2ADDRX {}
#[doc = "`write(|w| ..)` method takes [ucb2addrx::W](ucb2addrx::W) writer structure"]
impl crate::Writable for UCB2ADDRX {}
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb2addrx;
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2addmask](ucb2addmask) module"]
pub type UCB2ADDMASK = crate::Reg<u16, _UCB2ADDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2ADDMASK;
#[doc = "`read()` method returns [ucb2addmask::R](ucb2addmask::R) reader structure"]
impl crate::Readable for UCB2ADDMASK {}
#[doc = "`write(|w| ..)` method takes [ucb2addmask::W](ucb2addmask::W) writer structure"]
impl crate::Writable for UCB2ADDMASK {}
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb2addmask;
#[doc = "eUSCI_Bx I2C Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2i2csa](ucb2i2csa) module"]
pub type UCB2I2CSA = crate::Reg<u16, _UCB2I2CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2I2CSA;
#[doc = "`read()` method returns [ucb2i2csa::R](ucb2i2csa::R) reader structure"]
impl crate::Readable for UCB2I2CSA {}
#[doc = "`write(|w| ..)` method takes [ucb2i2csa::W](ucb2i2csa::W) writer structure"]
impl crate::Writable for UCB2I2CSA {}
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb2i2csa;
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2ie](ucb2ie) module"]
pub type UCB2IE = crate::Reg<u16, _UCB2IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2IE;
#[doc = "`read()` method returns [ucb2ie::R](ucb2ie::R) reader structure"]
impl crate::Readable for UCB2IE {}
#[doc = "`write(|w| ..)` method takes [ucb2ie::W](ucb2ie::W) writer structure"]
impl crate::Writable for UCB2IE {}
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb2ie;
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2ie_spi](ucb2ie_spi) module"]
pub type UCB2IE_SPI = crate::Reg<u16, _UCB2IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2IE_SPI;
#[doc = "`read()` method returns [ucb2ie_spi::R](ucb2ie_spi::R) reader structure"]
impl crate::Readable for UCB2IE_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb2ie_spi::W](ucb2ie_spi::W) writer structure"]
impl crate::Writable for UCB2IE_SPI {}
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb2ie_spi;
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2ifg](ucb2ifg) module"]
pub type UCB2IFG = crate::Reg<u16, _UCB2IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2IFG;
#[doc = "`read()` method returns [ucb2ifg::R](ucb2ifg::R) reader structure"]
impl crate::Readable for UCB2IFG {}
#[doc = "`write(|w| ..)` method takes [ucb2ifg::W](ucb2ifg::W) writer structure"]
impl crate::Writable for UCB2IFG {}
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb2ifg;
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2ifg_spi](ucb2ifg_spi) module"]
pub type UCB2IFG_SPI = crate::Reg<u16, _UCB2IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2IFG_SPI;
#[doc = "`read()` method returns [ucb2ifg_spi::R](ucb2ifg_spi::R) reader structure"]
impl crate::Readable for UCB2IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb2ifg_spi::W](ucb2ifg_spi::W) writer structure"]
impl crate::Writable for UCB2IFG_SPI {}
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb2ifg_spi;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2iv](ucb2iv) module"]
pub type UCB2IV = crate::Reg<u16, _UCB2IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2IV;
#[doc = "`read()` method returns [ucb2iv::R](ucb2iv::R) reader structure"]
impl crate::Readable for UCB2IV {}
#[doc = "`write(|w| ..)` method takes [ucb2iv::W](ucb2iv::W) writer structure"]
impl crate::Writable for UCB2IV {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb2iv;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb2iv_spi](ucb2iv_spi) module"]
pub type UCB2IV_SPI = crate::Reg<u16, _UCB2IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB2IV_SPI;
#[doc = "`read()` method returns [ucb2iv_spi::R](ucb2iv_spi::R) reader structure"]
impl crate::Readable for UCB2IV_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb2iv_spi::W](ucb2iv_spi::W) writer structure"]
impl crate::Writable for UCB2IV_SPI {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb2iv_spi;
