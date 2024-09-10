#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    swt: Swt,
    dac0_r12dh: Dac0R12dh,
    dac0_l12dh: Dac0L12dh,
    dac0_r8dh: Dac0R8dh,
    _reserved5: [u8; 0x18],
    dac0_do: Dac0Do,
    _reserved6: [u8; 0x04],
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
    #[doc = "0x2c - DAC0 data output register"]
    #[inline(always)]
    pub const fn dac0_do(&self) -> &Dac0Do {
        &self.dac0_do
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
#[doc = "DAC0_DO (r) register accessor: DAC0 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_do::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_do`]
module"]
#[doc(alias = "DAC0_DO")]
pub type Dac0Do = crate::Reg<dac0_do::Dac0DoSpec>;
#[doc = "DAC0 data output register"]
pub mod dac0_do;
#[doc = "STAT (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "status register"]
pub mod stat;
