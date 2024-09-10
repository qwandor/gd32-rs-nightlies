#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    snctl: Snctl,
    sntcfg: Sntcfg,
    _reserved2: [u8; 0xfc],
    snwtcfg: Snwtcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM/NOR Flash control registers"]
    #[inline(always)]
    pub const fn snctl(&self) -> &Snctl {
        &self.snctl
    }
    #[doc = "0x04 - SRAM/NOR Flash timing configuration registers"]
    #[inline(always)]
    pub const fn sntcfg(&self) -> &Sntcfg {
        &self.sntcfg
    }
    #[doc = "0x104 - SRAM/NOR write timing configuration registers"]
    #[inline(always)]
    pub const fn snwtcfg(&self) -> &Snwtcfg {
        &self.snwtcfg
    }
}
#[doc = "SNCTL (rw) register accessor: SRAM/NOR Flash control registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snctl`]
module"]
#[doc(alias = "SNCTL")]
pub type Snctl = crate::Reg<snctl::SnctlSpec>;
#[doc = "SRAM/NOR Flash control registers"]
pub mod snctl;
#[doc = "SNTCFG (rw) register accessor: SRAM/NOR Flash timing configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sntcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sntcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sntcfg`]
module"]
#[doc(alias = "SNTCFG")]
pub type Sntcfg = crate::Reg<sntcfg::SntcfgSpec>;
#[doc = "SRAM/NOR Flash timing configuration registers"]
pub mod sntcfg;
#[doc = "SNWTCFG (rw) register accessor: SRAM/NOR write timing configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snwtcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snwtcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snwtcfg`]
module"]
#[doc(alias = "SNWTCFG")]
pub type Snwtcfg = crate::Reg<snwtcfg::SnwtcfgSpec>;
#[doc = "SRAM/NOR write timing configuration registers"]
pub mod snwtcfg;
