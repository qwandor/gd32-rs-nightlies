#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipa_ctl: IpaCtl,
    ipa_intf: IpaIntf,
    ipa_intc: IpaIntc,
    ipa_fmaddr: IpaFmaddr,
    ipa_floff: IpaFloff,
    ipa_bmaddr: IpaBmaddr,
    ipa_bloff: IpaBloff,
    ipa_fpctl: IpaFpctl,
    ipa_fpv: IpaFpv,
    ipa_bpctl: IpaBpctl,
    ipa_bpv: IpaBpv,
    ipa_flmaddr: IpaFlmaddr,
    ipa_blmaddr: IpaBlmaddr,
    ipa_dpctl: IpaDpctl,
    _reserved_14_ipa_dpv: [u8; 0x04],
    ipa_dmaddr: IpaDmaddr,
    ipa_dloff: IpaDloff,
    ipa_ims: IpaIms,
    ipa_lm: IpaLm,
    ipa_itctl: IpaItctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ipa_ctl(&self) -> &IpaCtl {
        &self.ipa_ctl
    }
    #[doc = "0x04 - Interrupt flag register"]
    #[inline(always)]
    pub const fn ipa_intf(&self) -> &IpaIntf {
        &self.ipa_intf
    }
    #[doc = "0x08 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn ipa_intc(&self) -> &IpaIntc {
        &self.ipa_intc
    }
    #[doc = "0x0c - Foreground memory base address register"]
    #[inline(always)]
    pub const fn ipa_fmaddr(&self) -> &IpaFmaddr {
        &self.ipa_fmaddr
    }
    #[doc = "0x10 - Foreground line offset register"]
    #[inline(always)]
    pub const fn ipa_floff(&self) -> &IpaFloff {
        &self.ipa_floff
    }
    #[doc = "0x14 - Background memory base address register"]
    #[inline(always)]
    pub const fn ipa_bmaddr(&self) -> &IpaBmaddr {
        &self.ipa_bmaddr
    }
    #[doc = "0x18 - Background line offset register"]
    #[inline(always)]
    pub const fn ipa_bloff(&self) -> &IpaBloff {
        &self.ipa_bloff
    }
    #[doc = "0x1c - Foreground pixel control register"]
    #[inline(always)]
    pub const fn ipa_fpctl(&self) -> &IpaFpctl {
        &self.ipa_fpctl
    }
    #[doc = "0x20 - Foreground pixel value register"]
    #[inline(always)]
    pub const fn ipa_fpv(&self) -> &IpaFpv {
        &self.ipa_fpv
    }
    #[doc = "0x24 - Background pixel control register"]
    #[inline(always)]
    pub const fn ipa_bpctl(&self) -> &IpaBpctl {
        &self.ipa_bpctl
    }
    #[doc = "0x28 - Background pixel value register"]
    #[inline(always)]
    pub const fn ipa_bpv(&self) -> &IpaBpv {
        &self.ipa_bpv
    }
    #[doc = "0x2c - Foreground LUT memory base address register"]
    #[inline(always)]
    pub const fn ipa_flmaddr(&self) -> &IpaFlmaddr {
        &self.ipa_flmaddr
    }
    #[doc = "0x30 - Background LUT memory base address register"]
    #[inline(always)]
    pub const fn ipa_blmaddr(&self) -> &IpaBlmaddr {
        &self.ipa_blmaddr
    }
    #[doc = "0x34 - Destination pixel control register"]
    #[inline(always)]
    pub const fn ipa_dpctl(&self) -> &IpaDpctl {
        &self.ipa_dpctl
    }
    #[doc = "0x38 - Destination pixel value register(When the destination pixel format is ARGB4444,)"]
    #[inline(always)]
    pub const fn ipa_dpv_argb4444(&self) -> &IpaDpvArgb4444 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - Destination pixel value register(When the destination pixel format is ARGB1555)"]
    #[inline(always)]
    pub const fn ipa_dpv_argb1555(&self) -> &IpaDpvArgb1555 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - Destination pixel value register(When the destination pixel format is RGB565)"]
    #[inline(always)]
    pub const fn ipa_dpv_rgb565(&self) -> &IpaDpvRgb565 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - Destination pixel value register(When the destination pixel format is RGB888)"]
    #[inline(always)]
    pub const fn ipa_dpv_rgb888(&self) -> &IpaDpvRgb888 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - Destination pixel value register(When the destination pixel format is ARGB8888)"]
    #[inline(always)]
    pub const fn ipa_dpv_argb8888(&self) -> &IpaDpvArgb8888 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3c - Destination memory base address register"]
    #[inline(always)]
    pub const fn ipa_dmaddr(&self) -> &IpaDmaddr {
        &self.ipa_dmaddr
    }
    #[doc = "0x40 - Destination line offset register"]
    #[inline(always)]
    pub const fn ipa_dloff(&self) -> &IpaDloff {
        &self.ipa_dloff
    }
    #[doc = "0x44 - Image size register"]
    #[inline(always)]
    pub const fn ipa_ims(&self) -> &IpaIms {
        &self.ipa_ims
    }
    #[doc = "0x48 - Line mark register"]
    #[inline(always)]
    pub const fn ipa_lm(&self) -> &IpaLm {
        &self.ipa_lm
    }
    #[doc = "0x4c - Inter-timer control register"]
    #[inline(always)]
    pub const fn ipa_itctl(&self) -> &IpaItctl {
        &self.ipa_itctl
    }
}
#[doc = "IPA_CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_ctl`]
module"]
#[doc(alias = "IPA_CTL")]
pub type IpaCtl = crate::Reg<ipa_ctl::IpaCtlSpec>;
#[doc = "Control register"]
pub mod ipa_ctl;
#[doc = "IPA_INTF (r) register accessor: Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_intf`]
module"]
#[doc(alias = "IPA_INTF")]
pub type IpaIntf = crate::Reg<ipa_intf::IpaIntfSpec>;
#[doc = "Interrupt flag register"]
pub mod ipa_intf;
#[doc = "IPA_INTC (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_intc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_intc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_intc`]
module"]
#[doc(alias = "IPA_INTC")]
pub type IpaIntc = crate::Reg<ipa_intc::IpaIntcSpec>;
#[doc = "Interrupt flag clear register"]
pub mod ipa_intc;
#[doc = "IPA_FMADDR (rw) register accessor: Foreground memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_fmaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_fmaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_fmaddr`]
module"]
#[doc(alias = "IPA_FMADDR")]
pub type IpaFmaddr = crate::Reg<ipa_fmaddr::IpaFmaddrSpec>;
#[doc = "Foreground memory base address register"]
pub mod ipa_fmaddr;
#[doc = "IPA_FLOFF (rw) register accessor: Foreground line offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_floff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_floff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_floff`]
module"]
#[doc(alias = "IPA_FLOFF")]
pub type IpaFloff = crate::Reg<ipa_floff::IpaFloffSpec>;
#[doc = "Foreground line offset register"]
pub mod ipa_floff;
#[doc = "IPA_BMADDR (rw) register accessor: Background memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_bmaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_bmaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_bmaddr`]
module"]
#[doc(alias = "IPA_BMADDR")]
pub type IpaBmaddr = crate::Reg<ipa_bmaddr::IpaBmaddrSpec>;
#[doc = "Background memory base address register"]
pub mod ipa_bmaddr;
#[doc = "IPA_BLOFF (rw) register accessor: Background line offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_bloff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_bloff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_bloff`]
module"]
#[doc(alias = "IPA_BLOFF")]
pub type IpaBloff = crate::Reg<ipa_bloff::IpaBloffSpec>;
#[doc = "Background line offset register"]
pub mod ipa_bloff;
#[doc = "IPA_FPCTL (rw) register accessor: Foreground pixel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_fpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_fpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_fpctl`]
module"]
#[doc(alias = "IPA_FPCTL")]
pub type IpaFpctl = crate::Reg<ipa_fpctl::IpaFpctlSpec>;
#[doc = "Foreground pixel control register"]
pub mod ipa_fpctl;
#[doc = "IPA_FPV (rw) register accessor: Foreground pixel value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_fpv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_fpv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_fpv`]
module"]
#[doc(alias = "IPA_FPV")]
pub type IpaFpv = crate::Reg<ipa_fpv::IpaFpvSpec>;
#[doc = "Foreground pixel value register"]
pub mod ipa_fpv;
#[doc = "IPA_BPCTL (rw) register accessor: Background pixel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_bpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_bpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_bpctl`]
module"]
#[doc(alias = "IPA_BPCTL")]
pub type IpaBpctl = crate::Reg<ipa_bpctl::IpaBpctlSpec>;
#[doc = "Background pixel control register"]
pub mod ipa_bpctl;
#[doc = "IPA_BPV (rw) register accessor: Background pixel value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_bpv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_bpv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_bpv`]
module"]
#[doc(alias = "IPA_BPV")]
pub type IpaBpv = crate::Reg<ipa_bpv::IpaBpvSpec>;
#[doc = "Background pixel value register"]
pub mod ipa_bpv;
#[doc = "IPA_FLMADDR (rw) register accessor: Foreground LUT memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_flmaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_flmaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_flmaddr`]
module"]
#[doc(alias = "IPA_FLMADDR")]
pub type IpaFlmaddr = crate::Reg<ipa_flmaddr::IpaFlmaddrSpec>;
#[doc = "Foreground LUT memory base address register"]
pub mod ipa_flmaddr;
#[doc = "IPA_BLMADDR (rw) register accessor: Background LUT memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_blmaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_blmaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_blmaddr`]
module"]
#[doc(alias = "IPA_BLMADDR")]
pub type IpaBlmaddr = crate::Reg<ipa_blmaddr::IpaBlmaddrSpec>;
#[doc = "Background LUT memory base address register"]
pub mod ipa_blmaddr;
#[doc = "IPA_DPCTL (rw) register accessor: Destination pixel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_dpctl`]
module"]
#[doc(alias = "IPA_DPCTL")]
pub type IpaDpctl = crate::Reg<ipa_dpctl::IpaDpctlSpec>;
#[doc = "Destination pixel control register"]
pub mod ipa_dpctl;
#[doc = "IPA_DPV_ARGB8888 (rw) register accessor: Destination pixel value register(When the destination pixel format is ARGB8888)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpv_argb8888::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpv_argb8888::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_dpv_argb8888`]
module"]
#[doc(alias = "IPA_DPV_ARGB8888")]
pub type IpaDpvArgb8888 = crate::Reg<ipa_dpv_argb8888::IpaDpvArgb8888Spec>;
#[doc = "Destination pixel value register(When the destination pixel format is ARGB8888)"]
pub mod ipa_dpv_argb8888;
#[doc = "IPA_DPV_RGB888 (rw) register accessor: Destination pixel value register(When the destination pixel format is RGB888)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpv_rgb888::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpv_rgb888::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_dpv_rgb888`]
module"]
#[doc(alias = "IPA_DPV_RGB888")]
pub type IpaDpvRgb888 = crate::Reg<ipa_dpv_rgb888::IpaDpvRgb888Spec>;
#[doc = "Destination pixel value register(When the destination pixel format is RGB888)"]
pub mod ipa_dpv_rgb888;
#[doc = "IPA_DPV_RGB565 (rw) register accessor: Destination pixel value register(When the destination pixel format is RGB565)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpv_rgb565::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpv_rgb565::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_dpv_rgb565`]
module"]
#[doc(alias = "IPA_DPV_RGB565")]
pub type IpaDpvRgb565 = crate::Reg<ipa_dpv_rgb565::IpaDpvRgb565Spec>;
#[doc = "Destination pixel value register(When the destination pixel format is RGB565)"]
pub mod ipa_dpv_rgb565;
#[doc = "IPA_DPV_ARGB1555 (rw) register accessor: Destination pixel value register(When the destination pixel format is ARGB1555)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpv_argb1555::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpv_argb1555::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_dpv_argb1555`]
module"]
#[doc(alias = "IPA_DPV_ARGB1555")]
pub type IpaDpvArgb1555 = crate::Reg<ipa_dpv_argb1555::IpaDpvArgb1555Spec>;
#[doc = "Destination pixel value register(When the destination pixel format is ARGB1555)"]
pub mod ipa_dpv_argb1555;
#[doc = "IPA_DPV_ARGB4444 (rw) register accessor: Destination pixel value register(When the destination pixel format is ARGB4444,)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dpv_argb4444::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dpv_argb4444::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_dpv_argb4444`]
module"]
#[doc(alias = "IPA_DPV_ARGB4444")]
pub type IpaDpvArgb4444 = crate::Reg<ipa_dpv_argb4444::IpaDpvArgb4444Spec>;
#[doc = "Destination pixel value register(When the destination pixel format is ARGB4444,)"]
pub mod ipa_dpv_argb4444;
#[doc = "IPA_DMADDR (rw) register accessor: Destination memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dmaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dmaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_dmaddr`]
module"]
#[doc(alias = "IPA_DMADDR")]
pub type IpaDmaddr = crate::Reg<ipa_dmaddr::IpaDmaddrSpec>;
#[doc = "Destination memory base address register"]
pub mod ipa_dmaddr;
#[doc = "IPA_DLOFF (rw) register accessor: Destination line offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_dloff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_dloff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_dloff`]
module"]
#[doc(alias = "IPA_DLOFF")]
pub type IpaDloff = crate::Reg<ipa_dloff::IpaDloffSpec>;
#[doc = "Destination line offset register"]
pub mod ipa_dloff;
#[doc = "IPA_IMS (rw) register accessor: Image size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_ims::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_ims::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_ims`]
module"]
#[doc(alias = "IPA_IMS")]
pub type IpaIms = crate::Reg<ipa_ims::IpaImsSpec>;
#[doc = "Image size register"]
pub mod ipa_ims;
#[doc = "IPA_LM (rw) register accessor: Line mark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_lm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_lm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_lm`]
module"]
#[doc(alias = "IPA_LM")]
pub type IpaLm = crate::Reg<ipa_lm::IpaLmSpec>;
#[doc = "Line mark register"]
pub mod ipa_lm;
#[doc = "IPA_ITCTL (rw) register accessor: Inter-timer control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_itctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_itctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipa_itctl`]
module"]
#[doc(alias = "IPA_ITCTL")]
pub type IpaItctl = crate::Reg<ipa_itctl::IpaItctlSpec>;
#[doc = "Inter-timer control register"]
pub mod ipa_itctl;
