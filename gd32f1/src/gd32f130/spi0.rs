#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: Ctl0,
    _reserved1: [u8; 0x02],
    ctl1: Ctl1,
    _reserved2: [u8; 0x02],
    stat: Stat,
    _reserved3: [u8; 0x02],
    data: Data,
    _reserved4: [u8; 0x02],
    crcpoly: Crcpoly,
    _reserved5: [u8; 0x02],
    rcrc: Rcrc,
    _reserved6: [u8; 0x02],
    tcrc: Tcrc,
    _reserved7: [u8; 0x02],
    i2sctl: I2sctl,
    _reserved8: [u8; 0x02],
    i2spsc: I2spsc,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x04 - control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x08 - status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x0c - data register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x10 - CRC polynomial register"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> &Crcpoly {
        &self.crcpoly
    }
    #[doc = "0x14 - Receive CRC register"]
    #[inline(always)]
    pub const fn rcrc(&self) -> &Rcrc {
        &self.rcrc
    }
    #[doc = "0x18 - Transmit CRC register"]
    #[inline(always)]
    pub const fn tcrc(&self) -> &Tcrc {
        &self.tcrc
    }
    #[doc = "0x1c - I2S control register"]
    #[inline(always)]
    pub const fn i2sctl(&self) -> &I2sctl {
        &self.i2sctl
    }
    #[doc = "0x20 - I2S prescaler register"]
    #[inline(always)]
    pub const fn i2spsc(&self) -> &I2spsc {
        &self.i2spsc
    }
}
#[doc = "CTL0 (rw) register accessor: control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "STAT (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "status register"]
pub mod stat;
#[doc = "DATA (rw) register accessor: data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "data register"]
pub mod data;
#[doc = "CRCPOLY (rw) register accessor: CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcpoly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`]
module"]
#[doc(alias = "CRCPOLY")]
pub type Crcpoly = crate::Reg<crcpoly::CrcpolySpec>;
#[doc = "CRC polynomial register"]
pub mod crcpoly;
#[doc = "RCRC (r) register accessor: Receive CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcrc`]
module"]
#[doc(alias = "RCRC")]
pub type Rcrc = crate::Reg<rcrc::RcrcSpec>;
#[doc = "Receive CRC register"]
pub mod rcrc;
#[doc = "TCRC (r) register accessor: Transmit CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcrc`]
module"]
#[doc(alias = "TCRC")]
pub type Tcrc = crate::Reg<tcrc::TcrcSpec>;
#[doc = "Transmit CRC register"]
pub mod tcrc;
#[doc = "I2SCTL (rw) register accessor: I2S control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctl`]
module"]
#[doc(alias = "I2SCTL")]
pub type I2sctl = crate::Reg<i2sctl::I2sctlSpec>;
#[doc = "I2S control register"]
pub mod i2sctl;
#[doc = "I2SPSC (rw) register accessor: I2S prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2spsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2spsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2spsc`]
module"]
#[doc(alias = "I2SPSC")]
pub type I2spsc = crate::Reg<i2spsc::I2spscSpec>;
#[doc = "I2S prescaler register"]
pub mod i2spsc;
