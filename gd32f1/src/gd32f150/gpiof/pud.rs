#[doc = "Register `PUD` reader"]
pub type R = crate::R<PudSpec>;
#[doc = "Register `PUD` writer"]
pub type W = crate::W<PudSpec>;
#[doc = "Pin 0 pull-up or pull-down bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pud0 {
    #[doc = "0: No pull-up, pull-down (reset state)"]
    Floating = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
    #[doc = "2: Pull-down"]
    PullDown = 2,
}
impl From<Pud0> for u8 {
    #[inline(always)]
    fn from(variant: Pud0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pud0 {
    type Ux = u8;
}
#[doc = "Field `PUD0` reader - Pin 0 pull-up or pull-down bits"]
pub type Pud0R = crate::FieldReader<Pud0>;
impl Pud0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pud0> {
        match self.bits {
            0 => Some(Pud0::Floating),
            1 => Some(Pud0::PullUp),
            2 => Some(Pud0::PullDown),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down (reset state)"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == Pud0::Floating
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Pud0::PullUp
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Pud0::PullDown
    }
}
#[doc = "Field `PUD0` writer - Pin 0 pull-up or pull-down bits"]
pub type Pud0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Pud0>;
impl<'a, REG> Pud0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down (reset state)"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(Pud0::Floating)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Pud0::PullUp)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Pud0::PullDown)
    }
}
#[doc = "Field `PUD1` reader - Pin 1 pull-up or pull-down bits"]
pub use Pud0R as Pud1R;
#[doc = "Field `PUD2` reader - Pin 2 pull-up or pull-down bits"]
pub use Pud0R as Pud2R;
#[doc = "Field `PUD3` reader - Pin 3 pull-up or pull-down bits"]
pub use Pud0R as Pud3R;
#[doc = "Field `PUD4` reader - Pin 4 pull-up or pull-down bits"]
pub use Pud0R as Pud4R;
#[doc = "Field `PUD5` reader - Pin 5 pull-up or pull-down bits"]
pub use Pud0R as Pud5R;
#[doc = "Field `PUD6` reader - Pin 6 pull-up or pull-down bits"]
pub use Pud0R as Pud6R;
#[doc = "Field `PUD7` reader - Pin 7 pull-up or pull-down bits"]
pub use Pud0R as Pud7R;
#[doc = "Field `PUD8` reader - Pin 8 pull-up or pull-down bits"]
pub use Pud0R as Pud8R;
#[doc = "Field `PUD9` reader - Pin 9 pull-up or pull-down bits"]
pub use Pud0R as Pud9R;
#[doc = "Field `PUD10` reader - Pin 10 pull-up or pull-down bits"]
pub use Pud0R as Pud10R;
#[doc = "Field `PUD11` reader - Pin 11 pull-up or pull-down bits"]
pub use Pud0R as Pud11R;
#[doc = "Field `PUD12` reader - Pin 12 pull-up or pull-down bits"]
pub use Pud0R as Pud12R;
#[doc = "Field `PUD13` reader - Pin 13 pull-up or pull-down bits"]
pub use Pud0R as Pud13R;
#[doc = "Field `PUD14` reader - Pin 14 pull-up or pull-down bits"]
pub use Pud0R as Pud14R;
#[doc = "Field `PUD15` reader - Pin 15 pull-up or pull-down bits"]
pub use Pud0R as Pud15R;
#[doc = "Field `PUD1` writer - Pin 1 pull-up or pull-down bits"]
pub use Pud0W as Pud1W;
#[doc = "Field `PUD2` writer - Pin 2 pull-up or pull-down bits"]
pub use Pud0W as Pud2W;
#[doc = "Field `PUD3` writer - Pin 3 pull-up or pull-down bits"]
pub use Pud0W as Pud3W;
#[doc = "Field `PUD4` writer - Pin 4 pull-up or pull-down bits"]
pub use Pud0W as Pud4W;
#[doc = "Field `PUD5` writer - Pin 5 pull-up or pull-down bits"]
pub use Pud0W as Pud5W;
#[doc = "Field `PUD6` writer - Pin 6 pull-up or pull-down bits"]
pub use Pud0W as Pud6W;
#[doc = "Field `PUD7` writer - Pin 7 pull-up or pull-down bits"]
pub use Pud0W as Pud7W;
#[doc = "Field `PUD8` writer - Pin 8 pull-up or pull-down bits"]
pub use Pud0W as Pud8W;
#[doc = "Field `PUD9` writer - Pin 9 pull-up or pull-down bits"]
pub use Pud0W as Pud9W;
#[doc = "Field `PUD10` writer - Pin 10 pull-up or pull-down bits"]
pub use Pud0W as Pud10W;
#[doc = "Field `PUD11` writer - Pin 11 pull-up or pull-down bits"]
pub use Pud0W as Pud11W;
#[doc = "Field `PUD12` writer - Pin 12 pull-up or pull-down bits"]
pub use Pud0W as Pud12W;
#[doc = "Field `PUD13` writer - Pin 13 pull-up or pull-down bits"]
pub use Pud0W as Pud13W;
#[doc = "Field `PUD14` writer - Pin 14 pull-up or pull-down bits"]
pub use Pud0W as Pud14W;
#[doc = "Field `PUD15` writer - Pin 15 pull-up or pull-down bits"]
pub use Pud0W as Pud15W;
impl R {
    #[doc = "Bits 0:1 - Pin 0 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud0(&self) -> Pud0R {
        Pud0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin 1 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud1(&self) -> Pud1R {
        Pud1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin 2 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud2(&self) -> Pud2R {
        Pud2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin 3 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud3(&self) -> Pud3R {
        Pud3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin 4 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud4(&self) -> Pud4R {
        Pud4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin 5 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud5(&self) -> Pud5R {
        Pud5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin 6 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud6(&self) -> Pud6R {
        Pud6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin 7 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud7(&self) -> Pud7R {
        Pud7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin 8 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud8(&self) -> Pud8R {
        Pud8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin 9 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud9(&self) -> Pud9R {
        Pud9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin 10 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud10(&self) -> Pud10R {
        Pud10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin 11 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud11(&self) -> Pud11R {
        Pud11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin 12 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud12(&self) -> Pud12R {
        Pud12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin 13 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud13(&self) -> Pud13R {
        Pud13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Pin 14 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud14(&self) -> Pud14R {
        Pud14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Pin 15 pull-up or pull-down bits"]
    #[inline(always)]
    pub fn pud15(&self) -> Pud15R {
        Pud15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin 0 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud0(&mut self) -> Pud0W<PudSpec> {
        Pud0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pin 1 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud1(&mut self) -> Pud1W<PudSpec> {
        Pud1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Pin 2 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud2(&mut self) -> Pud2W<PudSpec> {
        Pud2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Pin 3 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud3(&mut self) -> Pud3W<PudSpec> {
        Pud3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin 4 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud4(&mut self) -> Pud4W<PudSpec> {
        Pud4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pin 5 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud5(&mut self) -> Pud5W<PudSpec> {
        Pud5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Pin 6 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud6(&mut self) -> Pud6W<PudSpec> {
        Pud6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Pin 7 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud7(&mut self) -> Pud7W<PudSpec> {
        Pud7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Pin 8 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud8(&mut self) -> Pud8W<PudSpec> {
        Pud8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Pin 9 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud9(&mut self) -> Pud9W<PudSpec> {
        Pud9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Pin 10 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud10(&mut self) -> Pud10W<PudSpec> {
        Pud10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Pin 11 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud11(&mut self) -> Pud11W<PudSpec> {
        Pud11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Pin 12 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud12(&mut self) -> Pud12W<PudSpec> {
        Pud12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Pin 13 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud13(&mut self) -> Pud13W<PudSpec> {
        Pud13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Pin 14 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud14(&mut self) -> Pud14W<PudSpec> {
        Pud14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Pin 15 pull-up or pull-down bits"]
    #[inline(always)]
    #[must_use]
    pub fn pud15(&mut self) -> Pud15W<PudSpec> {
        Pud15W::new(self, 30)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PudSpec;
impl crate::RegisterSpec for PudSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pud::R`](R) reader structure"]
impl crate::Readable for PudSpec {}
#[doc = "`write(|w| ..)` method takes [`pud::W`](W) writer structure"]
impl crate::Writable for PudSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUD to value 0"]
impl crate::Resettable for PudSpec {
    const RESET_VALUE: u32 = 0;
}
