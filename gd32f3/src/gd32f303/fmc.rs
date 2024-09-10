#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ws: Ws,
    key: Key,
    obkey: Obkey,
    stat0: Stat0,
    ctl0: Ctl0,
    addr0: Addr0,
    _reserved6: [u8; 0x04],
    obctl: Obctl,
    wp: Wp,
    _reserved8: [u8; 0x20],
    key1: Key1,
    _reserved9: [u8; 0x04],
    stat1: Stat1,
    ctl1: Ctl1,
    addr1: Addr1,
    _reserved12: [u8; 0xa4],
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
    #[doc = "0x0c - Status register 0"]
    #[inline(always)]
    pub const fn stat0(&self) -> &Stat0 {
        &self.stat0
    }
    #[doc = "0x10 - Control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x14 - Address register 0"]
    #[inline(always)]
    pub const fn addr0(&self) -> &Addr0 {
        &self.addr0
    }
    #[doc = "0x1c - Option byte control register"]
    #[inline(always)]
    pub const fn obctl(&self) -> &Obctl {
        &self.obctl
    }
    #[doc = "0x20 - Erase/Program Protection register"]
    #[inline(always)]
    pub const fn wp(&self) -> &Wp {
        &self.wp
    }
    #[doc = "0x44 - Unlock key register 1"]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x4c - Status register 1"]
    #[inline(always)]
    pub const fn stat1(&self) -> &Stat1 {
        &self.stat1
    }
    #[doc = "0x50 - Control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x54 - Address register 1"]
    #[inline(always)]
    pub const fn addr1(&self) -> &Addr1 {
        &self.addr1
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
#[doc = "STAT0 (rw) register accessor: Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat0`]
module"]
#[doc(alias = "STAT0")]
pub type Stat0 = crate::Reg<stat0::Stat0Spec>;
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "ADDR0 (w) register accessor: Address register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0`]
module"]
#[doc(alias = "ADDR0")]
pub type Addr0 = crate::Reg<addr0::Addr0Spec>;
#[doc = "Address register 0"]
pub mod addr0;
#[doc = "OBCTL (r) register accessor: Option byte control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obctl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obctl`]
module"]
#[doc(alias = "OBCTL")]
pub type Obctl = crate::Reg<obctl::ObctlSpec>;
#[doc = "Option byte control register"]
pub mod obctl;
#[doc = "WP (r) register accessor: Erase/Program Protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp`]
module"]
#[doc(alias = "WP")]
pub type Wp = crate::Reg<wp::WpSpec>;
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "KEY1 (w) register accessor: Unlock key register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`]
module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "Unlock key register 1"]
pub mod key1;
#[doc = "STAT1 (rw) register accessor: Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat1`]
module"]
#[doc(alias = "STAT1")]
pub type Stat1 = crate::Reg<stat1::Stat1Spec>;
#[doc = "Status register 1"]
pub mod stat1;
#[doc = "CTL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "ADDR1 (w) register accessor: Address register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr1`]
module"]
#[doc(alias = "ADDR1")]
pub type Addr1 = crate::Reg<addr1::Addr1Spec>;
#[doc = "Address register 1"]
pub mod addr1;
#[doc = "WSEN (rw) register accessor: Wait state enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wsen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wsen`]
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
