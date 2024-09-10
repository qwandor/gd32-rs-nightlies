#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sstat: Sstat,
    syncctl: Syncctl,
    syncdata: Syncdata,
}
impl RegisterBlock {
    #[doc = "0x00 - summary status register"]
    #[inline(always)]
    pub const fn sstat(&self) -> &Sstat {
        &self.sstat
    }
    #[doc = "0x04 - sync control register"]
    #[inline(always)]
    pub const fn syncctl(&self) -> &Syncctl {
        &self.syncctl
    }
    #[doc = "0x08 - Sync regular data register"]
    #[inline(always)]
    pub const fn syncdata(&self) -> &Syncdata {
        &self.syncdata
    }
}
#[doc = "SSTAT (r) register accessor: summary status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstat`]
module"]
#[doc(alias = "SSTAT")]
pub type Sstat = crate::Reg<sstat::SstatSpec>;
#[doc = "summary status register"]
pub mod sstat;
#[doc = "SYNCCTL (rw) register accessor: sync control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncctl`]
module"]
#[doc(alias = "SYNCCTL")]
pub type Syncctl = crate::Reg<syncctl::SyncctlSpec>;
#[doc = "sync control register"]
pub mod syncctl;
#[doc = "SYNCDATA (r) register accessor: Sync regular data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncdata`]
module"]
#[doc(alias = "SYNCDATA")]
pub type Syncdata = crate::Reg<syncdata::SyncdataSpec>;
#[doc = "Sync regular data register"]
pub mod syncdata;
