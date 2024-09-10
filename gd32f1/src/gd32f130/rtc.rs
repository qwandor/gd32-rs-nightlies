#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    time: Time,
    date: Date,
    ctl: Ctl,
    stat: Stat,
    psc: Psc,
    _reserved5: [u8; 0x08],
    alrm0td: Alrm0td,
    _reserved6: [u8; 0x04],
    wpk: Wpk,
    ss: Ss,
    shiftctl: Shiftctl,
    tts: Tts,
    dts: Dts,
    ssts: Ssts,
    hrfc: Hrfc,
    tamp: Tamp,
    alrm0ss: Alrm0ss,
    _reserved15: [u8; 0x08],
    bkp0: Bkp0,
    bkp1: Bkp1,
    bkp2: Bkp2,
    bkp3: Bkp3,
    bkp4: Bkp4,
}
impl RegisterBlock {
    #[doc = "0x00 - Time of day register"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x04 - date register"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x08 - control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x10 - Time prescaler register"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x1c - Alarm 0 Time and date register"]
    #[inline(always)]
    pub const fn alrm0td(&self) -> &Alrm0td {
        &self.alrm0td
    }
    #[doc = "0x24 - Write protection key register"]
    #[inline(always)]
    pub const fn wpk(&self) -> &Wpk {
        &self.wpk
    }
    #[doc = "0x28 - sub second register"]
    #[inline(always)]
    pub const fn ss(&self) -> &Ss {
        &self.ss
    }
    #[doc = "0x2c - Shift function control register"]
    #[inline(always)]
    pub const fn shiftctl(&self) -> &Shiftctl {
        &self.shiftctl
    }
    #[doc = "0x30 - Time of time stamp register"]
    #[inline(always)]
    pub const fn tts(&self) -> &Tts {
        &self.tts
    }
    #[doc = "0x34 - Date of time stamp register"]
    #[inline(always)]
    pub const fn dts(&self) -> &Dts {
        &self.dts
    }
    #[doc = "0x38 - Sub second of time stamp register"]
    #[inline(always)]
    pub const fn ssts(&self) -> &Ssts {
        &self.ssts
    }
    #[doc = "0x3c - High resolution frequency compensation register"]
    #[inline(always)]
    pub const fn hrfc(&self) -> &Hrfc {
        &self.hrfc
    }
    #[doc = "0x40 - Tamper register"]
    #[inline(always)]
    pub const fn tamp(&self) -> &Tamp {
        &self.tamp
    }
    #[doc = "0x44 - Alarm 0 sub second register"]
    #[inline(always)]
    pub const fn alrm0ss(&self) -> &Alrm0ss {
        &self.alrm0ss
    }
    #[doc = "0x50 - backup register"]
    #[inline(always)]
    pub const fn bkp0(&self) -> &Bkp0 {
        &self.bkp0
    }
    #[doc = "0x54 - backup register"]
    #[inline(always)]
    pub const fn bkp1(&self) -> &Bkp1 {
        &self.bkp1
    }
    #[doc = "0x58 - backup register"]
    #[inline(always)]
    pub const fn bkp2(&self) -> &Bkp2 {
        &self.bkp2
    }
    #[doc = "0x5c - backup register"]
    #[inline(always)]
    pub const fn bkp3(&self) -> &Bkp3 {
        &self.bkp3
    }
    #[doc = "0x60 - backup register"]
    #[inline(always)]
    pub const fn bkp4(&self) -> &Bkp4 {
        &self.bkp4
    }
}
#[doc = "TIME (rw) register accessor: Time of day register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "Time of day register"]
pub mod time;
#[doc = "DATE (rw) register accessor: date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`]
module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "date register"]
pub mod date;
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "control register"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "PSC (rw) register accessor: Time prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "Time prescaler register"]
pub mod psc;
#[doc = "ALRM0TD (rw) register accessor: Alarm 0 Time and date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm0td::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm0td::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrm0td`]
module"]
#[doc(alias = "ALRM0TD")]
pub type Alrm0td = crate::Reg<alrm0td::Alrm0tdSpec>;
#[doc = "Alarm 0 Time and date register"]
pub mod alrm0td;
#[doc = "WPK (w) register accessor: Write protection key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpk`]
module"]
#[doc(alias = "WPK")]
pub type Wpk = crate::Reg<wpk::WpkSpec>;
#[doc = "Write protection key register"]
pub mod wpk;
#[doc = "SS (r) register accessor: sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss`]
module"]
#[doc(alias = "SS")]
pub type Ss = crate::Reg<ss::SsSpec>;
#[doc = "sub second register"]
pub mod ss;
#[doc = "SHIFTCTL (w) register accessor: Shift function control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftctl`]
module"]
#[doc(alias = "SHIFTCTL")]
pub type Shiftctl = crate::Reg<shiftctl::ShiftctlSpec>;
#[doc = "Shift function control register"]
pub mod shiftctl;
#[doc = "TTS (r) register accessor: Time of time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tts`]
module"]
#[doc(alias = "TTS")]
pub type Tts = crate::Reg<tts::TtsSpec>;
#[doc = "Time of time stamp register"]
pub mod tts;
#[doc = "DTS (r) register accessor: Date of time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dts`]
module"]
#[doc(alias = "DTS")]
pub type Dts = crate::Reg<dts::DtsSpec>;
#[doc = "Date of time stamp register"]
pub mod dts;
#[doc = "SSTS (r) register accessor: Sub second of time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssts`]
module"]
#[doc(alias = "SSTS")]
pub type Ssts = crate::Reg<ssts::SstsSpec>;
#[doc = "Sub second of time stamp register"]
pub mod ssts;
#[doc = "HRFC (rw) register accessor: High resolution frequency compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrfc`]
module"]
#[doc(alias = "HRFC")]
pub type Hrfc = crate::Reg<hrfc::HrfcSpec>;
#[doc = "High resolution frequency compensation register"]
pub mod hrfc;
#[doc = "TAMP (rw) register accessor: Tamper register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp`]
module"]
#[doc(alias = "TAMP")]
pub type Tamp = crate::Reg<tamp::TampSpec>;
#[doc = "Tamper register"]
pub mod tamp;
#[doc = "ALRM0SS (rw) register accessor: Alarm 0 sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm0ss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm0ss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrm0ss`]
module"]
#[doc(alias = "ALRM0SS")]
pub type Alrm0ss = crate::Reg<alrm0ss::Alrm0ssSpec>;
#[doc = "Alarm 0 sub second register"]
pub mod alrm0ss;
#[doc = "BKP0 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp0`]
module"]
#[doc(alias = "BKP0")]
pub type Bkp0 = crate::Reg<bkp0::Bkp0Spec>;
#[doc = "backup register"]
pub mod bkp0;
#[doc = "BKP1 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp1`]
module"]
#[doc(alias = "BKP1")]
pub type Bkp1 = crate::Reg<bkp1::Bkp1Spec>;
#[doc = "backup register"]
pub mod bkp1;
#[doc = "BKP2 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp2`]
module"]
#[doc(alias = "BKP2")]
pub type Bkp2 = crate::Reg<bkp2::Bkp2Spec>;
#[doc = "backup register"]
pub mod bkp2;
#[doc = "BKP3 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp3`]
module"]
#[doc(alias = "BKP3")]
pub type Bkp3 = crate::Reg<bkp3::Bkp3Spec>;
#[doc = "backup register"]
pub mod bkp3;
#[doc = "BKP4 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp4`]
module"]
#[doc(alias = "BKP4")]
pub type Bkp4 = crate::Reg<bkp4::Bkp4Spec>;
#[doc = "backup register"]
pub mod bkp4;
