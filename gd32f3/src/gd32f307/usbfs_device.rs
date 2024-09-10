#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dcfg: Dcfg,
    dctl: Dctl,
    dstat: Dstat,
    _reserved3: [u8; 0x04],
    diepinten: Diepinten,
    doepinten: Doepinten,
    daepint: Daepint,
    daepinten: Daepinten,
    _reserved7: [u8; 0x08],
    dvbusdt: Dvbusdt,
    dvbuspt: Dvbuspt,
    _reserved9: [u8; 0x04],
    diepfeinten: Diepfeinten,
    _reserved10: [u8; 0xc8],
    diep0ctl: Diep0ctl,
    _reserved11: [u8; 0x04],
    diep0intf: Diep0intf,
    _reserved12: [u8; 0x04],
    diep0len: Diep0len,
    _reserved13: [u8; 0x04],
    diep0tfstat: Diep0tfstat,
    _reserved14: [u8; 0x04],
    diep1ctl: Diep1ctl,
    _reserved15: [u8; 0x04],
    diep1intf: Diep1intf,
    _reserved16: [u8; 0x04],
    diep1len: Diep1len,
    _reserved17: [u8; 0x04],
    diep1tfstat: Diep1tfstat,
    _reserved18: [u8; 0x04],
    diep2ctl: Diep2ctl,
    _reserved19: [u8; 0x04],
    diep2intf: Diep2intf,
    _reserved20: [u8; 0x04],
    diep2len: Diep2len,
    _reserved21: [u8; 0x04],
    diep2tfstat: Diep2tfstat,
    _reserved22: [u8; 0x04],
    diep3ctl: Diep3ctl,
    _reserved23: [u8; 0x04],
    diep3intf: Diep3intf,
    _reserved24: [u8; 0x04],
    diep3len: Diep3len,
    _reserved25: [u8; 0x04],
    diep3tfstat: Diep3tfstat,
    _reserved26: [u8; 0x0184],
    doep0ctl: Doep0ctl,
    _reserved27: [u8; 0x04],
    doep0intf: Doep0intf,
    _reserved28: [u8; 0x04],
    doep0len: Doep0len,
    _reserved29: [u8; 0x0c],
    doep1ctl: Doep1ctl,
    _reserved30: [u8; 0x04],
    doep1intf: Doep1intf,
    _reserved31: [u8; 0x04],
    doep1len: Doep1len,
    _reserved32: [u8; 0x0c],
    doep2ctl: Doep2ctl,
    _reserved33: [u8; 0x04],
    doep2intf: Doep2intf,
    _reserved34: [u8; 0x04],
    doep2len: Doep2len,
    _reserved35: [u8; 0x0c],
    doep3ctl: Doep3ctl,
    _reserved36: [u8; 0x04],
    doep3intf: Doep3intf,
    _reserved37: [u8; 0x04],
    doep3len: Doep3len,
}
impl RegisterBlock {
    #[doc = "0x00 - device configuration register (DCFG)"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &Dcfg {
        &self.dcfg
    }
    #[doc = "0x04 - device control register (DCTL)"]
    #[inline(always)]
    pub const fn dctl(&self) -> &Dctl {
        &self.dctl
    }
    #[doc = "0x08 - device status register (DSTAT)"]
    #[inline(always)]
    pub const fn dstat(&self) -> &Dstat {
        &self.dstat
    }
    #[doc = "0x10 - device IN endpoint common interrupt mask register (DIEPINTEN)"]
    #[inline(always)]
    pub const fn diepinten(&self) -> &Diepinten {
        &self.diepinten
    }
    #[doc = "0x14 - device OUT endpoint common interrupt enable register (DOEPINTEN)"]
    #[inline(always)]
    pub const fn doepinten(&self) -> &Doepinten {
        &self.doepinten
    }
    #[doc = "0x18 - device all endpoints interrupt register (DAEPINT)"]
    #[inline(always)]
    pub const fn daepint(&self) -> &Daepint {
        &self.daepint
    }
    #[doc = "0x1c - Device all endpoints interrupt enable register (DAEPINTEN)"]
    #[inline(always)]
    pub const fn daepinten(&self) -> &Daepinten {
        &self.daepinten
    }
    #[doc = "0x28 - device VBUS discharge time register"]
    #[inline(always)]
    pub const fn dvbusdt(&self) -> &Dvbusdt {
        &self.dvbusdt
    }
    #[doc = "0x2c - device VBUS pulsing time register"]
    #[inline(always)]
    pub const fn dvbuspt(&self) -> &Dvbuspt {
        &self.dvbuspt
    }
    #[doc = "0x34 - device IN endpoint FIFO empty interrupt enable register"]
    #[inline(always)]
    pub const fn diepfeinten(&self) -> &Diepfeinten {
        &self.diepfeinten
    }
    #[doc = "0x100 - device IN endpoint 0 control register (DIEP0CTL)"]
    #[inline(always)]
    pub const fn diep0ctl(&self) -> &Diep0ctl {
        &self.diep0ctl
    }
    #[doc = "0x108 - device endpoint-0 interrupt register"]
    #[inline(always)]
    pub const fn diep0intf(&self) -> &Diep0intf {
        &self.diep0intf
    }
    #[doc = "0x110 - device IN endpoint-0 transfer length register"]
    #[inline(always)]
    pub const fn diep0len(&self) -> &Diep0len {
        &self.diep0len
    }
    #[doc = "0x118 - device IN endpoint 0 transmit FIFO status register"]
    #[inline(always)]
    pub const fn diep0tfstat(&self) -> &Diep0tfstat {
        &self.diep0tfstat
    }
    #[doc = "0x120 - device in endpoint-1 control register"]
    #[inline(always)]
    pub const fn diep1ctl(&self) -> &Diep1ctl {
        &self.diep1ctl
    }
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn diep1intf(&self) -> &Diep1intf {
        &self.diep1intf
    }
    #[doc = "0x130 - device IN endpoint-1 transfer length register"]
    #[inline(always)]
    pub const fn diep1len(&self) -> &Diep1len {
        &self.diep1len
    }
    #[doc = "0x138 - device IN endpoint 1 transmit FIFO status register"]
    #[inline(always)]
    pub const fn diep1tfstat(&self) -> &Diep1tfstat {
        &self.diep1tfstat
    }
    #[doc = "0x140 - device endpoint-2 control register"]
    #[inline(always)]
    pub const fn diep2ctl(&self) -> &Diep2ctl {
        &self.diep2ctl
    }
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    #[inline(always)]
    pub const fn diep2intf(&self) -> &Diep2intf {
        &self.diep2intf
    }
    #[doc = "0x150 - device IN endpoint-2 transfer length register"]
    #[inline(always)]
    pub const fn diep2len(&self) -> &Diep2len {
        &self.diep2len
    }
    #[doc = "0x158 - device IN endpoint 2 transmit FIFO status register"]
    #[inline(always)]
    pub const fn diep2tfstat(&self) -> &Diep2tfstat {
        &self.diep2tfstat
    }
    #[doc = "0x160 - device endpoint-3 control register"]
    #[inline(always)]
    pub const fn diep3ctl(&self) -> &Diep3ctl {
        &self.diep3ctl
    }
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    #[inline(always)]
    pub const fn diep3intf(&self) -> &Diep3intf {
        &self.diep3intf
    }
    #[doc = "0x170 - device IN endpoint-3 transfer length register"]
    #[inline(always)]
    pub const fn diep3len(&self) -> &Diep3len {
        &self.diep3len
    }
    #[doc = "0x178 - device IN endpoint 3 transmit FIFO status register"]
    #[inline(always)]
    pub const fn diep3tfstat(&self) -> &Diep3tfstat {
        &self.diep3tfstat
    }
    #[doc = "0x300 - device endpoint-0 control register"]
    #[inline(always)]
    pub const fn doep0ctl(&self) -> &Doep0ctl {
        &self.doep0ctl
    }
    #[doc = "0x308 - device out endpoint-0 interrupt flag register"]
    #[inline(always)]
    pub const fn doep0intf(&self) -> &Doep0intf {
        &self.doep0intf
    }
    #[doc = "0x310 - device OUT endpoint-0 transfer length register"]
    #[inline(always)]
    pub const fn doep0len(&self) -> &Doep0len {
        &self.doep0len
    }
    #[doc = "0x320 - device endpoint-1 control register"]
    #[inline(always)]
    pub const fn doep1ctl(&self) -> &Doep1ctl {
        &self.doep1ctl
    }
    #[doc = "0x328 - device out endpoint-1 interrupt flag register"]
    #[inline(always)]
    pub const fn doep1intf(&self) -> &Doep1intf {
        &self.doep1intf
    }
    #[doc = "0x330 - device OUT endpoint-1 transfer length register"]
    #[inline(always)]
    pub const fn doep1len(&self) -> &Doep1len {
        &self.doep1len
    }
    #[doc = "0x340 - device endpoint-2 control register"]
    #[inline(always)]
    pub const fn doep2ctl(&self) -> &Doep2ctl {
        &self.doep2ctl
    }
    #[doc = "0x348 - device out endpoint-2 interrupt flag register"]
    #[inline(always)]
    pub const fn doep2intf(&self) -> &Doep2intf {
        &self.doep2intf
    }
    #[doc = "0x350 - device OUT endpoint-2 transfer length register"]
    #[inline(always)]
    pub const fn doep2len(&self) -> &Doep2len {
        &self.doep2len
    }
    #[doc = "0x360 - device endpoint-3 control register"]
    #[inline(always)]
    pub const fn doep3ctl(&self) -> &Doep3ctl {
        &self.doep3ctl
    }
    #[doc = "0x368 - device out endpoint-3 interrupt flag register"]
    #[inline(always)]
    pub const fn doep3intf(&self) -> &Doep3intf {
        &self.doep3intf
    }
    #[doc = "0x370 - device OUT endpoint-3 transfer length register"]
    #[inline(always)]
    pub const fn doep3len(&self) -> &Doep3len {
        &self.doep3len
    }
}
#[doc = "DCFG (rw) register accessor: device configuration register (DCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
#[doc(alias = "DCFG")]
pub type Dcfg = crate::Reg<dcfg::DcfgSpec>;
#[doc = "device configuration register (DCFG)"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: device control register (DCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
#[doc(alias = "DCTL")]
pub type Dctl = crate::Reg<dctl::DctlSpec>;
#[doc = "device control register (DCTL)"]
pub mod dctl;
#[doc = "DSTAT (r) register accessor: device status register (DSTAT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstat`]
module"]
#[doc(alias = "DSTAT")]
pub type Dstat = crate::Reg<dstat::DstatSpec>;
#[doc = "device status register (DSTAT)"]
pub mod dstat;
#[doc = "DIEPINTEN (rw) register accessor: device IN endpoint common interrupt mask register (DIEPINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepinten`]
module"]
#[doc(alias = "DIEPINTEN")]
pub type Diepinten = crate::Reg<diepinten::DiepintenSpec>;
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)"]
pub mod diepinten;
#[doc = "DOEPINTEN (rw) register accessor: device OUT endpoint common interrupt enable register (DOEPINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepinten`]
module"]
#[doc(alias = "DOEPINTEN")]
pub type Doepinten = crate::Reg<doepinten::DoepintenSpec>;
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)"]
pub mod doepinten;
#[doc = "DAEPINT (r) register accessor: device all endpoints interrupt register (DAEPINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daepint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daepint`]
module"]
#[doc(alias = "DAEPINT")]
pub type Daepint = crate::Reg<daepint::DaepintSpec>;
#[doc = "device all endpoints interrupt register (DAEPINT)"]
pub mod daepint;
#[doc = "DAEPINTEN (rw) register accessor: Device all endpoints interrupt enable register (DAEPINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daepinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daepinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daepinten`]
module"]
#[doc(alias = "DAEPINTEN")]
pub type Daepinten = crate::Reg<daepinten::DaepintenSpec>;
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)"]
pub mod daepinten;
#[doc = "DVBUSDT (rw) register accessor: device VBUS discharge time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdt`]
module"]
#[doc(alias = "DVBUSDT")]
pub type Dvbusdt = crate::Reg<dvbusdt::DvbusdtSpec>;
#[doc = "device VBUS discharge time register"]
pub mod dvbusdt;
#[doc = "DVBUSPT (rw) register accessor: device VBUS pulsing time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspt`]
module"]
#[doc(alias = "DVBUSPT")]
pub type Dvbuspt = crate::Reg<dvbuspt::DvbusptSpec>;
#[doc = "device VBUS pulsing time register"]
pub mod dvbuspt;
#[doc = "DIEPFEINTEN (rw) register accessor: device IN endpoint FIFO empty interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepfeinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepfeinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepfeinten`]
module"]
#[doc(alias = "DIEPFEINTEN")]
pub type Diepfeinten = crate::Reg<diepfeinten::DiepfeintenSpec>;
#[doc = "device IN endpoint FIFO empty interrupt enable register"]
pub mod diepfeinten;
#[doc = "DIEP0CTL (rw) register accessor: device IN endpoint 0 control register (DIEP0CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0ctl`]
module"]
#[doc(alias = "DIEP0CTL")]
pub type Diep0ctl = crate::Reg<diep0ctl::Diep0ctlSpec>;
#[doc = "device IN endpoint 0 control register (DIEP0CTL)"]
pub mod diep0ctl;
#[doc = "DIEP1CTL (rw) register accessor: device in endpoint-1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1ctl`]
module"]
#[doc(alias = "DIEP1CTL")]
pub type Diep1ctl = crate::Reg<diep1ctl::Diep1ctlSpec>;
#[doc = "device in endpoint-1 control register"]
pub mod diep1ctl;
#[doc = "DIEP2CTL (rw) register accessor: device endpoint-2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2ctl`]
module"]
#[doc(alias = "DIEP2CTL")]
pub type Diep2ctl = crate::Reg<diep2ctl::Diep2ctlSpec>;
#[doc = "device endpoint-2 control register"]
pub mod diep2ctl;
#[doc = "DIEP3CTL (rw) register accessor: device endpoint-3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3ctl`]
module"]
#[doc(alias = "DIEP3CTL")]
pub type Diep3ctl = crate::Reg<diep3ctl::Diep3ctlSpec>;
#[doc = "device endpoint-3 control register"]
pub mod diep3ctl;
#[doc = "DOEP0CTL (rw) register accessor: device endpoint-0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0ctl`]
module"]
#[doc(alias = "DOEP0CTL")]
pub type Doep0ctl = crate::Reg<doep0ctl::Doep0ctlSpec>;
#[doc = "device endpoint-0 control register"]
pub mod doep0ctl;
#[doc = "DOEP1CTL (rw) register accessor: device endpoint-1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1ctl`]
module"]
#[doc(alias = "DOEP1CTL")]
pub type Doep1ctl = crate::Reg<doep1ctl::Doep1ctlSpec>;
#[doc = "device endpoint-1 control register"]
pub mod doep1ctl;
#[doc = "DOEP2CTL (rw) register accessor: device endpoint-2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2ctl`]
module"]
#[doc(alias = "DOEP2CTL")]
pub type Doep2ctl = crate::Reg<doep2ctl::Doep2ctlSpec>;
#[doc = "device endpoint-2 control register"]
pub mod doep2ctl;
#[doc = "DOEP3CTL (rw) register accessor: device endpoint-3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3ctl`]
module"]
#[doc(alias = "DOEP3CTL")]
pub type Doep3ctl = crate::Reg<doep3ctl::Doep3ctlSpec>;
#[doc = "device endpoint-3 control register"]
pub mod doep3ctl;
#[doc = "DIEP0INTF (rw) register accessor: device endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0intf`]
module"]
#[doc(alias = "DIEP0INTF")]
pub type Diep0intf = crate::Reg<diep0intf::Diep0intfSpec>;
#[doc = "device endpoint-0 interrupt register"]
pub mod diep0intf;
#[doc = "DIEP1INTF (rw) register accessor: device endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1intf`]
module"]
#[doc(alias = "DIEP1INTF")]
pub type Diep1intf = crate::Reg<diep1intf::Diep1intfSpec>;
#[doc = "device endpoint-1 interrupt register"]
pub mod diep1intf;
#[doc = "DIEP2INTF (rw) register accessor: device endpoint-2 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2intf`]
module"]
#[doc(alias = "DIEP2INTF")]
pub type Diep2intf = crate::Reg<diep2intf::Diep2intfSpec>;
#[doc = "device endpoint-2 interrupt register"]
pub mod diep2intf;
#[doc = "DIEP3INTF (rw) register accessor: device endpoint-3 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3intf`]
module"]
#[doc(alias = "DIEP3INTF")]
pub type Diep3intf = crate::Reg<diep3intf::Diep3intfSpec>;
#[doc = "device endpoint-3 interrupt register"]
pub mod diep3intf;
#[doc = "DOEP0INTF (rw) register accessor: device out endpoint-0 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0intf`]
module"]
#[doc(alias = "DOEP0INTF")]
pub type Doep0intf = crate::Reg<doep0intf::Doep0intfSpec>;
#[doc = "device out endpoint-0 interrupt flag register"]
pub mod doep0intf;
#[doc = "DOEP1INTF (rw) register accessor: device out endpoint-1 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1intf`]
module"]
#[doc(alias = "DOEP1INTF")]
pub type Doep1intf = crate::Reg<doep1intf::Doep1intfSpec>;
#[doc = "device out endpoint-1 interrupt flag register"]
pub mod doep1intf;
#[doc = "DOEP2INTF (rw) register accessor: device out endpoint-2 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2intf`]
module"]
#[doc(alias = "DOEP2INTF")]
pub type Doep2intf = crate::Reg<doep2intf::Doep2intfSpec>;
#[doc = "device out endpoint-2 interrupt flag register"]
pub mod doep2intf;
#[doc = "DOEP3INTF (rw) register accessor: device out endpoint-3 interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3intf`]
module"]
#[doc(alias = "DOEP3INTF")]
pub type Doep3intf = crate::Reg<doep3intf::Doep3intfSpec>;
#[doc = "device out endpoint-3 interrupt flag register"]
pub mod doep3intf;
#[doc = "DIEP0LEN (rw) register accessor: device IN endpoint-0 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep0len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0len`]
module"]
#[doc(alias = "DIEP0LEN")]
pub type Diep0len = crate::Reg<diep0len::Diep0lenSpec>;
#[doc = "device IN endpoint-0 transfer length register"]
pub mod diep0len;
#[doc = "DOEP0LEN (rw) register accessor: device OUT endpoint-0 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep0len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep0len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0len`]
module"]
#[doc(alias = "DOEP0LEN")]
pub type Doep0len = crate::Reg<doep0len::Doep0lenSpec>;
#[doc = "device OUT endpoint-0 transfer length register"]
pub mod doep0len;
#[doc = "DIEP1LEN (rw) register accessor: device IN endpoint-1 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep1len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1len`]
module"]
#[doc(alias = "DIEP1LEN")]
pub type Diep1len = crate::Reg<diep1len::Diep1lenSpec>;
#[doc = "device IN endpoint-1 transfer length register"]
pub mod diep1len;
#[doc = "DIEP2LEN (rw) register accessor: device IN endpoint-2 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep2len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2len`]
module"]
#[doc(alias = "DIEP2LEN")]
pub type Diep2len = crate::Reg<diep2len::Diep2lenSpec>;
#[doc = "device IN endpoint-2 transfer length register"]
pub mod diep2len;
#[doc = "DIEP3LEN (rw) register accessor: device IN endpoint-3 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep3len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3len`]
module"]
#[doc(alias = "DIEP3LEN")]
pub type Diep3len = crate::Reg<diep3len::Diep3lenSpec>;
#[doc = "device IN endpoint-3 transfer length register"]
pub mod diep3len;
#[doc = "DOEP1LEN (rw) register accessor: device OUT endpoint-1 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep1len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep1len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1len`]
module"]
#[doc(alias = "DOEP1LEN")]
pub type Doep1len = crate::Reg<doep1len::Doep1lenSpec>;
#[doc = "device OUT endpoint-1 transfer length register"]
pub mod doep1len;
#[doc = "DOEP2LEN (rw) register accessor: device OUT endpoint-2 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep2len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep2len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2len`]
module"]
#[doc(alias = "DOEP2LEN")]
pub type Doep2len = crate::Reg<doep2len::Doep2lenSpec>;
#[doc = "device OUT endpoint-2 transfer length register"]
pub mod doep2len;
#[doc = "DOEP3LEN (rw) register accessor: device OUT endpoint-3 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doep3len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doep3len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3len`]
module"]
#[doc(alias = "DOEP3LEN")]
pub type Doep3len = crate::Reg<doep3len::Doep3lenSpec>;
#[doc = "device OUT endpoint-3 transfer length register"]
pub mod doep3len;
#[doc = "DIEP0TFSTAT (r) register accessor: device IN endpoint 0 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep0tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0tfstat`]
module"]
#[doc(alias = "DIEP0TFSTAT")]
pub type Diep0tfstat = crate::Reg<diep0tfstat::Diep0tfstatSpec>;
#[doc = "device IN endpoint 0 transmit FIFO status register"]
pub mod diep0tfstat;
#[doc = "DIEP1TFSTAT (r) register accessor: device IN endpoint 1 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1tfstat`]
module"]
#[doc(alias = "DIEP1TFSTAT")]
pub type Diep1tfstat = crate::Reg<diep1tfstat::Diep1tfstatSpec>;
#[doc = "device IN endpoint 1 transmit FIFO status register"]
pub mod diep1tfstat;
#[doc = "DIEP2TFSTAT (r) register accessor: device IN endpoint 2 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep2tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2tfstat`]
module"]
#[doc(alias = "DIEP2TFSTAT")]
pub type Diep2tfstat = crate::Reg<diep2tfstat::Diep2tfstatSpec>;
#[doc = "device IN endpoint 2 transmit FIFO status register"]
pub mod diep2tfstat;
#[doc = "DIEP3TFSTAT (r) register accessor: device IN endpoint 3 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep3tfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3tfstat`]
module"]
#[doc(alias = "DIEP3TFSTAT")]
pub type Diep3tfstat = crate::Reg<diep3tfstat::Diep3tfstatSpec>;
#[doc = "device IN endpoint 3 transmit FIFO status register"]
pub mod diep3tfstat;
