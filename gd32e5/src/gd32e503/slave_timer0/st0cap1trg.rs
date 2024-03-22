#[doc = "Register `ST0CAP1TRG` reader"]
pub type R = crate::R<St0cap1trgSpec>;
#[doc = "Register `ST0CAP1TRG` writer"]
pub type W = crate::W<St0cap1trgSpec>;
#[doc = "Field `CP1BSW` reader - Capture 1 triggered by software"]
pub type Cp1bswR = crate::BitReader;
#[doc = "Field `CP1BSW` writer - Capture 1 triggered by software"]
pub type Cp1bswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BUP` reader - Capture 1 triggered by update event"]
pub type Cp1bupR = crate::BitReader;
#[doc = "Field `CP1BUP` writer - Capture 1 triggered by update event"]
pub type Cp1bupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV0` reader - Capture 1 triggered by external event 0"]
pub type Cp1bexev0R = crate::BitReader;
#[doc = "Field `CP1BEXEV0` writer - Capture 1 triggered by external event 0"]
pub type Cp1bexev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV1` reader - Capture 1 triggered by external event 1"]
pub type Cp1bexev1R = crate::BitReader;
#[doc = "Field `CP1BEXEV1` writer - Capture 1 triggered by external event 1"]
pub type Cp1bexev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV2` reader - Capture 1 triggered by external event 2"]
pub type Cp1bexev2R = crate::BitReader;
#[doc = "Field `CP1BEXEV2` writer - Capture 1 triggered by external event 2"]
pub type Cp1bexev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV3` reader - Capture 1 triggered by external event 3"]
pub type Cp1bexev3R = crate::BitReader;
#[doc = "Field `CP1BEXEV3` writer - Capture 1 triggered by external event 3"]
pub type Cp1bexev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV4` reader - Capture 1 triggered by external event 4"]
pub type Cp1bexev4R = crate::BitReader;
#[doc = "Field `CP1BEXEV4` writer - Capture 1 triggered by external event 4"]
pub type Cp1bexev4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV5` reader - Capture 1 triggered by external event 5"]
pub type Cp1bexev5R = crate::BitReader;
#[doc = "Field `CP1BEXEV5` writer - Capture 1 triggered by external event 5"]
pub type Cp1bexev5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV6` reader - Capture 1 triggered by external event 6"]
pub type Cp1bexev6R = crate::BitReader;
#[doc = "Field `CP1BEXEV6` writer - Capture 1 triggered by external event 6"]
pub type Cp1bexev6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV7` reader - Capture 1 triggered by external event 7"]
pub type Cp1bexev7R = crate::BitReader;
#[doc = "Field `CP1BEXEV7` writer - Capture 1 triggered by external event 7"]
pub type Cp1bexev7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV8` reader - Capture 1 triggered by external event 8"]
pub type Cp1bexev8R = crate::BitReader;
#[doc = "Field `CP1BEXEV8` writer - Capture 1 triggered by external event 8"]
pub type Cp1bexev8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BEXEV9` reader - Capture 1 triggered by external event 9"]
pub type Cp1bexev9R = crate::BitReader;
#[doc = "Field `CP1BEXEV9` writer - Capture 1 triggered by external event 9"]
pub type Cp1bexev9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BST0A` reader - Capture 1 triggered by ST0CH0_O output inactive to active transition"]
pub type Cp1bst0aR = crate::BitReader;
#[doc = "Field `CP1BST0A` writer - Capture 1 triggered by ST0CH0_O output inactive to active transition"]
pub type Cp1bst0aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BST0NA` reader - Capture 1 triggered by ST0CH0_O output active to inactive transition"]
pub type Cp1bst0naR = crate::BitReader;
#[doc = "Field `CP1BST0NA` writer - Capture 1 triggered by ST0CH0_O output active to inactive transition"]
pub type Cp1bst0naW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BST0CMP0` reader - Capture 1 triggered by compare 0 event of Slave_TIMER0"]
pub type Cp1bst0cmp0R = crate::BitReader;
#[doc = "Field `CP1BST0CMP0` writer - Capture 1 triggered by compare 0 event of Slave_TIMER0"]
pub type Cp1bst0cmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CP1BST0CMP1` reader - Capture 1 triggered by compare 1 event of Slave_TIMER0"]
pub type Cp1bst0cmp1R = crate::BitReader;
#[doc = "Field `CP1BST0CMP1` writer - Capture 1 triggered by compare 1 event of Slave_TIMER0"]
pub type Cp1bst0cmp1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture 1 triggered by software"]
    #[inline(always)]
    pub fn cp1bsw(&self) -> Cp1bswR {
        Cp1bswR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture 1 triggered by update event"]
    #[inline(always)]
    pub fn cp1bup(&self) -> Cp1bupR {
        Cp1bupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture 1 triggered by external event 0"]
    #[inline(always)]
    pub fn cp1bexev0(&self) -> Cp1bexev0R {
        Cp1bexev0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture 1 triggered by external event 1"]
    #[inline(always)]
    pub fn cp1bexev1(&self) -> Cp1bexev1R {
        Cp1bexev1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture 1 triggered by external event 2"]
    #[inline(always)]
    pub fn cp1bexev2(&self) -> Cp1bexev2R {
        Cp1bexev2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture 1 triggered by external event 3"]
    #[inline(always)]
    pub fn cp1bexev3(&self) -> Cp1bexev3R {
        Cp1bexev3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture 1 triggered by external event 4"]
    #[inline(always)]
    pub fn cp1bexev4(&self) -> Cp1bexev4R {
        Cp1bexev4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture 1 triggered by external event 5"]
    #[inline(always)]
    pub fn cp1bexev5(&self) -> Cp1bexev5R {
        Cp1bexev5R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture 1 triggered by external event 6"]
    #[inline(always)]
    pub fn cp1bexev6(&self) -> Cp1bexev6R {
        Cp1bexev6R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture 1 triggered by external event 7"]
    #[inline(always)]
    pub fn cp1bexev7(&self) -> Cp1bexev7R {
        Cp1bexev7R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture 1 triggered by external event 8"]
    #[inline(always)]
    pub fn cp1bexev8(&self) -> Cp1bexev8R {
        Cp1bexev8R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture 1 triggered by external event 9"]
    #[inline(always)]
    pub fn cp1bexev9(&self) -> Cp1bexev9R {
        Cp1bexev9R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture 1 triggered by ST0CH0_O output inactive to active transition"]
    #[inline(always)]
    pub fn cp1bst0a(&self) -> Cp1bst0aR {
        Cp1bst0aR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture 1 triggered by ST0CH0_O output active to inactive transition"]
    #[inline(always)]
    pub fn cp1bst0na(&self) -> Cp1bst0naR {
        Cp1bst0naR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Capture 1 triggered by compare 0 event of Slave_TIMER0"]
    #[inline(always)]
    pub fn cp1bst0cmp0(&self) -> Cp1bst0cmp0R {
        Cp1bst0cmp0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture 1 triggered by compare 1 event of Slave_TIMER0"]
    #[inline(always)]
    pub fn cp1bst0cmp1(&self) -> Cp1bst0cmp1R {
        Cp1bst0cmp1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture 1 triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bsw(&mut self) -> Cp1bswW<St0cap1trgSpec> {
        Cp1bswW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture 1 triggered by update event"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bup(&mut self) -> Cp1bupW<St0cap1trgSpec> {
        Cp1bupW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture 1 triggered by external event 0"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev0(&mut self) -> Cp1bexev0W<St0cap1trgSpec> {
        Cp1bexev0W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture 1 triggered by external event 1"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev1(&mut self) -> Cp1bexev1W<St0cap1trgSpec> {
        Cp1bexev1W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture 1 triggered by external event 2"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev2(&mut self) -> Cp1bexev2W<St0cap1trgSpec> {
        Cp1bexev2W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture 1 triggered by external event 3"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev3(&mut self) -> Cp1bexev3W<St0cap1trgSpec> {
        Cp1bexev3W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture 1 triggered by external event 4"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev4(&mut self) -> Cp1bexev4W<St0cap1trgSpec> {
        Cp1bexev4W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture 1 triggered by external event 5"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev5(&mut self) -> Cp1bexev5W<St0cap1trgSpec> {
        Cp1bexev5W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture 1 triggered by external event 6"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev6(&mut self) -> Cp1bexev6W<St0cap1trgSpec> {
        Cp1bexev6W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture 1 triggered by external event 7"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev7(&mut self) -> Cp1bexev7W<St0cap1trgSpec> {
        Cp1bexev7W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture 1 triggered by external event 8"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev8(&mut self) -> Cp1bexev8W<St0cap1trgSpec> {
        Cp1bexev8W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture 1 triggered by external event 9"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bexev9(&mut self) -> Cp1bexev9W<St0cap1trgSpec> {
        Cp1bexev9W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture 1 triggered by ST0CH0_O output inactive to active transition"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bst0a(&mut self) -> Cp1bst0aW<St0cap1trgSpec> {
        Cp1bst0aW::new(self, 12)
    }
    #[doc = "Bit 13 - Capture 1 triggered by ST0CH0_O output active to inactive transition"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bst0na(&mut self) -> Cp1bst0naW<St0cap1trgSpec> {
        Cp1bst0naW::new(self, 13)
    }
    #[doc = "Bit 14 - Capture 1 triggered by compare 0 event of Slave_TIMER0"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bst0cmp0(&mut self) -> Cp1bst0cmp0W<St0cap1trgSpec> {
        Cp1bst0cmp0W::new(self, 14)
    }
    #[doc = "Bit 15 - Capture 1 triggered by compare 1 event of Slave_TIMER0"]
    #[inline(always)]
    #[must_use]
    pub fn cp1bst0cmp1(&mut self) -> Cp1bst0cmp1W<St0cap1trgSpec> {
        Cp1bst0cmp1W::new(self, 15)
    }
}
#[doc = "SHRTIMER Slave_TIMER0 capture 1 trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0cap1trg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0cap1trg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St0cap1trgSpec;
impl crate::RegisterSpec for St0cap1trgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0cap1trg::R`](R) reader structure"]
impl crate::Readable for St0cap1trgSpec {}
#[doc = "`write(|w| ..)` method takes [`st0cap1trg::W`](W) writer structure"]
impl crate::Writable for St0cap1trgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST0CAP1TRG to value 0"]
impl crate::Resettable for St0cap1trgSpec {
    const RESET_VALUE: u32 = 0;
}
