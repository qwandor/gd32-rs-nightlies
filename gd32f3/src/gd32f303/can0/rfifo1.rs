#[doc = "Register `RFIFO1` reader"]
pub type R = crate::R<Rfifo1Spec>;
#[doc = "Register `RFIFO1` writer"]
pub type W = crate::W<Rfifo1Spec>;
#[doc = "Field `RFL1` reader - Receive FIFO1 length"]
pub type Rfl1R = crate::FieldReader;
#[doc = "Receive FIFO1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rff1r {
    #[doc = "0: Receive FIFO is not full"]
    NotFull = 0,
    #[doc = "1: Receive FIFO is full"]
    Full = 1,
}
impl From<Rff1r> for bool {
    #[inline(always)]
    fn from(variant: Rff1r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF1` reader - Receive FIFO1 full"]
pub type Rff1R = crate::BitReader<Rff1r>;
impl Rff1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rff1r {
        match self.bits {
            false => Rff1r::NotFull,
            true => Rff1r::Full,
        }
    }
    #[doc = "Receive FIFO is not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Rff1r::NotFull
    }
    #[doc = "Receive FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rff1r::Full
    }
}
#[doc = "Receive FIFO1 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rff1wWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<Rff1wWO> for bool {
    #[inline(always)]
    fn from(variant: Rff1wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF1` writer - Receive FIFO1 full"]
pub type Rff1W<'a, REG> = crate::BitWriter<'a, REG, Rff1wWO>;
impl<'a, REG> Rff1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rff1wWO::Clear)
    }
}
#[doc = "Receive FIFO1 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfo1r {
    #[doc = "0: Receive FIFO is not overfull"]
    NotOverfull = 0,
    #[doc = "1: Receive FIFO is overfull"]
    Overfull = 1,
}
impl From<Rfo1r> for bool {
    #[inline(always)]
    fn from(variant: Rfo1r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO1` reader - Receive FIFO1 overfull"]
pub type Rfo1R = crate::BitReader<Rfo1r>;
impl Rfo1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfo1r {
        match self.bits {
            false => Rfo1r::NotOverfull,
            true => Rfo1r::Overfull,
        }
    }
    #[doc = "Receive FIFO is not overfull"]
    #[inline(always)]
    pub fn is_not_overfull(&self) -> bool {
        *self == Rfo1r::NotOverfull
    }
    #[doc = "Receive FIFO is overfull"]
    #[inline(always)]
    pub fn is_overfull(&self) -> bool {
        *self == Rfo1r::Overfull
    }
}
#[doc = "Receive FIFO1 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfo1wWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<Rfo1wWO> for bool {
    #[inline(always)]
    fn from(variant: Rfo1wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO1` writer - Receive FIFO1 overfull"]
pub type Rfo1W<'a, REG> = crate::BitWriter<'a, REG, Rfo1wWO>;
impl<'a, REG> Rfo1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rfo1wWO::Clear)
    }
}
#[doc = "Receive FIFO1 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfd1r {
    #[doc = "0: Dequeuing done"]
    Finished = 0,
    #[doc = "1: Dequeuing in progress"]
    InProgress = 1,
}
impl From<Rfd1r> for bool {
    #[inline(always)]
    fn from(variant: Rfd1r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD1` reader - Receive FIFO1 dequeue"]
pub type Rfd1R = crate::BitReader<Rfd1r>;
impl Rfd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfd1r {
        match self.bits {
            false => Rfd1r::Finished,
            true => Rfd1r::InProgress,
        }
    }
    #[doc = "Dequeuing done"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == Rfd1r::Finished
    }
    #[doc = "Dequeuing in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Rfd1r::InProgress
    }
}
#[doc = "Receive FIFO1 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfd1wWO {
    #[doc = "1: Start dequeuing"]
    Start = 1,
}
impl From<Rfd1wWO> for bool {
    #[inline(always)]
    fn from(variant: Rfd1wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD1` writer - Receive FIFO1 dequeue"]
pub type Rfd1W<'a, REG> = crate::BitWriter<'a, REG, Rfd1wWO>;
impl<'a, REG> Rfd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start dequeuing"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd1wWO::Start)
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive FIFO1 length"]
    #[inline(always)]
    pub fn rfl1(&self) -> Rfl1R {
        Rfl1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    pub fn rff1(&self) -> Rff1R {
        Rff1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    pub fn rfo1(&self) -> Rfo1R {
        Rfo1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    pub fn rfd1(&self) -> Rfd1R {
        Rfd1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    #[must_use]
    pub fn rff1(&mut self) -> Rff1W<Rfifo1Spec> {
        Rff1W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    #[must_use]
    pub fn rfo1(&mut self) -> Rfo1W<Rfifo1Spec> {
        Rfo1W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    #[must_use]
    pub fn rfd1(&mut self) -> Rfd1W<Rfifo1Spec> {
        Rfd1W::new(self, 5)
    }
}
#[doc = "Receive message FIFO1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfifo1Spec;
impl crate::RegisterSpec for Rfifo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo1::R`](R) reader structure"]
impl crate::Readable for Rfifo1Spec {}
#[doc = "`write(|w| ..)` method takes [`rfifo1::W`](W) writer structure"]
impl crate::Writable for Rfifo1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFIFO1 to value 0"]
impl crate::Resettable for Rfifo1Spec {
    const RESET_VALUE: u32 = 0;
}
