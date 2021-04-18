#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ucb3ctlw: [u8; 2usize],
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    pub ucb3ctlw1: UCB3CTLW1,
    _reserved2: [u8; 2usize],
    _reserved_2_ucb3: [u8; 2usize],
    _reserved_3_ucb3: [u8; 2usize],
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    pub ucb3tbcnt: UCB3TBCNT,
    _reserved_5_ucb3: [u8; 2usize],
    _reserved_6_ucb3: [u8; 2usize],
    _reserved7: [u8; 4usize],
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    pub ucb3i2coa0: UCB3I2COA0,
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    pub ucb3i2coa1: UCB3I2COA1,
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    pub ucb3i2coa2: UCB3I2COA2,
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    pub ucb3i2coa3: UCB3I2COA3,
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    pub ucb3addrx: UCB3ADDRX,
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    pub ucb3addmask: UCB3ADDMASK,
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    pub ucb3i2csa: UCB3I2CSA,
    _reserved14: [u8; 8usize],
    _reserved_14_ucb3: [u8; 2usize],
    _reserved_15_ucb3: [u8; 2usize],
    _reserved_16_ucb3: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb3ctlw0_spi(&self) -> &UCB3CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB3CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb3ctlw0_spi_mut(&self) -> &mut UCB3CTLW0_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB3CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb3ctlw0(&self) -> &UCB3CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB3CTLW0) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb3ctlw0_mut(&self) -> &mut UCB3CTLW0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB3CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb3brw_spi(&self) -> &UCB3BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB3BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb3brw_spi_mut(&self) -> &mut UCB3BRW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB3BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb3brw(&self) -> &UCB3BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB3BRW) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb3brw_mut(&self) -> &mut UCB3BRW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB3BRW) }
    }
    #[doc = "0x08 - UCB3STATW_SPI"]
    #[inline(always)]
    pub fn ucb3statw_spi(&self) -> &UCB3STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB3STATW_SPI) }
    }
    #[doc = "0x08 - UCB3STATW_SPI"]
    #[inline(always)]
    pub fn ucb3statw_spi_mut(&self) -> &mut UCB3STATW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut UCB3STATW_SPI) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb3statw(&self) -> &UCB3STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB3STATW) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb3statw_mut(&self) -> &mut UCB3STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut UCB3STATW) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb3rxbuf_spi(&self) -> &UCB3RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB3RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb3rxbuf_spi_mut(&self) -> &mut UCB3RXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCB3RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb3rxbuf(&self) -> &UCB3RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB3RXBUF) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb3rxbuf_mut(&self) -> &mut UCB3RXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCB3RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb3txbuf_spi(&self) -> &UCB3TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB3TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb3txbuf_spi_mut(&self) -> &mut UCB3TXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCB3TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb3txbuf(&self) -> &UCB3TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB3TXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb3txbuf_mut(&self) -> &mut UCB3TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCB3TXBUF) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb3ie_spi(&self) -> &UCB3IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB3IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb3ie_spi_mut(&self) -> &mut UCB3IE_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB3IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb3ie(&self) -> &UCB3IE {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB3IE) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb3ie_mut(&self) -> &mut UCB3IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB3IE) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb3ifg_spi(&self) -> &UCB3IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB3IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb3ifg_spi_mut(&self) -> &mut UCB3IFG_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB3IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb3ifg(&self) -> &UCB3IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB3IFG) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb3ifg_mut(&self) -> &mut UCB3IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB3IFG) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb3iv_spi(&self) -> &UCB3IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB3IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb3iv_spi_mut(&self) -> &mut UCB3IV_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(46usize) as *mut UCB3IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb3iv(&self) -> &UCB3IV {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB3IV) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb3iv_mut(&self) -> &mut UCB3IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(46usize) as *mut UCB3IV) }
    }
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3ctlw0](ucb3ctlw0) module"]
pub type UCB3CTLW0 = crate::Reg<u16, _UCB3CTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3CTLW0;
#[doc = "`read()` method returns [ucb3ctlw0::R](ucb3ctlw0::R) reader structure"]
impl crate::Readable for UCB3CTLW0 {}
#[doc = "`write(|w| ..)` method takes [ucb3ctlw0::W](ucb3ctlw0::W) writer structure"]
impl crate::Writable for UCB3CTLW0 {}
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb3ctlw0;
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3ctlw0_spi](ucb3ctlw0_spi) module"]
pub type UCB3CTLW0_SPI = crate::Reg<u16, _UCB3CTLW0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3CTLW0_SPI;
#[doc = "`read()` method returns [ucb3ctlw0_spi::R](ucb3ctlw0_spi::R) reader structure"]
impl crate::Readable for UCB3CTLW0_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb3ctlw0_spi::W](ucb3ctlw0_spi::W) writer structure"]
impl crate::Writable for UCB3CTLW0_SPI {}
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb3ctlw0_spi;
#[doc = "eUSCI_Bx Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3ctlw1](ucb3ctlw1) module"]
pub type UCB3CTLW1 = crate::Reg<u16, _UCB3CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3CTLW1;
#[doc = "`read()` method returns [ucb3ctlw1::R](ucb3ctlw1::R) reader structure"]
impl crate::Readable for UCB3CTLW1 {}
#[doc = "`write(|w| ..)` method takes [ucb3ctlw1::W](ucb3ctlw1::W) writer structure"]
impl crate::Writable for UCB3CTLW1 {}
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb3ctlw1;
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3brw](ucb3brw) module"]
pub type UCB3BRW = crate::Reg<u16, _UCB3BRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3BRW;
#[doc = "`read()` method returns [ucb3brw::R](ucb3brw::R) reader structure"]
impl crate::Readable for UCB3BRW {}
#[doc = "`write(|w| ..)` method takes [ucb3brw::W](ucb3brw::W) writer structure"]
impl crate::Writable for UCB3BRW {}
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb3brw;
#[doc = "eUSCI_Bx Bit Rate Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3brw_spi](ucb3brw_spi) module"]
pub type UCB3BRW_SPI = crate::Reg<u16, _UCB3BRW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3BRW_SPI;
#[doc = "`read()` method returns [ucb3brw_spi::R](ucb3brw_spi::R) reader structure"]
impl crate::Readable for UCB3BRW_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb3brw_spi::W](ucb3brw_spi::W) writer structure"]
impl crate::Writable for UCB3BRW_SPI {}
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb3brw_spi;
#[doc = "eUSCI_Bx Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3statw](ucb3statw) module"]
pub type UCB3STATW = crate::Reg<u16, _UCB3STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3STATW;
#[doc = "`read()` method returns [ucb3statw::R](ucb3statw::R) reader structure"]
impl crate::Readable for UCB3STATW {}
#[doc = "`write(|w| ..)` method takes [ucb3statw::W](ucb3statw::W) writer structure"]
impl crate::Writable for UCB3STATW {}
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb3statw;
#[doc = "UCB3STATW_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3statw_spi](ucb3statw_spi) module"]
pub type UCB3STATW_SPI = crate::Reg<u16, _UCB3STATW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3STATW_SPI;
#[doc = "`read()` method returns [ucb3statw_spi::R](ucb3statw_spi::R) reader structure"]
impl crate::Readable for UCB3STATW_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb3statw_spi::W](ucb3statw_spi::W) writer structure"]
impl crate::Writable for UCB3STATW_SPI {}
#[doc = "UCB3STATW_SPI"]
pub mod ucb3statw_spi;
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3tbcnt](ucb3tbcnt) module"]
pub type UCB3TBCNT = crate::Reg<u16, _UCB3TBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3TBCNT;
#[doc = "`read()` method returns [ucb3tbcnt::R](ucb3tbcnt::R) reader structure"]
impl crate::Readable for UCB3TBCNT {}
#[doc = "`write(|w| ..)` method takes [ucb3tbcnt::W](ucb3tbcnt::W) writer structure"]
impl crate::Writable for UCB3TBCNT {}
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb3tbcnt;
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3rxbuf](ucb3rxbuf) module"]
pub type UCB3RXBUF = crate::Reg<u16, _UCB3RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3RXBUF;
#[doc = "`read()` method returns [ucb3rxbuf::R](ucb3rxbuf::R) reader structure"]
impl crate::Readable for UCB3RXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb3rxbuf::W](ucb3rxbuf::W) writer structure"]
impl crate::Writable for UCB3RXBUF {}
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb3rxbuf;
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3rxbuf_spi](ucb3rxbuf_spi) module"]
pub type UCB3RXBUF_SPI = crate::Reg<u16, _UCB3RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3RXBUF_SPI;
#[doc = "`read()` method returns [ucb3rxbuf_spi::R](ucb3rxbuf_spi::R) reader structure"]
impl crate::Readable for UCB3RXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb3rxbuf_spi::W](ucb3rxbuf_spi::W) writer structure"]
impl crate::Writable for UCB3RXBUF_SPI {}
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb3rxbuf_spi;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3txbuf](ucb3txbuf) module"]
pub type UCB3TXBUF = crate::Reg<u16, _UCB3TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3TXBUF;
#[doc = "`read()` method returns [ucb3txbuf::R](ucb3txbuf::R) reader structure"]
impl crate::Readable for UCB3TXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb3txbuf::W](ucb3txbuf::W) writer structure"]
impl crate::Writable for UCB3TXBUF {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb3txbuf;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3txbuf_spi](ucb3txbuf_spi) module"]
pub type UCB3TXBUF_SPI = crate::Reg<u16, _UCB3TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3TXBUF_SPI;
#[doc = "`read()` method returns [ucb3txbuf_spi::R](ucb3txbuf_spi::R) reader structure"]
impl crate::Readable for UCB3TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb3txbuf_spi::W](ucb3txbuf_spi::W) writer structure"]
impl crate::Writable for UCB3TXBUF_SPI {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb3txbuf_spi;
#[doc = "eUSCI_Bx I2C Own Address 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3i2coa0](ucb3i2coa0) module"]
pub type UCB3I2COA0 = crate::Reg<u16, _UCB3I2COA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3I2COA0;
#[doc = "`read()` method returns [ucb3i2coa0::R](ucb3i2coa0::R) reader structure"]
impl crate::Readable for UCB3I2COA0 {}
#[doc = "`write(|w| ..)` method takes [ucb3i2coa0::W](ucb3i2coa0::W) writer structure"]
impl crate::Writable for UCB3I2COA0 {}
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb3i2coa0;
#[doc = "eUSCI_Bx I2C Own Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3i2coa1](ucb3i2coa1) module"]
pub type UCB3I2COA1 = crate::Reg<u16, _UCB3I2COA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3I2COA1;
#[doc = "`read()` method returns [ucb3i2coa1::R](ucb3i2coa1::R) reader structure"]
impl crate::Readable for UCB3I2COA1 {}
#[doc = "`write(|w| ..)` method takes [ucb3i2coa1::W](ucb3i2coa1::W) writer structure"]
impl crate::Writable for UCB3I2COA1 {}
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb3i2coa1;
#[doc = "eUSCI_Bx I2C Own Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3i2coa2](ucb3i2coa2) module"]
pub type UCB3I2COA2 = crate::Reg<u16, _UCB3I2COA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3I2COA2;
#[doc = "`read()` method returns [ucb3i2coa2::R](ucb3i2coa2::R) reader structure"]
impl crate::Readable for UCB3I2COA2 {}
#[doc = "`write(|w| ..)` method takes [ucb3i2coa2::W](ucb3i2coa2::W) writer structure"]
impl crate::Writable for UCB3I2COA2 {}
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb3i2coa2;
#[doc = "eUSCI_Bx I2C Own Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3i2coa3](ucb3i2coa3) module"]
pub type UCB3I2COA3 = crate::Reg<u16, _UCB3I2COA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3I2COA3;
#[doc = "`read()` method returns [ucb3i2coa3::R](ucb3i2coa3::R) reader structure"]
impl crate::Readable for UCB3I2COA3 {}
#[doc = "`write(|w| ..)` method takes [ucb3i2coa3::W](ucb3i2coa3::W) writer structure"]
impl crate::Writable for UCB3I2COA3 {}
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb3i2coa3;
#[doc = "eUSCI_Bx I2C Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3addrx](ucb3addrx) module"]
pub type UCB3ADDRX = crate::Reg<u16, _UCB3ADDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3ADDRX;
#[doc = "`read()` method returns [ucb3addrx::R](ucb3addrx::R) reader structure"]
impl crate::Readable for UCB3ADDRX {}
#[doc = "`write(|w| ..)` method takes [ucb3addrx::W](ucb3addrx::W) writer structure"]
impl crate::Writable for UCB3ADDRX {}
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb3addrx;
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3addmask](ucb3addmask) module"]
pub type UCB3ADDMASK = crate::Reg<u16, _UCB3ADDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3ADDMASK;
#[doc = "`read()` method returns [ucb3addmask::R](ucb3addmask::R) reader structure"]
impl crate::Readable for UCB3ADDMASK {}
#[doc = "`write(|w| ..)` method takes [ucb3addmask::W](ucb3addmask::W) writer structure"]
impl crate::Writable for UCB3ADDMASK {}
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb3addmask;
#[doc = "eUSCI_Bx I2C Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3i2csa](ucb3i2csa) module"]
pub type UCB3I2CSA = crate::Reg<u16, _UCB3I2CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3I2CSA;
#[doc = "`read()` method returns [ucb3i2csa::R](ucb3i2csa::R) reader structure"]
impl crate::Readable for UCB3I2CSA {}
#[doc = "`write(|w| ..)` method takes [ucb3i2csa::W](ucb3i2csa::W) writer structure"]
impl crate::Writable for UCB3I2CSA {}
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb3i2csa;
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3ie](ucb3ie) module"]
pub type UCB3IE = crate::Reg<u16, _UCB3IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3IE;
#[doc = "`read()` method returns [ucb3ie::R](ucb3ie::R) reader structure"]
impl crate::Readable for UCB3IE {}
#[doc = "`write(|w| ..)` method takes [ucb3ie::W](ucb3ie::W) writer structure"]
impl crate::Writable for UCB3IE {}
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb3ie;
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3ie_spi](ucb3ie_spi) module"]
pub type UCB3IE_SPI = crate::Reg<u16, _UCB3IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3IE_SPI;
#[doc = "`read()` method returns [ucb3ie_spi::R](ucb3ie_spi::R) reader structure"]
impl crate::Readable for UCB3IE_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb3ie_spi::W](ucb3ie_spi::W) writer structure"]
impl crate::Writable for UCB3IE_SPI {}
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb3ie_spi;
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3ifg](ucb3ifg) module"]
pub type UCB3IFG = crate::Reg<u16, _UCB3IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3IFG;
#[doc = "`read()` method returns [ucb3ifg::R](ucb3ifg::R) reader structure"]
impl crate::Readable for UCB3IFG {}
#[doc = "`write(|w| ..)` method takes [ucb3ifg::W](ucb3ifg::W) writer structure"]
impl crate::Writable for UCB3IFG {}
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb3ifg;
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3ifg_spi](ucb3ifg_spi) module"]
pub type UCB3IFG_SPI = crate::Reg<u16, _UCB3IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3IFG_SPI;
#[doc = "`read()` method returns [ucb3ifg_spi::R](ucb3ifg_spi::R) reader structure"]
impl crate::Readable for UCB3IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb3ifg_spi::W](ucb3ifg_spi::W) writer structure"]
impl crate::Writable for UCB3IFG_SPI {}
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb3ifg_spi;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3iv](ucb3iv) module"]
pub type UCB3IV = crate::Reg<u16, _UCB3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3IV;
#[doc = "`read()` method returns [ucb3iv::R](ucb3iv::R) reader structure"]
impl crate::Readable for UCB3IV {}
#[doc = "`write(|w| ..)` method takes [ucb3iv::W](ucb3iv::W) writer structure"]
impl crate::Writable for UCB3IV {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb3iv;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb3iv_spi](ucb3iv_spi) module"]
pub type UCB3IV_SPI = crate::Reg<u16, _UCB3IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB3IV_SPI;
#[doc = "`read()` method returns [ucb3iv_spi::R](ucb3iv_spi::R) reader structure"]
impl crate::Readable for UCB3IV_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb3iv_spi::W](ucb3iv_spi::W) writer structure"]
impl crate::Writable for UCB3IV_SPI {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb3iv_spi;
