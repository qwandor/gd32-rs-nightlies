#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0300],
    ivref_ctl: IvrefCtl,
}
impl RegisterBlock {
    #[doc = "0x300 - IVREF control register"]
    #[inline(always)]
    pub const fn ivref_ctl(&self) -> &IvrefCtl {
        &self.ivref_ctl
    }
}
#[doc = "IVREF_CTL (rw) register accessor: IVREF control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivref_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivref_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivref_ctl`]
module"]
#[doc(alias = "IVREF_CTL")]
pub type IvrefCtl = crate::Reg<ivref_ctl::IvrefCtlSpec>;
#[doc = "IVREF control register"]
pub mod ivref_ctl;
