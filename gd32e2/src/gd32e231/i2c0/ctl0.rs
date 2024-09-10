#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Peripheral enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cen {
    #[doc = "0: Peripheral disabled"]
    Disabled = 0,
    #[doc = "1: Peripheral enabled"]
    Enabled = 1,
}
impl From<I2cen> for bool {
    #[inline(always)]
    fn from(variant: I2cen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CEN` reader - Peripheral enable"]
pub type I2cenR = crate::BitReader<I2cen>;
impl I2cenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cen {
        match self.bits {
            false => I2cen::Disabled,
            true => I2cen::Enabled,
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2cen::Disabled
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2cen::Enabled
    }
}
#[doc = "Field `I2CEN` writer - Peripheral enable"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG, I2cen>;
impl<'a, REG> I2cenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2cen::Disabled)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2cen::Enabled)
    }
}
#[doc = "SMBus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smben {
    #[doc = "0: I2C Mode"]
    I2c = 0,
    #[doc = "1: SMBus"]
    Smbus = 1,
}
impl From<Smben> for bool {
    #[inline(always)]
    fn from(variant: Smben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBEN` reader - SMBus mode"]
pub type SmbenR = crate::BitReader<Smben>;
impl SmbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smben {
        match self.bits {
            false => Smben::I2c,
            true => Smben::Smbus,
        }
    }
    #[doc = "I2C Mode"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == Smben::I2c
    }
    #[doc = "SMBus"]
    #[inline(always)]
    pub fn is_smbus(&self) -> bool {
        *self == Smben::Smbus
    }
}
#[doc = "Field `SMBEN` writer - SMBus mode"]
pub type SmbenW<'a, REG> = crate::BitWriter<'a, REG, Smben>;
impl<'a, REG> SmbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C Mode"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(Smben::I2c)
    }
    #[doc = "SMBus"]
    #[inline(always)]
    pub fn smbus(self) -> &'a mut crate::W<REG> {
        self.variant(Smben::Smbus)
    }
}
#[doc = "SMBus type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smbsel {
    #[doc = "0: SMBus Device"]
    Device = 0,
    #[doc = "1: SMBus Host"]
    Host = 1,
}
impl From<Smbsel> for bool {
    #[inline(always)]
    fn from(variant: Smbsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBSEL` reader - SMBus type"]
pub type SmbselR = crate::BitReader<Smbsel>;
impl SmbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smbsel {
        match self.bits {
            false => Smbsel::Device,
            true => Smbsel::Host,
        }
    }
    #[doc = "SMBus Device"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == Smbsel::Device
    }
    #[doc = "SMBus Host"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == Smbsel::Host
    }
}
#[doc = "Field `SMBSEL` writer - SMBus type"]
pub type SmbselW<'a, REG> = crate::BitWriter<'a, REG, Smbsel>;
impl<'a, REG> SmbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBus Device"]
    #[inline(always)]
    pub fn device(self) -> &'a mut crate::W<REG> {
        self.variant(Smbsel::Device)
    }
    #[doc = "SMBus Host"]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(Smbsel::Host)
    }
}
#[doc = "ARP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arpen {
    #[doc = "0: ARP disabled"]
    Disabled = 0,
    #[doc = "1: ARP enabled"]
    Enabled = 1,
}
impl From<Arpen> for bool {
    #[inline(always)]
    fn from(variant: Arpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARPEN` reader - ARP enable"]
pub type ArpenR = crate::BitReader<Arpen>;
impl ArpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arpen {
        match self.bits {
            false => Arpen::Disabled,
            true => Arpen::Enabled,
        }
    }
    #[doc = "ARP disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Arpen::Disabled
    }
    #[doc = "ARP enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Arpen::Enabled
    }
}
#[doc = "Field `ARPEN` writer - ARP enable"]
pub type ArpenW<'a, REG> = crate::BitWriter<'a, REG, Arpen>;
impl<'a, REG> ArpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARP disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Arpen::Disabled)
    }
    #[doc = "ARP enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Arpen::Enabled)
    }
}
#[doc = "PEC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pecen {
    #[doc = "0: PEC calculation disabled"]
    Disabled = 0,
    #[doc = "1: PEC calculation enabled"]
    Enabled = 1,
}
impl From<Pecen> for bool {
    #[inline(always)]
    fn from(variant: Pecen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - PEC enable"]
pub type PecenR = crate::BitReader<Pecen>;
impl PecenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pecen {
        match self.bits {
            false => Pecen::Disabled,
            true => Pecen::Enabled,
        }
    }
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pecen::Disabled
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pecen::Enabled
    }
}
#[doc = "Field `PECEN` writer - PEC enable"]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG, Pecen>;
impl<'a, REG> PecenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pecen::Disabled)
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pecen::Enabled)
    }
}
#[doc = "General call enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcen {
    #[doc = "0: Slave won't respond to General Call"]
    NotRespond = 0,
    #[doc = "1: Slave will respond to General Call"]
    Respond = 1,
}
impl From<Gcen> for bool {
    #[inline(always)]
    fn from(variant: Gcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCEN` reader - General call enable"]
pub type GcenR = crate::BitReader<Gcen>;
impl GcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcen {
        match self.bits {
            false => Gcen::NotRespond,
            true => Gcen::Respond,
        }
    }
    #[doc = "Slave won't respond to General Call"]
    #[inline(always)]
    pub fn is_not_respond(&self) -> bool {
        *self == Gcen::NotRespond
    }
    #[doc = "Slave will respond to General Call"]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        *self == Gcen::Respond
    }
}
#[doc = "Field `GCEN` writer - General call enable"]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG, Gcen>;
impl<'a, REG> GcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave won't respond to General Call"]
    #[inline(always)]
    pub fn not_respond(self) -> &'a mut crate::W<REG> {
        self.variant(Gcen::NotRespond)
    }
    #[doc = "Slave will respond to General Call"]
    #[inline(always)]
    pub fn respond(self) -> &'a mut crate::W<REG> {
        self.variant(Gcen::Respond)
    }
}
#[doc = "SCL Stretching(Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ss {
    #[doc = "0: Clock stretching enabled"]
    Enabled = 0,
    #[doc = "1: Clock stretching disabled"]
    Disabled = 1,
}
impl From<Ss> for bool {
    #[inline(always)]
    fn from(variant: Ss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS` reader - SCL Stretching(Slave mode)"]
pub type SsR = crate::BitReader<Ss>;
impl SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ss {
        match self.bits {
            false => Ss::Enabled,
            true => Ss::Disabled,
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ss::Enabled
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ss::Disabled
    }
}
#[doc = "Field `SS` writer - SCL Stretching(Slave mode)"]
pub type SsW<'a, REG> = crate::BitWriter<'a, REG, Ss>;
impl<'a, REG> SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ss::Enabled)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ss::Disabled)
    }
}
#[doc = "Start generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: START will not be sent"]
    NoStart = 0,
    #[doc = "1: START will be sent"]
    Start = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start generation"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::NoStart,
            true => Start::Start,
        }
    }
    #[doc = "START will not be sent"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == Start::NoStart
    }
    #[doc = "START will be sent"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Start::Start
    }
}
#[doc = "Field `START` writer - Start generation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "START will not be sent"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::NoStart)
    }
    #[doc = "START will be sent"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Start)
    }
}
#[doc = "Stop condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: STOP will not be sent"]
    NoStop = 0,
    #[doc = "1: STOP will be sent"]
    Stop = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop condition"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::NoStop,
            true => Stop::Stop,
        }
    }
    #[doc = "STOP will not be sent"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == Stop::NoStop
    }
    #[doc = "STOP will be sent"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Stop::Stop
    }
}
#[doc = "Field `STOP` writer - Stop condition"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOP will not be sent"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::NoStop)
    }
    #[doc = "STOP will be sent"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Stop)
    }
}
#[doc = "Acknowledge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acken {
    #[doc = "0: No acknowledge returned"]
    Nak = 0,
    #[doc = "1: Acknowledge returned after a byte is received"]
    Ack = 1,
}
impl From<Acken> for bool {
    #[inline(always)]
    fn from(variant: Acken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKEN` reader - Acknowledge enable"]
pub type AckenR = crate::BitReader<Acken>;
impl AckenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acken {
        match self.bits {
            false => Acken::Nak,
            true => Acken::Ack,
        }
    }
    #[doc = "No acknowledge returned"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == Acken::Nak
    }
    #[doc = "Acknowledge returned after a byte is received"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Acken::Ack
    }
}
#[doc = "Field `ACKEN` writer - Acknowledge enable"]
pub type AckenW<'a, REG> = crate::BitWriter<'a, REG, Acken>;
impl<'a, REG> AckenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledge returned"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(Acken::Nak)
    }
    #[doc = "Acknowledge returned after a byte is received"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Acken::Ack)
    }
}
#[doc = "Acknowledge/PEC Position (for data reception)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Poap {
    #[doc = "0: ACK bit controls the (N)ACK of the current byte being received"]
    Current = 0,
    #[doc = "1: ACK bit controls the (N)ACK of the next byte to be received"]
    Next = 1,
}
impl From<Poap> for bool {
    #[inline(always)]
    fn from(variant: Poap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POAP` reader - Acknowledge/PEC Position (for data reception)"]
pub type PoapR = crate::BitReader<Poap>;
impl PoapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Poap {
        match self.bits {
            false => Poap::Current,
            true => Poap::Next,
        }
    }
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        *self == Poap::Current
    }
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    #[inline(always)]
    pub fn is_next(&self) -> bool {
        *self == Poap::Next
    }
}
#[doc = "Field `POAP` writer - Acknowledge/PEC Position (for data reception)"]
pub type PoapW<'a, REG> = crate::BitWriter<'a, REG, Poap>;
impl<'a, REG> PoapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    #[inline(always)]
    pub fn current(self) -> &'a mut crate::W<REG> {
        self.variant(Poap::Current)
    }
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    #[inline(always)]
    pub fn next(self) -> &'a mut crate::W<REG> {
        self.variant(Poap::Next)
    }
}
#[doc = "Packet error checking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pectrans {
    #[doc = "0: No PEC transfer"]
    Disabled = 0,
    #[doc = "1: PEC transfer"]
    Enabled = 1,
}
impl From<Pectrans> for bool {
    #[inline(always)]
    fn from(variant: Pectrans) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECTRANS` reader - Packet error checking"]
pub type PectransR = crate::BitReader<Pectrans>;
impl PectransR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pectrans {
        match self.bits {
            false => Pectrans::Disabled,
            true => Pectrans::Enabled,
        }
    }
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pectrans::Disabled
    }
    #[doc = "PEC transfer"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pectrans::Enabled
    }
}
#[doc = "Field `PECTRANS` writer - Packet error checking"]
pub type PectransW<'a, REG> = crate::BitWriter<'a, REG, Pectrans>;
impl<'a, REG> PectransW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pectrans::Disabled)
    }
    #[doc = "PEC transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pectrans::Enabled)
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Salt {
    #[doc = "0: SMBA pin released high"]
    Release = 0,
    #[doc = "1: SMBA pin driven low"]
    Drive = 1,
}
impl From<Salt> for bool {
    #[inline(always)]
    fn from(variant: Salt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SALT` reader - SMBus alert"]
pub type SaltR = crate::BitReader<Salt>;
impl SaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Salt {
        match self.bits {
            false => Salt::Release,
            true => Salt::Drive,
        }
    }
    #[doc = "SMBA pin released high"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == Salt::Release
    }
    #[doc = "SMBA pin driven low"]
    #[inline(always)]
    pub fn is_drive(&self) -> bool {
        *self == Salt::Drive
    }
}
#[doc = "Field `SALT` writer - SMBus alert"]
pub type SaltW<'a, REG> = crate::BitWriter<'a, REG, Salt>;
impl<'a, REG> SaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBA pin released high"]
    #[inline(always)]
    pub fn release(self) -> &'a mut crate::W<REG> {
        self.variant(Salt::Release)
    }
    #[doc = "SMBA pin driven low"]
    #[inline(always)]
    pub fn drive(self) -> &'a mut crate::W<REG> {
        self.variant(Salt::Drive)
    }
}
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sreset {
    #[doc = "0: I2C peripheral not under reset"]
    NotReset = 0,
    #[doc = "1: I2C peripheral under reset"]
    Reset = 1,
}
impl From<Sreset> for bool {
    #[inline(always)]
    fn from(variant: Sreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRESET` reader - Software reset"]
pub type SresetR = crate::BitReader<Sreset>;
impl SresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sreset {
        match self.bits {
            false => Sreset::NotReset,
            true => Sreset::Reset,
        }
    }
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == Sreset::NotReset
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Sreset::Reset
    }
}
#[doc = "Field `SRESET` writer - Software reset"]
pub type SresetW<'a, REG> = crate::BitWriter<'a, REG, Sreset>;
impl<'a, REG> SresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sreset::NotReset)
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Sreset::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smben(&self) -> SmbenR {
        SmbenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbsel(&self) -> SmbselR {
        SmbselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ArpenR {
        ArpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PecenR {
        PecenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GcenR {
        GcenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCL Stretching(Slave mode)"]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop condition"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn acken(&self) -> AckenR {
        AckenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn poap(&self) -> PoapR {
        PoapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pectrans(&self) -> PectransR {
        PectransR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn salt(&self) -> SaltR {
        SaltR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn sreset(&self) -> SresetR {
        SresetR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2cenW<Ctl0Spec> {
        I2cenW::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    #[must_use]
    pub fn smben(&mut self) -> SmbenW<Ctl0Spec> {
        SmbenW::new(self, 1)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    #[must_use]
    pub fn smbsel(&mut self) -> SmbselW<Ctl0Spec> {
        SmbselW::new(self, 3)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ArpenW<Ctl0Spec> {
        ArpenW::new(self, 4)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PecenW<Ctl0Spec> {
        PecenW::new(self, 5)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GcenW<Ctl0Spec> {
        GcenW::new(self, 6)
    }
    #[doc = "Bit 7 - SCL Stretching(Slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SsW<Ctl0Spec> {
        SsW::new(self, 7)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<Ctl0Spec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - Stop condition"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<Ctl0Spec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn acken(&mut self) -> AckenW<Ctl0Spec> {
        AckenW::new(self, 10)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    #[must_use]
    pub fn poap(&mut self) -> PoapW<Ctl0Spec> {
        PoapW::new(self, 11)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    #[must_use]
    pub fn pectrans(&mut self) -> PectransW<Ctl0Spec> {
        PectransW::new(self, 12)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn salt(&mut self) -> SaltW<Ctl0Spec> {
        SaltW::new(self, 13)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sreset(&mut self) -> SresetW<Ctl0Spec> {
        SresetW::new(self, 15)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
