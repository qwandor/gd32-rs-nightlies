#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stat: Stat,
    ctl0: Ctl0,
    ctl1: Ctl1,
    sampt0: Sampt0,
    sampt1: Sampt1,
    ioff0: Ioff0,
    ioff1: Ioff1,
    ioff2: Ioff2,
    ioff3: Ioff3,
    wdht: Wdht,
    wdlt: Wdlt,
    rsq0: Rsq0,
    rsq1: Rsq1,
    rsq2: Rsq2,
    isq: Isq,
    idata0: Idata0,
    idata1: Idata1,
    idata2: Idata2,
    idata3: Idata3,
    rdata: Rdata,
    _reserved20: [u8; 0x30],
    ovsampctl: Ovsampctl,
}
impl RegisterBlock {
    #[doc = "0x00 - status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x04 - control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x08 - control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x0c - Sample time register 0"]
    #[inline(always)]
    pub const fn sampt0(&self) -> &Sampt0 {
        &self.sampt0
    }
    #[doc = "0x10 - Sample time register 1"]
    #[inline(always)]
    pub const fn sampt1(&self) -> &Sampt1 {
        &self.sampt1
    }
    #[doc = "0x14 - Inserted channel data offset register 0"]
    #[inline(always)]
    pub const fn ioff0(&self) -> &Ioff0 {
        &self.ioff0
    }
    #[doc = "0x18 - Inserted channel data offset register 1"]
    #[inline(always)]
    pub const fn ioff1(&self) -> &Ioff1 {
        &self.ioff1
    }
    #[doc = "0x1c - Inserted channel data offset register 2"]
    #[inline(always)]
    pub const fn ioff2(&self) -> &Ioff2 {
        &self.ioff2
    }
    #[doc = "0x20 - Inserted channel data offset register 3"]
    #[inline(always)]
    pub const fn ioff3(&self) -> &Ioff3 {
        &self.ioff3
    }
    #[doc = "0x24 - watchdog higher threshold register"]
    #[inline(always)]
    pub const fn wdht(&self) -> &Wdht {
        &self.wdht
    }
    #[doc = "0x28 - watchdog lower threshold register"]
    #[inline(always)]
    pub const fn wdlt(&self) -> &Wdlt {
        &self.wdlt
    }
    #[doc = "0x2c - regular sequence register 0"]
    #[inline(always)]
    pub const fn rsq0(&self) -> &Rsq0 {
        &self.rsq0
    }
    #[doc = "0x30 - regular sequence register 1"]
    #[inline(always)]
    pub const fn rsq1(&self) -> &Rsq1 {
        &self.rsq1
    }
    #[doc = "0x34 - regular sequence register 2"]
    #[inline(always)]
    pub const fn rsq2(&self) -> &Rsq2 {
        &self.rsq2
    }
    #[doc = "0x38 - Inserted sequence register"]
    #[inline(always)]
    pub const fn isq(&self) -> &Isq {
        &self.isq
    }
    #[doc = "0x3c - Inserted data register 0"]
    #[inline(always)]
    pub const fn idata0(&self) -> &Idata0 {
        &self.idata0
    }
    #[doc = "0x40 - Inserted data register 1"]
    #[inline(always)]
    pub const fn idata1(&self) -> &Idata1 {
        &self.idata1
    }
    #[doc = "0x44 - Inserted data register 2"]
    #[inline(always)]
    pub const fn idata2(&self) -> &Idata2 {
        &self.idata2
    }
    #[doc = "0x48 - Inserted data register 3"]
    #[inline(always)]
    pub const fn idata3(&self) -> &Idata3 {
        &self.idata3
    }
    #[doc = "0x4c - regular data register"]
    #[inline(always)]
    pub const fn rdata(&self) -> &Rdata {
        &self.rdata
    }
    #[doc = "0x80 - Oversample control register"]
    #[inline(always)]
    pub const fn ovsampctl(&self) -> &Ovsampctl {
        &self.ovsampctl
    }
}
#[doc = "STAT (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "status register"]
pub mod stat;
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
#[doc = "SAMPT0 (rw) register accessor: Sample time register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampt0`]
module"]
#[doc(alias = "SAMPT0")]
pub type Sampt0 = crate::Reg<sampt0::Sampt0Spec>;
#[doc = "Sample time register 0"]
pub mod sampt0;
#[doc = "SAMPT1 (rw) register accessor: Sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampt1`]
module"]
#[doc(alias = "SAMPT1")]
pub type Sampt1 = crate::Reg<sampt1::Sampt1Spec>;
#[doc = "Sample time register 1"]
pub mod sampt1;
#[doc = "IOFF0 (rw) register accessor: Inserted channel data offset register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff0`]
module"]
#[doc(alias = "IOFF0")]
pub type Ioff0 = crate::Reg<ioff0::Ioff0Spec>;
#[doc = "Inserted channel data offset register 0"]
pub mod ioff0;
#[doc = "IOFF1 (rw) register accessor: Inserted channel data offset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff1`]
module"]
#[doc(alias = "IOFF1")]
pub type Ioff1 = crate::Reg<ioff1::Ioff1Spec>;
#[doc = "Inserted channel data offset register 1"]
pub mod ioff1;
#[doc = "IOFF2 (rw) register accessor: Inserted channel data offset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioff2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioff2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff2`]
module"]
#[doc(alias = "IOFF2")]
pub type Ioff2 = crate::Reg<ioff2::Ioff2Spec>;
#[doc = "Inserted channel data offset register 2"]
pub mod ioff2;
#[doc = "IOFF3 (rw) register accessor: Inserted channel data offset register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioff3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioff3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff3`]
module"]
#[doc(alias = "IOFF3")]
pub type Ioff3 = crate::Reg<ioff3::Ioff3Spec>;
#[doc = "Inserted channel data offset register 3"]
pub mod ioff3;
#[doc = "WDHT (rw) register accessor: watchdog higher threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdht::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdht::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdht`]
module"]
#[doc(alias = "WDHT")]
pub type Wdht = crate::Reg<wdht::WdhtSpec>;
#[doc = "watchdog higher threshold register"]
pub mod wdht;
#[doc = "WDLT (rw) register accessor: watchdog lower threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdlt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdlt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdlt`]
module"]
#[doc(alias = "WDLT")]
pub type Wdlt = crate::Reg<wdlt::WdltSpec>;
#[doc = "watchdog lower threshold register"]
pub mod wdlt;
#[doc = "RSQ0 (rw) register accessor: regular sequence register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsq0`]
module"]
#[doc(alias = "RSQ0")]
pub type Rsq0 = crate::Reg<rsq0::Rsq0Spec>;
#[doc = "regular sequence register 0"]
pub mod rsq0;
#[doc = "RSQ1 (rw) register accessor: regular sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsq1`]
module"]
#[doc(alias = "RSQ1")]
pub type Rsq1 = crate::Reg<rsq1::Rsq1Spec>;
#[doc = "regular sequence register 1"]
pub mod rsq1;
#[doc = "RSQ2 (rw) register accessor: regular sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsq2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsq2`]
module"]
#[doc(alias = "RSQ2")]
pub type Rsq2 = crate::Reg<rsq2::Rsq2Spec>;
#[doc = "regular sequence register 2"]
pub mod rsq2;
#[doc = "ISQ (rw) register accessor: Inserted sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isq`]
module"]
#[doc(alias = "ISQ")]
pub type Isq = crate::Reg<isq::IsqSpec>;
#[doc = "Inserted sequence register"]
pub mod isq;
#[doc = "IDATA0 (r) register accessor: Inserted data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata0`]
module"]
#[doc(alias = "IDATA0")]
pub type Idata0 = crate::Reg<idata0::Idata0Spec>;
#[doc = "Inserted data register 0"]
pub mod idata0;
#[doc = "IDATA1 (r) register accessor: Inserted data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata1`]
module"]
#[doc(alias = "IDATA1")]
pub type Idata1 = crate::Reg<idata1::Idata1Spec>;
#[doc = "Inserted data register 1"]
pub mod idata1;
#[doc = "IDATA2 (r) register accessor: Inserted data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata2`]
module"]
#[doc(alias = "IDATA2")]
pub type Idata2 = crate::Reg<idata2::Idata2Spec>;
#[doc = "Inserted data register 2"]
pub mod idata2;
#[doc = "IDATA3 (r) register accessor: Inserted data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata3`]
module"]
#[doc(alias = "IDATA3")]
pub type Idata3 = crate::Reg<idata3::Idata3Spec>;
#[doc = "Inserted data register 3"]
pub mod idata3;
#[doc = "RDATA (r) register accessor: regular data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`]
module"]
#[doc(alias = "RDATA")]
pub type Rdata = crate::Reg<rdata::RdataSpec>;
#[doc = "regular data register"]
pub mod rdata;
#[doc = "OVSAMPCTL (r) register accessor: Oversample control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovsampctl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ovsampctl`]
module"]
#[doc(alias = "OVSAMPCTL")]
pub type Ovsampctl = crate::Reg<ovsampctl::OvsampctlSpec>;
#[doc = "Oversample control register"]
pub mod ovsampctl;
