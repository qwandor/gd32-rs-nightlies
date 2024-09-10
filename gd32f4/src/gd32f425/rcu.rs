#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: Ctl0,
    pll: Pll,
    cfg0: Cfg0,
    int: Int,
    ahb1rst: Ahb1rst,
    ahb2rst: Ahb2rst,
    ahb3rst: Ahb3rst,
    _reserved7: [u8; 0x04],
    apb1rst: Apb1rst,
    apb2rst: Apb2rst,
    _reserved9: [u8; 0x08],
    ahb1en: Ahb1en,
    ahb2en: Ahb2en,
    ahb3en: Ahb3en,
    _reserved12: [u8; 0x04],
    apb1en: Apb1en,
    apb2en: Apb2en,
    _reserved14: [u8; 0x08],
    ahb1spen: Ahb1spen,
    ahb2spen: Ahb2spen,
    ahb3spen: Ahb3spen,
    _reserved17: [u8; 0x04],
    apb1spen: Apb1spen,
    apb2spen: Apb2spen,
    _reserved19: [u8; 0x08],
    bdctl: Bdctl,
    rstsck: Rstsck,
    _reserved21: [u8; 0x08],
    pllssctl: Pllssctl,
    plli2s: Plli2s,
    pllsai: Pllsai,
    cfg1: Cfg1,
    _reserved25: [u8; 0x30],
    addctl: Addctl,
    _reserved26: [u8; 0x08],
    addint: Addint,
    _reserved27: [u8; 0x10],
    addapb1rst: Addapb1rst,
    addapb1en: Addapb1en,
    addapb1spen: Addapb1spen,
    _reserved30: [u8; 0x14],
    vkey: Vkey,
    _reserved31: [u8; 0x30],
    dsv: Dsv,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x04 - PLL register (RCU_PLL)"]
    #[inline(always)]
    pub const fn pll(&self) -> &Pll {
        &self.pll
    }
    #[doc = "0x08 - Clock configuration register 0 (RCU_CFG0)"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x0c - Clock interrupt register (RCU_INT)"]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0x10 - AHB1 reset register"]
    #[inline(always)]
    pub const fn ahb1rst(&self) -> &Ahb1rst {
        &self.ahb1rst
    }
    #[doc = "0x14 - AHB2 reset register"]
    #[inline(always)]
    pub const fn ahb2rst(&self) -> &Ahb2rst {
        &self.ahb2rst
    }
    #[doc = "0x18 - AHB3 reset register"]
    #[inline(always)]
    pub const fn ahb3rst(&self) -> &Ahb3rst {
        &self.ahb3rst
    }
    #[doc = "0x20 - APB1 reset register (RCU_APB1RST)"]
    #[inline(always)]
    pub const fn apb1rst(&self) -> &Apb1rst {
        &self.apb1rst
    }
    #[doc = "0x24 - APB2 reset register (RCU_APB2RST)"]
    #[inline(always)]
    pub const fn apb2rst(&self) -> &Apb2rst {
        &self.apb2rst
    }
    #[doc = "0x30 - AHB1 enable register"]
    #[inline(always)]
    pub const fn ahb1en(&self) -> &Ahb1en {
        &self.ahb1en
    }
    #[doc = "0x34 - AHB2 enable register"]
    #[inline(always)]
    pub const fn ahb2en(&self) -> &Ahb2en {
        &self.ahb2en
    }
    #[doc = "0x38 - AHB3 clock enable register"]
    #[inline(always)]
    pub const fn ahb3en(&self) -> &Ahb3en {
        &self.ahb3en
    }
    #[doc = "0x40 - APB1 clock enable register (RCU_APB1EN)"]
    #[inline(always)]
    pub const fn apb1en(&self) -> &Apb1en {
        &self.apb1en
    }
    #[doc = "0x44 - APB2 clock enable register (RCU_APB2EN)"]
    #[inline(always)]
    pub const fn apb2en(&self) -> &Apb2en {
        &self.apb2en
    }
    #[doc = "0x50 - AHB1 sleep mode enable register"]
    #[inline(always)]
    pub const fn ahb1spen(&self) -> &Ahb1spen {
        &self.ahb1spen
    }
    #[doc = "0x54 - AHB2 sleep mode enable register"]
    #[inline(always)]
    pub const fn ahb2spen(&self) -> &Ahb2spen {
        &self.ahb2spen
    }
    #[doc = "0x58 - AHB3 Sleep mode enable register"]
    #[inline(always)]
    pub const fn ahb3spen(&self) -> &Ahb3spen {
        &self.ahb3spen
    }
    #[doc = "0x60 - APB1 sleep mode clock enable register (RCU_APB1EN)"]
    #[inline(always)]
    pub const fn apb1spen(&self) -> &Apb1spen {
        &self.apb1spen
    }
    #[doc = "0x64 - APB2 sleep mode enable register (RCU_APB2SPEN)"]
    #[inline(always)]
    pub const fn apb2spen(&self) -> &Apb2spen {
        &self.apb2spen
    }
    #[doc = "0x70 - Backup domain control register (RCU_BDCTL)"]
    #[inline(always)]
    pub const fn bdctl(&self) -> &Bdctl {
        &self.bdctl
    }
    #[doc = "0x74 - Reset source /clock register (RCU_RSTSCK)"]
    #[inline(always)]
    pub const fn rstsck(&self) -> &Rstsck {
        &self.rstsck
    }
    #[doc = "0x80 - PLL clock spread spectrum control register (RCU_PLLSSCTL)"]
    #[inline(always)]
    pub const fn pllssctl(&self) -> &Pllssctl {
        &self.pllssctl
    }
    #[doc = "0x84 - PLLI2S register (RCU_PLLI2S)"]
    #[inline(always)]
    pub const fn plli2s(&self) -> &Plli2s {
        &self.plli2s
    }
    #[doc = "0x88 - PLLSAI register (RCU_PLLSAI)"]
    #[inline(always)]
    pub const fn pllsai(&self) -> &Pllsai {
        &self.pllsai
    }
    #[doc = "0x8c - Clock Configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0xc0 - Additional clock control register"]
    #[inline(always)]
    pub const fn addctl(&self) -> &Addctl {
        &self.addctl
    }
    #[doc = "0xcc - Additional clock interrupt register"]
    #[inline(always)]
    pub const fn addint(&self) -> &Addint {
        &self.addint
    }
    #[doc = "0xe0 - APB1 additional reset register"]
    #[inline(always)]
    pub const fn addapb1rst(&self) -> &Addapb1rst {
        &self.addapb1rst
    }
    #[doc = "0xe4 - APB1 additional enable register"]
    #[inline(always)]
    pub const fn addapb1en(&self) -> &Addapb1en {
        &self.addapb1en
    }
    #[doc = "0xe8 - APB1 additional sleep mode enable register"]
    #[inline(always)]
    pub const fn addapb1spen(&self) -> &Addapb1spen {
        &self.addapb1spen
    }
    #[doc = "0x100 - Voltage key register"]
    #[inline(always)]
    pub const fn vkey(&self) -> &Vkey {
        &self.vkey
    }
    #[doc = "0x134 - Deep sleep mode Voltage register"]
    #[inline(always)]
    pub const fn dsv(&self) -> &Dsv {
        &self.dsv
    }
}
#[doc = "CTL0 (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control register"]
pub mod ctl0;
#[doc = "PLL (rw) register accessor: PLL register (RCU_PLL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll`]
module"]
#[doc(alias = "PLL")]
pub type Pll = crate::Reg<pll::PllSpec>;
#[doc = "PLL register (RCU_PLL)"]
pub mod pll;
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
#[doc = "AHB1RST (rw) register accessor: AHB1 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rst`]
module"]
#[doc(alias = "AHB1RST")]
pub type Ahb1rst = crate::Reg<ahb1rst::Ahb1rstSpec>;
#[doc = "AHB1 reset register"]
pub mod ahb1rst;
#[doc = "AHB2RST (rw) register accessor: AHB2 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rst`]
module"]
#[doc(alias = "AHB2RST")]
pub type Ahb2rst = crate::Reg<ahb2rst::Ahb2rstSpec>;
#[doc = "AHB2 reset register"]
pub mod ahb2rst;
#[doc = "AHB3RST (rw) register accessor: AHB3 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3rst`]
module"]
#[doc(alias = "AHB3RST")]
pub type Ahb3rst = crate::Reg<ahb3rst::Ahb3rstSpec>;
#[doc = "AHB3 reset register"]
pub mod ahb3rst;
#[doc = "APB1RST (rw) register accessor: APB1 reset register (RCU_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rst`]
module"]
#[doc(alias = "APB1RST")]
pub type Apb1rst = crate::Reg<apb1rst::Apb1rstSpec>;
#[doc = "APB1 reset register (RCU_APB1RST)"]
pub mod apb1rst;
#[doc = "APB2RST (rw) register accessor: APB2 reset register (RCU_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rst`]
module"]
#[doc(alias = "APB2RST")]
pub type Apb2rst = crate::Reg<apb2rst::Apb2rstSpec>;
#[doc = "APB2 reset register (RCU_APB2RST)"]
pub mod apb2rst;
#[doc = "AHB1EN (rw) register accessor: AHB1 enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1en`]
module"]
#[doc(alias = "AHB1EN")]
pub type Ahb1en = crate::Reg<ahb1en::Ahb1enSpec>;
#[doc = "AHB1 enable register"]
pub mod ahb1en;
#[doc = "AHB2EN (rw) register accessor: AHB2 enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2en`]
module"]
#[doc(alias = "AHB2EN")]
pub type Ahb2en = crate::Reg<ahb2en::Ahb2enSpec>;
#[doc = "AHB2 enable register"]
pub mod ahb2en;
#[doc = "AHB3EN (rw) register accessor: AHB3 clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3en`]
module"]
#[doc(alias = "AHB3EN")]
pub type Ahb3en = crate::Reg<ahb3en::Ahb3enSpec>;
#[doc = "AHB3 clock enable register"]
pub mod ahb3en;
#[doc = "APB1EN (rw) register accessor: APB1 clock enable register (RCU_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1en`]
module"]
#[doc(alias = "APB1EN")]
pub type Apb1en = crate::Reg<apb1en::Apb1enSpec>;
#[doc = "APB1 clock enable register (RCU_APB1EN)"]
pub mod apb1en;
#[doc = "APB2EN (rw) register accessor: APB2 clock enable register (RCU_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2en`]
module"]
#[doc(alias = "APB2EN")]
pub type Apb2en = crate::Reg<apb2en::Apb2enSpec>;
#[doc = "APB2 clock enable register (RCU_APB2EN)"]
pub mod apb2en;
#[doc = "AHB1SPEN (rw) register accessor: AHB1 sleep mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1spen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1spen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1spen`]
module"]
#[doc(alias = "AHB1SPEN")]
pub type Ahb1spen = crate::Reg<ahb1spen::Ahb1spenSpec>;
#[doc = "AHB1 sleep mode enable register"]
pub mod ahb1spen;
#[doc = "AHB2SPEN (rw) register accessor: AHB2 sleep mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2spen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2spen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2spen`]
module"]
#[doc(alias = "AHB2SPEN")]
pub type Ahb2spen = crate::Reg<ahb2spen::Ahb2spenSpec>;
#[doc = "AHB2 sleep mode enable register"]
pub mod ahb2spen;
#[doc = "AHB3SPEN (rw) register accessor: AHB3 Sleep mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3spen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3spen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3spen`]
module"]
#[doc(alias = "AHB3SPEN")]
pub type Ahb3spen = crate::Reg<ahb3spen::Ahb3spenSpec>;
#[doc = "AHB3 Sleep mode enable register"]
pub mod ahb3spen;
#[doc = "APB1SPEN (rw) register accessor: APB1 sleep mode clock enable register (RCU_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1spen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1spen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1spen`]
module"]
#[doc(alias = "APB1SPEN")]
pub type Apb1spen = crate::Reg<apb1spen::Apb1spenSpec>;
#[doc = "APB1 sleep mode clock enable register (RCU_APB1EN)"]
pub mod apb1spen;
#[doc = "APB2SPEN (rw) register accessor: APB2 sleep mode enable register (RCU_APB2SPEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2spen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2spen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2spen`]
module"]
#[doc(alias = "APB2SPEN")]
pub type Apb2spen = crate::Reg<apb2spen::Apb2spenSpec>;
#[doc = "APB2 sleep mode enable register (RCU_APB2SPEN)"]
pub mod apb2spen;
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
#[doc = "PLLSSCTL (rw) register accessor: PLL clock spread spectrum control register (RCU_PLLSSCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllssctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllssctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllssctl`]
module"]
#[doc(alias = "PLLSSCTL")]
pub type Pllssctl = crate::Reg<pllssctl::PllssctlSpec>;
#[doc = "PLL clock spread spectrum control register (RCU_PLLSSCTL)"]
pub mod pllssctl;
#[doc = "PLLI2S (rw) register accessor: PLLI2S register (RCU_PLLI2S)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plli2s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plli2s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plli2s`]
module"]
#[doc(alias = "PLLI2S")]
pub type Plli2s = crate::Reg<plli2s::Plli2sSpec>;
#[doc = "PLLI2S register (RCU_PLLI2S)"]
pub mod plli2s;
#[doc = "PLLSAI (rw) register accessor: PLLSAI register (RCU_PLLSAI)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsai::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsai::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllsai`]
module"]
#[doc(alias = "PLLSAI")]
pub type Pllsai = crate::Reg<pllsai::PllsaiSpec>;
#[doc = "PLLSAI register (RCU_PLLSAI)"]
pub mod pllsai;
#[doc = "CFG1 (rw) register accessor: Clock Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "Clock Configuration register 1"]
pub mod cfg1;
#[doc = "ADDCTL (rw) register accessor: Additional clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addctl`]
module"]
#[doc(alias = "ADDCTL")]
pub type Addctl = crate::Reg<addctl::AddctlSpec>;
#[doc = "Additional clock control register"]
pub mod addctl;
#[doc = "ADDINT (rw) register accessor: Additional clock interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addint`]
module"]
#[doc(alias = "ADDINT")]
pub type Addint = crate::Reg<addint::AddintSpec>;
#[doc = "Additional clock interrupt register"]
pub mod addint;
#[doc = "ADDAPB1RST (rw) register accessor: APB1 additional reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addapb1rst`]
module"]
#[doc(alias = "ADDAPB1RST")]
pub type Addapb1rst = crate::Reg<addapb1rst::Addapb1rstSpec>;
#[doc = "APB1 additional reset register"]
pub mod addapb1rst;
#[doc = "ADDAPB1EN (rw) register accessor: APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addapb1en`]
module"]
#[doc(alias = "ADDAPB1EN")]
pub type Addapb1en = crate::Reg<addapb1en::Addapb1enSpec>;
#[doc = "APB1 additional enable register"]
pub mod addapb1en;
#[doc = "ADDAPB1SPEN (rw) register accessor: APB1 additional sleep mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1spen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1spen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addapb1spen`]
module"]
#[doc(alias = "ADDAPB1SPEN")]
pub type Addapb1spen = crate::Reg<addapb1spen::Addapb1spenSpec>;
#[doc = "APB1 additional sleep mode enable register"]
pub mod addapb1spen;
#[doc = "VKEY (w) register accessor: Voltage key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vkey`]
module"]
#[doc(alias = "VKEY")]
pub type Vkey = crate::Reg<vkey::VkeySpec>;
#[doc = "Voltage key register"]
pub mod vkey;
#[doc = "DSV (rw) register accessor: Deep sleep mode Voltage register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsv`]
module"]
#[doc(alias = "DSV")]
pub type Dsv = crate::Reg<dsv::DsvSpec>;
#[doc = "Deep sleep mode Voltage register"]
pub mod dsv;
