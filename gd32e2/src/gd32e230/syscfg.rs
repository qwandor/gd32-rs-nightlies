#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg0: Cfg0,
    _reserved1: [u8; 0x04],
    extiss0: Extiss0,
    extiss1: Extiss1,
    extiss2: Extiss2,
    extiss3: Extiss3,
    cfg2: Cfg2,
    _reserved6: [u8; 0xe4],
    cpu_irq_lat: CpuIrqLat,
}
impl RegisterBlock {
    #[doc = "0x00 - System configuration register 0"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x08 - EXTI sources selection register 0"]
    #[inline(always)]
    pub const fn extiss0(&self) -> &Extiss0 {
        &self.extiss0
    }
    #[doc = "0x0c - EXTI sources selection register 1"]
    #[inline(always)]
    pub const fn extiss1(&self) -> &Extiss1 {
        &self.extiss1
    }
    #[doc = "0x10 - EXTI sources selection register 2"]
    #[inline(always)]
    pub const fn extiss2(&self) -> &Extiss2 {
        &self.extiss2
    }
    #[doc = "0x14 - EXTI sources selection register 3"]
    #[inline(always)]
    pub const fn extiss3(&self) -> &Extiss3 {
        &self.extiss3
    }
    #[doc = "0x18 - System configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x100 - IRQ Latency register"]
    #[inline(always)]
    pub const fn cpu_irq_lat(&self) -> &CpuIrqLat {
        &self.cpu_irq_lat
    }
}
#[doc = "CFG0 (rw) register accessor: System configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`]
module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "System configuration register 0"]
pub mod cfg0;
#[doc = "EXTISS0 (rw) register accessor: EXTI sources selection register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extiss0`]
module"]
#[doc(alias = "EXTISS0")]
pub type Extiss0 = crate::Reg<extiss0::Extiss0Spec>;
#[doc = "EXTI sources selection register 0"]
pub mod extiss0;
#[doc = "EXTISS1 (rw) register accessor: EXTI sources selection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extiss1`]
module"]
#[doc(alias = "EXTISS1")]
pub type Extiss1 = crate::Reg<extiss1::Extiss1Spec>;
#[doc = "EXTI sources selection register 1"]
pub mod extiss1;
#[doc = "EXTISS2 (rw) register accessor: EXTI sources selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extiss2`]
module"]
#[doc(alias = "EXTISS2")]
pub type Extiss2 = crate::Reg<extiss2::Extiss2Spec>;
#[doc = "EXTI sources selection register 2"]
pub mod extiss2;
#[doc = "EXTISS3 (rw) register accessor: EXTI sources selection register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extiss3`]
module"]
#[doc(alias = "EXTISS3")]
pub type Extiss3 = crate::Reg<extiss3::Extiss3Spec>;
#[doc = "EXTI sources selection register 3"]
pub mod extiss3;
#[doc = "CFG2 (rw) register accessor: System configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "System configuration register 2"]
pub mod cfg2;
#[doc = "CPU_IRQ_LAT (rw) register accessor: IRQ Latency register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_irq_lat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_irq_lat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_irq_lat`]
module"]
#[doc(alias = "CPU_IRQ_LAT")]
pub type CpuIrqLat = crate::Reg<cpu_irq_lat::CpuIrqLatSpec>;
#[doc = "IRQ Latency register"]
pub mod cpu_irq_lat;
