#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    spsz: Spsz,
    bpsz: Bpsz,
    asz: Asz,
    tsz: Tsz,
    ctl: Ctl,
    _reserved5: [u8; 0x08],
    rl: Rl,
    _reserved6: [u8; 0x04],
    bgc: Bgc,
    _reserved7: [u8; 0x04],
    inten: Inten,
    intf: Intf,
    intc: Intc,
    lm: Lm,
    cppos: Cppos,
    stat: Stat,
    _reserved13: [u8; 0x38],
    l0ctl: L0ctl,
    l0hpos: L0hpos,
    l0vpos: L0vpos,
    l0ckey: L0ckey,
    l0ppf: L0ppf,
    l0sa: L0sa,
    l0dc: L0dc,
    l0blend: L0blend,
    _reserved21: [u8; 0x08],
    l0fbaddr: L0fbaddr,
    l0fllen: L0fllen,
    l0ftln: L0ftln,
    _reserved24: [u8; 0x0c],
    l0lut: L0lut,
    _reserved25: [u8; 0x3c],
    l1ctl: L1ctl,
    l1hpos: L1hpos,
    l1vpos: L1vpos,
    l1ckey: L1ckey,
    l1ppf: L1ppf,
    l1sa: L1sa,
    l1dc: L1dc,
    l1blend: L1blend,
    _reserved33: [u8; 0x08],
    l1fbaddr: L1fbaddr,
    l1fllen: L1fllen,
    l1ftln: L1ftln,
    _reserved36: [u8; 0x0c],
    l1lut: L1lut,
}
impl RegisterBlock {
    #[doc = "0x08 - Synchronous pulse size register"]
    #[inline(always)]
    pub const fn spsz(&self) -> &Spsz {
        &self.spsz
    }
    #[doc = "0x0c - Back-porch size register"]
    #[inline(always)]
    pub const fn bpsz(&self) -> &Bpsz {
        &self.bpsz
    }
    #[doc = "0x10 - Active size register"]
    #[inline(always)]
    pub const fn asz(&self) -> &Asz {
        &self.asz
    }
    #[doc = "0x14 - Total size register"]
    #[inline(always)]
    pub const fn tsz(&self) -> &Tsz {
        &self.tsz
    }
    #[doc = "0x18 - Control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x24 - Reload layer register"]
    #[inline(always)]
    pub const fn rl(&self) -> &Rl {
        &self.rl
    }
    #[doc = "0x2c - Background color register"]
    #[inline(always)]
    pub const fn bgc(&self) -> &Bgc {
        &self.bgc
    }
    #[doc = "0x34 - Interrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x38 - Interrupt flag register"]
    #[inline(always)]
    pub const fn intf(&self) -> &Intf {
        &self.intf
    }
    #[doc = "0x3c - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn intc(&self) -> &Intc {
        &self.intc
    }
    #[doc = "0x40 - Line mark register"]
    #[inline(always)]
    pub const fn lm(&self) -> &Lm {
        &self.lm
    }
    #[doc = "0x44 - Current pixel position register"]
    #[inline(always)]
    pub const fn cppos(&self) -> &Cppos {
        &self.cppos
    }
    #[doc = "0x48 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x84 - Layer 0 control register"]
    #[inline(always)]
    pub const fn l0ctl(&self) -> &L0ctl {
        &self.l0ctl
    }
    #[doc = "0x88 - Layer 0 horizontal position parameters register"]
    #[inline(always)]
    pub const fn l0hpos(&self) -> &L0hpos {
        &self.l0hpos
    }
    #[doc = "0x8c - Layer 0 vertical position parameters register"]
    #[inline(always)]
    pub const fn l0vpos(&self) -> &L0vpos {
        &self.l0vpos
    }
    #[doc = "0x90 - Layer 0 color key register"]
    #[inline(always)]
    pub const fn l0ckey(&self) -> &L0ckey {
        &self.l0ckey
    }
    #[doc = "0x94 - Layer 0 packeted pixel format register"]
    #[inline(always)]
    pub const fn l0ppf(&self) -> &L0ppf {
        &self.l0ppf
    }
    #[doc = "0x98 - Layer 0 specified alpha register"]
    #[inline(always)]
    pub const fn l0sa(&self) -> &L0sa {
        &self.l0sa
    }
    #[doc = "0x9c - Layer 0 default color register"]
    #[inline(always)]
    pub const fn l0dc(&self) -> &L0dc {
        &self.l0dc
    }
    #[doc = "0xa0 - Layer 0 blending register"]
    #[inline(always)]
    pub const fn l0blend(&self) -> &L0blend {
        &self.l0blend
    }
    #[doc = "0xac - Layer 0 frame base address register"]
    #[inline(always)]
    pub const fn l0fbaddr(&self) -> &L0fbaddr {
        &self.l0fbaddr
    }
    #[doc = "0xb0 - Layer 0 frame line length register"]
    #[inline(always)]
    pub const fn l0fllen(&self) -> &L0fllen {
        &self.l0fllen
    }
    #[doc = "0xb4 - Layer 0 frame total line number register"]
    #[inline(always)]
    pub const fn l0ftln(&self) -> &L0ftln {
        &self.l0ftln
    }
    #[doc = "0xc4 - Layer 0 look up table register"]
    #[inline(always)]
    pub const fn l0lut(&self) -> &L0lut {
        &self.l0lut
    }
    #[doc = "0x104 - Layer 1 control register"]
    #[inline(always)]
    pub const fn l1ctl(&self) -> &L1ctl {
        &self.l1ctl
    }
    #[doc = "0x108 - Layer 1 horizontal position parameters register"]
    #[inline(always)]
    pub const fn l1hpos(&self) -> &L1hpos {
        &self.l1hpos
    }
    #[doc = "0x10c - Layer 1 vertical position parameters register"]
    #[inline(always)]
    pub const fn l1vpos(&self) -> &L1vpos {
        &self.l1vpos
    }
    #[doc = "0x110 - Layer 1 color key register"]
    #[inline(always)]
    pub const fn l1ckey(&self) -> &L1ckey {
        &self.l1ckey
    }
    #[doc = "0x114 - Layer 1 packeted pixel format register"]
    #[inline(always)]
    pub const fn l1ppf(&self) -> &L1ppf {
        &self.l1ppf
    }
    #[doc = "0x118 - Layer 1 specified alpha register"]
    #[inline(always)]
    pub const fn l1sa(&self) -> &L1sa {
        &self.l1sa
    }
    #[doc = "0x11c - Layer 1 default color register"]
    #[inline(always)]
    pub const fn l1dc(&self) -> &L1dc {
        &self.l1dc
    }
    #[doc = "0x120 - Layer 1 blending register"]
    #[inline(always)]
    pub const fn l1blend(&self) -> &L1blend {
        &self.l1blend
    }
    #[doc = "0x12c - Layer 1 frame base address register"]
    #[inline(always)]
    pub const fn l1fbaddr(&self) -> &L1fbaddr {
        &self.l1fbaddr
    }
    #[doc = "0x130 - Layer 1 frame line length register"]
    #[inline(always)]
    pub const fn l1fllen(&self) -> &L1fllen {
        &self.l1fllen
    }
    #[doc = "0x134 - Layer 1 frame total line number register"]
    #[inline(always)]
    pub const fn l1ftln(&self) -> &L1ftln {
        &self.l1ftln
    }
    #[doc = "0x144 - Layer 1 look up table register"]
    #[inline(always)]
    pub const fn l1lut(&self) -> &L1lut {
        &self.l1lut
    }
}
#[doc = "SPSZ (rw) register accessor: Synchronous pulse size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spsz`]
module"]
#[doc(alias = "SPSZ")]
pub type Spsz = crate::Reg<spsz::SpszSpec>;
#[doc = "Synchronous pulse size register"]
pub mod spsz;
#[doc = "BPSZ (rw) register accessor: Back-porch size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpsz`]
module"]
#[doc(alias = "BPSZ")]
pub type Bpsz = crate::Reg<bpsz::BpszSpec>;
#[doc = "Back-porch size register"]
pub mod bpsz;
#[doc = "ASZ (rw) register accessor: Active size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asz`]
module"]
#[doc(alias = "ASZ")]
pub type Asz = crate::Reg<asz::AszSpec>;
#[doc = "Active size register"]
pub mod asz;
#[doc = "TSZ (rw) register accessor: Total size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsz`]
module"]
#[doc(alias = "TSZ")]
pub type Tsz = crate::Reg<tsz::TszSpec>;
#[doc = "Total size register"]
pub mod tsz;
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "RL (rw) register accessor: Reload layer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rl`]
module"]
#[doc(alias = "RL")]
pub type Rl = crate::Reg<rl::RlSpec>;
#[doc = "Reload layer register"]
pub mod rl;
#[doc = "BGC (rw) register accessor: Background color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgc`]
module"]
#[doc(alias = "BGC")]
pub type Bgc = crate::Reg<bgc::BgcSpec>;
#[doc = "Background color register"]
pub mod bgc;
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "INTF (r) register accessor: Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
#[doc(alias = "INTF")]
pub type Intf = crate::Reg<intf::IntfSpec>;
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "INTC (w) register accessor: Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc`]
module"]
#[doc(alias = "INTC")]
pub type Intc = crate::Reg<intc::IntcSpec>;
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "LM (rw) register accessor: Line mark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lm`]
module"]
#[doc(alias = "LM")]
pub type Lm = crate::Reg<lm::LmSpec>;
#[doc = "Line mark register"]
pub mod lm;
#[doc = "CPPOS (r) register accessor: Current pixel position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cppos::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cppos`]
module"]
#[doc(alias = "CPPOS")]
pub type Cppos = crate::Reg<cppos::CpposSpec>;
#[doc = "Current pixel position register"]
pub mod cppos;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "L0CTL (rw) register accessor: Layer 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0ctl`]
module"]
#[doc(alias = "L0CTL")]
pub type L0ctl = crate::Reg<l0ctl::L0ctlSpec>;
#[doc = "Layer 0 control register"]
pub mod l0ctl;
#[doc = "L1CTL (rw) register accessor: Layer 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1ctl`]
module"]
#[doc(alias = "L1CTL")]
pub type L1ctl = crate::Reg<l1ctl::L1ctlSpec>;
#[doc = "Layer 1 control register"]
pub mod l1ctl;
#[doc = "L0HPOS (rw) register accessor: Layer 0 horizontal position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0hpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0hpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0hpos`]
module"]
#[doc(alias = "L0HPOS")]
pub type L0hpos = crate::Reg<l0hpos::L0hposSpec>;
#[doc = "Layer 0 horizontal position parameters register"]
pub mod l0hpos;
#[doc = "L1HPOS (rw) register accessor: Layer 1 horizontal position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1hpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1hpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1hpos`]
module"]
#[doc(alias = "L1HPOS")]
pub type L1hpos = crate::Reg<l1hpos::L1hposSpec>;
#[doc = "Layer 1 horizontal position parameters register"]
pub mod l1hpos;
#[doc = "L0VPOS (rw) register accessor: Layer 0 vertical position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0vpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0vpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0vpos`]
module"]
#[doc(alias = "L0VPOS")]
pub type L0vpos = crate::Reg<l0vpos::L0vposSpec>;
#[doc = "Layer 0 vertical position parameters register"]
pub mod l0vpos;
#[doc = "L1VPOS (rw) register accessor: Layer 1 vertical position parameters register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1vpos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1vpos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1vpos`]
module"]
#[doc(alias = "L1VPOS")]
pub type L1vpos = crate::Reg<l1vpos::L1vposSpec>;
#[doc = "Layer 1 vertical position parameters register"]
pub mod l1vpos;
#[doc = "L0CKEY (rw) register accessor: Layer 0 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ckey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ckey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0ckey`]
module"]
#[doc(alias = "L0CKEY")]
pub type L0ckey = crate::Reg<l0ckey::L0ckeySpec>;
#[doc = "Layer 0 color key register"]
pub mod l0ckey;
#[doc = "L1CKEY (rw) register accessor: Layer 1 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ckey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ckey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1ckey`]
module"]
#[doc(alias = "L1CKEY")]
pub type L1ckey = crate::Reg<l1ckey::L1ckeySpec>;
#[doc = "Layer 1 color key register"]
pub mod l1ckey;
#[doc = "L0PPF (rw) register accessor: Layer 0 packeted pixel format register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ppf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ppf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0ppf`]
module"]
#[doc(alias = "L0PPF")]
pub type L0ppf = crate::Reg<l0ppf::L0ppfSpec>;
#[doc = "Layer 0 packeted pixel format register"]
pub mod l0ppf;
#[doc = "L1PPF (rw) register accessor: Layer 1 packeted pixel format register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ppf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ppf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1ppf`]
module"]
#[doc(alias = "L1PPF")]
pub type L1ppf = crate::Reg<l1ppf::L1ppfSpec>;
#[doc = "Layer 1 packeted pixel format register"]
pub mod l1ppf;
#[doc = "L0SA (rw) register accessor: Layer 0 specified alpha register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0sa`]
module"]
#[doc(alias = "L0SA")]
pub type L0sa = crate::Reg<l0sa::L0saSpec>;
#[doc = "Layer 0 specified alpha register"]
pub mod l0sa;
#[doc = "L1SA (rw) register accessor: Layer 1 specified alpha register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1sa`]
module"]
#[doc(alias = "L1SA")]
pub type L1sa = crate::Reg<l1sa::L1saSpec>;
#[doc = "Layer 1 specified alpha register"]
pub mod l1sa;
#[doc = "L0DC (rw) register accessor: Layer 0 default color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0dc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0dc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0dc`]
module"]
#[doc(alias = "L0DC")]
pub type L0dc = crate::Reg<l0dc::L0dcSpec>;
#[doc = "Layer 0 default color register"]
pub mod l0dc;
#[doc = "L1DC (rw) register accessor: Layer 1 default color register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1dc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1dc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1dc`]
module"]
#[doc(alias = "L1DC")]
pub type L1dc = crate::Reg<l1dc::L1dcSpec>;
#[doc = "Layer 1 default color register"]
pub mod l1dc;
#[doc = "L0BLEND (rw) register accessor: Layer 0 blending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0blend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0blend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0blend`]
module"]
#[doc(alias = "L0BLEND")]
pub type L0blend = crate::Reg<l0blend::L0blendSpec>;
#[doc = "Layer 0 blending register"]
pub mod l0blend;
#[doc = "L1BLEND (rw) register accessor: Layer 1 blending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1blend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1blend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1blend`]
module"]
#[doc(alias = "L1BLEND")]
pub type L1blend = crate::Reg<l1blend::L1blendSpec>;
#[doc = "Layer 1 blending register"]
pub mod l1blend;
#[doc = "L0FBADDR (rw) register accessor: Layer 0 frame base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0fbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0fbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0fbaddr`]
module"]
#[doc(alias = "L0FBADDR")]
pub type L0fbaddr = crate::Reg<l0fbaddr::L0fbaddrSpec>;
#[doc = "Layer 0 frame base address register"]
pub mod l0fbaddr;
#[doc = "L1FBADDR (rw) register accessor: Layer 1 frame base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1fbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1fbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1fbaddr`]
module"]
#[doc(alias = "L1FBADDR")]
pub type L1fbaddr = crate::Reg<l1fbaddr::L1fbaddrSpec>;
#[doc = "Layer 1 frame base address register"]
pub mod l1fbaddr;
#[doc = "L0FLLEN (rw) register accessor: Layer 0 frame line length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0fllen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0fllen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0fllen`]
module"]
#[doc(alias = "L0FLLEN")]
pub type L0fllen = crate::Reg<l0fllen::L0fllenSpec>;
#[doc = "Layer 0 frame line length register"]
pub mod l0fllen;
#[doc = "L1FLLEN (rw) register accessor: Layer 1 frame line length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1fllen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1fllen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1fllen`]
module"]
#[doc(alias = "L1FLLEN")]
pub type L1fllen = crate::Reg<l1fllen::L1fllenSpec>;
#[doc = "Layer 1 frame line length register"]
pub mod l1fllen;
#[doc = "L0FTLN (rw) register accessor: Layer 0 frame total line number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0ftln::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0ftln::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0ftln`]
module"]
#[doc(alias = "L0FTLN")]
pub type L0ftln = crate::Reg<l0ftln::L0ftlnSpec>;
#[doc = "Layer 0 frame total line number register"]
pub mod l0ftln;
#[doc = "L1FTLN (rw) register accessor: Layer 1 frame total line number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ftln::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ftln::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1ftln`]
module"]
#[doc(alias = "L1FTLN")]
pub type L1ftln = crate::Reg<l1ftln::L1ftlnSpec>;
#[doc = "Layer 1 frame total line number register"]
pub mod l1ftln;
#[doc = "L0LUT (rw) register accessor: Layer 0 look up table register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0lut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0lut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0lut`]
module"]
#[doc(alias = "L0LUT")]
pub type L0lut = crate::Reg<l0lut::L0lutSpec>;
#[doc = "Layer 0 look up table register"]
pub mod l0lut;
#[doc = "L1LUT (rw) register accessor: Layer 1 look up table register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1lut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1lut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1lut`]
module"]
#[doc(alias = "L1LUT")]
pub type L1lut = crate::Reg<l1lut::L1lutSpec>;
#[doc = "Layer 1 look up table register"]
pub mod l1lut;
