#[doc = "Register `STAT0` reader"]
pub type R = crate::R<Stat0Spec>;
#[doc = "Register `STAT0` writer"]
pub type W = crate::W<Stat0Spec>;
#[doc = "Start bit (Master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbsend {
    #[doc = "0: No Start condition"]
    NoStart = 0,
    #[doc = "1: Start condition generated"]
    Start = 1,
}
impl From<Sbsend> for bool {
    #[inline(always)]
    fn from(variant: Sbsend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSEND` reader - Start bit (Master mode)"]
pub type SbsendR = crate::BitReader<Sbsend>;
impl SbsendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbsend {
        match self.bits {
            false => Sbsend::NoStart,
            true => Sbsend::Start,
        }
    }
    #[doc = "No Start condition"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == Sbsend::NoStart
    }
    #[doc = "Start condition generated"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Sbsend::Start
    }
}
#[doc = "Address sent (master mode)/matched (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addsend {
    #[doc = "0: Adress mismatched or not received"]
    NotMatch = 0,
    #[doc = "1: Received slave address matched with one of the enabled slave addresses"]
    Match = 1,
}
impl From<Addsend> for bool {
    #[inline(always)]
    fn from(variant: Addsend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDSEND` reader - Address sent (master mode)/matched (slave mode)"]
pub type AddsendR = crate::BitReader<Addsend>;
impl AddsendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addsend {
        match self.bits {
            false => Addsend::NotMatch,
            true => Addsend::Match,
        }
    }
    #[doc = "Adress mismatched or not received"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == Addsend::NotMatch
    }
    #[doc = "Received slave address matched with one of the enabled slave addresses"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == Addsend::Match
    }
}
#[doc = "Byte transmission completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Btc {
    #[doc = "0: Data byte transfer not done"]
    NotFinished = 0,
    #[doc = "1: Data byte transfer successful"]
    Finished = 1,
}
impl From<Btc> for bool {
    #[inline(always)]
    fn from(variant: Btc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTC` reader - Byte transmission completed"]
pub type BtcR = crate::BitReader<Btc>;
impl BtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Btc {
        match self.bits {
            false => Btc::NotFinished,
            true => Btc::Finished,
        }
    }
    #[doc = "Data byte transfer not done"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == Btc::NotFinished
    }
    #[doc = "Data byte transfer successful"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == Btc::Finished
    }
}
#[doc = "Header of 10-bit address is sent in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Add10send {
    #[doc = "0: No header of 10-bit address is sent"]
    NoHeader = 0,
    #[doc = "1: Header of 10-bit address is sent"]
    Header = 1,
}
impl From<Add10send> for bool {
    #[inline(always)]
    fn from(variant: Add10send) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD10SEND` reader - Header of 10-bit address is sent in master mode"]
pub type Add10sendR = crate::BitReader<Add10send>;
impl Add10sendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Add10send {
        match self.bits {
            false => Add10send::NoHeader,
            true => Add10send::Header,
        }
    }
    #[doc = "No header of 10-bit address is sent"]
    #[inline(always)]
    pub fn is_no_header(&self) -> bool {
        *self == Add10send::NoHeader
    }
    #[doc = "Header of 10-bit address is sent"]
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == Add10send::Header
    }
}
#[doc = "Stop detection (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpdet {
    #[doc = "0: No Stop condition detected"]
    NoStop = 0,
    #[doc = "1: Stop condition detected"]
    Stop = 1,
}
impl From<Stpdet> for bool {
    #[inline(always)]
    fn from(variant: Stpdet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPDET` reader - Stop detection (slave mode)"]
pub type StpdetR = crate::BitReader<Stpdet>;
impl StpdetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpdet {
        match self.bits {
            false => Stpdet::NoStop,
            true => Stpdet::Stop,
        }
    }
    #[doc = "No Stop condition detected"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == Stpdet::NoStop
    }
    #[doc = "Stop condition detected"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Stpdet::Stop
    }
}
#[doc = "I2C_DATA is not Empty during receiving\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbne {
    #[doc = "0: Data register empty"]
    Empty = 0,
    #[doc = "1: Data register not empty, software can read"]
    NotEmpty = 1,
}
impl From<Rbne> for bool {
    #[inline(always)]
    fn from(variant: Rbne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBNE` reader - I2C_DATA is not Empty during receiving"]
pub type RbneR = crate::BitReader<Rbne>;
impl RbneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbne {
        match self.bits {
            false => Rbne::Empty,
            true => Rbne::NotEmpty,
        }
    }
    #[doc = "Data register empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rbne::Empty
    }
    #[doc = "Data register not empty, software can read"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Rbne::NotEmpty
    }
}
#[doc = "I2C_DATA is Empty during transmitting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbe {
    #[doc = "0: Data register not empty"]
    NotEmpty = 0,
    #[doc = "1: Data register empty, software can write"]
    Empty = 1,
}
impl From<Tbe> for bool {
    #[inline(always)]
    fn from(variant: Tbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBE` reader - I2C_DATA is Empty during transmitting"]
pub type TbeR = crate::BitReader<Tbe>;
impl TbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbe {
        match self.bits {
            false => Tbe::NotEmpty,
            true => Tbe::Empty,
        }
    }
    #[doc = "Data register not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Tbe::NotEmpty
    }
    #[doc = "Data register empty, software can write"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Tbe::Empty
    }
}
#[doc = "Bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Berr {
    #[doc = "0: No misplaced Start or Stop condition"]
    NoError = 0,
    #[doc = "1: Misplaced Start or Stop condition"]
    Error = 1,
}
impl From<Berr> for bool {
    #[inline(always)]
    fn from(variant: Berr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERR` reader - Bus error"]
pub type BerrR = crate::BitReader<Berr>;
impl BerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Berr {
        match self.bits {
            false => Berr::NoError,
            true => Berr::Error,
        }
    }
    #[doc = "No misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Berr::NoError
    }
    #[doc = "Misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Berr::Error
    }
}
#[doc = "Field `BERR` writer - Bus error"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG, Berr>;
impl<'a, REG> BerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Berr::NoError)
    }
    #[doc = "Misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Berr::Error)
    }
}
#[doc = "Arbitration lost (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lostarb {
    #[doc = "0: No Arbitration Lost detected"]
    NoLost = 0,
    #[doc = "1: Arbitration Lost detected"]
    Lost = 1,
}
impl From<Lostarb> for bool {
    #[inline(always)]
    fn from(variant: Lostarb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOSTARB` reader - Arbitration lost (master mode)"]
pub type LostarbR = crate::BitReader<Lostarb>;
impl LostarbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lostarb {
        match self.bits {
            false => Lostarb::NoLost,
            true => Lostarb::Lost,
        }
    }
    #[doc = "No Arbitration Lost detected"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == Lostarb::NoLost
    }
    #[doc = "Arbitration Lost detected"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == Lostarb::Lost
    }
}
#[doc = "Field `LOSTARB` writer - Arbitration lost (master mode)"]
pub type LostarbW<'a, REG> = crate::BitWriter<'a, REG, Lostarb>;
impl<'a, REG> LostarbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Arbitration Lost detected"]
    #[inline(always)]
    pub fn no_lost(self) -> &'a mut crate::W<REG> {
        self.variant(Lostarb::NoLost)
    }
    #[doc = "Arbitration Lost detected"]
    #[inline(always)]
    pub fn lost(self) -> &'a mut crate::W<REG> {
        self.variant(Lostarb::Lost)
    }
}
#[doc = "Acknowledge error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aerr {
    #[doc = "0: No acknowledge error"]
    NoError = 0,
    #[doc = "1: Acknowledge error"]
    Error = 1,
}
impl From<Aerr> for bool {
    #[inline(always)]
    fn from(variant: Aerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AERR` reader - Acknowledge error"]
pub type AerrR = crate::BitReader<Aerr>;
impl AerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aerr {
        match self.bits {
            false => Aerr::NoError,
            true => Aerr::Error,
        }
    }
    #[doc = "No acknowledge error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Aerr::NoError
    }
    #[doc = "Acknowledge error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Aerr::Error
    }
}
#[doc = "Field `AERR` writer - Acknowledge error"]
pub type AerrW<'a, REG> = crate::BitWriter<'a, REG, Aerr>;
impl<'a, REG> AerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledge error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Aerr::NoError)
    }
    #[doc = "Acknowledge error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Aerr::Error)
    }
}
#[doc = "Overrun/Underrun occurs in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ouerr {
    #[doc = "0: No overrun/underrun occured"]
    NoOverrun = 0,
    #[doc = "1: Overrun/underrun occured"]
    Overrun = 1,
}
impl From<Ouerr> for bool {
    #[inline(always)]
    fn from(variant: Ouerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUERR` reader - Overrun/Underrun occurs in slave mode"]
pub type OuerrR = crate::BitReader<Ouerr>;
impl OuerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ouerr {
        match self.bits {
            false => Ouerr::NoOverrun,
            true => Ouerr::Overrun,
        }
    }
    #[doc = "No overrun/underrun occured"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == Ouerr::NoOverrun
    }
    #[doc = "Overrun/underrun occured"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == Ouerr::Overrun
    }
}
#[doc = "Field `OUERR` writer - Overrun/Underrun occurs in slave mode"]
pub type OuerrW<'a, REG> = crate::BitWriter<'a, REG, Ouerr>;
impl<'a, REG> OuerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun/underrun occured"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Ouerr::NoOverrun)
    }
    #[doc = "Overrun/underrun occured"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Ouerr::Overrun)
    }
}
#[doc = "PEC error when receiving data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pecerr {
    #[doc = "0: No PEC error"]
    NoError = 0,
    #[doc = "1: PEC error"]
    Error = 1,
}
impl From<Pecerr> for bool {
    #[inline(always)]
    fn from(variant: Pecerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` reader - PEC error when receiving data"]
pub type PecerrR = crate::BitReader<Pecerr>;
impl PecerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pecerr {
        match self.bits {
            false => Pecerr::NoError,
            true => Pecerr::Error,
        }
    }
    #[doc = "No PEC error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Pecerr::NoError
    }
    #[doc = "PEC error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Pecerr::Error
    }
}
#[doc = "Field `PECERR` writer - PEC error when receiving data"]
pub type PecerrW<'a, REG> = crate::BitWriter<'a, REG, Pecerr>;
impl<'a, REG> PecerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No PEC error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Pecerr::NoError)
    }
    #[doc = "PEC error"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Pecerr::Error)
    }
}
#[doc = "Timeout signal in SMBus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smbto {
    #[doc = "0: No Timeout error"]
    NoTimeout = 0,
    #[doc = "1: SCL remained low for 25 ms"]
    Timeout = 1,
}
impl From<Smbto> for bool {
    #[inline(always)]
    fn from(variant: Smbto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBTO` reader - Timeout signal in SMBus mode"]
pub type SmbtoR = crate::BitReader<Smbto>;
impl SmbtoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smbto {
        match self.bits {
            false => Smbto::NoTimeout,
            true => Smbto::Timeout,
        }
    }
    #[doc = "No Timeout error"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == Smbto::NoTimeout
    }
    #[doc = "SCL remained low for 25 ms"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == Smbto::Timeout
    }
}
#[doc = "Field `SMBTO` writer - Timeout signal in SMBus mode"]
pub type SmbtoW<'a, REG> = crate::BitWriter<'a, REG, Smbto>;
impl<'a, REG> SmbtoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Timeout error"]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Smbto::NoTimeout)
    }
    #[doc = "SCL remained low for 25 ms"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut crate::W<REG> {
        self.variant(Smbto::Timeout)
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smbalt {
    #[doc = "0: SMBA not pulled down or no alert occured"]
    NoAlert = 0,
    #[doc = "1: SMBA pulled down or alert occurred"]
    Alert = 1,
}
impl From<Smbalt> for bool {
    #[inline(always)]
    fn from(variant: Smbalt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBALT` reader - SMBus alert"]
pub type SmbaltR = crate::BitReader<Smbalt>;
impl SmbaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smbalt {
        match self.bits {
            false => Smbalt::NoAlert,
            true => Smbalt::Alert,
        }
    }
    #[doc = "SMBA not pulled down or no alert occured"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == Smbalt::NoAlert
    }
    #[doc = "SMBA pulled down or alert occurred"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == Smbalt::Alert
    }
}
#[doc = "Field `SMBALT` writer - SMBus alert"]
pub type SmbaltW<'a, REG> = crate::BitWriter<'a, REG, Smbalt>;
impl<'a, REG> SmbaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBA not pulled down or no alert occured"]
    #[inline(always)]
    pub fn no_alert(self) -> &'a mut crate::W<REG> {
        self.variant(Smbalt::NoAlert)
    }
    #[doc = "SMBA pulled down or alert occurred"]
    #[inline(always)]
    pub fn alert(self) -> &'a mut crate::W<REG> {
        self.variant(Smbalt::Alert)
    }
}
impl R {
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn sbsend(&self) -> SbsendR {
        SbsendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addsend(&self) -> AddsendR {
        AddsendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transmission completed"]
    #[inline(always)]
    pub fn btc(&self) -> BtcR {
        BtcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Header of 10-bit address is sent in master mode"]
    #[inline(always)]
    pub fn add10send(&self) -> Add10sendR {
        Add10sendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stpdet(&self) -> StpdetR {
        StpdetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C_DATA is not Empty during receiving"]
    #[inline(always)]
    pub fn rbne(&self) -> RbneR {
        RbneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C_DATA is Empty during transmitting"]
    #[inline(always)]
    pub fn tbe(&self) -> TbeR {
        TbeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn lostarb(&self) -> LostarbR {
        LostarbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    pub fn aerr(&self) -> AerrR {
        AerrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun/Underrun occurs in slave mode"]
    #[inline(always)]
    pub fn ouerr(&self) -> OuerrR {
        OuerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    pub fn pecerr(&self) -> PecerrR {
        PecerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    pub fn smbto(&self) -> SmbtoR {
        SmbtoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalt(&self) -> SmbaltR {
        SmbaltR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BerrW<Stat0Spec> {
        BerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn lostarb(&mut self) -> LostarbW<Stat0Spec> {
        LostarbW::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AerrW<Stat0Spec> {
        AerrW::new(self, 10)
    }
    #[doc = "Bit 11 - Overrun/Underrun occurs in slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn ouerr(&mut self) -> OuerrW<Stat0Spec> {
        OuerrW::new(self, 11)
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    #[must_use]
    pub fn pecerr(&mut self) -> PecerrW<Stat0Spec> {
        PecerrW::new(self, 12)
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    #[must_use]
    pub fn smbto(&mut self) -> SmbtoW<Stat0Spec> {
        SmbtoW::new(self, 14)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn smbalt(&mut self) -> SmbaltW<Stat0Spec> {
        SmbaltW::new(self, 15)
    }
}
#[doc = "Transfer status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat0Spec;
impl crate::RegisterSpec for Stat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for Stat0Spec {}
#[doc = "`write(|w| ..)` method takes [`stat0::W`](W) writer structure"]
impl crate::Writable for Stat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for Stat0Spec {
    const RESET_VALUE: u32 = 0;
}
