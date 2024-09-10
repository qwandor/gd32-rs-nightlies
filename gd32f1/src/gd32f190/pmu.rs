#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    cs: Cs,
}
impl RegisterBlock {
    #[doc = "0x00 - power control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - power control/status register"]
    #[inline(always)]
    pub const fn cs(&self) -> &Cs {
        &self.cs
    }
}
#[doc = "CTL (rw) register accessor: power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "power control register"]
pub mod ctl;
#[doc = "CS (rw) register accessor: power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
#[doc(alias = "CS")]
pub type Cs = crate::Reg<cs::CsSpec>;
#[doc = "power control/status register"]
pub mod cs;
