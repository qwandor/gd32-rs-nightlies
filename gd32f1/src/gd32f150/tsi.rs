#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    inten: Inten,
    intc: Intc,
    intf: Intf,
    phm: Phm,
    _reserved5: [u8; 0x04],
    asw: Asw,
    _reserved6: [u8; 0x04],
    sampcfg: Sampcfg,
    _reserved7: [u8; 0x04],
    chcfg: Chcfg,
    _reserved8: [u8; 0x04],
    gctl: Gctl,
    g0cycn: G0cycn,
    g1cycn: G1cycn,
    g2cycn: G2cycn,
    g3cycn: G3cycn,
    g4cycn: G4cycn,
    g5cycn: G5cycn,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - interrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x08 - interrupt flag clear register"]
    #[inline(always)]
    pub const fn intc(&self) -> &Intc {
        &self.intc
    }
    #[doc = "0x0c - interrupt flag register"]
    #[inline(always)]
    pub const fn intf(&self) -> &Intf {
        &self.intf
    }
    #[doc = "0x10 - Pin hysteresis mode register"]
    #[inline(always)]
    pub const fn phm(&self) -> &Phm {
        &self.phm
    }
    #[doc = "0x18 - I/O analog switch register"]
    #[inline(always)]
    pub const fn asw(&self) -> &Asw {
        &self.asw
    }
    #[doc = "0x20 - I/O sample configuration register"]
    #[inline(always)]
    pub const fn sampcfg(&self) -> &Sampcfg {
        &self.sampcfg
    }
    #[doc = "0x28 - I/O channel configuration register"]
    #[inline(always)]
    pub const fn chcfg(&self) -> &Chcfg {
        &self.chcfg
    }
    #[doc = "0x30 - I/O group control register"]
    #[inline(always)]
    pub const fn gctl(&self) -> &Gctl {
        &self.gctl
    }
    #[doc = "0x34 - I/O group x cycle number register"]
    #[inline(always)]
    pub const fn g0cycn(&self) -> &G0cycn {
        &self.g0cycn
    }
    #[doc = "0x38 - I/O group x cycle number register"]
    #[inline(always)]
    pub const fn g1cycn(&self) -> &G1cycn {
        &self.g1cycn
    }
    #[doc = "0x3c - I/O group x cycle number register"]
    #[inline(always)]
    pub const fn g2cycn(&self) -> &G2cycn {
        &self.g2cycn
    }
    #[doc = "0x40 - I/O group x cycle number register"]
    #[inline(always)]
    pub const fn g3cycn(&self) -> &G3cycn {
        &self.g3cycn
    }
    #[doc = "0x44 - I/O group x cycle number register"]
    #[inline(always)]
    pub const fn g4cycn(&self) -> &G4cycn {
        &self.g4cycn
    }
    #[doc = "0x48 - I/O group x cycle number register"]
    #[inline(always)]
    pub const fn g5cycn(&self) -> &G5cycn {
        &self.g5cycn
    }
}
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "control register"]
pub mod ctl;
#[doc = "INTEN (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "interrupt enable register"]
pub mod inten;
#[doc = "INTC (rw) register accessor: interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc`]
module"]
#[doc(alias = "INTC")]
pub type Intc = crate::Reg<intc::IntcSpec>;
#[doc = "interrupt flag clear register"]
pub mod intc;
#[doc = "INTF (rw) register accessor: interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
#[doc(alias = "INTF")]
pub type Intf = crate::Reg<intf::IntfSpec>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "PHM (rw) register accessor: Pin hysteresis mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phm`]
module"]
#[doc(alias = "PHM")]
pub type Phm = crate::Reg<phm::PhmSpec>;
#[doc = "Pin hysteresis mode register"]
pub mod phm;
#[doc = "ASW (rw) register accessor: I/O analog switch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asw`]
module"]
#[doc(alias = "ASW")]
pub type Asw = crate::Reg<asw::AswSpec>;
#[doc = "I/O analog switch register"]
pub mod asw;
#[doc = "SAMPCFG (rw) register accessor: I/O sample configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampcfg`]
module"]
#[doc(alias = "SAMPCFG")]
pub type Sampcfg = crate::Reg<sampcfg::SampcfgSpec>;
#[doc = "I/O sample configuration register"]
pub mod sampcfg;
#[doc = "CHCFG (rw) register accessor: I/O channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chcfg`]
module"]
#[doc(alias = "CHCFG")]
pub type Chcfg = crate::Reg<chcfg::ChcfgSpec>;
#[doc = "I/O channel configuration register"]
pub mod chcfg;
#[doc = "GCTL (rw) register accessor: I/O group control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gctl`]
module"]
#[doc(alias = "GCTL")]
pub type Gctl = crate::Reg<gctl::GctlSpec>;
#[doc = "I/O group control register"]
pub mod gctl;
#[doc = "G0CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g0cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g0cycn`]
module"]
#[doc(alias = "G0CYCN")]
pub type G0cycn = crate::Reg<g0cycn::G0cycnSpec>;
#[doc = "I/O group x cycle number register"]
pub mod g0cycn;
#[doc = "G1CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g1cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g1cycn`]
module"]
#[doc(alias = "G1CYCN")]
pub type G1cycn = crate::Reg<g1cycn::G1cycnSpec>;
#[doc = "I/O group x cycle number register"]
pub mod g1cycn;
#[doc = "G2CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g2cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g2cycn`]
module"]
#[doc(alias = "G2CYCN")]
pub type G2cycn = crate::Reg<g2cycn::G2cycnSpec>;
#[doc = "I/O group x cycle number register"]
pub mod g2cycn;
#[doc = "G3CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g3cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g3cycn`]
module"]
#[doc(alias = "G3CYCN")]
pub type G3cycn = crate::Reg<g3cycn::G3cycnSpec>;
#[doc = "I/O group x cycle number register"]
pub mod g3cycn;
#[doc = "G4CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g4cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g4cycn`]
module"]
#[doc(alias = "G4CYCN")]
pub type G4cycn = crate::Reg<g4cycn::G4cycnSpec>;
#[doc = "I/O group x cycle number register"]
pub mod g4cycn;
#[doc = "G5CYCN (r) register accessor: I/O group x cycle number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g5cycn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g5cycn`]
module"]
#[doc(alias = "G5CYCN")]
pub type G5cycn = crate::Reg<g5cycn::G5cycnSpec>;
#[doc = "I/O group x cycle number register"]
pub mod g5cycn;
