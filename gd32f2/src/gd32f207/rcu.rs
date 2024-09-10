#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: Ctl0,
    cfg0: Cfg0,
    int: Int,
    apb2rst: Apb2rst,
    apb1rst: Apb1rst,
    ahb1en: Ahb1en,
    apb2en: Apb2en,
    apb1en: Apb1en,
    bdctl: Bdctl,
    rstsck: Rstsck,
    ahb1rst: Ahb1rst,
    cfg1: Cfg1,
    _reserved12: [u8; 0x04],
    dsv: Dsv,
    _reserved13: [u8; 0x28],
    ahb2en: Ahb2en,
    addapb2en: Addapb2en,
    addapb1en: Addapb1en,
    _reserved16: [u8; 0x04],
    ahb2rst: Ahb2rst,
    addapb2rst: Addapb2rst,
    addapb1rst: Addapb1rst,
    _reserved19: [u8; 0x04],
    cfg2: Cfg2,
    _reserved20: [u8; 0x0c],
    plltctl: Plltctl,
    plltint: Plltint,
    plltcfg: Plltcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
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
    #[doc = "0x14 - AHB1 enable register"]
    #[inline(always)]
    pub const fn ahb1en(&self) -> &Ahb1en {
        &self.ahb1en
    }
    #[doc = "0x18 - APB2 clock enable register (RCU_APB2EN)"]
    #[inline(always)]
    pub const fn apb2en(&self) -> &Apb2en {
        &self.apb2en
    }
    #[doc = "0x1c - APB1 clock enable register (RCU_APB1EN)"]
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
    #[doc = "0x28 - AHB1 reset register"]
    #[inline(always)]
    pub const fn ahb1rst(&self) -> &Ahb1rst {
        &self.ahb1rst
    }
    #[doc = "0x2c - Clock Configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x34 - Deep sleep mode Voltage register"]
    #[inline(always)]
    pub const fn dsv(&self) -> &Dsv {
        &self.dsv
    }
    #[doc = "0x60 - AHB2 enable register"]
    #[inline(always)]
    pub const fn ahb2en(&self) -> &Ahb2en {
        &self.ahb2en
    }
    #[doc = "0x64 - APB2 additional enable register"]
    #[inline(always)]
    pub const fn addapb2en(&self) -> &Addapb2en {
        &self.addapb2en
    }
    #[doc = "0x68 - APB1 additional enable register"]
    #[inline(always)]
    pub const fn addapb1en(&self) -> &Addapb1en {
        &self.addapb1en
    }
    #[doc = "0x70 - AHB2 reset register"]
    #[inline(always)]
    pub const fn ahb2rst(&self) -> &Ahb2rst {
        &self.ahb2rst
    }
    #[doc = "0x74 - APB2 additional enable register"]
    #[inline(always)]
    pub const fn addapb2rst(&self) -> &Addapb2rst {
        &self.addapb2rst
    }
    #[doc = "0x78 - APB1 additional enable register"]
    #[inline(always)]
    pub const fn addapb1rst(&self) -> &Addapb1rst {
        &self.addapb1rst
    }
    #[doc = "0x80 - configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x90 - PLLT control register"]
    #[inline(always)]
    pub const fn plltctl(&self) -> &Plltctl {
        &self.plltctl
    }
    #[doc = "0x94 - PLLT interrupt register"]
    #[inline(always)]
    pub const fn plltint(&self) -> &Plltint {
        &self.plltint
    }
    #[doc = "0x98 - PLLT configuration register"]
    #[inline(always)]
    pub const fn plltcfg(&self) -> &Plltcfg {
        &self.plltcfg
    }
}
#[doc = "CTL0 (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control register"]
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
#[doc = "AHB1EN (rw) register accessor: AHB1 enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1en`]
module"]
#[doc(alias = "AHB1EN")]
pub type Ahb1en = crate::Reg<ahb1en::Ahb1enSpec>;
#[doc = "AHB1 enable register"]
pub mod ahb1en;
#[doc = "APB2EN (rw) register accessor: APB2 clock enable register (RCU_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2en`]
module"]
#[doc(alias = "APB2EN")]
pub type Apb2en = crate::Reg<apb2en::Apb2enSpec>;
#[doc = "APB2 clock enable register (RCU_APB2EN)"]
pub mod apb2en;
#[doc = "APB1EN (rw) register accessor: APB1 clock enable register (RCU_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1en`]
module"]
#[doc(alias = "APB1EN")]
pub type Apb1en = crate::Reg<apb1en::Apb1enSpec>;
#[doc = "APB1 clock enable register (RCU_APB1EN)"]
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
#[doc = "AHB1RST (rw) register accessor: AHB1 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rst`]
module"]
#[doc(alias = "AHB1RST")]
pub type Ahb1rst = crate::Reg<ahb1rst::Ahb1rstSpec>;
#[doc = "AHB1 reset register"]
pub mod ahb1rst;
#[doc = "CFG1 (rw) register accessor: Clock Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "Clock Configuration register 1"]
pub mod cfg1;
#[doc = "DSV (rw) register accessor: Deep sleep mode Voltage register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsv`]
module"]
#[doc(alias = "DSV")]
pub type Dsv = crate::Reg<dsv::DsvSpec>;
#[doc = "Deep sleep mode Voltage register"]
pub mod dsv;
#[doc = "AHB2EN (rw) register accessor: AHB2 enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2en`]
module"]
#[doc(alias = "AHB2EN")]
pub type Ahb2en = crate::Reg<ahb2en::Ahb2enSpec>;
#[doc = "AHB2 enable register"]
pub mod ahb2en;
#[doc = "ADDAPB2EN (rw) register accessor: APB2 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addapb2en`]
module"]
#[doc(alias = "ADDAPB2EN")]
pub type Addapb2en = crate::Reg<addapb2en::Addapb2enSpec>;
#[doc = "APB2 additional enable register"]
pub mod addapb2en;
#[doc = "ADDAPB1EN (rw) register accessor: APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addapb1en`]
module"]
#[doc(alias = "ADDAPB1EN")]
pub type Addapb1en = crate::Reg<addapb1en::Addapb1enSpec>;
#[doc = "APB1 additional enable register"]
pub mod addapb1en;
#[doc = "AHB2RST (rw) register accessor: AHB2 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rst`]
module"]
#[doc(alias = "AHB2RST")]
pub type Ahb2rst = crate::Reg<ahb2rst::Ahb2rstSpec>;
#[doc = "AHB2 reset register"]
pub mod ahb2rst;
#[doc = "ADDAPB2RST (rw) register accessor: APB2 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addapb2rst`]
module"]
#[doc(alias = "ADDAPB2RST")]
pub type Addapb2rst = crate::Reg<addapb2rst::Addapb2rstSpec>;
#[doc = "APB2 additional enable register"]
pub mod addapb2rst;
#[doc = "ADDAPB1RST (rw) register accessor: APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addapb1rst`]
module"]
#[doc(alias = "ADDAPB1RST")]
pub type Addapb1rst = crate::Reg<addapb1rst::Addapb1rstSpec>;
#[doc = "APB1 additional enable register"]
pub mod addapb1rst;
#[doc = "CFG2 (rw) register accessor: configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "configuration register 2"]
pub mod cfg2;
#[doc = "PLLTCTL (rw) register accessor: PLLT control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plltctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plltctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plltctl`]
module"]
#[doc(alias = "PLLTCTL")]
pub type Plltctl = crate::Reg<plltctl::PlltctlSpec>;
#[doc = "PLLT control register"]
pub mod plltctl;
#[doc = "PLLTINT (rw) register accessor: PLLT interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plltint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plltint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plltint`]
module"]
#[doc(alias = "PLLTINT")]
pub type Plltint = crate::Reg<plltint::PlltintSpec>;
#[doc = "PLLT interrupt register"]
pub mod plltint;
#[doc = "PLLTCFG (rw) register accessor: PLLT configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plltcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plltcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plltcfg`]
module"]
#[doc(alias = "PLLTCFG")]
pub type Plltcfg = crate::Reg<plltcfg::PlltcfgSpec>;
#[doc = "PLLT configuration register"]
pub mod plltcfg;
