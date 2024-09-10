#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Transmit mailbox empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmeie {
    #[doc = "0: Transmit mailbox empty interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Transmit mailbox empty interrupt is enabled"]
    Enabled = 1,
}
impl From<Tmeie> for bool {
    #[inline(always)]
    fn from(variant: Tmeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMEIE` reader - Transmit mailbox empty interrupt enable"]
pub type TmeieR = crate::BitReader<Tmeie>;
impl TmeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmeie {
        match self.bits {
            false => Tmeie::Disabled,
            true => Tmeie::Enabled,
        }
    }
    #[doc = "Transmit mailbox empty interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tmeie::Disabled
    }
    #[doc = "Transmit mailbox empty interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tmeie::Enabled
    }
}
#[doc = "Field `TMEIE` writer - Transmit mailbox empty interrupt enable"]
pub type TmeieW<'a, REG> = crate::BitWriter<'a, REG, Tmeie>;
impl<'a, REG> TmeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit mailbox empty interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tmeie::Disabled)
    }
    #[doc = "Transmit mailbox empty interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tmeie::Enabled)
    }
}
#[doc = "Receive FIFO0 not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfneie0 {
    #[doc = "0: Receive FIFO not empty interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Receive FIFO not empty interrupt is enabled"]
    Enabled = 1,
}
impl From<Rfneie0> for bool {
    #[inline(always)]
    fn from(variant: Rfneie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFNEIE0` reader - Receive FIFO0 not empty interrupt enable"]
pub type Rfneie0R = crate::BitReader<Rfneie0>;
impl Rfneie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfneie0 {
        match self.bits {
            false => Rfneie0::Disabled,
            true => Rfneie0::Enabled,
        }
    }
    #[doc = "Receive FIFO not empty interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rfneie0::Disabled
    }
    #[doc = "Receive FIFO not empty interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rfneie0::Enabled
    }
}
#[doc = "Field `RFNEIE0` writer - Receive FIFO0 not empty interrupt enable"]
pub type Rfneie0W<'a, REG> = crate::BitWriter<'a, REG, Rfneie0>;
impl<'a, REG> Rfneie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO not empty interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rfneie0::Disabled)
    }
    #[doc = "Receive FIFO not empty interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rfneie0::Enabled)
    }
}
#[doc = "Receive FIFO0 full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffie0 {
    #[doc = "0: Receive FIFO full interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Receive FIFO full interrupt is enabled"]
    Enabled = 1,
}
impl From<Rffie0> for bool {
    #[inline(always)]
    fn from(variant: Rffie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFIE0` reader - Receive FIFO0 full interrupt enable"]
pub type Rffie0R = crate::BitReader<Rffie0>;
impl Rffie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rffie0 {
        match self.bits {
            false => Rffie0::Disabled,
            true => Rffie0::Enabled,
        }
    }
    #[doc = "Receive FIFO full interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rffie0::Disabled
    }
    #[doc = "Receive FIFO full interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rffie0::Enabled
    }
}
#[doc = "Field `RFFIE0` writer - Receive FIFO0 full interrupt enable"]
pub type Rffie0W<'a, REG> = crate::BitWriter<'a, REG, Rffie0>;
impl<'a, REG> Rffie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO full interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rffie0::Disabled)
    }
    #[doc = "Receive FIFO full interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rffie0::Enabled)
    }
}
#[doc = "Receive FIFO0 overfull interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfoie0 {
    #[doc = "0: Receive FIFO overfull interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Receive FIFO overfull interrupt is enabled"]
    Enabled = 1,
}
impl From<Rfoie0> for bool {
    #[inline(always)]
    fn from(variant: Rfoie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOIE0` reader - Receive FIFO0 overfull interrupt enable"]
pub type Rfoie0R = crate::BitReader<Rfoie0>;
impl Rfoie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfoie0 {
        match self.bits {
            false => Rfoie0::Disabled,
            true => Rfoie0::Enabled,
        }
    }
    #[doc = "Receive FIFO overfull interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rfoie0::Disabled
    }
    #[doc = "Receive FIFO overfull interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rfoie0::Enabled
    }
}
#[doc = "Field `RFOIE0` writer - Receive FIFO0 overfull interrupt enable"]
pub type Rfoie0W<'a, REG> = crate::BitWriter<'a, REG, Rfoie0>;
impl<'a, REG> Rfoie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO overfull interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rfoie0::Disabled)
    }
    #[doc = "Receive FIFO overfull interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rfoie0::Enabled)
    }
}
#[doc = "Field `RFFIE1` reader - Receive FIFO1 full interrupt enable"]
pub use Rffie0R as Rffie1R;
#[doc = "Field `RFFIE1` writer - Receive FIFO1 full interrupt enable"]
pub use Rffie0W as Rffie1W;
#[doc = "Field `RFNEIE1` reader - Receive FIFO1 not empty interrupt enable"]
pub use Rfneie0R as Rfneie1R;
#[doc = "Field `RFNEIE1` writer - Receive FIFO1 not empty interrupt enable"]
pub use Rfneie0W as Rfneie1W;
#[doc = "Field `RFOIE1` reader - Receive FIFO1 overfull interrupt enable"]
pub use Rfoie0R as Rfoie1R;
#[doc = "Field `RFOIE1` writer - Receive FIFO1 overfull interrupt enable"]
pub use Rfoie0W as Rfoie1W;
#[doc = "Warning error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Werrie {
    #[doc = "0: Warning error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Warning error interrupt is enabled"]
    Enabled = 1,
}
impl From<Werrie> for bool {
    #[inline(always)]
    fn from(variant: Werrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WERRIE` reader - Warning error interrupt enable"]
pub type WerrieR = crate::BitReader<Werrie>;
impl WerrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Werrie {
        match self.bits {
            false => Werrie::Disabled,
            true => Werrie::Enabled,
        }
    }
    #[doc = "Warning error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Werrie::Disabled
    }
    #[doc = "Warning error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Werrie::Enabled
    }
}
#[doc = "Field `WERRIE` writer - Warning error interrupt enable"]
pub type WerrieW<'a, REG> = crate::BitWriter<'a, REG, Werrie>;
impl<'a, REG> WerrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Warning error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Werrie::Disabled)
    }
    #[doc = "Warning error interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Werrie::Enabled)
    }
}
#[doc = "Passive error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perrie {
    #[doc = "0: Passive error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Passive error interrupt is enabled"]
    Enabled = 1,
}
impl From<Perrie> for bool {
    #[inline(always)]
    fn from(variant: Perrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIE` reader - Passive error interrupt enable"]
pub type PerrieR = crate::BitReader<Perrie>;
impl PerrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perrie {
        match self.bits {
            false => Perrie::Disabled,
            true => Perrie::Enabled,
        }
    }
    #[doc = "Passive error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Perrie::Disabled
    }
    #[doc = "Passive error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Perrie::Enabled
    }
}
#[doc = "Field `PERRIE` writer - Passive error interrupt enable"]
pub type PerrieW<'a, REG> = crate::BitWriter<'a, REG, Perrie>;
impl<'a, REG> PerrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Perrie::Disabled)
    }
    #[doc = "Passive error interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Perrie::Enabled)
    }
}
#[doc = "Bus-off interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boie {
    #[doc = "0: Bus-off interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Bus-off interrupt is enabled"]
    Enabled = 1,
}
impl From<Boie> for bool {
    #[inline(always)]
    fn from(variant: Boie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOIE` reader - Bus-off interrupt enable"]
pub type BoieR = crate::BitReader<Boie>;
impl BoieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boie {
        match self.bits {
            false => Boie::Disabled,
            true => Boie::Enabled,
        }
    }
    #[doc = "Bus-off interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Boie::Disabled
    }
    #[doc = "Bus-off interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Boie::Enabled
    }
}
#[doc = "Field `BOIE` writer - Bus-off interrupt enable"]
pub type BoieW<'a, REG> = crate::BitWriter<'a, REG, Boie>;
impl<'a, REG> BoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus-off interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Boie::Disabled)
    }
    #[doc = "Bus-off interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Boie::Enabled)
    }
}
#[doc = "Error number interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errnie {
    #[doc = "0: Error number interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error number interrupt is enabled"]
    Enabled = 1,
}
impl From<Errnie> for bool {
    #[inline(always)]
    fn from(variant: Errnie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRNIE` reader - Error number interrupt enable"]
pub type ErrnieR = crate::BitReader<Errnie>;
impl ErrnieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errnie {
        match self.bits {
            false => Errnie::Disabled,
            true => Errnie::Enabled,
        }
    }
    #[doc = "Error number interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errnie::Disabled
    }
    #[doc = "Error number interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errnie::Enabled
    }
}
#[doc = "Field `ERRNIE` writer - Error number interrupt enable"]
pub type ErrnieW<'a, REG> = crate::BitWriter<'a, REG, Errnie>;
impl<'a, REG> ErrnieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error number interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errnie::Disabled)
    }
    #[doc = "Error number interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errnie::Enabled)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt is enabled"]
    Enabled = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader<Errie>;
impl ErrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errie {
        match self.bits {
            false => Errie::Disabled,
            true => Errie::Enabled,
        }
    }
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errie::Disabled
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errie::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Disabled)
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Enabled)
    }
}
#[doc = "Wakeup interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wie {
    #[doc = "0: Wakeup interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Wakeup interrupt is enabled"]
    Enabled = 1,
}
impl From<Wie> for bool {
    #[inline(always)]
    fn from(variant: Wie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIE` reader - Wakeup interrupt enable"]
pub type WieR = crate::BitReader<Wie>;
impl WieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wie {
        match self.bits {
            false => Wie::Disabled,
            true => Wie::Enabled,
        }
    }
    #[doc = "Wakeup interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wie::Disabled
    }
    #[doc = "Wakeup interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wie::Enabled
    }
}
#[doc = "Field `WIE` writer - Wakeup interrupt enable"]
pub type WieW<'a, REG> = crate::BitWriter<'a, REG, Wie>;
impl<'a, REG> WieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wie::Disabled)
    }
    #[doc = "Wakeup interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wie::Enabled)
    }
}
#[doc = "Sleep working interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slpwie {
    #[doc = "0: Sleep working interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Sleep working interrupt is enabled"]
    Enabled = 1,
}
impl From<Slpwie> for bool {
    #[inline(always)]
    fn from(variant: Slpwie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPWIE` reader - Sleep working interrupt enable"]
pub type SlpwieR = crate::BitReader<Slpwie>;
impl SlpwieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slpwie {
        match self.bits {
            false => Slpwie::Disabled,
            true => Slpwie::Enabled,
        }
    }
    #[doc = "Sleep working interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Slpwie::Disabled
    }
    #[doc = "Sleep working interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Slpwie::Enabled
    }
}
#[doc = "Field `SLPWIE` writer - Sleep working interrupt enable"]
pub type SlpwieW<'a, REG> = crate::BitWriter<'a, REG, Slpwie>;
impl<'a, REG> SlpwieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sleep working interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slpwie::Disabled)
    }
    #[doc = "Sleep working interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slpwie::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    pub fn tmeie(&self) -> TmeieR {
        TmeieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO0 not empty interrupt enable"]
    #[inline(always)]
    pub fn rfneie0(&self) -> Rfneie0R {
        Rfneie0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO0 full interrupt enable"]
    #[inline(always)]
    pub fn rffie0(&self) -> Rffie0R {
        Rffie0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO0 overfull interrupt enable"]
    #[inline(always)]
    pub fn rfoie0(&self) -> Rfoie0R {
        Rfoie0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO1 not empty interrupt enable"]
    #[inline(always)]
    pub fn rfneie1(&self) -> Rfneie1R {
        Rfneie1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO1 full interrupt enable"]
    #[inline(always)]
    pub fn rffie1(&self) -> Rffie1R {
        Rffie1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO1 overfull interrupt enable"]
    #[inline(always)]
    pub fn rfoie1(&self) -> Rfoie1R {
        Rfoie1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Warning error interrupt enable"]
    #[inline(always)]
    pub fn werrie(&self) -> WerrieR {
        WerrieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Passive error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PerrieR {
        PerrieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    pub fn boie(&self) -> BoieR {
        BoieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error number interrupt enable"]
    #[inline(always)]
    pub fn errnie(&self) -> ErrnieR {
        ErrnieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wie(&self) -> WieR {
        WieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sleep working interrupt enable"]
    #[inline(always)]
    pub fn slpwie(&self) -> SlpwieR {
        SlpwieR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmeie(&mut self) -> TmeieW<IntenSpec> {
        TmeieW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO0 not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfneie0(&mut self) -> Rfneie0W<IntenSpec> {
        Rfneie0W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO0 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffie0(&mut self) -> Rffie0W<IntenSpec> {
        Rffie0W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO0 overfull interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfoie0(&mut self) -> Rfoie0W<IntenSpec> {
        Rfoie0W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO1 not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfneie1(&mut self) -> Rfneie1W<IntenSpec> {
        Rfneie1W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO1 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffie1(&mut self) -> Rffie1W<IntenSpec> {
        Rffie1W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive FIFO1 overfull interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfoie1(&mut self) -> Rfoie1W<IntenSpec> {
        Rfoie1W::new(self, 6)
    }
    #[doc = "Bit 8 - Warning error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn werrie(&mut self) -> WerrieW<IntenSpec> {
        WerrieW::new(self, 8)
    }
    #[doc = "Bit 9 - Passive error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PerrieW<IntenSpec> {
        PerrieW::new(self, 9)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn boie(&mut self) -> BoieW<IntenSpec> {
        BoieW::new(self, 10)
    }
    #[doc = "Bit 11 - Error number interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errnie(&mut self) -> ErrnieW<IntenSpec> {
        ErrnieW::new(self, 11)
    }
    #[doc = "Bit 15 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<IntenSpec> {
        ErrieW::new(self, 15)
    }
    #[doc = "Bit 16 - Wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wie(&mut self) -> WieW<IntenSpec> {
        WieW::new(self, 16)
    }
    #[doc = "Bit 17 - Sleep working interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn slpwie(&mut self) -> SlpwieW<IntenSpec> {
        SlpwieW::new(self, 17)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
