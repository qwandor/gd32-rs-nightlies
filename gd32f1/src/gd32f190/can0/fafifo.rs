#[doc = "Register `FAFIFO` reader"]
pub type R = crate::R<FafifoSpec>;
#[doc = "Register `FAFIFO` writer"]
pub type W = crate::W<FafifoSpec>;
#[doc = "Filter 0 associated FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Faf0 {
    #[doc = "0: Filter associated with FIFO0"]
    Fifo0 = 0,
    #[doc = "1: Filter associated with FIFO1"]
    Fifo1 = 1,
}
impl From<Faf0> for bool {
    #[inline(always)]
    fn from(variant: Faf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAF0` reader - Filter 0 associated FIFO"]
pub type Faf0R = crate::BitReader<Faf0>;
impl Faf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Faf0 {
        match self.bits {
            false => Faf0::Fifo0,
            true => Faf0::Fifo1,
        }
    }
    #[doc = "Filter associated with FIFO0"]
    #[inline(always)]
    pub fn is_fifo0(&self) -> bool {
        *self == Faf0::Fifo0
    }
    #[doc = "Filter associated with FIFO1"]
    #[inline(always)]
    pub fn is_fifo1(&self) -> bool {
        *self == Faf0::Fifo1
    }
}
#[doc = "Field `FAF0` writer - Filter 0 associated FIFO"]
pub type Faf0W<'a, REG> = crate::BitWriter<'a, REG, Faf0>;
impl<'a, REG> Faf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter associated with FIFO0"]
    #[inline(always)]
    pub fn fifo0(self) -> &'a mut crate::W<REG> {
        self.variant(Faf0::Fifo0)
    }
    #[doc = "Filter associated with FIFO1"]
    #[inline(always)]
    pub fn fifo1(self) -> &'a mut crate::W<REG> {
        self.variant(Faf0::Fifo1)
    }
}
#[doc = "Field `FAF1` reader - Filter 1 associated FIFO"]
pub use Faf0R as Faf1R;
#[doc = "Field `FAF2` reader - Filter 2 associated FIFO"]
pub use Faf0R as Faf2R;
#[doc = "Field `FAF3` reader - Filter 3 associated FIFO"]
pub use Faf0R as Faf3R;
#[doc = "Field `FAF4` reader - Filter 4 associated FIFO"]
pub use Faf0R as Faf4R;
#[doc = "Field `FAF5` reader - Filter 5 associated FIFO"]
pub use Faf0R as Faf5R;
#[doc = "Field `FAF6` reader - Filter 6 associated FIFO"]
pub use Faf0R as Faf6R;
#[doc = "Field `FAF7` reader - Filter 7 associated FIFO"]
pub use Faf0R as Faf7R;
#[doc = "Field `FAF8` reader - Filter 8 associated FIFO"]
pub use Faf0R as Faf8R;
#[doc = "Field `FAF9` reader - Filter 9 associated FIFO"]
pub use Faf0R as Faf9R;
#[doc = "Field `FAF10` reader - Filter 10 associated FIFO"]
pub use Faf0R as Faf10R;
#[doc = "Field `FAF11` reader - Filter 11 associated FIFO"]
pub use Faf0R as Faf11R;
#[doc = "Field `FAF12` reader - Filter 12 associated FIFO"]
pub use Faf0R as Faf12R;
#[doc = "Field `FAF13` reader - Filter 13 associated FIFO"]
pub use Faf0R as Faf13R;
#[doc = "Field `FAF14` reader - Filter 14 associated FIFO"]
pub use Faf0R as Faf14R;
#[doc = "Field `FAF15` reader - Filter 15 associated FIFO"]
pub use Faf0R as Faf15R;
#[doc = "Field `FAF16` reader - Filter 16 associated FIFO"]
pub use Faf0R as Faf16R;
#[doc = "Field `FAF17` reader - Filter 17 associated FIFO"]
pub use Faf0R as Faf17R;
#[doc = "Field `FAF18` reader - Filter 18 associated FIFO"]
pub use Faf0R as Faf18R;
#[doc = "Field `FAF19` reader - Filter 19 associated FIFO"]
pub use Faf0R as Faf19R;
#[doc = "Field `FAF20` reader - Filter 20 associated FIFO"]
pub use Faf0R as Faf20R;
#[doc = "Field `FAF21` reader - Filter 21 associated FIFO"]
pub use Faf0R as Faf21R;
#[doc = "Field `FAF22` reader - Filter 22 associated FIFO"]
pub use Faf0R as Faf22R;
#[doc = "Field `FAF23` reader - Filter 23 associated FIFO"]
pub use Faf0R as Faf23R;
#[doc = "Field `FAF24` reader - Filter 24 associated FIFO"]
pub use Faf0R as Faf24R;
#[doc = "Field `FAF25` reader - Filter 25 associated FIFO"]
pub use Faf0R as Faf25R;
#[doc = "Field `FAF26` reader - Filter 26 associated FIFO"]
pub use Faf0R as Faf26R;
#[doc = "Field `FAF27` reader - Filter 27 associated FIFO"]
pub use Faf0R as Faf27R;
#[doc = "Field `FAF1` writer - Filter 1 associated FIFO"]
pub use Faf0W as Faf1W;
#[doc = "Field `FAF2` writer - Filter 2 associated FIFO"]
pub use Faf0W as Faf2W;
#[doc = "Field `FAF3` writer - Filter 3 associated FIFO"]
pub use Faf0W as Faf3W;
#[doc = "Field `FAF4` writer - Filter 4 associated FIFO"]
pub use Faf0W as Faf4W;
#[doc = "Field `FAF5` writer - Filter 5 associated FIFO"]
pub use Faf0W as Faf5W;
#[doc = "Field `FAF6` writer - Filter 6 associated FIFO"]
pub use Faf0W as Faf6W;
#[doc = "Field `FAF7` writer - Filter 7 associated FIFO"]
pub use Faf0W as Faf7W;
#[doc = "Field `FAF8` writer - Filter 8 associated FIFO"]
pub use Faf0W as Faf8W;
#[doc = "Field `FAF9` writer - Filter 9 associated FIFO"]
pub use Faf0W as Faf9W;
#[doc = "Field `FAF10` writer - Filter 10 associated FIFO"]
pub use Faf0W as Faf10W;
#[doc = "Field `FAF11` writer - Filter 11 associated FIFO"]
pub use Faf0W as Faf11W;
#[doc = "Field `FAF12` writer - Filter 12 associated FIFO"]
pub use Faf0W as Faf12W;
#[doc = "Field `FAF13` writer - Filter 13 associated FIFO"]
pub use Faf0W as Faf13W;
#[doc = "Field `FAF14` writer - Filter 14 associated FIFO"]
pub use Faf0W as Faf14W;
#[doc = "Field `FAF15` writer - Filter 15 associated FIFO"]
pub use Faf0W as Faf15W;
#[doc = "Field `FAF16` writer - Filter 16 associated FIFO"]
pub use Faf0W as Faf16W;
#[doc = "Field `FAF17` writer - Filter 17 associated FIFO"]
pub use Faf0W as Faf17W;
#[doc = "Field `FAF18` writer - Filter 18 associated FIFO"]
pub use Faf0W as Faf18W;
#[doc = "Field `FAF19` writer - Filter 19 associated FIFO"]
pub use Faf0W as Faf19W;
#[doc = "Field `FAF20` writer - Filter 20 associated FIFO"]
pub use Faf0W as Faf20W;
#[doc = "Field `FAF21` writer - Filter 21 associated FIFO"]
pub use Faf0W as Faf21W;
#[doc = "Field `FAF22` writer - Filter 22 associated FIFO"]
pub use Faf0W as Faf22W;
#[doc = "Field `FAF23` writer - Filter 23 associated FIFO"]
pub use Faf0W as Faf23W;
#[doc = "Field `FAF24` writer - Filter 24 associated FIFO"]
pub use Faf0W as Faf24W;
#[doc = "Field `FAF25` writer - Filter 25 associated FIFO"]
pub use Faf0W as Faf25W;
#[doc = "Field `FAF26` writer - Filter 26 associated FIFO"]
pub use Faf0W as Faf26W;
#[doc = "Field `FAF27` writer - Filter 27 associated FIFO"]
pub use Faf0W as Faf27W;
impl R {
    #[doc = "Bit 0 - Filter 0 associated FIFO"]
    #[inline(always)]
    pub fn faf0(&self) -> Faf0R {
        Faf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter 1 associated FIFO"]
    #[inline(always)]
    pub fn faf1(&self) -> Faf1R {
        Faf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter 2 associated FIFO"]
    #[inline(always)]
    pub fn faf2(&self) -> Faf2R {
        Faf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter 3 associated FIFO"]
    #[inline(always)]
    pub fn faf3(&self) -> Faf3R {
        Faf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter 4 associated FIFO"]
    #[inline(always)]
    pub fn faf4(&self) -> Faf4R {
        Faf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter 5 associated FIFO"]
    #[inline(always)]
    pub fn faf5(&self) -> Faf5R {
        Faf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter 6 associated FIFO"]
    #[inline(always)]
    pub fn faf6(&self) -> Faf6R {
        Faf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter 7 associated FIFO"]
    #[inline(always)]
    pub fn faf7(&self) -> Faf7R {
        Faf7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter 8 associated FIFO"]
    #[inline(always)]
    pub fn faf8(&self) -> Faf8R {
        Faf8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter 9 associated FIFO"]
    #[inline(always)]
    pub fn faf9(&self) -> Faf9R {
        Faf9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter 10 associated FIFO"]
    #[inline(always)]
    pub fn faf10(&self) -> Faf10R {
        Faf10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter 11 associated FIFO"]
    #[inline(always)]
    pub fn faf11(&self) -> Faf11R {
        Faf11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter 12 associated FIFO"]
    #[inline(always)]
    pub fn faf12(&self) -> Faf12R {
        Faf12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter 13 associated FIFO"]
    #[inline(always)]
    pub fn faf13(&self) -> Faf13R {
        Faf13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter 14 associated FIFO"]
    #[inline(always)]
    pub fn faf14(&self) -> Faf14R {
        Faf14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter 15 associated FIFO"]
    #[inline(always)]
    pub fn faf15(&self) -> Faf15R {
        Faf15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter 16 associated FIFO"]
    #[inline(always)]
    pub fn faf16(&self) -> Faf16R {
        Faf16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter 17 associated FIFO"]
    #[inline(always)]
    pub fn faf17(&self) -> Faf17R {
        Faf17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter 18 associated FIFO"]
    #[inline(always)]
    pub fn faf18(&self) -> Faf18R {
        Faf18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter 19 associated FIFO"]
    #[inline(always)]
    pub fn faf19(&self) -> Faf19R {
        Faf19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter 20 associated FIFO"]
    #[inline(always)]
    pub fn faf20(&self) -> Faf20R {
        Faf20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter 21 associated FIFO"]
    #[inline(always)]
    pub fn faf21(&self) -> Faf21R {
        Faf21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter 22 associated FIFO"]
    #[inline(always)]
    pub fn faf22(&self) -> Faf22R {
        Faf22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter 23 associated FIFO"]
    #[inline(always)]
    pub fn faf23(&self) -> Faf23R {
        Faf23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter 24 associated FIFO"]
    #[inline(always)]
    pub fn faf24(&self) -> Faf24R {
        Faf24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter 25 associated FIFO"]
    #[inline(always)]
    pub fn faf25(&self) -> Faf25R {
        Faf25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter 26 associated FIFO"]
    #[inline(always)]
    pub fn faf26(&self) -> Faf26R {
        Faf26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter 27 associated FIFO"]
    #[inline(always)]
    pub fn faf27(&self) -> Faf27R {
        Faf27R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter 0 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf0(&mut self) -> Faf0W<FafifoSpec> {
        Faf0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter 1 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf1(&mut self) -> Faf1W<FafifoSpec> {
        Faf1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter 2 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf2(&mut self) -> Faf2W<FafifoSpec> {
        Faf2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter 3 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf3(&mut self) -> Faf3W<FafifoSpec> {
        Faf3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter 4 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf4(&mut self) -> Faf4W<FafifoSpec> {
        Faf4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter 5 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf5(&mut self) -> Faf5W<FafifoSpec> {
        Faf5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter 6 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf6(&mut self) -> Faf6W<FafifoSpec> {
        Faf6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter 7 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf7(&mut self) -> Faf7W<FafifoSpec> {
        Faf7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter 8 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf8(&mut self) -> Faf8W<FafifoSpec> {
        Faf8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter 9 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf9(&mut self) -> Faf9W<FafifoSpec> {
        Faf9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter 10 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf10(&mut self) -> Faf10W<FafifoSpec> {
        Faf10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter 11 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf11(&mut self) -> Faf11W<FafifoSpec> {
        Faf11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter 12 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf12(&mut self) -> Faf12W<FafifoSpec> {
        Faf12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter 13 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf13(&mut self) -> Faf13W<FafifoSpec> {
        Faf13W::new(self, 13)
    }
    #[doc = "Bit 14 - Filter 14 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf14(&mut self) -> Faf14W<FafifoSpec> {
        Faf14W::new(self, 14)
    }
    #[doc = "Bit 15 - Filter 15 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf15(&mut self) -> Faf15W<FafifoSpec> {
        Faf15W::new(self, 15)
    }
    #[doc = "Bit 16 - Filter 16 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf16(&mut self) -> Faf16W<FafifoSpec> {
        Faf16W::new(self, 16)
    }
    #[doc = "Bit 17 - Filter 17 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf17(&mut self) -> Faf17W<FafifoSpec> {
        Faf17W::new(self, 17)
    }
    #[doc = "Bit 18 - Filter 18 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf18(&mut self) -> Faf18W<FafifoSpec> {
        Faf18W::new(self, 18)
    }
    #[doc = "Bit 19 - Filter 19 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf19(&mut self) -> Faf19W<FafifoSpec> {
        Faf19W::new(self, 19)
    }
    #[doc = "Bit 20 - Filter 20 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf20(&mut self) -> Faf20W<FafifoSpec> {
        Faf20W::new(self, 20)
    }
    #[doc = "Bit 21 - Filter 21 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf21(&mut self) -> Faf21W<FafifoSpec> {
        Faf21W::new(self, 21)
    }
    #[doc = "Bit 22 - Filter 22 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf22(&mut self) -> Faf22W<FafifoSpec> {
        Faf22W::new(self, 22)
    }
    #[doc = "Bit 23 - Filter 23 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf23(&mut self) -> Faf23W<FafifoSpec> {
        Faf23W::new(self, 23)
    }
    #[doc = "Bit 24 - Filter 24 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf24(&mut self) -> Faf24W<FafifoSpec> {
        Faf24W::new(self, 24)
    }
    #[doc = "Bit 25 - Filter 25 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf25(&mut self) -> Faf25W<FafifoSpec> {
        Faf25W::new(self, 25)
    }
    #[doc = "Bit 26 - Filter 26 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf26(&mut self) -> Faf26W<FafifoSpec> {
        Faf26W::new(self, 26)
    }
    #[doc = "Bit 27 - Filter 27 associated FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn faf27(&mut self) -> Faf27W<FafifoSpec> {
        Faf27W::new(self, 27)
    }
}
#[doc = "Filter associated FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fafifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fafifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FafifoSpec;
impl crate::RegisterSpec for FafifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fafifo::R`](R) reader structure"]
impl crate::Readable for FafifoSpec {}
#[doc = "`write(|w| ..)` method takes [`fafifo::W`](W) writer structure"]
impl crate::Writable for FafifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAFIFO to value 0"]
impl crate::Resettable for FafifoSpec {
    const RESET_VALUE: u32 = 0;
}
