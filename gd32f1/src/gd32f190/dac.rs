#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    swt: Swt,
    dac0_r12dh: Dac0R12dh,
    dac0_l12dh: Dac0L12dh,
    dac0_r8dh: Dac0R8dh,
    dac1_r12dh: Dac1R12dh,
    dac1_l12dh: Dac1L12dh,
    dac1_r8dh: Dac1R8dh,
    dacc_r12dh: DaccR12dh,
    dacc_l12dh: DaccL12dh,
    dacc_r8dh: DaccR8dh,
    dac0_do: Dac0Do,
    dac1_do: Dac1Do,
    stat: Stat,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - software trigger register"]
    #[inline(always)]
    pub const fn swt(&self) -> &Swt {
        &self.swt
    }
    #[doc = "0x08 - DAC0 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dac0_r12dh(&self) -> &Dac0R12dh {
        &self.dac0_r12dh
    }
    #[doc = "0x0c - DAC0 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dac0_l12dh(&self) -> &Dac0L12dh {
        &self.dac0_l12dh
    }
    #[doc = "0x10 - DAC0 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dac0_r8dh(&self) -> &Dac0R8dh {
        &self.dac0_r8dh
    }
    #[doc = "0x14 - DAC1 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dac1_r12dh(&self) -> &Dac1R12dh {
        &self.dac1_r12dh
    }
    #[doc = "0x18 - DAC1 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dac1_l12dh(&self) -> &Dac1L12dh {
        &self.dac1_l12dh
    }
    #[doc = "0x1c - DAC1 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dac1_r8dh(&self) -> &Dac1R8dh {
        &self.dac1_r8dh
    }
    #[doc = "0x20 - DAC concurrent mode 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dacc_r12dh(&self) -> &DaccR12dh {
        &self.dacc_r12dh
    }
    #[doc = "0x24 - DAC concurrent mode 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dacc_l12dh(&self) -> &DaccL12dh {
        &self.dacc_l12dh
    }
    #[doc = "0x28 - DAC concurrent mode 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dacc_r8dh(&self) -> &DaccR8dh {
        &self.dacc_r8dh
    }
    #[doc = "0x2c - DAC0 data output register"]
    #[inline(always)]
    pub const fn dac0_do(&self) -> &Dac0Do {
        &self.dac0_do
    }
    #[doc = "0x30 - DAC1 data output register"]
    #[inline(always)]
    pub const fn dac1_do(&self) -> &Dac1Do {
        &self.dac1_do
    }
    #[doc = "0x34 - status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "control register"]
pub mod ctl;
#[doc = "SWT (w) register accessor: software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swt`]
module"]
#[doc(alias = "SWT")]
pub type Swt = crate::Reg<swt::SwtSpec>;
#[doc = "software trigger register"]
pub mod swt;
#[doc = "DAC0_R12DH (rw) register accessor: DAC0 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_r12dh`]
module"]
#[doc(alias = "DAC0_R12DH")]
pub type Dac0R12dh = crate::Reg<dac0_r12dh::Dac0R12dhSpec>;
#[doc = "DAC0 12-bit right-aligned data holding register"]
pub mod dac0_r12dh;
#[doc = "DAC0_L12DH (rw) register accessor: DAC0 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_l12dh`]
module"]
#[doc(alias = "DAC0_L12DH")]
pub type Dac0L12dh = crate::Reg<dac0_l12dh::Dac0L12dhSpec>;
#[doc = "DAC0 12-bit left aligned data holding register"]
pub mod dac0_l12dh;
#[doc = "DAC0_R8DH (rw) register accessor: DAC0 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_r8dh`]
module"]
#[doc(alias = "DAC0_R8DH")]
pub type Dac0R8dh = crate::Reg<dac0_r8dh::Dac0R8dhSpec>;
#[doc = "DAC0 8-bit right aligned data holding register"]
pub mod dac0_r8dh;
#[doc = "DAC1_R12DH (rw) register accessor: DAC1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_r12dh`]
module"]
#[doc(alias = "DAC1_R12DH")]
pub type Dac1R12dh = crate::Reg<dac1_r12dh::Dac1R12dhSpec>;
#[doc = "DAC1 12-bit right-aligned data holding register"]
pub mod dac1_r12dh;
#[doc = "DAC1_L12DH (rw) register accessor: DAC1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_l12dh`]
module"]
#[doc(alias = "DAC1_L12DH")]
pub type Dac1L12dh = crate::Reg<dac1_l12dh::Dac1L12dhSpec>;
#[doc = "DAC1 12-bit left aligned data holding register"]
pub mod dac1_l12dh;
#[doc = "DAC1_R8DH (rw) register accessor: DAC1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_r8dh`]
module"]
#[doc(alias = "DAC1_R8DH")]
pub type Dac1R8dh = crate::Reg<dac1_r8dh::Dac1R8dhSpec>;
#[doc = "DAC1 8-bit right aligned data holding register"]
pub mod dac1_r8dh;
#[doc = "DACC_R12DH (rw) register accessor: DAC concurrent mode 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacc_r12dh`]
module"]
#[doc(alias = "DACC_R12DH")]
pub type DaccR12dh = crate::Reg<dacc_r12dh::DaccR12dhSpec>;
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register"]
pub mod dacc_r12dh;
#[doc = "DACC_L12DH (rw) register accessor: DAC concurrent mode 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacc_l12dh`]
module"]
#[doc(alias = "DACC_L12DH")]
pub type DaccL12dh = crate::Reg<dacc_l12dh::DaccL12dhSpec>;
#[doc = "DAC concurrent mode 12-bit left aligned data holding register"]
pub mod dacc_l12dh;
#[doc = "DACC_R8DH (rw) register accessor: DAC concurrent mode 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacc_r8dh`]
module"]
#[doc(alias = "DACC_R8DH")]
pub type DaccR8dh = crate::Reg<dacc_r8dh::DaccR8dhSpec>;
#[doc = "DAC concurrent mode 8-bit right aligned data holding register"]
pub mod dacc_r8dh;
#[doc = "DAC0_DO (r) register accessor: DAC0 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_do::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_do`]
module"]
#[doc(alias = "DAC0_DO")]
pub type Dac0Do = crate::Reg<dac0_do::Dac0DoSpec>;
#[doc = "DAC0 data output register"]
pub mod dac0_do;
#[doc = "DAC1_DO (r) register accessor: DAC1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_do::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_do`]
module"]
#[doc(alias = "DAC1_DO")]
pub type Dac1Do = crate::Reg<dac1_do::Dac1DoSpec>;
#[doc = "DAC1 data output register"]
pub mod dac1_do;
#[doc = "STAT (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "status register"]
pub mod stat;
