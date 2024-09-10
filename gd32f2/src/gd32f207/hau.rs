#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    di: Di,
    cfg: Cfg,
    do0: Do0,
    do1: Do1,
    do2: Do2,
    do3: Do3,
    do4: Do4,
    inten: Inten,
    stat: Stat,
    _reserved10: [u8; 0x02fc],
    do5: Do5,
    do6: Do6,
    do7: Do7,
}
impl RegisterBlock {
    #[doc = "0x00 - HAU control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - HAU data input register"]
    #[inline(always)]
    pub const fn di(&self) -> &Di {
        &self.di
    }
    #[doc = "0x08 - HAU configuration register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x0c - HAU data output register"]
    #[inline(always)]
    pub const fn do0(&self) -> &Do0 {
        &self.do0
    }
    #[doc = "0x10 - HAU data output register"]
    #[inline(always)]
    pub const fn do1(&self) -> &Do1 {
        &self.do1
    }
    #[doc = "0x14 - HAU data output register"]
    #[inline(always)]
    pub const fn do2(&self) -> &Do2 {
        &self.do2
    }
    #[doc = "0x18 - HAU data output register"]
    #[inline(always)]
    pub const fn do3(&self) -> &Do3 {
        &self.do3
    }
    #[doc = "0x1c - HAU data output register"]
    #[inline(always)]
    pub const fn do4(&self) -> &Do4 {
        &self.do4
    }
    #[doc = "0x20 - HAU interrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x24 - HAU status and interrupt flag register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x324 - HAU data output register"]
    #[inline(always)]
    pub const fn do5(&self) -> &Do5 {
        &self.do5
    }
    #[doc = "0x328 - HAU data output register"]
    #[inline(always)]
    pub const fn do6(&self) -> &Do6 {
        &self.do6
    }
    #[doc = "0x32c - HAU data output register"]
    #[inline(always)]
    pub const fn do7(&self) -> &Do7 {
        &self.do7
    }
}
#[doc = "CTL (rw) register accessor: HAU control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "HAU control register"]
pub mod ctl;
#[doc = "DI (rw) register accessor: HAU data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`di::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`di::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@di`]
module"]
#[doc(alias = "DI")]
pub type Di = crate::Reg<di::DiSpec>;
#[doc = "HAU data input register"]
pub mod di;
#[doc = "CFG (rw) register accessor: HAU configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "HAU configuration register"]
pub mod cfg;
#[doc = "DO0 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@do0`]
module"]
#[doc(alias = "DO0")]
pub type Do0 = crate::Reg<do0::Do0Spec>;
#[doc = "HAU data output register"]
pub mod do0;
#[doc = "DO1 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@do1`]
module"]
#[doc(alias = "DO1")]
pub type Do1 = crate::Reg<do1::Do1Spec>;
#[doc = "HAU data output register"]
pub mod do1;
#[doc = "DO2 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@do2`]
module"]
#[doc(alias = "DO2")]
pub type Do2 = crate::Reg<do2::Do2Spec>;
#[doc = "HAU data output register"]
pub mod do2;
#[doc = "DO3 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@do3`]
module"]
#[doc(alias = "DO3")]
pub type Do3 = crate::Reg<do3::Do3Spec>;
#[doc = "HAU data output register"]
pub mod do3;
#[doc = "DO4 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@do4`]
module"]
#[doc(alias = "DO4")]
pub type Do4 = crate::Reg<do4::Do4Spec>;
#[doc = "HAU data output register"]
pub mod do4;
#[doc = "DO5 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@do5`]
module"]
#[doc(alias = "DO5")]
pub type Do5 = crate::Reg<do5::Do5Spec>;
#[doc = "HAU data output register"]
pub mod do5;
#[doc = "DO6 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@do6`]
module"]
#[doc(alias = "DO6")]
pub type Do6 = crate::Reg<do6::Do6Spec>;
#[doc = "HAU data output register"]
pub mod do6;
#[doc = "DO7 (r) register accessor: HAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@do7`]
module"]
#[doc(alias = "DO7")]
pub type Do7 = crate::Reg<do7::Do7Spec>;
#[doc = "HAU data output register"]
pub mod do7;
#[doc = "INTEN (rw) register accessor: HAU interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "HAU interrupt enable register"]
pub mod inten;
#[doc = "STAT (rw) register accessor: HAU status and interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "HAU status and interrupt flag register"]
pub mod stat;
