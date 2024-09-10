#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hctl: Hctl,
    hft: Hft,
    hfinfr: Hfinfr,
    _reserved3: [u8; 0x04],
    hptfqstat: Hptfqstat,
    hachint: Hachint,
    hachinten: Hachinten,
    _reserved6: [u8; 0x24],
    hpcs: Hpcs,
    _reserved7: [u8; 0xbc],
    hch0ctl: Hch0ctl,
    _reserved8: [u8; 0x04],
    hch0intf: Hch0intf,
    hch0inten: Hch0inten,
    hch0len: Hch0len,
    _reserved11: [u8; 0x0c],
    hch1ctl: Hch1ctl,
    _reserved12: [u8; 0x04],
    hch1intf: Hch1intf,
    hch1inten: Hch1inten,
    hch1len: Hch1len,
    _reserved15: [u8; 0x0c],
    hch2ctl: Hch2ctl,
    _reserved16: [u8; 0x04],
    hch2intf: Hch2intf,
    hch2inten: Hch2inten,
    hch2len: Hch2len,
    _reserved19: [u8; 0x0c],
    hch3ctl: Hch3ctl,
    _reserved20: [u8; 0x04],
    hch3intf: Hch3intf,
    hch3inten: Hch3inten,
    hch3len: Hch3len,
    _reserved23: [u8; 0x0c],
    hch4ctl: Hch4ctl,
    _reserved24: [u8; 0x04],
    hch4intf: Hch4intf,
    hch4inten: Hch4inten,
    hch4len: Hch4len,
    _reserved27: [u8; 0x0c],
    hch5ctl: Hch5ctl,
    _reserved28: [u8; 0x04],
    hch5intf: Hch5intf,
    hch5inten: Hch5inten,
    hch5len: Hch5len,
    _reserved31: [u8; 0x0c],
    hch6ctl: Hch6ctl,
    _reserved32: [u8; 0x04],
    hch6intf: Hch6intf,
    hch6inten: Hch6inten,
    hch6len: Hch6len,
    _reserved35: [u8; 0x0c],
    hch7ctl: Hch7ctl,
    _reserved36: [u8; 0x04],
    hch7intf: Hch7intf,
    hch7inten: Hch7inten,
    hch7len: Hch7len,
}
impl RegisterBlock {
    #[doc = "0x00 - host configuration register (HCTL)"]
    #[inline(always)]
    pub const fn hctl(&self) -> &Hctl {
        &self.hctl
    }
    #[doc = "0x04 - Host frame interval register"]
    #[inline(always)]
    pub const fn hft(&self) -> &Hft {
        &self.hft
    }
    #[doc = "0x08 - OTG_FS host frame number/frame time remaining register (HFINFR)"]
    #[inline(always)]
    pub const fn hfinfr(&self) -> &Hfinfr {
        &self.hfinfr
    }
    #[doc = "0x10 - Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
    #[inline(always)]
    pub const fn hptfqstat(&self) -> &Hptfqstat {
        &self.hptfqstat
    }
    #[doc = "0x14 - Host all channels interrupt register"]
    #[inline(always)]
    pub const fn hachint(&self) -> &Hachint {
        &self.hachint
    }
    #[doc = "0x18 - host all channels interrupt mask register"]
    #[inline(always)]
    pub const fn hachinten(&self) -> &Hachinten {
        &self.hachinten
    }
    #[doc = "0x40 - Host port control and status register (USBFS_HPCS)"]
    #[inline(always)]
    pub const fn hpcs(&self) -> &Hpcs {
        &self.hpcs
    }
    #[doc = "0x100 - host channel-0 characteristics register (HCH0CTL)"]
    #[inline(always)]
    pub const fn hch0ctl(&self) -> &Hch0ctl {
        &self.hch0ctl
    }
    #[doc = "0x108 - host channel-0 interrupt register (USBFS_HCHxINTF)"]
    #[inline(always)]
    pub const fn hch0intf(&self) -> &Hch0intf {
        &self.hch0intf
    }
    #[doc = "0x10c - host channel-0 interrupt enable register (HCH0INTEN)"]
    #[inline(always)]
    pub const fn hch0inten(&self) -> &Hch0inten {
        &self.hch0inten
    }
    #[doc = "0x110 - host channel-0 transfer length register"]
    #[inline(always)]
    pub const fn hch0len(&self) -> &Hch0len {
        &self.hch0len
    }
    #[doc = "0x120 - host channel-1 characteristics register (HCH1CTL)"]
    #[inline(always)]
    pub const fn hch1ctl(&self) -> &Hch1ctl {
        &self.hch1ctl
    }
    #[doc = "0x128 - host channel-1 interrupt register (HCH1INTF)"]
    #[inline(always)]
    pub const fn hch1intf(&self) -> &Hch1intf {
        &self.hch1intf
    }
    #[doc = "0x12c - host channel-1 interrupt enable register (HCH1INTEN)"]
    #[inline(always)]
    pub const fn hch1inten(&self) -> &Hch1inten {
        &self.hch1inten
    }
    #[doc = "0x130 - host channel-1 transfer length register"]
    #[inline(always)]
    pub const fn hch1len(&self) -> &Hch1len {
        &self.hch1len
    }
    #[doc = "0x140 - host channel-2 characteristics register (HCH2CTL)"]
    #[inline(always)]
    pub const fn hch2ctl(&self) -> &Hch2ctl {
        &self.hch2ctl
    }
    #[doc = "0x148 - host channel-2 interrupt register (HCH2INTF)"]
    #[inline(always)]
    pub const fn hch2intf(&self) -> &Hch2intf {
        &self.hch2intf
    }
    #[doc = "0x14c - host channel-2 interrupt enable register (HCH2INTEN)"]
    #[inline(always)]
    pub const fn hch2inten(&self) -> &Hch2inten {
        &self.hch2inten
    }
    #[doc = "0x150 - host channel-2 transfer length register"]
    #[inline(always)]
    pub const fn hch2len(&self) -> &Hch2len {
        &self.hch2len
    }
    #[doc = "0x160 - host channel-3 characteristics register (HCH3CTL)"]
    #[inline(always)]
    pub const fn hch3ctl(&self) -> &Hch3ctl {
        &self.hch3ctl
    }
    #[doc = "0x168 - host channel-3 interrupt register (HCH3INTF)"]
    #[inline(always)]
    pub const fn hch3intf(&self) -> &Hch3intf {
        &self.hch3intf
    }
    #[doc = "0x16c - host channel-3 interrupt enable register (HCH3INTEN)"]
    #[inline(always)]
    pub const fn hch3inten(&self) -> &Hch3inten {
        &self.hch3inten
    }
    #[doc = "0x170 - host channel-3 transfer length register"]
    #[inline(always)]
    pub const fn hch3len(&self) -> &Hch3len {
        &self.hch3len
    }
    #[doc = "0x180 - host channel-4 characteristics register (HCH4CTL)"]
    #[inline(always)]
    pub const fn hch4ctl(&self) -> &Hch4ctl {
        &self.hch4ctl
    }
    #[doc = "0x188 - host channel-4 interrupt register (HCH4INTF)"]
    #[inline(always)]
    pub const fn hch4intf(&self) -> &Hch4intf {
        &self.hch4intf
    }
    #[doc = "0x18c - host channel-4 interrupt enable register (HCH4INTEN)"]
    #[inline(always)]
    pub const fn hch4inten(&self) -> &Hch4inten {
        &self.hch4inten
    }
    #[doc = "0x190 - host channel-4 transfer length register"]
    #[inline(always)]
    pub const fn hch4len(&self) -> &Hch4len {
        &self.hch4len
    }
    #[doc = "0x1a0 - host channel-5 characteristics register (HCH5CTL)"]
    #[inline(always)]
    pub const fn hch5ctl(&self) -> &Hch5ctl {
        &self.hch5ctl
    }
    #[doc = "0x1a8 - host channel-5 interrupt register (HCH5INTF)"]
    #[inline(always)]
    pub const fn hch5intf(&self) -> &Hch5intf {
        &self.hch5intf
    }
    #[doc = "0x1ac - host channel-5 interrupt enable register (HCH5INTEN)"]
    #[inline(always)]
    pub const fn hch5inten(&self) -> &Hch5inten {
        &self.hch5inten
    }
    #[doc = "0x1b0 - host channel-5 transfer length register"]
    #[inline(always)]
    pub const fn hch5len(&self) -> &Hch5len {
        &self.hch5len
    }
    #[doc = "0x1c0 - host channel-6 characteristics register (HCH6CTL)"]
    #[inline(always)]
    pub const fn hch6ctl(&self) -> &Hch6ctl {
        &self.hch6ctl
    }
    #[doc = "0x1c8 - host channel-6 interrupt register (HCH6INTF)"]
    #[inline(always)]
    pub const fn hch6intf(&self) -> &Hch6intf {
        &self.hch6intf
    }
    #[doc = "0x1cc - host channel-6 interrupt enable register (HCH6INTEN)"]
    #[inline(always)]
    pub const fn hch6inten(&self) -> &Hch6inten {
        &self.hch6inten
    }
    #[doc = "0x1d0 - host channel-6 transfer length register"]
    #[inline(always)]
    pub const fn hch6len(&self) -> &Hch6len {
        &self.hch6len
    }
    #[doc = "0x1e0 - host channel-7 characteristics register (HCH7CTL)"]
    #[inline(always)]
    pub const fn hch7ctl(&self) -> &Hch7ctl {
        &self.hch7ctl
    }
    #[doc = "0x1e8 - host channel-7 interrupt register (HCH7INTF)"]
    #[inline(always)]
    pub const fn hch7intf(&self) -> &Hch7intf {
        &self.hch7intf
    }
    #[doc = "0x1ec - host channel-7 interrupt enable register (HCH7INTEN)"]
    #[inline(always)]
    pub const fn hch7inten(&self) -> &Hch7inten {
        &self.hch7inten
    }
    #[doc = "0x1f0 - host channel-7 transfer length register"]
    #[inline(always)]
    pub const fn hch7len(&self) -> &Hch7len {
        &self.hch7len
    }
}
#[doc = "HCTL (rw) register accessor: host configuration register (HCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctl`]
module"]
#[doc(alias = "HCTL")]
pub type Hctl = crate::Reg<hctl::HctlSpec>;
#[doc = "host configuration register (HCTL)"]
pub mod hctl;
#[doc = "HFT (rw) register accessor: Host frame interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hft::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hft::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hft`]
module"]
#[doc(alias = "HFT")]
pub type Hft = crate::Reg<hft::HftSpec>;
#[doc = "Host frame interval register"]
pub mod hft;
#[doc = "HFINFR (r) register accessor: OTG_FS host frame number/frame time remaining register (HFINFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfinfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfinfr`]
module"]
#[doc(alias = "HFINFR")]
pub type Hfinfr = crate::Reg<hfinfr::HfinfrSpec>;
#[doc = "OTG_FS host frame number/frame time remaining register (HFINFR)"]
pub mod hfinfr;
#[doc = "HPTFQSTAT (r) register accessor: Host periodic transmit FIFO/queue status register (HPTFQSTAT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptfqstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptfqstat`]
module"]
#[doc(alias = "HPTFQSTAT")]
pub type Hptfqstat = crate::Reg<hptfqstat::HptfqstatSpec>;
#[doc = "Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
pub mod hptfqstat;
#[doc = "HACHINT (r) register accessor: Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hachint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hachint`]
module"]
#[doc(alias = "HACHINT")]
pub type Hachint = crate::Reg<hachint::HachintSpec>;
#[doc = "Host all channels interrupt register"]
pub mod hachint;
#[doc = "HACHINTEN (rw) register accessor: host all channels interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hachinten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hachinten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hachinten`]
module"]
#[doc(alias = "HACHINTEN")]
pub type Hachinten = crate::Reg<hachinten::HachintenSpec>;
#[doc = "host all channels interrupt mask register"]
pub mod hachinten;
#[doc = "HPCS (rw) register accessor: Host port control and status register (USBFS_HPCS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcs`]
module"]
#[doc(alias = "HPCS")]
pub type Hpcs = crate::Reg<hpcs::HpcsSpec>;
#[doc = "Host port control and status register (USBFS_HPCS)"]
pub mod hpcs;
#[doc = "HCH0CTL (rw) register accessor: host channel-0 characteristics register (HCH0CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch0ctl`]
module"]
#[doc(alias = "HCH0CTL")]
pub type Hch0ctl = crate::Reg<hch0ctl::Hch0ctlSpec>;
#[doc = "host channel-0 characteristics register (HCH0CTL)"]
pub mod hch0ctl;
#[doc = "HCH1CTL (rw) register accessor: host channel-1 characteristics register (HCH1CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch1ctl`]
module"]
#[doc(alias = "HCH1CTL")]
pub type Hch1ctl = crate::Reg<hch1ctl::Hch1ctlSpec>;
#[doc = "host channel-1 characteristics register (HCH1CTL)"]
pub mod hch1ctl;
#[doc = "HCH2CTL (rw) register accessor: host channel-2 characteristics register (HCH2CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch2ctl`]
module"]
#[doc(alias = "HCH2CTL")]
pub type Hch2ctl = crate::Reg<hch2ctl::Hch2ctlSpec>;
#[doc = "host channel-2 characteristics register (HCH2CTL)"]
pub mod hch2ctl;
#[doc = "HCH3CTL (rw) register accessor: host channel-3 characteristics register (HCH3CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch3ctl`]
module"]
#[doc(alias = "HCH3CTL")]
pub type Hch3ctl = crate::Reg<hch3ctl::Hch3ctlSpec>;
#[doc = "host channel-3 characteristics register (HCH3CTL)"]
pub mod hch3ctl;
#[doc = "HCH4CTL (rw) register accessor: host channel-4 characteristics register (HCH4CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch4ctl`]
module"]
#[doc(alias = "HCH4CTL")]
pub type Hch4ctl = crate::Reg<hch4ctl::Hch4ctlSpec>;
#[doc = "host channel-4 characteristics register (HCH4CTL)"]
pub mod hch4ctl;
#[doc = "HCH5CTL (rw) register accessor: host channel-5 characteristics register (HCH5CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch5ctl`]
module"]
#[doc(alias = "HCH5CTL")]
pub type Hch5ctl = crate::Reg<hch5ctl::Hch5ctlSpec>;
#[doc = "host channel-5 characteristics register (HCH5CTL)"]
pub mod hch5ctl;
#[doc = "HCH6CTL (rw) register accessor: host channel-6 characteristics register (HCH6CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch6ctl`]
module"]
#[doc(alias = "HCH6CTL")]
pub type Hch6ctl = crate::Reg<hch6ctl::Hch6ctlSpec>;
#[doc = "host channel-6 characteristics register (HCH6CTL)"]
pub mod hch6ctl;
#[doc = "HCH7CTL (rw) register accessor: host channel-7 characteristics register (HCH7CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch7ctl`]
module"]
#[doc(alias = "HCH7CTL")]
pub type Hch7ctl = crate::Reg<hch7ctl::Hch7ctlSpec>;
#[doc = "host channel-7 characteristics register (HCH7CTL)"]
pub mod hch7ctl;
#[doc = "HCH0INTF (rw) register accessor: host channel-0 interrupt register (USBFS_HCHxINTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch0intf`]
module"]
#[doc(alias = "HCH0INTF")]
pub type Hch0intf = crate::Reg<hch0intf::Hch0intfSpec>;
#[doc = "host channel-0 interrupt register (USBFS_HCHxINTF)"]
pub mod hch0intf;
#[doc = "HCH1INTF (rw) register accessor: host channel-1 interrupt register (HCH1INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch1intf`]
module"]
#[doc(alias = "HCH1INTF")]
pub type Hch1intf = crate::Reg<hch1intf::Hch1intfSpec>;
#[doc = "host channel-1 interrupt register (HCH1INTF)"]
pub mod hch1intf;
#[doc = "HCH2INTF (rw) register accessor: host channel-2 interrupt register (HCH2INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch2intf`]
module"]
#[doc(alias = "HCH2INTF")]
pub type Hch2intf = crate::Reg<hch2intf::Hch2intfSpec>;
#[doc = "host channel-2 interrupt register (HCH2INTF)"]
pub mod hch2intf;
#[doc = "HCH3INTF (rw) register accessor: host channel-3 interrupt register (HCH3INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch3intf`]
module"]
#[doc(alias = "HCH3INTF")]
pub type Hch3intf = crate::Reg<hch3intf::Hch3intfSpec>;
#[doc = "host channel-3 interrupt register (HCH3INTF)"]
pub mod hch3intf;
#[doc = "HCH4INTF (rw) register accessor: host channel-4 interrupt register (HCH4INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch4intf`]
module"]
#[doc(alias = "HCH4INTF")]
pub type Hch4intf = crate::Reg<hch4intf::Hch4intfSpec>;
#[doc = "host channel-4 interrupt register (HCH4INTF)"]
pub mod hch4intf;
#[doc = "HCH5INTF (rw) register accessor: host channel-5 interrupt register (HCH5INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch5intf`]
module"]
#[doc(alias = "HCH5INTF")]
pub type Hch5intf = crate::Reg<hch5intf::Hch5intfSpec>;
#[doc = "host channel-5 interrupt register (HCH5INTF)"]
pub mod hch5intf;
#[doc = "HCH6INTF (rw) register accessor: host channel-6 interrupt register (HCH6INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch6intf`]
module"]
#[doc(alias = "HCH6INTF")]
pub type Hch6intf = crate::Reg<hch6intf::Hch6intfSpec>;
#[doc = "host channel-6 interrupt register (HCH6INTF)"]
pub mod hch6intf;
#[doc = "HCH7INTF (rw) register accessor: host channel-7 interrupt register (HCH7INTF)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7intf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7intf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch7intf`]
module"]
#[doc(alias = "HCH7INTF")]
pub type Hch7intf = crate::Reg<hch7intf::Hch7intfSpec>;
#[doc = "host channel-7 interrupt register (HCH7INTF)"]
pub mod hch7intf;
#[doc = "HCH0INTEN (rw) register accessor: host channel-0 interrupt enable register (HCH0INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch0inten`]
module"]
#[doc(alias = "HCH0INTEN")]
pub type Hch0inten = crate::Reg<hch0inten::Hch0intenSpec>;
#[doc = "host channel-0 interrupt enable register (HCH0INTEN)"]
pub mod hch0inten;
#[doc = "HCH1INTEN (rw) register accessor: host channel-1 interrupt enable register (HCH1INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch1inten`]
module"]
#[doc(alias = "HCH1INTEN")]
pub type Hch1inten = crate::Reg<hch1inten::Hch1intenSpec>;
#[doc = "host channel-1 interrupt enable register (HCH1INTEN)"]
pub mod hch1inten;
#[doc = "HCH2INTEN (rw) register accessor: host channel-2 interrupt enable register (HCH2INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch2inten`]
module"]
#[doc(alias = "HCH2INTEN")]
pub type Hch2inten = crate::Reg<hch2inten::Hch2intenSpec>;
#[doc = "host channel-2 interrupt enable register (HCH2INTEN)"]
pub mod hch2inten;
#[doc = "HCH3INTEN (rw) register accessor: host channel-3 interrupt enable register (HCH3INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch3inten`]
module"]
#[doc(alias = "HCH3INTEN")]
pub type Hch3inten = crate::Reg<hch3inten::Hch3intenSpec>;
#[doc = "host channel-3 interrupt enable register (HCH3INTEN)"]
pub mod hch3inten;
#[doc = "HCH4INTEN (rw) register accessor: host channel-4 interrupt enable register (HCH4INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch4inten`]
module"]
#[doc(alias = "HCH4INTEN")]
pub type Hch4inten = crate::Reg<hch4inten::Hch4intenSpec>;
#[doc = "host channel-4 interrupt enable register (HCH4INTEN)"]
pub mod hch4inten;
#[doc = "HCH5INTEN (rw) register accessor: host channel-5 interrupt enable register (HCH5INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch5inten`]
module"]
#[doc(alias = "HCH5INTEN")]
pub type Hch5inten = crate::Reg<hch5inten::Hch5intenSpec>;
#[doc = "host channel-5 interrupt enable register (HCH5INTEN)"]
pub mod hch5inten;
#[doc = "HCH6INTEN (rw) register accessor: host channel-6 interrupt enable register (HCH6INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch6inten`]
module"]
#[doc(alias = "HCH6INTEN")]
pub type Hch6inten = crate::Reg<hch6inten::Hch6intenSpec>;
#[doc = "host channel-6 interrupt enable register (HCH6INTEN)"]
pub mod hch6inten;
#[doc = "HCH7INTEN (rw) register accessor: host channel-7 interrupt enable register (HCH7INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch7inten`]
module"]
#[doc(alias = "HCH7INTEN")]
pub type Hch7inten = crate::Reg<hch7inten::Hch7intenSpec>;
#[doc = "host channel-7 interrupt enable register (HCH7INTEN)"]
pub mod hch7inten;
#[doc = "HCH0LEN (rw) register accessor: host channel-0 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch0len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch0len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch0len`]
module"]
#[doc(alias = "HCH0LEN")]
pub type Hch0len = crate::Reg<hch0len::Hch0lenSpec>;
#[doc = "host channel-0 transfer length register"]
pub mod hch0len;
#[doc = "HCH1LEN (rw) register accessor: host channel-1 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch1len`]
module"]
#[doc(alias = "HCH1LEN")]
pub type Hch1len = crate::Reg<hch1len::Hch1lenSpec>;
#[doc = "host channel-1 transfer length register"]
pub mod hch1len;
#[doc = "HCH2LEN (rw) register accessor: host channel-2 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch2len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch2len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch2len`]
module"]
#[doc(alias = "HCH2LEN")]
pub type Hch2len = crate::Reg<hch2len::Hch2lenSpec>;
#[doc = "host channel-2 transfer length register"]
pub mod hch2len;
#[doc = "HCH3LEN (rw) register accessor: host channel-3 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch3len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch3len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch3len`]
module"]
#[doc(alias = "HCH3LEN")]
pub type Hch3len = crate::Reg<hch3len::Hch3lenSpec>;
#[doc = "host channel-3 transfer length register"]
pub mod hch3len;
#[doc = "HCH4LEN (rw) register accessor: host channel-4 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch4len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch4len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch4len`]
module"]
#[doc(alias = "HCH4LEN")]
pub type Hch4len = crate::Reg<hch4len::Hch4lenSpec>;
#[doc = "host channel-4 transfer length register"]
pub mod hch4len;
#[doc = "HCH5LEN (rw) register accessor: host channel-5 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch5len`]
module"]
#[doc(alias = "HCH5LEN")]
pub type Hch5len = crate::Reg<hch5len::Hch5lenSpec>;
#[doc = "host channel-5 transfer length register"]
pub mod hch5len;
#[doc = "HCH6LEN (rw) register accessor: host channel-6 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch6len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch6len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch6len`]
module"]
#[doc(alias = "HCH6LEN")]
pub type Hch6len = crate::Reg<hch6len::Hch6lenSpec>;
#[doc = "host channel-6 transfer length register"]
pub mod hch6len;
#[doc = "HCH7LEN (rw) register accessor: host channel-7 transfer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch7len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch7len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hch7len`]
module"]
#[doc(alias = "HCH7LEN")]
pub type Hch7len = crate::Reg<hch7len::Hch7lenSpec>;
#[doc = "host channel-7 transfer length register"]
pub mod hch7len;
