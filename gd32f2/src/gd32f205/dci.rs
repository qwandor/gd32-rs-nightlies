#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    stat0: Stat0,
    stat1: Stat1,
    inten: Inten,
    intf: Intf,
    intc: Intc,
    sc: Sc,
    scumsk: Scumsk,
    cwspos: Cwspos,
    cwsz: Cwsz,
    data: Data,
}
impl RegisterBlock {
    #[doc = "0x00 - DCI Control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - DCI Status register 0"]
    #[inline(always)]
    pub const fn stat0(&self) -> &Stat0 {
        &self.stat0
    }
    #[doc = "0x08 - DCI Status register1"]
    #[inline(always)]
    pub const fn stat1(&self) -> &Stat1 {
        &self.stat1
    }
    #[doc = "0x0c - DCI inrerrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x10 - DCI Interrupt flag register"]
    #[inline(always)]
    pub const fn intf(&self) -> &Intf {
        &self.intf
    }
    #[doc = "0x14 - DCI Interrupt flag clear register"]
    #[inline(always)]
    pub const fn intc(&self) -> &Intc {
        &self.intc
    }
    #[doc = "0x18 - DCI Synchronization codes register"]
    #[inline(always)]
    pub const fn sc(&self) -> &Sc {
        &self.sc
    }
    #[doc = "0x1c - DCI Synchronization codes unmask register"]
    #[inline(always)]
    pub const fn scumsk(&self) -> &Scumsk {
        &self.scumsk
    }
    #[doc = "0x20 - DCI Cropping window start position register"]
    #[inline(always)]
    pub const fn cwspos(&self) -> &Cwspos {
        &self.cwspos
    }
    #[doc = "0x24 - DCI Cropping window size register"]
    #[inline(always)]
    pub const fn cwsz(&self) -> &Cwsz {
        &self.cwsz
    }
    #[doc = "0x28 - DCI DATA register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
}
#[doc = "CTL (rw) register accessor: DCI Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "DCI Control register"]
pub mod ctl;
#[doc = "STAT0 (r) register accessor: DCI Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat0`]
module"]
#[doc(alias = "STAT0")]
pub type Stat0 = crate::Reg<stat0::Stat0Spec>;
#[doc = "DCI Status register 0"]
pub mod stat0;
#[doc = "STAT1 (r) register accessor: DCI Status register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat1`]
module"]
#[doc(alias = "STAT1")]
pub type Stat1 = crate::Reg<stat1::Stat1Spec>;
#[doc = "DCI Status register1"]
pub mod stat1;
#[doc = "INTEN (rw) register accessor: DCI inrerrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "DCI inrerrupt enable register"]
pub mod inten;
#[doc = "INTF (r) register accessor: DCI Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
#[doc(alias = "INTF")]
pub type Intf = crate::Reg<intf::IntfSpec>;
#[doc = "DCI Interrupt flag register"]
pub mod intf;
#[doc = "INTC (w) register accessor: DCI Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc`]
module"]
#[doc(alias = "INTC")]
pub type Intc = crate::Reg<intc::IntcSpec>;
#[doc = "DCI Interrupt flag clear register"]
pub mod intc;
#[doc = "SC (rw) register accessor: DCI Synchronization codes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc`]
module"]
#[doc(alias = "SC")]
pub type Sc = crate::Reg<sc::ScSpec>;
#[doc = "DCI Synchronization codes register"]
pub mod sc;
#[doc = "SCUMSK (rw) register accessor: DCI Synchronization codes unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scumsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scumsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scumsk`]
module"]
#[doc(alias = "SCUMSK")]
pub type Scumsk = crate::Reg<scumsk::ScumskSpec>;
#[doc = "DCI Synchronization codes unmask register"]
pub mod scumsk;
#[doc = "CWSPOS (rw) register accessor: DCI Cropping window start position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwspos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwspos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwspos`]
module"]
#[doc(alias = "CWSPOS")]
pub type Cwspos = crate::Reg<cwspos::CwsposSpec>;
#[doc = "DCI Cropping window start position register"]
pub mod cwspos;
#[doc = "CWSZ (rw) register accessor: DCI Cropping window size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwsz`]
module"]
#[doc(alias = "CWSZ")]
pub type Cwsz = crate::Reg<cwsz::CwszSpec>;
#[doc = "DCI Cropping window size register"]
pub mod cwsz;
#[doc = "DATA (r) register accessor: DCI DATA register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "DCI DATA register"]
pub mod data;
