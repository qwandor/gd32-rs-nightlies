#[doc = "Register `RFIFO0` reader"]
pub type R = crate::R<Rfifo0Spec>;
#[doc = "Register `RFIFO0` writer"]
pub type W = crate::W<Rfifo0Spec>;
#[doc = "Field `RFL0` reader - Receive FIFO0 length"]
pub type Rfl0R = crate::FieldReader;
#[doc = "Receive FIFO0 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rff0r {
    #[doc = "0: Receive FIFO is not full"]
    NotFull = 0,
    #[doc = "1: Receive FIFO is full"]
    Full = 1,
}
impl From<Rff0r> for bool {
    #[inline(always)]
    fn from(variant: Rff0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF0` reader - Receive FIFO0 full"]
pub type Rff0R = crate::BitReader<Rff0r>;
impl Rff0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rff0r {
        match self.bits {
            false => Rff0r::NotFull,
            true => Rff0r::Full,
        }
    }
    #[doc = "Receive FIFO is not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Rff0r::NotFull
    }
    #[doc = "Receive FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rff0r::Full
    }
}
#[doc = "Receive FIFO0 full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rff0wWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<Rff0wWO> for bool {
    #[inline(always)]
    fn from(variant: Rff0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF0` writer - Receive FIFO0 full"]
pub type Rff0W<'a, REG> = crate::BitWriter<'a, REG, Rff0wWO>;
impl<'a, REG> Rff0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rff0wWO::Clear)
    }
}
#[doc = "Receive FIFO0 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfo0r {
    #[doc = "0: Receive FIFO is not overfull"]
    NotOverfull = 0,
    #[doc = "1: Receive FIFO is overfull"]
    Overfull = 1,
}
impl From<Rfo0r> for bool {
    #[inline(always)]
    fn from(variant: Rfo0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO0` reader - Receive FIFO0 overfull"]
pub type Rfo0R = crate::BitReader<Rfo0r>;
impl Rfo0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfo0r {
        match self.bits {
            false => Rfo0r::NotOverfull,
            true => Rfo0r::Overfull,
        }
    }
    #[doc = "Receive FIFO is not overfull"]
    #[inline(always)]
    pub fn is_not_overfull(&self) -> bool {
        *self == Rfo0r::NotOverfull
    }
    #[doc = "Receive FIFO is overfull"]
    #[inline(always)]
    pub fn is_overfull(&self) -> bool {
        *self == Rfo0r::Overfull
    }
}
#[doc = "Receive FIFO0 overfull\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfo0wWO {
    #[doc = "1: Clears flag"]
    Clear = 1,
}
impl From<Rfo0wWO> for bool {
    #[inline(always)]
    fn from(variant: Rfo0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFO0` writer - Receive FIFO0 overfull"]
pub type Rfo0W<'a, REG> = crate::BitWriter<'a, REG, Rfo0wWO>;
impl<'a, REG> Rfo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rfo0wWO::Clear)
    }
}
#[doc = "Receive FIFO0 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfd0r {
    #[doc = "0: Dequeuing done"]
    Finished = 0,
    #[doc = "1: Dequeuing in progress"]
    InProgress = 1,
}
impl From<Rfd0r> for bool {
    #[inline(always)]
    fn from(variant: Rfd0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD0` reader - Receive FIFO0 dequeue"]
pub type Rfd0R = crate::BitReader<Rfd0r>;
impl Rfd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfd0r {
        match self.bits {
            false => Rfd0r::Finished,
            true => Rfd0r::InProgress,
        }
    }
    #[doc = "Dequeuing done"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == Rfd0r::Finished
    }
    #[doc = "Dequeuing in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Rfd0r::InProgress
    }
}
#[doc = "Receive FIFO0 dequeue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfd0wWO {
    #[doc = "1: Start dequeuing"]
    Start = 1,
}
impl From<Rfd0wWO> for bool {
    #[inline(always)]
    fn from(variant: Rfd0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFD0` writer - Receive FIFO0 dequeue"]
pub type Rfd0W<'a, REG> = crate::BitWriter<'a, REG, Rfd0wWO>;
impl<'a, REG> Rfd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start dequeuing"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd0wWO::Start)
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive FIFO0 length"]
    #[inline(always)]
    pub fn rfl0(&self) -> Rfl0R {
        Rfl0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO0 full"]
    #[inline(always)]
    pub fn rff0(&self) -> Rff0R {
        Rff0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO0 overfull"]
    #[inline(always)]
    pub fn rfo0(&self) -> Rfo0R {
        Rfo0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO0 dequeue"]
    #[inline(always)]
    pub fn rfd0(&self) -> Rfd0R {
        Rfd0R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO0 full"]
    #[inline(always)]
    #[must_use]
    pub fn rff0(&mut self) -> Rff0W<Rfifo0Spec> {
        Rff0W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO0 overfull"]
    #[inline(always)]
    #[must_use]
    pub fn rfo0(&mut self) -> Rfo0W<Rfifo0Spec> {
        Rfo0W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO0 dequeue"]
    #[inline(always)]
    #[must_use]
    pub fn rfd0(&mut self) -> Rfd0W<Rfifo0Spec> {
        Rfd0W::new(self, 5)
    }
}
#[doc = "Receive message FIFO0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfifo0Spec;
impl crate::RegisterSpec for Rfifo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo0::R`](R) reader structure"]
impl crate::Readable for Rfifo0Spec {}
#[doc = "`write(|w| ..)` method takes [`rfifo0::W`](W) writer structure"]
impl crate::Writable for Rfifo0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFIFO0 to value 0"]
impl crate::Resettable for Rfifo0Spec {
    const RESET_VALUE: u32 = 0;
}
