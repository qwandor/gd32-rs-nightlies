#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ec: Ec,
    pcf0: Pcf0,
    extiss0: Extiss0,
    extiss1: Extiss1,
    extiss2: Extiss2,
    extiss3: Extiss3,
    _reserved6: [u8; 0x04],
    pcf1: Pcf1,
    _reserved7: [u8; 0x1c],
    pcf2: Pcf2,
    pcf3: Pcf3,
    pcf4: Pcf4,
    pcf5: Pcf5,
}
impl RegisterBlock {
    #[doc = "0x00 - Event control register"]
    #[inline(always)]
    pub const fn ec(&self) -> &Ec {
        &self.ec
    }
    #[doc = "0x04 - AFIO port configuration register 0"]
    #[inline(always)]
    pub const fn pcf0(&self) -> &Pcf0 {
        &self.pcf0
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
    #[doc = "0x1c - AFIO port configuration register 1"]
    #[inline(always)]
    pub const fn pcf1(&self) -> &Pcf1 {
        &self.pcf1
    }
    #[doc = "0x3c - AFIO port configuration register 2"]
    #[inline(always)]
    pub const fn pcf2(&self) -> &Pcf2 {
        &self.pcf2
    }
    #[doc = "0x40 - AFIO port configuration register 3"]
    #[inline(always)]
    pub const fn pcf3(&self) -> &Pcf3 {
        &self.pcf3
    }
    #[doc = "0x44 - AFIO port configuration register 4"]
    #[inline(always)]
    pub const fn pcf4(&self) -> &Pcf4 {
        &self.pcf4
    }
    #[doc = "0x48 - AFIO port configuration register 5"]
    #[inline(always)]
    pub const fn pcf5(&self) -> &Pcf5 {
        &self.pcf5
    }
}
#[doc = "EC (rw) register accessor: Event control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ec`]
module"]
#[doc(alias = "EC")]
pub type Ec = crate::Reg<ec::EcSpec>;
#[doc = "Event control register"]
pub mod ec;
#[doc = "PCF0 (rw) register accessor: AFIO port configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcf0`]
module"]
#[doc(alias = "PCF0")]
pub type Pcf0 = crate::Reg<pcf0::Pcf0Spec>;
#[doc = "AFIO port configuration register 0"]
pub mod pcf0;
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
#[doc = "PCF1 (rw) register accessor: AFIO port configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcf1`]
module"]
#[doc(alias = "PCF1")]
pub type Pcf1 = crate::Reg<pcf1::Pcf1Spec>;
#[doc = "AFIO port configuration register 1"]
pub mod pcf1;
#[doc = "PCF2 (rw) register accessor: AFIO port configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcf2`]
module"]
#[doc(alias = "PCF2")]
pub type Pcf2 = crate::Reg<pcf2::Pcf2Spec>;
#[doc = "AFIO port configuration register 2"]
pub mod pcf2;
#[doc = "PCF3 (rw) register accessor: AFIO port configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcf3`]
module"]
#[doc(alias = "PCF3")]
pub type Pcf3 = crate::Reg<pcf3::Pcf3Spec>;
#[doc = "AFIO port configuration register 3"]
pub mod pcf3;
#[doc = "PCF4 (rw) register accessor: AFIO port configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcf4`]
module"]
#[doc(alias = "PCF4")]
pub type Pcf4 = crate::Reg<pcf4::Pcf4Spec>;
#[doc = "AFIO port configuration register 4"]
pub mod pcf4;
#[doc = "PCF5 (rw) register accessor: AFIO port configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcf5`]
module"]
#[doc(alias = "PCF5")]
pub type Pcf5 = crate::Reg<pcf5::Pcf5Spec>;
#[doc = "AFIO port configuration register 5"]
pub mod pcf5;
