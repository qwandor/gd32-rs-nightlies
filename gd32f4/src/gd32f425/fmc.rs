#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ws: Ws,
    key: Key,
    obkey: Obkey,
    stat: Stat,
    ctl: Ctl,
    obctl0: Obctl0,
    obctl1: Obctl1,
    _reserved7: [u8; 0xe0],
    wsen: Wsen,
    pid: Pid,
}
impl RegisterBlock {
    #[doc = "0x00 - wait state counter register"]
    #[inline(always)]
    pub const fn ws(&self) -> &Ws {
        &self.ws
    }
    #[doc = "0x04 - Unlock key register"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x08 - Option byte unlock key register"]
    #[inline(always)]
    pub const fn obkey(&self) -> &Obkey {
        &self.obkey
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x10 - Control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x14 - Option byte control register 0"]
    #[inline(always)]
    pub const fn obctl0(&self) -> &Obctl0 {
        &self.obctl0
    }
    #[doc = "0x18 - Option byte control register 1"]
    #[inline(always)]
    pub const fn obctl1(&self) -> &Obctl1 {
        &self.obctl1
    }
    #[doc = "0xfc - Wait state enable register"]
    #[inline(always)]
    pub const fn wsen(&self) -> &Wsen {
        &self.wsen
    }
    #[doc = "0x100 - Product ID register"]
    #[inline(always)]
    pub const fn pid(&self) -> &Pid {
        &self.pid
    }
}
#[doc = "WS (rw) register accessor: wait state counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ws::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ws::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ws`]
module"]
#[doc(alias = "WS")]
pub type Ws = crate::Reg<ws::WsSpec>;
#[doc = "wait state counter register"]
pub mod ws;
#[doc = "KEY (w) register accessor: Unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`]
module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "Unlock key register"]
pub mod key;
#[doc = "OBKEY (w) register accessor: Option byte unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obkey`]
module"]
#[doc(alias = "OBKEY")]
pub type Obkey = crate::Reg<obkey::ObkeySpec>;
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "STAT (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "OBCTL0 (rw) register accessor: Option byte control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obctl0`]
module"]
#[doc(alias = "OBCTL0")]
pub type Obctl0 = crate::Reg<obctl0::Obctl0Spec>;
#[doc = "Option byte control register 0"]
pub mod obctl0;
#[doc = "OBCTL1 (rw) register accessor: Option byte control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obctl1`]
module"]
#[doc(alias = "OBCTL1")]
pub type Obctl1 = crate::Reg<obctl1::Obctl1Spec>;
#[doc = "Option byte control register 1"]
pub mod obctl1;
#[doc = "WSEN (r) register accessor: Wait state enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsen::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wsen`]
module"]
#[doc(alias = "WSEN")]
pub type Wsen = crate::Reg<wsen::WsenSpec>;
#[doc = "Wait state enable register"]
pub mod wsen;
#[doc = "PID (r) register accessor: Product ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "PID")]
pub type Pid = crate::Reg<pid::PidSpec>;
#[doc = "Product ID register"]
pub mod pid;
