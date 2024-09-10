#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ep0cs: Ep0cs,
    _reserved1: [u8; 0x02],
    ep1cs: Ep1cs,
    _reserved2: [u8; 0x02],
    ep2cs: Ep2cs,
    _reserved3: [u8; 0x02],
    ep3cs: Ep3cs,
    _reserved4: [u8; 0x02],
    ep4cs: Ep4cs,
    _reserved5: [u8; 0x02],
    ep5cs: Ep5cs,
    _reserved6: [u8; 0x02],
    ep6cs: Ep6cs,
    _reserved7: [u8; 0x02],
    ep7cs: Ep7cs,
    _reserved8: [u8; 0x22],
    ctl: Ctl,
    _reserved9: [u8; 0x02],
    intf: Intf,
    _reserved10: [u8; 0x02],
    stat: Stat,
    _reserved11: [u8; 0x02],
    daddr: Daddr,
    _reserved12: [u8; 0x02],
    baddr: Baddr,
    _reserved13: [u8; 0xae],
    sep0: Sep0,
    _reserved14: [u8; 0x02],
    sep1: Sep1,
    _reserved15: [u8; 0x02],
    sep2: Sep2,
    _reserved16: [u8; 0x02],
    sep3: Sep3,
    _reserved17: [u8; 0x02],
    sep4: Sep4,
    _reserved18: [u8; 0x02],
    sep5: Sep5,
    _reserved19: [u8; 0x02],
    sep6: Sep6,
    _reserved20: [u8; 0x02],
    sep7: Sep7,
    _reserved21: [u8; 0x22],
    lpmctl: Lpmctl,
    _reserved22: [u8; 0x02],
    lpmintf: Lpmintf,
}
impl RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    #[inline(always)]
    pub const fn ep0cs(&self) -> &Ep0cs {
        &self.ep0cs
    }
    #[doc = "0x04 - endpoint 1 register"]
    #[inline(always)]
    pub const fn ep1cs(&self) -> &Ep1cs {
        &self.ep1cs
    }
    #[doc = "0x08 - endpoint 2 register"]
    #[inline(always)]
    pub const fn ep2cs(&self) -> &Ep2cs {
        &self.ep2cs
    }
    #[doc = "0x0c - endpoint 3 register"]
    #[inline(always)]
    pub const fn ep3cs(&self) -> &Ep3cs {
        &self.ep3cs
    }
    #[doc = "0x10 - endpoint 4 register"]
    #[inline(always)]
    pub const fn ep4cs(&self) -> &Ep4cs {
        &self.ep4cs
    }
    #[doc = "0x14 - endpoint 5 register"]
    #[inline(always)]
    pub const fn ep5cs(&self) -> &Ep5cs {
        &self.ep5cs
    }
    #[doc = "0x18 - endpoint 6 register"]
    #[inline(always)]
    pub const fn ep6cs(&self) -> &Ep6cs {
        &self.ep6cs
    }
    #[doc = "0x1c - endpoint 7 register"]
    #[inline(always)]
    pub const fn ep7cs(&self) -> &Ep7cs {
        &self.ep7cs
    }
    #[doc = "0x40 - control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x44 - interrupt flag register"]
    #[inline(always)]
    pub const fn intf(&self) -> &Intf {
        &self.intf
    }
    #[doc = "0x48 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x4c - device address register"]
    #[inline(always)]
    pub const fn daddr(&self) -> &Daddr {
        &self.daddr
    }
    #[doc = "0x50 - Buffer address register"]
    #[inline(always)]
    pub const fn baddr(&self) -> &Baddr {
        &self.baddr
    }
    #[doc = "0x100 - USB sub-endpoint 0 register"]
    #[inline(always)]
    pub const fn sep0(&self) -> &Sep0 {
        &self.sep0
    }
    #[doc = "0x104 - USB sub-endpoint 1 register"]
    #[inline(always)]
    pub const fn sep1(&self) -> &Sep1 {
        &self.sep1
    }
    #[doc = "0x108 - USB sub-endpoint 2 register"]
    #[inline(always)]
    pub const fn sep2(&self) -> &Sep2 {
        &self.sep2
    }
    #[doc = "0x10c - USB sub-endpoint 3 register"]
    #[inline(always)]
    pub const fn sep3(&self) -> &Sep3 {
        &self.sep3
    }
    #[doc = "0x110 - USB sub-endpoint 4 register"]
    #[inline(always)]
    pub const fn sep4(&self) -> &Sep4 {
        &self.sep4
    }
    #[doc = "0x114 - USB sub-endpoint 5 register"]
    #[inline(always)]
    pub const fn sep5(&self) -> &Sep5 {
        &self.sep5
    }
    #[doc = "0x118 - USB sub-endpoint 6 register"]
    #[inline(always)]
    pub const fn sep6(&self) -> &Sep6 {
        &self.sep6
    }
    #[doc = "0x11c - USB sub-endpoint 7 register"]
    #[inline(always)]
    pub const fn sep7(&self) -> &Sep7 {
        &self.sep7
    }
    #[doc = "0x140 - USB LPM control register"]
    #[inline(always)]
    pub const fn lpmctl(&self) -> &Lpmctl {
        &self.lpmctl
    }
    #[doc = "0x144 - USB LPM interrupt flag register"]
    #[inline(always)]
    pub const fn lpmintf(&self) -> &Lpmintf {
        &self.lpmintf
    }
}
#[doc = "EP0CS (rw) register accessor: endpoint 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0cs`]
module"]
#[doc(alias = "EP0CS")]
pub type Ep0cs = crate::Reg<ep0cs::Ep0csSpec>;
#[doc = "endpoint 0 register"]
pub mod ep0cs;
#[doc = "EP1CS (rw) register accessor: endpoint 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1cs`]
module"]
#[doc(alias = "EP1CS")]
pub type Ep1cs = crate::Reg<ep1cs::Ep1csSpec>;
#[doc = "endpoint 1 register"]
pub mod ep1cs;
#[doc = "EP2CS (rw) register accessor: endpoint 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2cs`]
module"]
#[doc(alias = "EP2CS")]
pub type Ep2cs = crate::Reg<ep2cs::Ep2csSpec>;
#[doc = "endpoint 2 register"]
pub mod ep2cs;
#[doc = "EP3CS (rw) register accessor: endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3cs`]
module"]
#[doc(alias = "EP3CS")]
pub type Ep3cs = crate::Reg<ep3cs::Ep3csSpec>;
#[doc = "endpoint 3 register"]
pub mod ep3cs;
#[doc = "EP4CS (rw) register accessor: endpoint 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4cs`]
module"]
#[doc(alias = "EP4CS")]
pub type Ep4cs = crate::Reg<ep4cs::Ep4csSpec>;
#[doc = "endpoint 4 register"]
pub mod ep4cs;
#[doc = "EP5CS (rw) register accessor: endpoint 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5cs`]
module"]
#[doc(alias = "EP5CS")]
pub type Ep5cs = crate::Reg<ep5cs::Ep5csSpec>;
#[doc = "endpoint 5 register"]
pub mod ep5cs;
#[doc = "EP6CS (rw) register accessor: endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6cs`]
module"]
#[doc(alias = "EP6CS")]
pub type Ep6cs = crate::Reg<ep6cs::Ep6csSpec>;
#[doc = "endpoint 6 register"]
pub mod ep6cs;
#[doc = "EP7CS (rw) register accessor: endpoint 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7cs`]
module"]
#[doc(alias = "EP7CS")]
pub type Ep7cs = crate::Reg<ep7cs::Ep7csSpec>;
#[doc = "endpoint 7 register"]
pub mod ep7cs;
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "control register"]
pub mod ctl;
#[doc = "INTF (rw) register accessor: interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
#[doc(alias = "INTF")]
pub type Intf = crate::Reg<intf::IntfSpec>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "STAT (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "DADDR (rw) register accessor: device address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`]
module"]
#[doc(alias = "DADDR")]
pub type Daddr = crate::Reg<daddr::DaddrSpec>;
#[doc = "device address register"]
pub mod daddr;
#[doc = "BADDR (rw) register accessor: Buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baddr`]
module"]
#[doc(alias = "BADDR")]
pub type Baddr = crate::Reg<baddr::BaddrSpec>;
#[doc = "Buffer address register"]
pub mod baddr;
#[doc = "SEP0 (rw) register accessor: USB sub-endpoint 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sep0`]
module"]
#[doc(alias = "SEP0")]
pub type Sep0 = crate::Reg<sep0::Sep0Spec>;
#[doc = "USB sub-endpoint 0 register"]
pub mod sep0;
#[doc = "SEP1 (rw) register accessor: USB sub-endpoint 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sep1`]
module"]
#[doc(alias = "SEP1")]
pub type Sep1 = crate::Reg<sep1::Sep1Spec>;
#[doc = "USB sub-endpoint 1 register"]
pub mod sep1;
#[doc = "SEP2 (rw) register accessor: USB sub-endpoint 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sep2`]
module"]
#[doc(alias = "SEP2")]
pub type Sep2 = crate::Reg<sep2::Sep2Spec>;
#[doc = "USB sub-endpoint 2 register"]
pub mod sep2;
#[doc = "SEP3 (rw) register accessor: USB sub-endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sep3`]
module"]
#[doc(alias = "SEP3")]
pub type Sep3 = crate::Reg<sep3::Sep3Spec>;
#[doc = "USB sub-endpoint 3 register"]
pub mod sep3;
#[doc = "SEP4 (rw) register accessor: USB sub-endpoint 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sep4`]
module"]
#[doc(alias = "SEP4")]
pub type Sep4 = crate::Reg<sep4::Sep4Spec>;
#[doc = "USB sub-endpoint 4 register"]
pub mod sep4;
#[doc = "SEP5 (rw) register accessor: USB sub-endpoint 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sep5`]
module"]
#[doc(alias = "SEP5")]
pub type Sep5 = crate::Reg<sep5::Sep5Spec>;
#[doc = "USB sub-endpoint 5 register"]
pub mod sep5;
#[doc = "SEP6 (rw) register accessor: USB sub-endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sep6`]
module"]
#[doc(alias = "SEP6")]
pub type Sep6 = crate::Reg<sep6::Sep6Spec>;
#[doc = "USB sub-endpoint 6 register"]
pub mod sep6;
#[doc = "SEP7 (rw) register accessor: USB sub-endpoint 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sep7`]
module"]
#[doc(alias = "SEP7")]
pub type Sep7 = crate::Reg<sep7::Sep7Spec>;
#[doc = "USB sub-endpoint 7 register"]
pub mod sep7;
#[doc = "LPMCTL (rw) register accessor: USB LPM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmctl`]
module"]
#[doc(alias = "LPMCTL")]
pub type Lpmctl = crate::Reg<lpmctl::LpmctlSpec>;
#[doc = "USB LPM control register"]
pub mod lpmctl;
#[doc = "LPMINTF (rw) register accessor: USB LPM interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmintf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmintf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmintf`]
module"]
#[doc(alias = "LPMINTF")]
pub type Lpmintf = crate::Reg<lpmintf::LpmintfSpec>;
#[doc = "USB LPM interrupt flag register"]
pub mod lpmintf;
