#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: Ctl0,
    cfg0: Cfg0,
    int: Int,
    apb2rst: Apb2rst,
    apb1rst: Apb1rst,
    ahben: Ahben,
    apb2en: Apb2en,
    apb1en: Apb1en,
    bdctl: Bdctl,
    rstsck: Rstsck,
    ahbrst: Ahbrst,
    cfg1: Cfg1,
    cfg2: Cfg2,
    ctl1: Ctl1,
    _reserved14: [u8; 0xc8],
    vkey: Vkey,
    _reserved15: [u8; 0x30],
    dsv: Dsv,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x04 - Clock configuration register 0 (RCU_CFG0)"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x08 - Clock interrupt register (RCU_INT)"]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0x0c - APB2 reset register (RCU_APB2RST)"]
    #[inline(always)]
    pub const fn apb2rst(&self) -> &Apb2rst {
        &self.apb2rst
    }
    #[doc = "0x10 - APB1 reset register (RCU_APB1RST)"]
    #[inline(always)]
    pub const fn apb1rst(&self) -> &Apb1rst {
        &self.apb1rst
    }
    #[doc = "0x14 - AHB enable register (RCU_AHBEN)"]
    #[inline(always)]
    pub const fn ahben(&self) -> &Ahben {
        &self.ahben
    }
    #[doc = "0x18 - APB2 enable register (RCU_APB2EN)"]
    #[inline(always)]
    pub const fn apb2en(&self) -> &Apb2en {
        &self.apb2en
    }
    #[doc = "0x1c - APB1 enable register (RCU_APB1EN)"]
    #[inline(always)]
    pub const fn apb1en(&self) -> &Apb1en {
        &self.apb1en
    }
    #[doc = "0x20 - Backup domain control register (RCU_BDCTL)"]
    #[inline(always)]
    pub const fn bdctl(&self) -> &Bdctl {
        &self.bdctl
    }
    #[doc = "0x24 - Reset source /clock register (RCU_RSTSCK)"]
    #[inline(always)]
    pub const fn rstsck(&self) -> &Rstsck {
        &self.rstsck
    }
    #[doc = "0x28 - AHB reset register"]
    #[inline(always)]
    pub const fn ahbrst(&self) -> &Ahbrst {
        &self.ahbrst
    }
    #[doc = "0x2c - Configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x30 - Configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x34 - Control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x100 - Voltage key register"]
    #[inline(always)]
    pub const fn vkey(&self) -> &Vkey {
        &self.vkey
    }
    #[doc = "0x134 - Deep-sleep mode voltage register"]
    #[inline(always)]
    pub const fn dsv(&self) -> &Dsv {
        &self.dsv
    }
}
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "CFG0 (rw) register accessor: Clock configuration register 0 (RCU_CFG0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`]
module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "Clock configuration register 0 (RCU_CFG0)"]
pub mod cfg0;
#[doc = "INT (rw) register accessor: Clock interrupt register (RCU_INT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
#[doc(alias = "INT")]
pub type Int = crate::Reg<int::IntSpec>;
#[doc = "Clock interrupt register (RCU_INT)"]
pub mod int;
#[doc = "APB2RST (rw) register accessor: APB2 reset register (RCU_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rst`]
module"]
#[doc(alias = "APB2RST")]
pub type Apb2rst = crate::Reg<apb2rst::Apb2rstSpec>;
#[doc = "APB2 reset register (RCU_APB2RST)"]
pub mod apb2rst;
#[doc = "APB1RST (rw) register accessor: APB1 reset register (RCU_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rst`]
module"]
#[doc(alias = "APB1RST")]
pub type Apb1rst = crate::Reg<apb1rst::Apb1rstSpec>;
#[doc = "APB1 reset register (RCU_APB1RST)"]
pub mod apb1rst;
#[doc = "AHBEN (rw) register accessor: AHB enable register (RCU_AHBEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben`]
module"]
#[doc(alias = "AHBEN")]
pub type Ahben = crate::Reg<ahben::AhbenSpec>;
#[doc = "AHB enable register (RCU_AHBEN)"]
pub mod ahben;
#[doc = "APB2EN (rw) register accessor: APB2 enable register (RCU_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2en`]
module"]
#[doc(alias = "APB2EN")]
pub type Apb2en = crate::Reg<apb2en::Apb2enSpec>;
#[doc = "APB2 enable register (RCU_APB2EN)"]
pub mod apb2en;
#[doc = "APB1EN (rw) register accessor: APB1 enable register (RCU_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1en`]
module"]
#[doc(alias = "APB1EN")]
pub type Apb1en = crate::Reg<apb1en::Apb1enSpec>;
#[doc = "APB1 enable register (RCU_APB1EN)"]
pub mod apb1en;
#[doc = "BDCTL (rw) register accessor: Backup domain control register (RCU_BDCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdctl`]
module"]
#[doc(alias = "BDCTL")]
pub type Bdctl = crate::Reg<bdctl::BdctlSpec>;
#[doc = "Backup domain control register (RCU_BDCTL)"]
pub mod bdctl;
#[doc = "RSTSCK (rw) register accessor: Reset source /clock register (RCU_RSTSCK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsck`]
module"]
#[doc(alias = "RSTSCK")]
pub type Rstsck = crate::Reg<rstsck::RstsckSpec>;
#[doc = "Reset source /clock register (RCU_RSTSCK)"]
pub mod rstsck;
#[doc = "AHBRST (rw) register accessor: AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst`]
module"]
#[doc(alias = "AHBRST")]
pub type Ahbrst = crate::Reg<ahbrst::AhbrstSpec>;
#[doc = "AHB reset register"]
pub mod ahbrst;
#[doc = "CFG1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "Configuration register 1"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "Configuration register 2"]
pub mod cfg2;
#[doc = "CTL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "VKEY (w) register accessor: Voltage key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vkey`]
module"]
#[doc(alias = "VKEY")]
pub type Vkey = crate::Reg<vkey::VkeySpec>;
#[doc = "Voltage key register"]
pub mod vkey;
#[doc = "DSV (rw) register accessor: Deep-sleep mode voltage register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsv`]
module"]
#[doc(alias = "DSV")]
pub type Dsv = crate::Reg<dsv::DsvSpec>;
#[doc = "Deep-sleep mode voltage register"]
pub mod dsv;
