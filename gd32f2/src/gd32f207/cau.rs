#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    stat0: Stat0,
    di: Di,
    do_: Do,
    dmaen: Dmaen,
    inten: Inten,
    stat1: Stat1,
    intf: Intf,
    key0h: Key0h,
    key0l: Key0l,
    key1h: Key1h,
    key1l: Key1l,
    key2h: Key2h,
    key2l: Key2l,
    key3h: Key3h,
    key3l: Key3l,
    iv0h: Iv0h,
    iv0l: Iv0l,
    iv1h: Iv1h,
    iv1l: Iv1l,
}
impl RegisterBlock {
    #[doc = "0x00 - CAU control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - CAU status register 0"]
    #[inline(always)]
    pub const fn stat0(&self) -> &Stat0 {
        &self.stat0
    }
    #[doc = "0x08 - CAU data input register"]
    #[inline(always)]
    pub const fn di(&self) -> &Di {
        &self.di
    }
    #[doc = "0x0c - CAU data output register"]
    #[inline(always)]
    pub const fn do_(&self) -> &Do {
        &self.do_
    }
    #[doc = "0x10 - CAU DMA enable register"]
    #[inline(always)]
    pub const fn dmaen(&self) -> &Dmaen {
        &self.dmaen
    }
    #[doc = "0x14 - CAU interrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x18 - CAU interrupt status flag register 1"]
    #[inline(always)]
    pub const fn stat1(&self) -> &Stat1 {
        &self.stat1
    }
    #[doc = "0x1c - CAU enable interrupt status flag register"]
    #[inline(always)]
    pub const fn intf(&self) -> &Intf {
        &self.intf
    }
    #[doc = "0x20 - CAU key register"]
    #[inline(always)]
    pub const fn key0h(&self) -> &Key0h {
        &self.key0h
    }
    #[doc = "0x24 - CAU key register"]
    #[inline(always)]
    pub const fn key0l(&self) -> &Key0l {
        &self.key0l
    }
    #[doc = "0x28 - CAU key register"]
    #[inline(always)]
    pub const fn key1h(&self) -> &Key1h {
        &self.key1h
    }
    #[doc = "0x2c - CAU key register"]
    #[inline(always)]
    pub const fn key1l(&self) -> &Key1l {
        &self.key1l
    }
    #[doc = "0x30 - CAU key register"]
    #[inline(always)]
    pub const fn key2h(&self) -> &Key2h {
        &self.key2h
    }
    #[doc = "0x34 - CAU key register"]
    #[inline(always)]
    pub const fn key2l(&self) -> &Key2l {
        &self.key2l
    }
    #[doc = "0x38 - CAU key register"]
    #[inline(always)]
    pub const fn key3h(&self) -> &Key3h {
        &self.key3h
    }
    #[doc = "0x3c - CAU key register"]
    #[inline(always)]
    pub const fn key3l(&self) -> &Key3l {
        &self.key3l
    }
    #[doc = "0x40 - CAU initialization register"]
    #[inline(always)]
    pub const fn iv0h(&self) -> &Iv0h {
        &self.iv0h
    }
    #[doc = "0x44 - CAU initialization register"]
    #[inline(always)]
    pub const fn iv0l(&self) -> &Iv0l {
        &self.iv0l
    }
    #[doc = "0x48 - CAU initialization register"]
    #[inline(always)]
    pub const fn iv1h(&self) -> &Iv1h {
        &self.iv1h
    }
    #[doc = "0x4c - CAU initialization register"]
    #[inline(always)]
    pub const fn iv1l(&self) -> &Iv1l {
        &self.iv1l
    }
}
#[doc = "CTL (rw) register accessor: CAU control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "CAU control register"]
pub mod ctl;
#[doc = "STAT0 (r) register accessor: CAU status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat0`]
module"]
#[doc(alias = "STAT0")]
pub type Stat0 = crate::Reg<stat0::Stat0Spec>;
#[doc = "CAU status register 0"]
pub mod stat0;
#[doc = "DI (rw) register accessor: CAU data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`di::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`di::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@di`]
module"]
#[doc(alias = "DI")]
pub type Di = crate::Reg<di::DiSpec>;
#[doc = "CAU data input register"]
pub mod di;
#[doc = "DO (r) register accessor: CAU data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`do_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@do_`]
module"]
#[doc(alias = "DO")]
pub type Do = crate::Reg<do_::DoSpec>;
#[doc = "CAU data output register"]
pub mod do_;
#[doc = "DMAEN (rw) register accessor: CAU DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaen`]
module"]
#[doc(alias = "DMAEN")]
pub type Dmaen = crate::Reg<dmaen::DmaenSpec>;
#[doc = "CAU DMA enable register"]
pub mod dmaen;
#[doc = "INTEN (rw) register accessor: CAU interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "CAU interrupt enable register"]
pub mod inten;
#[doc = "STAT1 (r) register accessor: CAU interrupt status flag register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat1`]
module"]
#[doc(alias = "STAT1")]
pub type Stat1 = crate::Reg<stat1::Stat1Spec>;
#[doc = "CAU interrupt status flag register 1"]
pub mod stat1;
#[doc = "INTF (r) register accessor: CAU enable interrupt status flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
#[doc(alias = "INTF")]
pub type Intf = crate::Reg<intf::IntfSpec>;
#[doc = "CAU enable interrupt status flag register"]
pub mod intf;
#[doc = "KEY0H (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0h`]
module"]
#[doc(alias = "KEY0H")]
pub type Key0h = crate::Reg<key0h::Key0hSpec>;
#[doc = "CAU key register"]
pub mod key0h;
#[doc = "KEY0L (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0l::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0l`]
module"]
#[doc(alias = "KEY0L")]
pub type Key0l = crate::Reg<key0l::Key0lSpec>;
#[doc = "CAU key register"]
pub mod key0l;
#[doc = "KEY1H (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1h`]
module"]
#[doc(alias = "KEY1H")]
pub type Key1h = crate::Reg<key1h::Key1hSpec>;
#[doc = "CAU key register"]
pub mod key1h;
#[doc = "KEY1L (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1l::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1l`]
module"]
#[doc(alias = "KEY1L")]
pub type Key1l = crate::Reg<key1l::Key1lSpec>;
#[doc = "CAU key register"]
pub mod key1l;
#[doc = "KEY2H (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2h`]
module"]
#[doc(alias = "KEY2H")]
pub type Key2h = crate::Reg<key2h::Key2hSpec>;
#[doc = "CAU key register"]
pub mod key2h;
#[doc = "KEY2L (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2l::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2l`]
module"]
#[doc(alias = "KEY2L")]
pub type Key2l = crate::Reg<key2l::Key2lSpec>;
#[doc = "CAU key register"]
pub mod key2l;
#[doc = "KEY3H (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key3h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3h`]
module"]
#[doc(alias = "KEY3H")]
pub type Key3h = crate::Reg<key3h::Key3hSpec>;
#[doc = "CAU key register"]
pub mod key3h;
#[doc = "KEY3L (w) register accessor: CAU key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key3l::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3l`]
module"]
#[doc(alias = "KEY3L")]
pub type Key3l = crate::Reg<key3l::Key3lSpec>;
#[doc = "CAU key register"]
pub mod key3l;
#[doc = "IV0H (rw) register accessor: CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv0h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv0h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv0h`]
module"]
#[doc(alias = "IV0H")]
pub type Iv0h = crate::Reg<iv0h::Iv0hSpec>;
#[doc = "CAU initialization register"]
pub mod iv0h;
#[doc = "IV0L (rw) register accessor: CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv0l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv0l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv0l`]
module"]
#[doc(alias = "IV0L")]
pub type Iv0l = crate::Reg<iv0l::Iv0lSpec>;
#[doc = "CAU initialization register"]
pub mod iv0l;
#[doc = "IV1H (rw) register accessor: CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv1h`]
module"]
#[doc(alias = "IV1H")]
pub type Iv1h = crate::Reg<iv1h::Iv1hSpec>;
#[doc = "CAU initialization register"]
pub mod iv1h;
#[doc = "IV1L (rw) register accessor: CAU initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv1l`]
module"]
#[doc(alias = "IV1L")]
pub type Iv1l = crate::Reg<iv1l::Iv1lSpec>;
#[doc = "CAU initialization register"]
pub mod iv1l;
