#[doc = "Register `FMCFG` reader"]
pub type R = crate::R<FmcfgSpec>;
#[doc = "Register `FMCFG` writer"]
pub type W = crate::W<FmcfgSpec>;
#[doc = "Filter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmod0 {
    #[doc = "0: Mask mode"]
    Mask = 0,
    #[doc = "1: List mode"]
    List = 1,
}
impl From<Fmod0> for bool {
    #[inline(always)]
    fn from(variant: Fmod0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMOD0` reader - Filter mode"]
pub type Fmod0R = crate::BitReader<Fmod0>;
impl Fmod0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmod0 {
        match self.bits {
            false => Fmod0::Mask,
            true => Fmod0::List,
        }
    }
    #[doc = "Mask mode"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Fmod0::Mask
    }
    #[doc = "List mode"]
    #[inline(always)]
    pub fn is_list(&self) -> bool {
        *self == Fmod0::List
    }
}
#[doc = "Field `FMOD0` writer - Filter mode"]
pub type Fmod0W<'a, REG> = crate::BitWriter<'a, REG, Fmod0>;
impl<'a, REG> Fmod0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask mode"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Fmod0::Mask)
    }
    #[doc = "List mode"]
    #[inline(always)]
    pub fn list(self) -> &'a mut crate::W<REG> {
        self.variant(Fmod0::List)
    }
}
#[doc = "Field `FMOD1` reader - Filter mode"]
pub use Fmod0R as Fmod1R;
#[doc = "Field `FMOD2` reader - Filter mode"]
pub use Fmod0R as Fmod2R;
#[doc = "Field `FMOD3` reader - Filter mode"]
pub use Fmod0R as Fmod3R;
#[doc = "Field `FMOD4` reader - Filter mode"]
pub use Fmod0R as Fmod4R;
#[doc = "Field `FMOD5` reader - Filter mode"]
pub use Fmod0R as Fmod5R;
#[doc = "Field `FMOD6` reader - Filter mode"]
pub use Fmod0R as Fmod6R;
#[doc = "Field `FMOD7` reader - Filter mode"]
pub use Fmod0R as Fmod7R;
#[doc = "Field `FMOD8` reader - Filter mode"]
pub use Fmod0R as Fmod8R;
#[doc = "Field `FMOD9` reader - Filter mode"]
pub use Fmod0R as Fmod9R;
#[doc = "Field `FMOD10` reader - Filter mode"]
pub use Fmod0R as Fmod10R;
#[doc = "Field `FMOD11` reader - Filter mode"]
pub use Fmod0R as Fmod11R;
#[doc = "Field `FMOD12` reader - Filter mode"]
pub use Fmod0R as Fmod12R;
#[doc = "Field `FMOD13` reader - Filter mode"]
pub use Fmod0R as Fmod13R;
#[doc = "Field `FMOD14` reader - Filter mode"]
pub use Fmod0R as Fmod14R;
#[doc = "Field `FMOD15` reader - Filter mode"]
pub use Fmod0R as Fmod15R;
#[doc = "Field `FMOD16` reader - Filter mode"]
pub use Fmod0R as Fmod16R;
#[doc = "Field `FMOD17` reader - Filter mode"]
pub use Fmod0R as Fmod17R;
#[doc = "Field `FMOD18` reader - Filter mode"]
pub use Fmod0R as Fmod18R;
#[doc = "Field `FMOD19` reader - Filter mode"]
pub use Fmod0R as Fmod19R;
#[doc = "Field `FMOD20` reader - Filter mode"]
pub use Fmod0R as Fmod20R;
#[doc = "Field `FMOD21` reader - Filter mode"]
pub use Fmod0R as Fmod21R;
#[doc = "Field `FMOD22` reader - Filter mode"]
pub use Fmod0R as Fmod22R;
#[doc = "Field `FMOD23` reader - Filter mode"]
pub use Fmod0R as Fmod23R;
#[doc = "Field `FMOD24` reader - Filter mode"]
pub use Fmod0R as Fmod24R;
#[doc = "Field `FMOD25` reader - Filter mode"]
pub use Fmod0R as Fmod25R;
#[doc = "Field `FMOD26` reader - Filter mode"]
pub use Fmod0R as Fmod26R;
#[doc = "Field `FMOD27` reader - Filter mode"]
pub use Fmod0R as Fmod27R;
#[doc = "Field `FMOD1` writer - Filter mode"]
pub use Fmod0W as Fmod1W;
#[doc = "Field `FMOD2` writer - Filter mode"]
pub use Fmod0W as Fmod2W;
#[doc = "Field `FMOD3` writer - Filter mode"]
pub use Fmod0W as Fmod3W;
#[doc = "Field `FMOD4` writer - Filter mode"]
pub use Fmod0W as Fmod4W;
#[doc = "Field `FMOD5` writer - Filter mode"]
pub use Fmod0W as Fmod5W;
#[doc = "Field `FMOD6` writer - Filter mode"]
pub use Fmod0W as Fmod6W;
#[doc = "Field `FMOD7` writer - Filter mode"]
pub use Fmod0W as Fmod7W;
#[doc = "Field `FMOD8` writer - Filter mode"]
pub use Fmod0W as Fmod8W;
#[doc = "Field `FMOD9` writer - Filter mode"]
pub use Fmod0W as Fmod9W;
#[doc = "Field `FMOD10` writer - Filter mode"]
pub use Fmod0W as Fmod10W;
#[doc = "Field `FMOD11` writer - Filter mode"]
pub use Fmod0W as Fmod11W;
#[doc = "Field `FMOD12` writer - Filter mode"]
pub use Fmod0W as Fmod12W;
#[doc = "Field `FMOD13` writer - Filter mode"]
pub use Fmod0W as Fmod13W;
#[doc = "Field `FMOD14` writer - Filter mode"]
pub use Fmod0W as Fmod14W;
#[doc = "Field `FMOD15` writer - Filter mode"]
pub use Fmod0W as Fmod15W;
#[doc = "Field `FMOD16` writer - Filter mode"]
pub use Fmod0W as Fmod16W;
#[doc = "Field `FMOD17` writer - Filter mode"]
pub use Fmod0W as Fmod17W;
#[doc = "Field `FMOD18` writer - Filter mode"]
pub use Fmod0W as Fmod18W;
#[doc = "Field `FMOD19` writer - Filter mode"]
pub use Fmod0W as Fmod19W;
#[doc = "Field `FMOD20` writer - Filter mode"]
pub use Fmod0W as Fmod20W;
#[doc = "Field `FMOD21` writer - Filter mode"]
pub use Fmod0W as Fmod21W;
#[doc = "Field `FMOD22` writer - Filter mode"]
pub use Fmod0W as Fmod22W;
#[doc = "Field `FMOD23` writer - Filter mode"]
pub use Fmod0W as Fmod23W;
#[doc = "Field `FMOD24` writer - Filter mode"]
pub use Fmod0W as Fmod24W;
#[doc = "Field `FMOD25` writer - Filter mode"]
pub use Fmod0W as Fmod25W;
#[doc = "Field `FMOD26` writer - Filter mode"]
pub use Fmod0W as Fmod26W;
#[doc = "Field `FMOD27` writer - Filter mode"]
pub use Fmod0W as Fmod27W;
impl R {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fmod0(&self) -> Fmod0R {
        Fmod0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fmod1(&self) -> Fmod1R {
        Fmod1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fmod2(&self) -> Fmod2R {
        Fmod2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fmod3(&self) -> Fmod3R {
        Fmod3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fmod4(&self) -> Fmod4R {
        Fmod4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fmod5(&self) -> Fmod5R {
        Fmod5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fmod6(&self) -> Fmod6R {
        Fmod6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fmod7(&self) -> Fmod7R {
        Fmod7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fmod8(&self) -> Fmod8R {
        Fmod8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fmod9(&self) -> Fmod9R {
        Fmod9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fmod10(&self) -> Fmod10R {
        Fmod10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fmod11(&self) -> Fmod11R {
        Fmod11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fmod12(&self) -> Fmod12R {
        Fmod12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fmod13(&self) -> Fmod13R {
        Fmod13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    pub fn fmod14(&self) -> Fmod14R {
        Fmod14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    pub fn fmod15(&self) -> Fmod15R {
        Fmod15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    pub fn fmod16(&self) -> Fmod16R {
        Fmod16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    pub fn fmod17(&self) -> Fmod17R {
        Fmod17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    pub fn fmod18(&self) -> Fmod18R {
        Fmod18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    pub fn fmod19(&self) -> Fmod19R {
        Fmod19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    pub fn fmod20(&self) -> Fmod20R {
        Fmod20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    pub fn fmod21(&self) -> Fmod21R {
        Fmod21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    pub fn fmod22(&self) -> Fmod22R {
        Fmod22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    pub fn fmod23(&self) -> Fmod23R {
        Fmod23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    pub fn fmod24(&self) -> Fmod24R {
        Fmod24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    pub fn fmod25(&self) -> Fmod25R {
        Fmod25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    pub fn fmod26(&self) -> Fmod26R {
        Fmod26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    pub fn fmod27(&self) -> Fmod27R {
        Fmod27R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod0(&mut self) -> Fmod0W<FmcfgSpec> {
        Fmod0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod1(&mut self) -> Fmod1W<FmcfgSpec> {
        Fmod1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod2(&mut self) -> Fmod2W<FmcfgSpec> {
        Fmod2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod3(&mut self) -> Fmod3W<FmcfgSpec> {
        Fmod3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod4(&mut self) -> Fmod4W<FmcfgSpec> {
        Fmod4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod5(&mut self) -> Fmod5W<FmcfgSpec> {
        Fmod5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod6(&mut self) -> Fmod6W<FmcfgSpec> {
        Fmod6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod7(&mut self) -> Fmod7W<FmcfgSpec> {
        Fmod7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod8(&mut self) -> Fmod8W<FmcfgSpec> {
        Fmod8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod9(&mut self) -> Fmod9W<FmcfgSpec> {
        Fmod9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod10(&mut self) -> Fmod10W<FmcfgSpec> {
        Fmod10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod11(&mut self) -> Fmod11W<FmcfgSpec> {
        Fmod11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod12(&mut self) -> Fmod12W<FmcfgSpec> {
        Fmod12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod13(&mut self) -> Fmod13W<FmcfgSpec> {
        Fmod13W::new(self, 13)
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod14(&mut self) -> Fmod14W<FmcfgSpec> {
        Fmod14W::new(self, 14)
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod15(&mut self) -> Fmod15W<FmcfgSpec> {
        Fmod15W::new(self, 15)
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod16(&mut self) -> Fmod16W<FmcfgSpec> {
        Fmod16W::new(self, 16)
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod17(&mut self) -> Fmod17W<FmcfgSpec> {
        Fmod17W::new(self, 17)
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod18(&mut self) -> Fmod18W<FmcfgSpec> {
        Fmod18W::new(self, 18)
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod19(&mut self) -> Fmod19W<FmcfgSpec> {
        Fmod19W::new(self, 19)
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod20(&mut self) -> Fmod20W<FmcfgSpec> {
        Fmod20W::new(self, 20)
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod21(&mut self) -> Fmod21W<FmcfgSpec> {
        Fmod21W::new(self, 21)
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod22(&mut self) -> Fmod22W<FmcfgSpec> {
        Fmod22W::new(self, 22)
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod23(&mut self) -> Fmod23W<FmcfgSpec> {
        Fmod23W::new(self, 23)
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod24(&mut self) -> Fmod24W<FmcfgSpec> {
        Fmod24W::new(self, 24)
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod25(&mut self) -> Fmod25W<FmcfgSpec> {
        Fmod25W::new(self, 25)
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod26(&mut self) -> Fmod26W<FmcfgSpec> {
        Fmod26W::new(self, 26)
    }
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod27(&mut self) -> Fmod27W<FmcfgSpec> {
        Fmod27W::new(self, 27)
    }
}
#[doc = "Filter mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmcfgSpec;
impl crate::RegisterSpec for FmcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmcfg::R`](R) reader structure"]
impl crate::Readable for FmcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fmcfg::W`](W) writer structure"]
impl crate::Writable for FmcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMCFG to value 0"]
impl crate::Resettable for FmcfgSpec {
    const RESET_VALUE: u32 = 0;
}
