#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cs: Cs,
}
impl RegisterBlock {
    #[doc = "0x00 - control and status register"]
    #[inline(always)]
    pub const fn cs(&self) -> &Cs {
        &self.cs
    }
}
#[doc = "CS (rw) register accessor: control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
#[doc(alias = "CS")]
pub type Cs = crate::Reg<cs::CsSpec>;
#[doc = "control and status register"]
pub mod cs;
