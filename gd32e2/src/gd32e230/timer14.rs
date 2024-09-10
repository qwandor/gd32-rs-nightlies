#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: Ctl0,
    ctl1: Ctl1,
    smcfg: Smcfg,
    dmainten: Dmainten,
    intf: Intf,
    swevg: Swevg,
    _reserved_6_chctl0: [u8; 0x04],
    _reserved7: [u8; 0x04],
    chctl2: Chctl2,
    cnt: Cnt,
    psc: Psc,
    car: Car,
    crep: Crep,
    ch0cv: Ch0cv,
    ch1cv: Ch1cv,
    _reserved14: [u8; 0x08],
    cchp: Cchp,
    dmacfg: Dmacfg,
    dmatb: Dmatb,
    _reserved17: [u8; 0xac],
    cfg: Cfg,
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
    #[doc = "0x08 - slave mode configuration register"]
    #[inline(always)]
    pub const fn smcfg(&self) -> &Smcfg {
        &self.smcfg
    }
    #[doc = "0x0c - DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dmainten(&self) -> &Dmainten {
        &self.dmainten
    }
    #[doc = "0x10 - interrupt flag register"]
    #[inline(always)]
    pub const fn intf(&self) -> &Intf {
        &self.intf
    }
    #[doc = "0x14 - event generation register"]
    #[inline(always)]
    pub const fn swevg(&self) -> &Swevg {
        &self.swevg
    }
    #[doc = "0x18 - capture/compare mode register 0 (input mode)"]
    #[inline(always)]
    pub const fn chctl0_input(&self) -> &Chctl0Input {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - capture/compare mode register (output mode)"]
    #[inline(always)]
    pub const fn chctl0_output(&self) -> &Chctl0Output {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20 - capture/compare enable register"]
    #[inline(always)]
    pub const fn chctl2(&self) -> &Chctl2 {
        &self.chctl2
    }
    #[doc = "0x24 - counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x28 - prescaler"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x2c - auto-reload register"]
    #[inline(always)]
    pub const fn car(&self) -> &Car {
        &self.car
    }
    #[doc = "0x30 - repetition counter register"]
    #[inline(always)]
    pub const fn crep(&self) -> &Crep {
        &self.crep
    }
    #[doc = "0x34 - capture/compare register 0"]
    #[inline(always)]
    pub const fn ch0cv(&self) -> &Ch0cv {
        &self.ch0cv
    }
    #[doc = "0x38 - capture/compare register 1"]
    #[inline(always)]
    pub const fn ch1cv(&self) -> &Ch1cv {
        &self.ch1cv
    }
    #[doc = "0x44 - break and dead-time register"]
    #[inline(always)]
    pub const fn cchp(&self) -> &Cchp {
        &self.cchp
    }
    #[doc = "0x48 - DMA configuration register"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &Dmacfg {
        &self.dmacfg
    }
    #[doc = "0x4c - DMA transfer buffer register"]
    #[inline(always)]
    pub const fn dmatb(&self) -> &Dmatb {
        &self.dmatb
    }
    #[doc = "0xfc - configuration register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
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
#[doc = "SMCFG (rw) register accessor: slave mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcfg`]
module"]
#[doc(alias = "SMCFG")]
pub type Smcfg = crate::Reg<smcfg::SmcfgSpec>;
#[doc = "slave mode configuration register"]
pub mod smcfg;
#[doc = "DMAINTEN (rw) register accessor: DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmainten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmainten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmainten`]
module"]
#[doc(alias = "DMAINTEN")]
pub type Dmainten = crate::Reg<dmainten::DmaintenSpec>;
#[doc = "DMA/Interrupt enable register"]
pub mod dmainten;
#[doc = "INTF (rw) register accessor: interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
#[doc(alias = "INTF")]
pub type Intf = crate::Reg<intf::IntfSpec>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "SWEVG (w) register accessor: event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swevg`]
module"]
#[doc(alias = "SWEVG")]
pub type Swevg = crate::Reg<swevg::SwevgSpec>;
#[doc = "event generation register"]
pub mod swevg;
#[doc = "CHCTL0_Output (rw) register accessor: capture/compare mode register (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl0_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl0_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl0_output`]
module"]
#[doc(alias = "CHCTL0_Output")]
pub type Chctl0Output = crate::Reg<chctl0_output::Chctl0OutputSpec>;
#[doc = "capture/compare mode register (output mode)"]
pub mod chctl0_output;
#[doc = "CHCTL0_Input (rw) register accessor: capture/compare mode register 0 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl0_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl0_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl0_input`]
module"]
#[doc(alias = "CHCTL0_Input")]
pub type Chctl0Input = crate::Reg<chctl0_input::Chctl0InputSpec>;
#[doc = "capture/compare mode register 0 (input mode)"]
pub mod chctl0_input;
#[doc = "CHCTL2 (rw) register accessor: capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl2`]
module"]
#[doc(alias = "CHCTL2")]
pub type Chctl2 = crate::Reg<chctl2::Chctl2Spec>;
#[doc = "capture/compare enable register"]
pub mod chctl2;
#[doc = "CNT (rw) register accessor: counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "CAR (rw) register accessor: auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@car`]
module"]
#[doc(alias = "CAR")]
pub type Car = crate::Reg<car::CarSpec>;
#[doc = "auto-reload register"]
pub mod car;
#[doc = "CREP (rw) register accessor: repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crep`]
module"]
#[doc(alias = "CREP")]
pub type Crep = crate::Reg<crep::CrepSpec>;
#[doc = "repetition counter register"]
pub mod crep;
#[doc = "CH0CV (rw) register accessor: capture/compare register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cv`]
module"]
#[doc(alias = "CH0CV")]
pub type Ch0cv = crate::Reg<ch0cv::Ch0cvSpec>;
#[doc = "capture/compare register 0"]
pub mod ch0cv;
#[doc = "CH1CV (rw) register accessor: capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cv`]
module"]
#[doc(alias = "CH1CV")]
pub type Ch1cv = crate::Reg<ch1cv::Ch1cvSpec>;
#[doc = "capture/compare register 1"]
pub mod ch1cv;
#[doc = "CCHP (rw) register accessor: break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cchp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cchp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cchp`]
module"]
#[doc(alias = "CCHP")]
pub type Cchp = crate::Reg<cchp::CchpSpec>;
#[doc = "break and dead-time register"]
pub mod cchp;
#[doc = "DMACFG (rw) register accessor: DMA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
#[doc(alias = "DMACFG")]
pub type Dmacfg = crate::Reg<dmacfg::DmacfgSpec>;
#[doc = "DMA configuration register"]
pub mod dmacfg;
#[doc = "DMATB (rw) register accessor: DMA transfer buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatb`]
module"]
#[doc(alias = "DMATB")]
pub type Dmatb = crate::Reg<dmatb::DmatbSpec>;
#[doc = "DMA transfer buffer register"]
pub mod dmatb;
#[doc = "CFG (rw) register accessor: configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "configuration register"]
pub mod cfg;
