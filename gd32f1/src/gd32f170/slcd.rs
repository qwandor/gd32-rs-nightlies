#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    cfg: Cfg,
    stat: Stat,
    statc: Statc,
    _reserved4: [u8; 0x04],
    data0: Data0,
    _reserved5: [u8; 0x04],
    data1: Data1,
    _reserved6: [u8; 0x04],
    data2: Data2,
    _reserved7: [u8; 0x04],
    data3: Data3,
    _reserved8: [u8; 0x04],
    data4: Data4,
    _reserved9: [u8; 0x04],
    data5: Data5,
    _reserved10: [u8; 0x04],
    data6: Data6,
    _reserved11: [u8; 0x04],
    data7: Data7,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - SLCD configuration register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - SLCD status flag register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x0c - SLCD status flag clear register"]
    #[inline(always)]
    pub const fn statc(&self) -> &Statc {
        &self.statc
    }
    #[doc = "0x14 - SLCD display data register"]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x1c - SLCD display data register"]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x24 - SLCD display data register"]
    #[inline(always)]
    pub const fn data2(&self) -> &Data2 {
        &self.data2
    }
    #[doc = "0x2c - SLCD display data register"]
    #[inline(always)]
    pub const fn data3(&self) -> &Data3 {
        &self.data3
    }
    #[doc = "0x34 - SLCD display data register"]
    #[inline(always)]
    pub const fn data4(&self) -> &Data4 {
        &self.data4
    }
    #[doc = "0x3c - SLCD display data register"]
    #[inline(always)]
    pub const fn data5(&self) -> &Data5 {
        &self.data5
    }
    #[doc = "0x44 - SLCD display data register"]
    #[inline(always)]
    pub const fn data6(&self) -> &Data6 {
        &self.data6
    }
    #[doc = "0x4c - SLCD display data register"]
    #[inline(always)]
    pub const fn data7(&self) -> &Data7 {
        &self.data7
    }
}
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "CFG (rw) register accessor: SLCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "SLCD configuration register"]
pub mod cfg;
#[doc = "STAT (rw) register accessor: SLCD status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "SLCD status flag register"]
pub mod stat;
#[doc = "STATC (rw) register accessor: SLCD status flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statc`]
module"]
#[doc(alias = "STATC")]
pub type Statc = crate::Reg<statc::StatcSpec>;
#[doc = "SLCD status flag clear register"]
pub mod statc;
#[doc = "DATA0 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`]
module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "SLCD display data register"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`]
module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "SLCD display data register"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2`]
module"]
#[doc(alias = "DATA2")]
pub type Data2 = crate::Reg<data2::Data2Spec>;
#[doc = "SLCD display data register"]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3`]
module"]
#[doc(alias = "DATA3")]
pub type Data3 = crate::Reg<data3::Data3Spec>;
#[doc = "SLCD display data register"]
pub mod data3;
#[doc = "DATA4 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data4`]
module"]
#[doc(alias = "DATA4")]
pub type Data4 = crate::Reg<data4::Data4Spec>;
#[doc = "SLCD display data register"]
pub mod data4;
#[doc = "DATA5 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data5`]
module"]
#[doc(alias = "DATA5")]
pub type Data5 = crate::Reg<data5::Data5Spec>;
#[doc = "SLCD display data register"]
pub mod data5;
#[doc = "DATA6 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data6`]
module"]
#[doc(alias = "DATA6")]
pub type Data6 = crate::Reg<data6::Data6Spec>;
#[doc = "SLCD display data register"]
pub mod data6;
#[doc = "DATA7 (rw) register accessor: SLCD display data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data7`]
module"]
#[doc(alias = "DATA7")]
pub type Data7 = crate::Reg<data7::Data7Spec>;
#[doc = "SLCD display data register"]
pub mod data7;
