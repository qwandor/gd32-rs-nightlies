#[doc = "Register `TSTAT` reader"]
pub type R = crate::R<TstatSpec>;
#[doc = "Register `TSTAT` writer"]
pub type W = crate::W<TstatSpec>;
#[doc = "Mailbox 0 transmit finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtf0r {
    #[doc = "0: Mailbox transmission still in progress"]
    InProgress = 0,
    #[doc = "1: Mailbox transmission finished"]
    Finished = 1,
}
impl From<Mtf0r> for bool {
    #[inline(always)]
    fn from(variant: Mtf0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTF0` reader - Mailbox 0 transmit finished"]
pub type Mtf0R = crate::BitReader<Mtf0r>;
impl Mtf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtf0r {
        match self.bits {
            false => Mtf0r::InProgress,
            true => Mtf0r::Finished,
        }
    }
    #[doc = "Mailbox transmission still in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Mtf0r::InProgress
    }
    #[doc = "Mailbox transmission finished"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == Mtf0r::Finished
    }
}
#[doc = "Mailbox 0 transmit finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtf0wWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<Mtf0wWO> for bool {
    #[inline(always)]
    fn from(variant: Mtf0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTF0` writer - Mailbox 0 transmit finished"]
pub type Mtf0W<'a, REG> = crate::BitWriter<'a, REG, Mtf0wWO>;
impl<'a, REG> Mtf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Mtf0wWO::Clear)
    }
}
#[doc = "Mailbox 0 transmit finished and no error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtfnerr0r {
    #[doc = "0: Mailbox transmission finished with an error"]
    FinishedWithError = 0,
    #[doc = "1: Mailbox transmission finished with no error"]
    FinishedNoError = 1,
}
impl From<Mtfnerr0r> for bool {
    #[inline(always)]
    fn from(variant: Mtfnerr0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTFNERR0` reader - Mailbox 0 transmit finished and no error"]
pub type Mtfnerr0R = crate::BitReader<Mtfnerr0r>;
impl Mtfnerr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtfnerr0r {
        match self.bits {
            false => Mtfnerr0r::FinishedWithError,
            true => Mtfnerr0r::FinishedNoError,
        }
    }
    #[doc = "Mailbox transmission finished with an error"]
    #[inline(always)]
    pub fn is_finished_with_error(&self) -> bool {
        *self == Mtfnerr0r::FinishedWithError
    }
    #[doc = "Mailbox transmission finished with no error"]
    #[inline(always)]
    pub fn is_finished_no_error(&self) -> bool {
        *self == Mtfnerr0r::FinishedNoError
    }
}
#[doc = "Mailbox 0 transmit finished and no error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtfnerr0wWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<Mtfnerr0wWO> for bool {
    #[inline(always)]
    fn from(variant: Mtfnerr0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTFNERR0` writer - Mailbox 0 transmit finished and no error"]
pub type Mtfnerr0W<'a, REG> = crate::BitWriter<'a, REG, Mtfnerr0wWO>;
impl<'a, REG> Mtfnerr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Mtfnerr0wWO::Clear)
    }
}
#[doc = "Mailbox 0 arbitration lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mal0r {
    #[doc = "0: Arbitration was not lost"]
    NoArbitrationLost = 0,
    #[doc = "1: Arbitration lost"]
    ArbitrationLost = 1,
}
impl From<Mal0r> for bool {
    #[inline(always)]
    fn from(variant: Mal0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAL0` reader - Mailbox 0 arbitration lost"]
pub type Mal0R = crate::BitReader<Mal0r>;
impl Mal0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mal0r {
        match self.bits {
            false => Mal0r::NoArbitrationLost,
            true => Mal0r::ArbitrationLost,
        }
    }
    #[doc = "Arbitration was not lost"]
    #[inline(always)]
    pub fn is_no_arbitration_lost(&self) -> bool {
        *self == Mal0r::NoArbitrationLost
    }
    #[doc = "Arbitration lost"]
    #[inline(always)]
    pub fn is_arbitration_lost(&self) -> bool {
        *self == Mal0r::ArbitrationLost
    }
}
#[doc = "Mailbox 0 arbitration lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mal0wWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<Mal0wWO> for bool {
    #[inline(always)]
    fn from(variant: Mal0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAL0` writer - Mailbox 0 arbitration lost"]
pub type Mal0W<'a, REG> = crate::BitWriter<'a, REG, Mal0wWO>;
impl<'a, REG> Mal0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Mal0wWO::Clear)
    }
}
#[doc = "Mailbox 0 transmit error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mte0r {
    #[doc = "0: There was no error"]
    NoError = 0,
    #[doc = "1: An error was detected"]
    Error = 1,
}
impl From<Mte0r> for bool {
    #[inline(always)]
    fn from(variant: Mte0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTE0` reader - Mailbox 0 transmit error"]
pub type Mte0R = crate::BitReader<Mte0r>;
impl Mte0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mte0r {
        match self.bits {
            false => Mte0r::NoError,
            true => Mte0r::Error,
        }
    }
    #[doc = "There was no error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Mte0r::NoError
    }
    #[doc = "An error was detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Mte0r::Error
    }
}
#[doc = "Mailbox 0 transmit error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mte0wWO {
    #[doc = "1: Clears error flag"]
    Clear = 1,
}
impl From<Mte0wWO> for bool {
    #[inline(always)]
    fn from(variant: Mte0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTE0` writer - Mailbox 0 transmit error"]
pub type Mte0W<'a, REG> = crate::BitWriter<'a, REG, Mte0wWO>;
impl<'a, REG> Mte0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Mte0wWO::Clear)
    }
}
#[doc = "Mailbox 0 stop transmitting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mst0r {
    #[doc = "0: Mailbox is not stopped, or is empty"]
    NotStop = 0,
    #[doc = "1: Stop mailbox transmitting"]
    Stop = 1,
}
impl From<Mst0r> for bool {
    #[inline(always)]
    fn from(variant: Mst0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST0` reader - Mailbox 0 stop transmitting"]
pub type Mst0R = crate::BitReader<Mst0r>;
impl Mst0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mst0r {
        match self.bits {
            false => Mst0r::NotStop,
            true => Mst0r::Stop,
        }
    }
    #[doc = "Mailbox is not stopped, or is empty"]
    #[inline(always)]
    pub fn is_not_stop(&self) -> bool {
        *self == Mst0r::NotStop
    }
    #[doc = "Stop mailbox transmitting"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Mst0r::Stop
    }
}
#[doc = "Mailbox 0 stop transmitting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mst0wWO {
    #[doc = "1: Stop mailbox transmitting"]
    Stop = 1,
}
impl From<Mst0wWO> for bool {
    #[inline(always)]
    fn from(variant: Mst0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST0` writer - Mailbox 0 stop transmitting"]
pub type Mst0W<'a, REG> = crate::BitWriter<'a, REG, Mst0wWO>;
impl<'a, REG> Mst0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop mailbox transmitting"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Mst0wWO::Stop)
    }
}
#[doc = "Field `MAL1` reader - Mailbox 1 arbitration lost"]
pub use Mal0R as Mal1R;
#[doc = "Field `MAL2` reader - Mailbox 2 arbitration lost"]
pub use Mal0R as Mal2R;
#[doc = "Field `MAL1` writer - Mailbox 1 arbitration lost"]
pub use Mal0W as Mal1W;
#[doc = "Field `MAL2` writer - Mailbox 2 arbitration lost"]
pub use Mal0W as Mal2W;
#[doc = "Field `MST1` reader - Mailbox 1 stop transmitting"]
pub use Mst0R as Mst1R;
#[doc = "Field `MST2` reader - Mailbox 2 stop transmitting"]
pub use Mst0R as Mst2R;
#[doc = "Field `MST1` writer - Mailbox 1 stop transmitting"]
pub use Mst0W as Mst1W;
#[doc = "Field `MST2` writer - Mailbox 2 stop transmitting"]
pub use Mst0W as Mst2W;
#[doc = "Field `MTE1` reader - Mailbox 1 transmit error"]
pub use Mte0R as Mte1R;
#[doc = "Field `MTE2` reader - Mailbox 2 transmit error"]
pub use Mte0R as Mte2R;
#[doc = "Field `MTE1` writer - Mailbox 1 transmit error"]
pub use Mte0W as Mte1W;
#[doc = "Field `MTE2` writer - Mailbox 2 transmit error"]
pub use Mte0W as Mte2W;
#[doc = "Field `MTF1` reader - Mailbox 1 transmit finished"]
pub use Mtf0R as Mtf1R;
#[doc = "Field `MTF2` reader - Mailbox 2 transmit finished"]
pub use Mtf0R as Mtf2R;
#[doc = "Field `MTF1` writer - Mailbox 1 transmit finished"]
pub use Mtf0W as Mtf1W;
#[doc = "Field `MTF2` writer - Mailbox 2 transmit finished"]
pub use Mtf0W as Mtf2W;
#[doc = "Field `MTFNERR1` reader - Mailbox 1 transmit finished and no error"]
pub use Mtfnerr0R as Mtfnerr1R;
#[doc = "Field `MTFNERR2` reader - Mailbox 2 transmit finished and no error"]
pub use Mtfnerr0R as Mtfnerr2R;
#[doc = "Field `MTFNERR1` writer - Mailbox 1 transmit finished and no error"]
pub use Mtfnerr0W as Mtfnerr1W;
#[doc = "Field `MTFNERR2` writer - Mailbox 2 transmit finished and no error"]
pub use Mtfnerr0W as Mtfnerr2W;
#[doc = "Field `NUM` reader - number of the transmit FIFO mailbox in which the frame will be transmitted if at least one mailbox is empty"]
pub type NumR = crate::FieldReader;
#[doc = "Transmit mailbox 0 empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tme0 {
    #[doc = "0: Transmit mailbox not empty"]
    NotEmpty = 0,
    #[doc = "1: Transmit mailbox is empty"]
    Empty = 1,
}
impl From<Tme0> for bool {
    #[inline(always)]
    fn from(variant: Tme0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TME0` reader - Transmit mailbox 0 empty"]
pub type Tme0R = crate::BitReader<Tme0>;
impl Tme0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tme0 {
        match self.bits {
            false => Tme0::NotEmpty,
            true => Tme0::Empty,
        }
    }
    #[doc = "Transmit mailbox not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Tme0::NotEmpty
    }
    #[doc = "Transmit mailbox is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Tme0::Empty
    }
}
#[doc = "Field `TME1` reader - Transmit mailbox 1 empty"]
pub use Tme0R as Tme1R;
#[doc = "Field `TME2` reader - Transmit mailbox 2 empty"]
pub use Tme0R as Tme2R;
#[doc = "Transmit mailbox 0 last sending in transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmls0 {
    #[doc = "0: The mailbox doesn't have the last sending order"]
    NotLast = 0,
    #[doc = "1: The mailbox has the last sending order with at least two frames pending"]
    Last = 1,
}
impl From<Tmls0> for bool {
    #[inline(always)]
    fn from(variant: Tmls0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMLS0` reader - Transmit mailbox 0 last sending in transmit FIFO"]
pub type Tmls0R = crate::BitReader<Tmls0>;
impl Tmls0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmls0 {
        match self.bits {
            false => Tmls0::NotLast,
            true => Tmls0::Last,
        }
    }
    #[doc = "The mailbox doesn't have the last sending order"]
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == Tmls0::NotLast
    }
    #[doc = "The mailbox has the last sending order with at least two frames pending"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == Tmls0::Last
    }
}
#[doc = "Field `TMLS1` reader - Transmit mailbox 1 last sending in transmit FIFO"]
pub use Tmls0R as Tmls1R;
#[doc = "Field `TMLS2` reader - Transmit mailbox 2 last sending in transmit FIFO"]
pub use Tmls0R as Tmls2R;
impl R {
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    pub fn mtf0(&self) -> Mtf0R {
        Mtf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr0(&self) -> Mtfnerr0R {
        Mtfnerr0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    pub fn mal0(&self) -> Mal0R {
        Mal0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    pub fn mte0(&self) -> Mte0R {
        Mte0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    pub fn mst0(&self) -> Mst0R {
        Mst0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    pub fn mtf1(&self) -> Mtf1R {
        Mtf1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr1(&self) -> Mtfnerr1R {
        Mtfnerr1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    pub fn mal1(&self) -> Mal1R {
        Mal1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    pub fn mte1(&self) -> Mte1R {
        Mte1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    pub fn mst1(&self) -> Mst1R {
        Mst1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    pub fn mtf2(&self) -> Mtf2R {
        Mtf2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr2(&self) -> Mtfnerr2R {
        Mtfnerr2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    pub fn mal2(&self) -> Mal2R {
        Mal2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    pub fn mte2(&self) -> Mte2R {
        Mte2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    pub fn mst2(&self) -> Mst2R {
        Mst2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - number of the transmit FIFO mailbox in which the frame will be transmitted if at least one mailbox is empty"]
    #[inline(always)]
    pub fn num(&self) -> NumR {
        NumR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Transmit mailbox 0 empty"]
    #[inline(always)]
    pub fn tme0(&self) -> Tme0R {
        Tme0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit mailbox 1 empty"]
    #[inline(always)]
    pub fn tme1(&self) -> Tme1R {
        Tme1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit mailbox 2 empty"]
    #[inline(always)]
    pub fn tme2(&self) -> Tme2R {
        Tme2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit mailbox 0 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls0(&self) -> Tmls0R {
        Tmls0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit mailbox 1 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls1(&self) -> Tmls1R {
        Tmls1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit mailbox 2 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls2(&self) -> Tmls2R {
        Tmls2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    #[must_use]
    pub fn mtf0(&mut self) -> Mtf0W<TstatSpec> {
        Mtf0W::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    #[must_use]
    pub fn mtfnerr0(&mut self) -> Mtfnerr0W<TstatSpec> {
        Mtfnerr0W::new(self, 1)
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn mal0(&mut self) -> Mal0W<TstatSpec> {
        Mal0W::new(self, 2)
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn mte0(&mut self) -> Mte0W<TstatSpec> {
        Mte0W::new(self, 3)
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn mst0(&mut self) -> Mst0W<TstatSpec> {
        Mst0W::new(self, 7)
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    #[must_use]
    pub fn mtf1(&mut self) -> Mtf1W<TstatSpec> {
        Mtf1W::new(self, 8)
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    #[must_use]
    pub fn mtfnerr1(&mut self) -> Mtfnerr1W<TstatSpec> {
        Mtfnerr1W::new(self, 9)
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn mal1(&mut self) -> Mal1W<TstatSpec> {
        Mal1W::new(self, 10)
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn mte1(&mut self) -> Mte1W<TstatSpec> {
        Mte1W::new(self, 11)
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn mst1(&mut self) -> Mst1W<TstatSpec> {
        Mst1W::new(self, 15)
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    #[must_use]
    pub fn mtf2(&mut self) -> Mtf2W<TstatSpec> {
        Mtf2W::new(self, 16)
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    #[must_use]
    pub fn mtfnerr2(&mut self) -> Mtfnerr2W<TstatSpec> {
        Mtfnerr2W::new(self, 17)
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn mal2(&mut self) -> Mal2W<TstatSpec> {
        Mal2W::new(self, 18)
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn mte2(&mut self) -> Mte2W<TstatSpec> {
        Mte2W::new(self, 19)
    }
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn mst2(&mut self) -> Mst2W<TstatSpec> {
        Mst2W::new(self, 23)
    }
}
#[doc = "Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstatSpec;
impl crate::RegisterSpec for TstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstat::R`](R) reader structure"]
impl crate::Readable for TstatSpec {}
#[doc = "`write(|w| ..)` method takes [`tstat::W`](W) writer structure"]
impl crate::Writable for TstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSTAT to value 0x1c00_0000"]
impl crate::Resettable for TstatSpec {
    const RESET_VALUE: u32 = 0x1c00_0000;
}
