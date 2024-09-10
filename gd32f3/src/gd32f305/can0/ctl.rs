#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Initial working mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwmod {
    #[doc = "0: Disable initial working mode"]
    Disabled = 0,
    #[doc = "1: Enable initial working mode"]
    Enabled = 1,
}
impl From<Iwmod> for bool {
    #[inline(always)]
    fn from(variant: Iwmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWMOD` reader - Initial working mode"]
pub type IwmodR = crate::BitReader<Iwmod>;
impl IwmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwmod {
        match self.bits {
            false => Iwmod::Disabled,
            true => Iwmod::Enabled,
        }
    }
    #[doc = "Disable initial working mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Iwmod::Disabled
    }
    #[doc = "Enable initial working mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Iwmod::Enabled
    }
}
#[doc = "Field `IWMOD` writer - Initial working mode"]
pub type IwmodW<'a, REG> = crate::BitWriter<'a, REG, Iwmod>;
impl<'a, REG> IwmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable initial working mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iwmod::Disabled)
    }
    #[doc = "Enable initial working mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iwmod::Enabled)
    }
}
#[doc = "Sleep working mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slpwmod {
    #[doc = "0: Disable sleep mode, bus activity detected"]
    Active = 0,
    #[doc = "1: Enable sleep mode"]
    Sleep = 1,
}
impl From<Slpwmod> for bool {
    #[inline(always)]
    fn from(variant: Slpwmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPWMOD` reader - Sleep working mode"]
pub type SlpwmodR = crate::BitReader<Slpwmod>;
impl SlpwmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slpwmod {
        match self.bits {
            false => Slpwmod::Active,
            true => Slpwmod::Sleep,
        }
    }
    #[doc = "Disable sleep mode, bus activity detected"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Slpwmod::Active
    }
    #[doc = "Enable sleep mode"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == Slpwmod::Sleep
    }
}
#[doc = "Field `SLPWMOD` writer - Sleep working mode"]
pub type SlpwmodW<'a, REG> = crate::BitWriter<'a, REG, Slpwmod>;
impl<'a, REG> SlpwmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable sleep mode, bus activity detected"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Slpwmod::Active)
    }
    #[doc = "Enable sleep mode"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(Slpwmod::Sleep)
    }
}
#[doc = "Transmit FIFO order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfo {
    #[doc = "0: Order by identifier of the frame"]
    Identifier = 0,
    #[doc = "1: First in first out order"]
    Fifo = 1,
}
impl From<Tfo> for bool {
    #[inline(always)]
    fn from(variant: Tfo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFO` reader - Transmit FIFO order"]
pub type TfoR = crate::BitReader<Tfo>;
impl TfoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfo {
        match self.bits {
            false => Tfo::Identifier,
            true => Tfo::Fifo,
        }
    }
    #[doc = "Order by identifier of the frame"]
    #[inline(always)]
    pub fn is_identifier(&self) -> bool {
        *self == Tfo::Identifier
    }
    #[doc = "First in first out order"]
    #[inline(always)]
    pub fn is_fifo(&self) -> bool {
        *self == Tfo::Fifo
    }
}
#[doc = "Field `TFO` writer - Transmit FIFO order"]
pub type TfoW<'a, REG> = crate::BitWriter<'a, REG, Tfo>;
impl<'a, REG> TfoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Order by identifier of the frame"]
    #[inline(always)]
    pub fn identifier(self) -> &'a mut crate::W<REG> {
        self.variant(Tfo::Identifier)
    }
    #[doc = "First in first out order"]
    #[inline(always)]
    pub fn fifo(self) -> &'a mut crate::W<REG> {
        self.variant(Tfo::Fifo)
    }
}
#[doc = "Receive FIFO overwrite disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfod {
    #[doc = "0: Overwrite full receive FIFO with incoming frame"]
    Overwrite = 0,
    #[doc = "1: Discard incoming frame when receive FIFO is full"]
    Discard = 1,
}
impl From<Rfod> for bool {
    #[inline(always)]
    fn from(variant: Rfod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOD` reader - Receive FIFO overwrite disable"]
pub type RfodR = crate::BitReader<Rfod>;
impl RfodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfod {
        match self.bits {
            false => Rfod::Overwrite,
            true => Rfod::Discard,
        }
    }
    #[doc = "Overwrite full receive FIFO with incoming frame"]
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == Rfod::Overwrite
    }
    #[doc = "Discard incoming frame when receive FIFO is full"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == Rfod::Discard
    }
}
#[doc = "Field `RFOD` writer - Receive FIFO overwrite disable"]
pub type RfodW<'a, REG> = crate::BitWriter<'a, REG, Rfod>;
impl<'a, REG> RfodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overwrite full receive FIFO with incoming frame"]
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Rfod::Overwrite)
    }
    #[doc = "Discard incoming frame when receive FIFO is full"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(Rfod::Discard)
    }
}
#[doc = "Automatic retransmission disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ard {
    #[doc = "0: Enable automatic retransmission"]
    Enabled = 0,
    #[doc = "1: Disable automatic retransmission"]
    Disabled = 1,
}
impl From<Ard> for bool {
    #[inline(always)]
    fn from(variant: Ard) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARD` reader - Automatic retransmission disable"]
pub type ArdR = crate::BitReader<Ard>;
impl ArdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ard {
        match self.bits {
            false => Ard::Enabled,
            true => Ard::Disabled,
        }
    }
    #[doc = "Enable automatic retransmission"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ard::Enabled
    }
    #[doc = "Disable automatic retransmission"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ard::Disabled
    }
}
#[doc = "Field `ARD` writer - Automatic retransmission disable"]
pub type ArdW<'a, REG> = crate::BitWriter<'a, REG, Ard>;
impl<'a, REG> ArdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable automatic retransmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ard::Enabled)
    }
    #[doc = "Disable automatic retransmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ard::Disabled)
    }
}
#[doc = "Automatic wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Awu {
    #[doc = "0: Sleep state is set by software"]
    Manual = 0,
    #[doc = "1: Sleep state is set automatically by hardware"]
    Automatic = 1,
}
impl From<Awu> for bool {
    #[inline(always)]
    fn from(variant: Awu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWU` reader - Automatic wakeup"]
pub type AwuR = crate::BitReader<Awu>;
impl AwuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Awu {
        match self.bits {
            false => Awu::Manual,
            true => Awu::Automatic,
        }
    }
    #[doc = "Sleep state is set by software"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Awu::Manual
    }
    #[doc = "Sleep state is set automatically by hardware"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == Awu::Automatic
    }
}
#[doc = "Field `AWU` writer - Automatic wakeup"]
pub type AwuW<'a, REG> = crate::BitWriter<'a, REG, Awu>;
impl<'a, REG> AwuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sleep state is set by software"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Awu::Manual)
    }
    #[doc = "Sleep state is set automatically by hardware"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(Awu::Automatic)
    }
}
#[doc = "Automatic bus-off recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abor {
    #[doc = "0: Bus off state is set by software"]
    Manual = 0,
    #[doc = "1: Bus off state is set automatically by hardware"]
    Automatic = 1,
}
impl From<Abor> for bool {
    #[inline(always)]
    fn from(variant: Abor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABOR` reader - Automatic bus-off recovery"]
pub type AborR = crate::BitReader<Abor>;
impl AborR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abor {
        match self.bits {
            false => Abor::Manual,
            true => Abor::Automatic,
        }
    }
    #[doc = "Bus off state is set by software"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Abor::Manual
    }
    #[doc = "Bus off state is set automatically by hardware"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == Abor::Automatic
    }
}
#[doc = "Field `ABOR` writer - Automatic bus-off recovery"]
pub type AborW<'a, REG> = crate::BitWriter<'a, REG, Abor>;
impl<'a, REG> AborW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus off state is set by software"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Abor::Manual)
    }
    #[doc = "Bus off state is set automatically by hardware"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(Abor::Automatic)
    }
}
#[doc = "Time-triggered communication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttc {
    #[doc = "0: Disable time-triggered communication"]
    Disabled = 0,
    #[doc = "1: Enable time-triggered communication"]
    Enabled = 1,
}
impl From<Ttc> for bool {
    #[inline(always)]
    fn from(variant: Ttc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTC` reader - Time-triggered communication"]
pub type TtcR = crate::BitReader<Ttc>;
impl TtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttc {
        match self.bits {
            false => Ttc::Disabled,
            true => Ttc::Enabled,
        }
    }
    #[doc = "Disable time-triggered communication"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ttc::Disabled
    }
    #[doc = "Enable time-triggered communication"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ttc::Enabled
    }
}
#[doc = "Field `TTC` writer - Time-triggered communication"]
pub type TtcW<'a, REG> = crate::BitWriter<'a, REG, Ttc>;
impl<'a, REG> TtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable time-triggered communication"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Disabled)
    }
    #[doc = "Enable time-triggered communication"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Enabled)
    }
}
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstr {
    #[doc = "0: Finished resetting"]
    NotResetting = 0,
    #[doc = "1: Reset in progress"]
    Resetting = 1,
}
impl From<Swrstr> for bool {
    #[inline(always)]
    fn from(variant: Swrstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software reset"]
pub type SwrstR = crate::BitReader<Swrstr>;
impl SwrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstr {
        match self.bits {
            false => Swrstr::NotResetting,
            true => Swrstr::Resetting,
        }
    }
    #[doc = "Finished resetting"]
    #[inline(always)]
    pub fn is_not_resetting(&self) -> bool {
        *self == Swrstr::NotResetting
    }
    #[doc = "Reset in progress"]
    #[inline(always)]
    pub fn is_resetting(&self) -> bool {
        *self == Swrstr::Resetting
    }
}
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwrstwWO {
    #[doc = "1: Reset CAN"]
    Reset = 1,
}
impl From<SwrstwWO> for bool {
    #[inline(always)]
    fn from(variant: SwrstwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` writer - Software reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG, SwrstwWO>;
impl<'a, REG> SwrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset CAN"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SwrstwWO::Reset)
    }
}
#[doc = "Debug freeze\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfz {
    #[doc = "0: Continue running CAN during debug"]
    Continue = 0,
    #[doc = "1: Stop CAN reception and transmission during debug hold"]
    Stop = 1,
}
impl From<Dfz> for bool {
    #[inline(always)]
    fn from(variant: Dfz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFZ` reader - Debug freeze"]
pub type DfzR = crate::BitReader<Dfz>;
impl DfzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfz {
        match self.bits {
            false => Dfz::Continue,
            true => Dfz::Stop,
        }
    }
    #[doc = "Continue running CAN during debug"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == Dfz::Continue
    }
    #[doc = "Stop CAN reception and transmission during debug hold"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Dfz::Stop
    }
}
#[doc = "Field `DFZ` writer - Debug freeze"]
pub type DfzW<'a, REG> = crate::BitWriter<'a, REG, Dfz>;
impl<'a, REG> DfzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue running CAN during debug"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(Dfz::Continue)
    }
    #[doc = "Stop CAN reception and transmission during debug hold"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Dfz::Stop)
    }
}
impl R {
    #[doc = "Bit 0 - Initial working mode"]
    #[inline(always)]
    pub fn iwmod(&self) -> IwmodR {
        IwmodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep working mode"]
    #[inline(always)]
    pub fn slpwmod(&self) -> SlpwmodR {
        SlpwmodR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO order"]
    #[inline(always)]
    pub fn tfo(&self) -> TfoR {
        TfoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overwrite disable"]
    #[inline(always)]
    pub fn rfod(&self) -> RfodR {
        RfodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic retransmission disable"]
    #[inline(always)]
    pub fn ard(&self) -> ArdR {
        ArdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic wakeup"]
    #[inline(always)]
    pub fn awu(&self) -> AwuR {
        AwuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic bus-off recovery"]
    #[inline(always)]
    pub fn abor(&self) -> AborR {
        AborR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Time-triggered communication"]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        TtcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    pub fn dfz(&self) -> DfzR {
        DfzR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initial working mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwmod(&mut self) -> IwmodW<CtlSpec> {
        IwmodW::new(self, 0)
    }
    #[doc = "Bit 1 - Sleep working mode"]
    #[inline(always)]
    #[must_use]
    pub fn slpwmod(&mut self) -> SlpwmodW<CtlSpec> {
        SlpwmodW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO order"]
    #[inline(always)]
    #[must_use]
    pub fn tfo(&mut self) -> TfoW<CtlSpec> {
        TfoW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO overwrite disable"]
    #[inline(always)]
    #[must_use]
    pub fn rfod(&mut self) -> RfodW<CtlSpec> {
        RfodW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic retransmission disable"]
    #[inline(always)]
    #[must_use]
    pub fn ard(&mut self) -> ArdW<CtlSpec> {
        ArdW::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn awu(&mut self) -> AwuW<CtlSpec> {
        AwuW::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic bus-off recovery"]
    #[inline(always)]
    #[must_use]
    pub fn abor(&mut self) -> AborW<CtlSpec> {
        AborW::new(self, 6)
    }
    #[doc = "Bit 7 - Time-triggered communication"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TtcW<CtlSpec> {
        TtcW::new(self, 7)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtlSpec> {
        SwrstW::new(self, 15)
    }
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    #[must_use]
    pub fn dfz(&mut self) -> DfzW<CtlSpec> {
        DfzW::new(self, 16)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0001_0002"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x0001_0002;
}
