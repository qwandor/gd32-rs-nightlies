#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    time: Time,
    date: Date,
    ctl: Ctl,
    stat: Stat,
    psc: Psc,
    wut: Wut,
    cosc: Cosc,
    alrm0td: Alrm0td,
    alrm1td: Alrm1td,
    wpk: Wpk,
    ss: Ss,
    shiftctl: Shiftctl,
    tts: Tts,
    dts: Dts,
    ssts: Ssts,
    hrfc: Hrfc,
    tamp: Tamp,
    alrm0ss: Alrm0ss,
    alrm1ss: Alrm1ss,
    _reserved19: [u8; 0x04],
    bkp0: Bkp0,
    bkp1: Bkp1,
    bkp2: Bkp2,
    bkp3: Bkp3,
    bkp4: Bkp4,
    bkp5: Bkp5,
    bkp6: Bkp6,
    bkp7: Bkp7,
    bkp8: Bkp8,
    bkp9: Bkp9,
    bkp10: Bkp10,
    bkp11: Bkp11,
    bkp12: Bkp12,
    bkp13: Bkp13,
    bkp14: Bkp14,
    bkp15: Bkp15,
    bkp16: Bkp16,
    bkp17: Bkp17,
    bkp18: Bkp18,
    bkp19: Bkp19,
}
impl RegisterBlock {
    #[doc = "0x00 - time register"]
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
    #[doc = "0x0c - status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x10 - prescaler register"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x14 - Wakeup timer register"]
    #[inline(always)]
    pub const fn wut(&self) -> &Wut {
        &self.wut
    }
    #[doc = "0x18 - Coarse calibration register"]
    #[inline(always)]
    pub const fn cosc(&self) -> &Cosc {
        &self.cosc
    }
    #[doc = "0x1c - Alarm 0 time and date register"]
    #[inline(always)]
    pub const fn alrm0td(&self) -> &Alrm0td {
        &self.alrm0td
    }
    #[doc = "0x20 - Alarm 1 time and date register"]
    #[inline(always)]
    pub const fn alrm1td(&self) -> &Alrm1td {
        &self.alrm1td
    }
    #[doc = "0x24 - write protection register"]
    #[inline(always)]
    pub const fn wpk(&self) -> &Wpk {
        &self.wpk
    }
    #[doc = "0x28 - sub second register"]
    #[inline(always)]
    pub const fn ss(&self) -> &Ss {
        &self.ss
    }
    #[doc = "0x2c - shift function control register"]
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
    #[doc = "0x3c - calibration register"]
    #[inline(always)]
    pub const fn hrfc(&self) -> &Hrfc {
        &self.hrfc
    }
    #[doc = "0x40 - tamper and alternate function configuration register"]
    #[inline(always)]
    pub const fn tamp(&self) -> &Tamp {
        &self.tamp
    }
    #[doc = "0x44 - alarm A sub second register"]
    #[inline(always)]
    pub const fn alrm0ss(&self) -> &Alrm0ss {
        &self.alrm0ss
    }
    #[doc = "0x48 - Alarm 1 sub second register"]
    #[inline(always)]
    pub const fn alrm1ss(&self) -> &Alrm1ss {
        &self.alrm1ss
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
    #[doc = "0x64 - backup register"]
    #[inline(always)]
    pub const fn bkp5(&self) -> &Bkp5 {
        &self.bkp5
    }
    #[doc = "0x68 - backup register"]
    #[inline(always)]
    pub const fn bkp6(&self) -> &Bkp6 {
        &self.bkp6
    }
    #[doc = "0x6c - backup register"]
    #[inline(always)]
    pub const fn bkp7(&self) -> &Bkp7 {
        &self.bkp7
    }
    #[doc = "0x70 - backup register"]
    #[inline(always)]
    pub const fn bkp8(&self) -> &Bkp8 {
        &self.bkp8
    }
    #[doc = "0x74 - backup register"]
    #[inline(always)]
    pub const fn bkp9(&self) -> &Bkp9 {
        &self.bkp9
    }
    #[doc = "0x78 - backup register"]
    #[inline(always)]
    pub const fn bkp10(&self) -> &Bkp10 {
        &self.bkp10
    }
    #[doc = "0x7c - backup register"]
    #[inline(always)]
    pub const fn bkp11(&self) -> &Bkp11 {
        &self.bkp11
    }
    #[doc = "0x80 - backup register"]
    #[inline(always)]
    pub const fn bkp12(&self) -> &Bkp12 {
        &self.bkp12
    }
    #[doc = "0x84 - backup register"]
    #[inline(always)]
    pub const fn bkp13(&self) -> &Bkp13 {
        &self.bkp13
    }
    #[doc = "0x88 - backup register"]
    #[inline(always)]
    pub const fn bkp14(&self) -> &Bkp14 {
        &self.bkp14
    }
    #[doc = "0x8c - backup register"]
    #[inline(always)]
    pub const fn bkp15(&self) -> &Bkp15 {
        &self.bkp15
    }
    #[doc = "0x90 - backup register"]
    #[inline(always)]
    pub const fn bkp16(&self) -> &Bkp16 {
        &self.bkp16
    }
    #[doc = "0x94 - backup register"]
    #[inline(always)]
    pub const fn bkp17(&self) -> &Bkp17 {
        &self.bkp17
    }
    #[doc = "0x98 - backup register"]
    #[inline(always)]
    pub const fn bkp18(&self) -> &Bkp18 {
        &self.bkp18
    }
    #[doc = "0x9c - backup register"]
    #[inline(always)]
    pub const fn bkp19(&self) -> &Bkp19 {
        &self.bkp19
    }
}
#[doc = "TIME (rw) register accessor: time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "time register"]
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
#[doc = "STAT (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "status register"]
pub mod stat;
#[doc = "PSC (rw) register accessor: prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "prescaler register"]
pub mod psc;
#[doc = "WUT (rw) register accessor: Wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wut`]
module"]
#[doc(alias = "WUT")]
pub type Wut = crate::Reg<wut::WutSpec>;
#[doc = "Wakeup timer register"]
pub mod wut;
#[doc = "COSC (rw) register accessor: Coarse calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cosc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cosc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cosc`]
module"]
#[doc(alias = "COSC")]
pub type Cosc = crate::Reg<cosc::CoscSpec>;
#[doc = "Coarse calibration register"]
pub mod cosc;
#[doc = "ALRM0TD (rw) register accessor: Alarm 0 time and date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm0td::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm0td::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrm0td`]
module"]
#[doc(alias = "ALRM0TD")]
pub type Alrm0td = crate::Reg<alrm0td::Alrm0tdSpec>;
#[doc = "Alarm 0 time and date register"]
pub mod alrm0td;
#[doc = "ALRM1TD (rw) register accessor: Alarm 1 time and date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm1td::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm1td::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrm1td`]
module"]
#[doc(alias = "ALRM1TD")]
pub type Alrm1td = crate::Reg<alrm1td::Alrm1tdSpec>;
#[doc = "Alarm 1 time and date register"]
pub mod alrm1td;
#[doc = "WPK (w) register accessor: write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpk`]
module"]
#[doc(alias = "WPK")]
pub type Wpk = crate::Reg<wpk::WpkSpec>;
#[doc = "write protection register"]
pub mod wpk;
#[doc = "SS (r) register accessor: sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss`]
module"]
#[doc(alias = "SS")]
pub type Ss = crate::Reg<ss::SsSpec>;
#[doc = "sub second register"]
pub mod ss;
#[doc = "SHIFTCTL (w) register accessor: shift function control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftctl`]
module"]
#[doc(alias = "SHIFTCTL")]
pub type Shiftctl = crate::Reg<shiftctl::ShiftctlSpec>;
#[doc = "shift function control register"]
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
#[doc = "HRFC (rw) register accessor: calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrfc`]
module"]
#[doc(alias = "HRFC")]
pub type Hrfc = crate::Reg<hrfc::HrfcSpec>;
#[doc = "calibration register"]
pub mod hrfc;
#[doc = "TAMP (rw) register accessor: tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp`]
module"]
#[doc(alias = "TAMP")]
pub type Tamp = crate::Reg<tamp::TampSpec>;
#[doc = "tamper and alternate function configuration register"]
pub mod tamp;
#[doc = "ALRM0SS (rw) register accessor: alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm0ss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm0ss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrm0ss`]
module"]
#[doc(alias = "ALRM0SS")]
pub type Alrm0ss = crate::Reg<alrm0ss::Alrm0ssSpec>;
#[doc = "alarm A sub second register"]
pub mod alrm0ss;
#[doc = "ALRM1SS (rw) register accessor: Alarm 1 sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrm1ss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrm1ss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrm1ss`]
module"]
#[doc(alias = "ALRM1SS")]
pub type Alrm1ss = crate::Reg<alrm1ss::Alrm1ssSpec>;
#[doc = "Alarm 1 sub second register"]
pub mod alrm1ss;
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
#[doc = "BKP5 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp5`]
module"]
#[doc(alias = "BKP5")]
pub type Bkp5 = crate::Reg<bkp5::Bkp5Spec>;
#[doc = "backup register"]
pub mod bkp5;
#[doc = "BKP6 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp6`]
module"]
#[doc(alias = "BKP6")]
pub type Bkp6 = crate::Reg<bkp6::Bkp6Spec>;
#[doc = "backup register"]
pub mod bkp6;
#[doc = "BKP7 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp7`]
module"]
#[doc(alias = "BKP7")]
pub type Bkp7 = crate::Reg<bkp7::Bkp7Spec>;
#[doc = "backup register"]
pub mod bkp7;
#[doc = "BKP8 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp8`]
module"]
#[doc(alias = "BKP8")]
pub type Bkp8 = crate::Reg<bkp8::Bkp8Spec>;
#[doc = "backup register"]
pub mod bkp8;
#[doc = "BKP9 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp9`]
module"]
#[doc(alias = "BKP9")]
pub type Bkp9 = crate::Reg<bkp9::Bkp9Spec>;
#[doc = "backup register"]
pub mod bkp9;
#[doc = "BKP10 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp10`]
module"]
#[doc(alias = "BKP10")]
pub type Bkp10 = crate::Reg<bkp10::Bkp10Spec>;
#[doc = "backup register"]
pub mod bkp10;
#[doc = "BKP11 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp11`]
module"]
#[doc(alias = "BKP11")]
pub type Bkp11 = crate::Reg<bkp11::Bkp11Spec>;
#[doc = "backup register"]
pub mod bkp11;
#[doc = "BKP12 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp12`]
module"]
#[doc(alias = "BKP12")]
pub type Bkp12 = crate::Reg<bkp12::Bkp12Spec>;
#[doc = "backup register"]
pub mod bkp12;
#[doc = "BKP13 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp13`]
module"]
#[doc(alias = "BKP13")]
pub type Bkp13 = crate::Reg<bkp13::Bkp13Spec>;
#[doc = "backup register"]
pub mod bkp13;
#[doc = "BKP14 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp14`]
module"]
#[doc(alias = "BKP14")]
pub type Bkp14 = crate::Reg<bkp14::Bkp14Spec>;
#[doc = "backup register"]
pub mod bkp14;
#[doc = "BKP15 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp15`]
module"]
#[doc(alias = "BKP15")]
pub type Bkp15 = crate::Reg<bkp15::Bkp15Spec>;
#[doc = "backup register"]
pub mod bkp15;
#[doc = "BKP16 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp16`]
module"]
#[doc(alias = "BKP16")]
pub type Bkp16 = crate::Reg<bkp16::Bkp16Spec>;
#[doc = "backup register"]
pub mod bkp16;
#[doc = "BKP17 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp17`]
module"]
#[doc(alias = "BKP17")]
pub type Bkp17 = crate::Reg<bkp17::Bkp17Spec>;
#[doc = "backup register"]
pub mod bkp17;
#[doc = "BKP18 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp18`]
module"]
#[doc(alias = "BKP18")]
pub type Bkp18 = crate::Reg<bkp18::Bkp18Spec>;
#[doc = "backup register"]
pub mod bkp18;
#[doc = "BKP19 (rw) register accessor: backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp19`]
module"]
#[doc(alias = "BKP19")]
pub type Bkp19 = crate::Reg<bkp19::Bkp19Spec>;
#[doc = "backup register"]
pub mod bkp19;
