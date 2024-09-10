#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x5c],
    opa_ctl: OpaCtl,
    opa_bt: OpaBt,
    opa_lpbt: OpaLpbt,
    _reserved3: [u8; 0x0298],
    ivref_ctl: IvrefCtl,
}
impl RegisterBlock {
    #[doc = "0x5c - OPA control register"]
    #[inline(always)]
    pub const fn opa_ctl(&self) -> &OpaCtl {
        &self.opa_ctl
    }
    #[doc = "0x60 - OPA offset trimming for normal mode register"]
    #[inline(always)]
    pub const fn opa_bt(&self) -> &OpaBt {
        &self.opa_bt
    }
    #[doc = "0x64 - OPA offset trimming for low power mode register"]
    #[inline(always)]
    pub const fn opa_lpbt(&self) -> &OpaLpbt {
        &self.opa_lpbt
    }
    #[doc = "0x300 - IVREF control register"]
    #[inline(always)]
    pub const fn ivref_ctl(&self) -> &IvrefCtl {
        &self.ivref_ctl
    }
}
#[doc = "OPA_CTL (rw) register accessor: OPA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa_ctl`]
module"]
#[doc(alias = "OPA_CTL")]
pub type OpaCtl = crate::Reg<opa_ctl::OpaCtlSpec>;
#[doc = "OPA control register"]
pub mod opa_ctl;
#[doc = "OPA_BT (rw) register accessor: OPA offset trimming for normal mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_bt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_bt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa_bt`]
module"]
#[doc(alias = "OPA_BT")]
pub type OpaBt = crate::Reg<opa_bt::OpaBtSpec>;
#[doc = "OPA offset trimming for normal mode register"]
pub mod opa_bt;
#[doc = "OPA_LPBT (rw) register accessor: OPA offset trimming for low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa_lpbt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa_lpbt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opa_lpbt`]
module"]
#[doc(alias = "OPA_LPBT")]
pub type OpaLpbt = crate::Reg<opa_lpbt::OpaLpbtSpec>;
#[doc = "OPA offset trimming for low power mode register"]
pub mod opa_lpbt;
#[doc = "IVREF_CTL (rw) register accessor: IVREF control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivref_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivref_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivref_ctl`]
module"]
#[doc(alias = "IVREF_CTL")]
pub type IvrefCtl = crate::Reg<ivref_ctl::IvrefCtlSpec>;
#[doc = "IVREF control register"]
pub mod ivref_ctl;
