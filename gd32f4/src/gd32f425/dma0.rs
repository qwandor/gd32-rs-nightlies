#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intf0: Intf0,
    intf1: Intf1,
    intc0: Intc0,
    intc1: Intc1,
    ch0ctl: Ch0ctl,
    ch0cnt: Ch0cnt,
    ch0paddr: Ch0paddr,
    ch0m0addr: Ch0m0addr,
    ch0m1addr: Ch0m1addr,
    ch0fctl: Ch0fctl,
    ch1ctl: Ch1ctl,
    ch1cnt: Ch1cnt,
    ch1paddr: Ch1paddr,
    ch1m0addr: Ch1m0addr,
    ch1m1addr: Ch1m1addr,
    ch1fctl: Ch1fctl,
    ch2ctl: Ch2ctl,
    ch2cnt: Ch2cnt,
    ch2paddr: Ch2paddr,
    ch2m0addr: Ch2m0addr,
    ch2m1addr: Ch2m1addr,
    ch2fctl: Ch2fctl,
    ch3ctl: Ch3ctl,
    ch3cnt: Ch3cnt,
    ch3paddr: Ch3paddr,
    ch3m0addr: Ch3m0addr,
    ch3m1addr: Ch3m1addr,
    ch3fctl: Ch3fctl,
    ch4ctl: Ch4ctl,
    ch4cnt: Ch4cnt,
    ch4paddr: Ch4paddr,
    ch4m0addr: Ch4m0addr,
    ch4m1addr: Ch4m1addr,
    ch4fctl: Ch4fctl,
    ch5ctl: Ch5ctl,
    ch5cnt: Ch5cnt,
    ch5paddr: Ch5paddr,
    ch5m0addr: Ch5m0addr,
    ch5m1addr: Ch5m1addr,
    ch5fctl: Ch5fctl,
    ch6ctl: Ch6ctl,
    ch6cnt: Ch6cnt,
    ch6paddr: Ch6paddr,
    ch6m0addr: Ch6m0addr,
    ch6m1addr: Ch6m1addr,
    ch6fctl: Ch6fctl,
    ch7ctl: Ch7ctl,
    ch7cnt: Ch7cnt,
    ch7paddr: Ch7paddr,
    ch7m0addr: Ch7m0addr,
    ch7m1addr: Ch7m1addr,
    ch7fctl: Ch7fctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt flag register 0"]
    #[inline(always)]
    pub const fn intf0(&self) -> &Intf0 {
        &self.intf0
    }
    #[doc = "0x04 - Interrupt flag register 1"]
    #[inline(always)]
    pub const fn intf1(&self) -> &Intf1 {
        &self.intf1
    }
    #[doc = "0x08 - Interrupt flag clear register 0"]
    #[inline(always)]
    pub const fn intc0(&self) -> &Intc0 {
        &self.intc0
    }
    #[doc = "0x0c - Interrupt flag clear register 1"]
    #[inline(always)]
    pub const fn intc1(&self) -> &Intc1 {
        &self.intc1
    }
    #[doc = "0x10 - Channel 0 control register"]
    #[inline(always)]
    pub const fn ch0ctl(&self) -> &Ch0ctl {
        &self.ch0ctl
    }
    #[doc = "0x14 - Channel 0 counter register"]
    #[inline(always)]
    pub const fn ch0cnt(&self) -> &Ch0cnt {
        &self.ch0cnt
    }
    #[doc = "0x18 - Channel 0 peripheral base address register"]
    #[inline(always)]
    pub const fn ch0paddr(&self) -> &Ch0paddr {
        &self.ch0paddr
    }
    #[doc = "0x1c - Channel 0 memory 0 base address register"]
    #[inline(always)]
    pub const fn ch0m0addr(&self) -> &Ch0m0addr {
        &self.ch0m0addr
    }
    #[doc = "0x20 - Channel 0 memory 1 base address register"]
    #[inline(always)]
    pub const fn ch0m1addr(&self) -> &Ch0m1addr {
        &self.ch0m1addr
    }
    #[doc = "0x24 - Channel 0 FIFO control register"]
    #[inline(always)]
    pub const fn ch0fctl(&self) -> &Ch0fctl {
        &self.ch0fctl
    }
    #[doc = "0x28 - Channel 1 control register"]
    #[inline(always)]
    pub const fn ch1ctl(&self) -> &Ch1ctl {
        &self.ch1ctl
    }
    #[doc = "0x2c - Channel 1 counter register"]
    #[inline(always)]
    pub const fn ch1cnt(&self) -> &Ch1cnt {
        &self.ch1cnt
    }
    #[doc = "0x30 - Channel 1 peripheral base address register"]
    #[inline(always)]
    pub const fn ch1paddr(&self) -> &Ch1paddr {
        &self.ch1paddr
    }
    #[doc = "0x34 - Channel 1 memory 0 base address register"]
    #[inline(always)]
    pub const fn ch1m0addr(&self) -> &Ch1m0addr {
        &self.ch1m0addr
    }
    #[doc = "0x38 - Channel 1 memory 1 base address register"]
    #[inline(always)]
    pub const fn ch1m1addr(&self) -> &Ch1m1addr {
        &self.ch1m1addr
    }
    #[doc = "0x3c - Channel 1 FIFO control register"]
    #[inline(always)]
    pub const fn ch1fctl(&self) -> &Ch1fctl {
        &self.ch1fctl
    }
    #[doc = "0x40 - Channel 2 control register"]
    #[inline(always)]
    pub const fn ch2ctl(&self) -> &Ch2ctl {
        &self.ch2ctl
    }
    #[doc = "0x44 - Channel 2 counter register"]
    #[inline(always)]
    pub const fn ch2cnt(&self) -> &Ch2cnt {
        &self.ch2cnt
    }
    #[doc = "0x48 - Channel 2 peripheral base address register"]
    #[inline(always)]
    pub const fn ch2paddr(&self) -> &Ch2paddr {
        &self.ch2paddr
    }
    #[doc = "0x4c - Channel 2 memory 0 base address register"]
    #[inline(always)]
    pub const fn ch2m0addr(&self) -> &Ch2m0addr {
        &self.ch2m0addr
    }
    #[doc = "0x50 - Channel 2 memory 1 base address register"]
    #[inline(always)]
    pub const fn ch2m1addr(&self) -> &Ch2m1addr {
        &self.ch2m1addr
    }
    #[doc = "0x54 - Channel 2 FIFO control register"]
    #[inline(always)]
    pub const fn ch2fctl(&self) -> &Ch2fctl {
        &self.ch2fctl
    }
    #[doc = "0x58 - Channel 3 control register"]
    #[inline(always)]
    pub const fn ch3ctl(&self) -> &Ch3ctl {
        &self.ch3ctl
    }
    #[doc = "0x5c - Channel 3 counter register"]
    #[inline(always)]
    pub const fn ch3cnt(&self) -> &Ch3cnt {
        &self.ch3cnt
    }
    #[doc = "0x60 - Channel 3 peripheral base address register"]
    #[inline(always)]
    pub const fn ch3paddr(&self) -> &Ch3paddr {
        &self.ch3paddr
    }
    #[doc = "0x64 - Channel 3 memory 0 base address register"]
    #[inline(always)]
    pub const fn ch3m0addr(&self) -> &Ch3m0addr {
        &self.ch3m0addr
    }
    #[doc = "0x68 - Channel 3 memory 1 base address register"]
    #[inline(always)]
    pub const fn ch3m1addr(&self) -> &Ch3m1addr {
        &self.ch3m1addr
    }
    #[doc = "0x6c - Channel 3 FIFO control register"]
    #[inline(always)]
    pub const fn ch3fctl(&self) -> &Ch3fctl {
        &self.ch3fctl
    }
    #[doc = "0x70 - Channel 4 control register"]
    #[inline(always)]
    pub const fn ch4ctl(&self) -> &Ch4ctl {
        &self.ch4ctl
    }
    #[doc = "0x74 - Channel 4 counter register"]
    #[inline(always)]
    pub const fn ch4cnt(&self) -> &Ch4cnt {
        &self.ch4cnt
    }
    #[doc = "0x78 - Channel 4 peripheral base address register"]
    #[inline(always)]
    pub const fn ch4paddr(&self) -> &Ch4paddr {
        &self.ch4paddr
    }
    #[doc = "0x7c - Channel 4 memory 0 base address register"]
    #[inline(always)]
    pub const fn ch4m0addr(&self) -> &Ch4m0addr {
        &self.ch4m0addr
    }
    #[doc = "0x80 - Channel 4 memory 1 base address register"]
    #[inline(always)]
    pub const fn ch4m1addr(&self) -> &Ch4m1addr {
        &self.ch4m1addr
    }
    #[doc = "0x84 - Channel 4 FIFO control register"]
    #[inline(always)]
    pub const fn ch4fctl(&self) -> &Ch4fctl {
        &self.ch4fctl
    }
    #[doc = "0x88 - Channel 5 control register"]
    #[inline(always)]
    pub const fn ch5ctl(&self) -> &Ch5ctl {
        &self.ch5ctl
    }
    #[doc = "0x8c - Channel 5 counter register"]
    #[inline(always)]
    pub const fn ch5cnt(&self) -> &Ch5cnt {
        &self.ch5cnt
    }
    #[doc = "0x90 - Channel 5 peripheral base address register"]
    #[inline(always)]
    pub const fn ch5paddr(&self) -> &Ch5paddr {
        &self.ch5paddr
    }
    #[doc = "0x94 - Channel 5 memory 0 base address register"]
    #[inline(always)]
    pub const fn ch5m0addr(&self) -> &Ch5m0addr {
        &self.ch5m0addr
    }
    #[doc = "0x98 - Channel 5 memory 1 base address register"]
    #[inline(always)]
    pub const fn ch5m1addr(&self) -> &Ch5m1addr {
        &self.ch5m1addr
    }
    #[doc = "0x9c - Channel 5 FIFO control register"]
    #[inline(always)]
    pub const fn ch5fctl(&self) -> &Ch5fctl {
        &self.ch5fctl
    }
    #[doc = "0xa0 - Channel 6 control register"]
    #[inline(always)]
    pub const fn ch6ctl(&self) -> &Ch6ctl {
        &self.ch6ctl
    }
    #[doc = "0xa4 - Channel 6 counter register"]
    #[inline(always)]
    pub const fn ch6cnt(&self) -> &Ch6cnt {
        &self.ch6cnt
    }
    #[doc = "0xa8 - Channel 6 peripheral base address register"]
    #[inline(always)]
    pub const fn ch6paddr(&self) -> &Ch6paddr {
        &self.ch6paddr
    }
    #[doc = "0xac - Channel 6 memory 0 base address register"]
    #[inline(always)]
    pub const fn ch6m0addr(&self) -> &Ch6m0addr {
        &self.ch6m0addr
    }
    #[doc = "0xb0 - Channel 6 memory 1 base address register"]
    #[inline(always)]
    pub const fn ch6m1addr(&self) -> &Ch6m1addr {
        &self.ch6m1addr
    }
    #[doc = "0xb4 - Channel 6 FIFO control register"]
    #[inline(always)]
    pub const fn ch6fctl(&self) -> &Ch6fctl {
        &self.ch6fctl
    }
    #[doc = "0xb8 - Channel 7 control register"]
    #[inline(always)]
    pub const fn ch7ctl(&self) -> &Ch7ctl {
        &self.ch7ctl
    }
    #[doc = "0xbc - Channel 7 counter register"]
    #[inline(always)]
    pub const fn ch7cnt(&self) -> &Ch7cnt {
        &self.ch7cnt
    }
    #[doc = "0xc0 - Channel 7 peripheral base address register"]
    #[inline(always)]
    pub const fn ch7paddr(&self) -> &Ch7paddr {
        &self.ch7paddr
    }
    #[doc = "0xc4 - Channel 7 memory 0 base address register"]
    #[inline(always)]
    pub const fn ch7m0addr(&self) -> &Ch7m0addr {
        &self.ch7m0addr
    }
    #[doc = "0xc8 - Channel 7 memory 1 base address register"]
    #[inline(always)]
    pub const fn ch7m1addr(&self) -> &Ch7m1addr {
        &self.ch7m1addr
    }
    #[doc = "0xcc - Channel 7 FIFO control register"]
    #[inline(always)]
    pub const fn ch7fctl(&self) -> &Ch7fctl {
        &self.ch7fctl
    }
}
#[doc = "INTF0 (r) register accessor: Interrupt flag register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf0`]
module"]
#[doc(alias = "INTF0")]
pub type Intf0 = crate::Reg<intf0::Intf0Spec>;
#[doc = "Interrupt flag register 0"]
pub mod intf0;
#[doc = "INTF1 (r) register accessor: Interrupt flag register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf1`]
module"]
#[doc(alias = "INTF1")]
pub type Intf1 = crate::Reg<intf1::Intf1Spec>;
#[doc = "Interrupt flag register 1"]
pub mod intf1;
#[doc = "INTC0 (w) register accessor: Interrupt flag clear register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc0`]
module"]
#[doc(alias = "INTC0")]
pub type Intc0 = crate::Reg<intc0::Intc0Spec>;
#[doc = "Interrupt flag clear register 0"]
pub mod intc0;
#[doc = "INTC1 (w) register accessor: Interrupt flag clear register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc1`]
module"]
#[doc(alias = "INTC1")]
pub type Intc1 = crate::Reg<intc1::Intc1Spec>;
#[doc = "Interrupt flag clear register 1"]
pub mod intc1;
#[doc = "CH0CTL (rw) register accessor: Channel 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ctl`]
module"]
#[doc(alias = "CH0CTL")]
pub type Ch0ctl = crate::Reg<ch0ctl::Ch0ctlSpec>;
#[doc = "Channel 0 control register"]
pub mod ch0ctl;
#[doc = "CH0CNT (rw) register accessor: Channel 0 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cnt`]
module"]
#[doc(alias = "CH0CNT")]
pub type Ch0cnt = crate::Reg<ch0cnt::Ch0cntSpec>;
#[doc = "Channel 0 counter register"]
pub mod ch0cnt;
#[doc = "CH0PADDR (rw) register accessor: Channel 0 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0paddr`]
module"]
#[doc(alias = "CH0PADDR")]
pub type Ch0paddr = crate::Reg<ch0paddr::Ch0paddrSpec>;
#[doc = "Channel 0 peripheral base address register"]
pub mod ch0paddr;
#[doc = "CH0M0ADDR (rw) register accessor: Channel 0 memory 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0m0addr`]
module"]
#[doc(alias = "CH0M0ADDR")]
pub type Ch0m0addr = crate::Reg<ch0m0addr::Ch0m0addrSpec>;
#[doc = "Channel 0 memory 0 base address register"]
pub mod ch0m0addr;
#[doc = "CH0M1ADDR (rw) register accessor: Channel 0 memory 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0m1addr`]
module"]
#[doc(alias = "CH0M1ADDR")]
pub type Ch0m1addr = crate::Reg<ch0m1addr::Ch0m1addrSpec>;
#[doc = "Channel 0 memory 1 base address register"]
pub mod ch0m1addr;
#[doc = "CH0FCTL (rw) register accessor: Channel 0 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0fctl`]
module"]
#[doc(alias = "CH0FCTL")]
pub type Ch0fctl = crate::Reg<ch0fctl::Ch0fctlSpec>;
#[doc = "Channel 0 FIFO control register"]
pub mod ch0fctl;
#[doc = "CH1CTL (rw) register accessor: Channel 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ctl`]
module"]
#[doc(alias = "CH1CTL")]
pub type Ch1ctl = crate::Reg<ch1ctl::Ch1ctlSpec>;
#[doc = "Channel 1 control register"]
pub mod ch1ctl;
#[doc = "CH1CNT (rw) register accessor: Channel 1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cnt`]
module"]
#[doc(alias = "CH1CNT")]
pub type Ch1cnt = crate::Reg<ch1cnt::Ch1cntSpec>;
#[doc = "Channel 1 counter register"]
pub mod ch1cnt;
#[doc = "CH1PADDR (rw) register accessor: Channel 1 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1paddr`]
module"]
#[doc(alias = "CH1PADDR")]
pub type Ch1paddr = crate::Reg<ch1paddr::Ch1paddrSpec>;
#[doc = "Channel 1 peripheral base address register"]
pub mod ch1paddr;
#[doc = "CH1M0ADDR (rw) register accessor: Channel 1 memory 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1m0addr`]
module"]
#[doc(alias = "CH1M0ADDR")]
pub type Ch1m0addr = crate::Reg<ch1m0addr::Ch1m0addrSpec>;
#[doc = "Channel 1 memory 0 base address register"]
pub mod ch1m0addr;
#[doc = "CH1M1ADDR (rw) register accessor: Channel 1 memory 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1m1addr`]
module"]
#[doc(alias = "CH1M1ADDR")]
pub type Ch1m1addr = crate::Reg<ch1m1addr::Ch1m1addrSpec>;
#[doc = "Channel 1 memory 1 base address register"]
pub mod ch1m1addr;
#[doc = "CH1FCTL (rw) register accessor: Channel 1 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1fctl`]
module"]
#[doc(alias = "CH1FCTL")]
pub type Ch1fctl = crate::Reg<ch1fctl::Ch1fctlSpec>;
#[doc = "Channel 1 FIFO control register"]
pub mod ch1fctl;
#[doc = "CH2CTL (rw) register accessor: Channel 2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ctl`]
module"]
#[doc(alias = "CH2CTL")]
pub type Ch2ctl = crate::Reg<ch2ctl::Ch2ctlSpec>;
#[doc = "Channel 2 control register"]
pub mod ch2ctl;
#[doc = "CH2CNT (rw) register accessor: Channel 2 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cnt`]
module"]
#[doc(alias = "CH2CNT")]
pub type Ch2cnt = crate::Reg<ch2cnt::Ch2cntSpec>;
#[doc = "Channel 2 counter register"]
pub mod ch2cnt;
#[doc = "CH2PADDR (rw) register accessor: Channel 2 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2paddr`]
module"]
#[doc(alias = "CH2PADDR")]
pub type Ch2paddr = crate::Reg<ch2paddr::Ch2paddrSpec>;
#[doc = "Channel 2 peripheral base address register"]
pub mod ch2paddr;
#[doc = "CH2M0ADDR (rw) register accessor: Channel 2 memory 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2m0addr`]
module"]
#[doc(alias = "CH2M0ADDR")]
pub type Ch2m0addr = crate::Reg<ch2m0addr::Ch2m0addrSpec>;
#[doc = "Channel 2 memory 0 base address register"]
pub mod ch2m0addr;
#[doc = "CH2M1ADDR (rw) register accessor: Channel 2 memory 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2m1addr`]
module"]
#[doc(alias = "CH2M1ADDR")]
pub type Ch2m1addr = crate::Reg<ch2m1addr::Ch2m1addrSpec>;
#[doc = "Channel 2 memory 1 base address register"]
pub mod ch2m1addr;
#[doc = "CH2FCTL (rw) register accessor: Channel 2 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2fctl`]
module"]
#[doc(alias = "CH2FCTL")]
pub type Ch2fctl = crate::Reg<ch2fctl::Ch2fctlSpec>;
#[doc = "Channel 2 FIFO control register"]
pub mod ch2fctl;
#[doc = "CH3CTL (rw) register accessor: Channel 3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ctl`]
module"]
#[doc(alias = "CH3CTL")]
pub type Ch3ctl = crate::Reg<ch3ctl::Ch3ctlSpec>;
#[doc = "Channel 3 control register"]
pub mod ch3ctl;
#[doc = "CH3CNT (rw) register accessor: Channel 3 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cnt`]
module"]
#[doc(alias = "CH3CNT")]
pub type Ch3cnt = crate::Reg<ch3cnt::Ch3cntSpec>;
#[doc = "Channel 3 counter register"]
pub mod ch3cnt;
#[doc = "CH3PADDR (rw) register accessor: Channel 3 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3paddr`]
module"]
#[doc(alias = "CH3PADDR")]
pub type Ch3paddr = crate::Reg<ch3paddr::Ch3paddrSpec>;
#[doc = "Channel 3 peripheral base address register"]
pub mod ch3paddr;
#[doc = "CH3M0ADDR (rw) register accessor: Channel 3 memory 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3m0addr`]
module"]
#[doc(alias = "CH3M0ADDR")]
pub type Ch3m0addr = crate::Reg<ch3m0addr::Ch3m0addrSpec>;
#[doc = "Channel 3 memory 0 base address register"]
pub mod ch3m0addr;
#[doc = "CH3M1ADDR (rw) register accessor: Channel 3 memory 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3m1addr`]
module"]
#[doc(alias = "CH3M1ADDR")]
pub type Ch3m1addr = crate::Reg<ch3m1addr::Ch3m1addrSpec>;
#[doc = "Channel 3 memory 1 base address register"]
pub mod ch3m1addr;
#[doc = "CH3FCTL (rw) register accessor: Channel 3 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3fctl`]
module"]
#[doc(alias = "CH3FCTL")]
pub type Ch3fctl = crate::Reg<ch3fctl::Ch3fctlSpec>;
#[doc = "Channel 3 FIFO control register"]
pub mod ch3fctl;
#[doc = "CH4CTL (rw) register accessor: Channel 4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4ctl`]
module"]
#[doc(alias = "CH4CTL")]
pub type Ch4ctl = crate::Reg<ch4ctl::Ch4ctlSpec>;
#[doc = "Channel 4 control register"]
pub mod ch4ctl;
#[doc = "CH4CNT (rw) register accessor: Channel 4 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cnt`]
module"]
#[doc(alias = "CH4CNT")]
pub type Ch4cnt = crate::Reg<ch4cnt::Ch4cntSpec>;
#[doc = "Channel 4 counter register"]
pub mod ch4cnt;
#[doc = "CH4PADDR (rw) register accessor: Channel 4 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4paddr`]
module"]
#[doc(alias = "CH4PADDR")]
pub type Ch4paddr = crate::Reg<ch4paddr::Ch4paddrSpec>;
#[doc = "Channel 4 peripheral base address register"]
pub mod ch4paddr;
#[doc = "CH4M0ADDR (rw) register accessor: Channel 4 memory 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4m0addr`]
module"]
#[doc(alias = "CH4M0ADDR")]
pub type Ch4m0addr = crate::Reg<ch4m0addr::Ch4m0addrSpec>;
#[doc = "Channel 4 memory 0 base address register"]
pub mod ch4m0addr;
#[doc = "CH4M1ADDR (rw) register accessor: Channel 4 memory 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4m1addr`]
module"]
#[doc(alias = "CH4M1ADDR")]
pub type Ch4m1addr = crate::Reg<ch4m1addr::Ch4m1addrSpec>;
#[doc = "Channel 4 memory 1 base address register"]
pub mod ch4m1addr;
#[doc = "CH4FCTL (rw) register accessor: Channel 4 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4fctl`]
module"]
#[doc(alias = "CH4FCTL")]
pub type Ch4fctl = crate::Reg<ch4fctl::Ch4fctlSpec>;
#[doc = "Channel 4 FIFO control register"]
pub mod ch4fctl;
#[doc = "CH5CTL (rw) register accessor: Channel 5 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5ctl`]
module"]
#[doc(alias = "CH5CTL")]
pub type Ch5ctl = crate::Reg<ch5ctl::Ch5ctlSpec>;
#[doc = "Channel 5 control register"]
pub mod ch5ctl;
#[doc = "CH5CNT (rw) register accessor: Channel 5 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cnt`]
module"]
#[doc(alias = "CH5CNT")]
pub type Ch5cnt = crate::Reg<ch5cnt::Ch5cntSpec>;
#[doc = "Channel 5 counter register"]
pub mod ch5cnt;
#[doc = "CH5PADDR (rw) register accessor: Channel 5 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5paddr`]
module"]
#[doc(alias = "CH5PADDR")]
pub type Ch5paddr = crate::Reg<ch5paddr::Ch5paddrSpec>;
#[doc = "Channel 5 peripheral base address register"]
pub mod ch5paddr;
#[doc = "CH5M0ADDR (rw) register accessor: Channel 5 memory 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5m0addr`]
module"]
#[doc(alias = "CH5M0ADDR")]
pub type Ch5m0addr = crate::Reg<ch5m0addr::Ch5m0addrSpec>;
#[doc = "Channel 5 memory 0 base address register"]
pub mod ch5m0addr;
#[doc = "CH5M1ADDR (rw) register accessor: Channel 5 memory 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5m1addr`]
module"]
#[doc(alias = "CH5M1ADDR")]
pub type Ch5m1addr = crate::Reg<ch5m1addr::Ch5m1addrSpec>;
#[doc = "Channel 5 memory 1 base address register"]
pub mod ch5m1addr;
#[doc = "CH5FCTL (rw) register accessor: Channel 5 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5fctl`]
module"]
#[doc(alias = "CH5FCTL")]
pub type Ch5fctl = crate::Reg<ch5fctl::Ch5fctlSpec>;
#[doc = "Channel 5 FIFO control register"]
pub mod ch5fctl;
#[doc = "CH6CTL (rw) register accessor: Channel 6 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6ctl`]
module"]
#[doc(alias = "CH6CTL")]
pub type Ch6ctl = crate::Reg<ch6ctl::Ch6ctlSpec>;
#[doc = "Channel 6 control register"]
pub mod ch6ctl;
#[doc = "CH6CNT (rw) register accessor: Channel 6 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cnt`]
module"]
#[doc(alias = "CH6CNT")]
pub type Ch6cnt = crate::Reg<ch6cnt::Ch6cntSpec>;
#[doc = "Channel 6 counter register"]
pub mod ch6cnt;
#[doc = "CH6PADDR (rw) register accessor: Channel 6 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6paddr`]
module"]
#[doc(alias = "CH6PADDR")]
pub type Ch6paddr = crate::Reg<ch6paddr::Ch6paddrSpec>;
#[doc = "Channel 6 peripheral base address register"]
pub mod ch6paddr;
#[doc = "CH6M0ADDR (rw) register accessor: Channel 6 memory 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6m0addr`]
module"]
#[doc(alias = "CH6M0ADDR")]
pub type Ch6m0addr = crate::Reg<ch6m0addr::Ch6m0addrSpec>;
#[doc = "Channel 6 memory 0 base address register"]
pub mod ch6m0addr;
#[doc = "CH6M1ADDR (rw) register accessor: Channel 6 memory 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6m1addr`]
module"]
#[doc(alias = "CH6M1ADDR")]
pub type Ch6m1addr = crate::Reg<ch6m1addr::Ch6m1addrSpec>;
#[doc = "Channel 6 memory 1 base address register"]
pub mod ch6m1addr;
#[doc = "CH6FCTL (rw) register accessor: Channel 6 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6fctl`]
module"]
#[doc(alias = "CH6FCTL")]
pub type Ch6fctl = crate::Reg<ch6fctl::Ch6fctlSpec>;
#[doc = "Channel 6 FIFO control register"]
pub mod ch6fctl;
#[doc = "CH7CTL (rw) register accessor: Channel 7 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7ctl`]
module"]
#[doc(alias = "CH7CTL")]
pub type Ch7ctl = crate::Reg<ch7ctl::Ch7ctlSpec>;
#[doc = "Channel 7 control register"]
pub mod ch7ctl;
#[doc = "CH7CNT (rw) register accessor: Channel 7 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7cnt`]
module"]
#[doc(alias = "CH7CNT")]
pub type Ch7cnt = crate::Reg<ch7cnt::Ch7cntSpec>;
#[doc = "Channel 7 counter register"]
pub mod ch7cnt;
#[doc = "CH7PADDR (rw) register accessor: Channel 7 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7paddr`]
module"]
#[doc(alias = "CH7PADDR")]
pub type Ch7paddr = crate::Reg<ch7paddr::Ch7paddrSpec>;
#[doc = "Channel 7 peripheral base address register"]
pub mod ch7paddr;
#[doc = "CH7M0ADDR (rw) register accessor: Channel 7 memory 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7m0addr`]
module"]
#[doc(alias = "CH7M0ADDR")]
pub type Ch7m0addr = crate::Reg<ch7m0addr::Ch7m0addrSpec>;
#[doc = "Channel 7 memory 0 base address register"]
pub mod ch7m0addr;
#[doc = "CH7M1ADDR (rw) register accessor: Channel 7 memory 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7m1addr`]
module"]
#[doc(alias = "CH7M1ADDR")]
pub type Ch7m1addr = crate::Reg<ch7m1addr::Ch7m1addrSpec>;
#[doc = "Channel 7 memory 1 base address register"]
pub mod ch7m1addr;
#[doc = "CH7FCTL (rw) register accessor: Channel 7 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch7fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch7fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7fctl`]
module"]
#[doc(alias = "CH7FCTL")]
pub type Ch7fctl = crate::Reg<ch7fctl::Ch7fctlSpec>;
#[doc = "Channel 7 FIFO control register"]
pub mod ch7fctl;
