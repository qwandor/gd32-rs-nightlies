#[doc = "Register `FW` reader"]
pub type R = crate::R<FwSpec>;
#[doc = "Register `FW` writer"]
pub type W = crate::W<FwSpec>;
#[doc = "Filter working\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fw0 {
    #[doc = "0: Filter working disabled"]
    Disabled = 0,
    #[doc = "1: Filter working enabled"]
    Enabled = 1,
}
impl From<Fw0> for bool {
    #[inline(always)]
    fn from(variant: Fw0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FW0` reader - Filter working"]
pub type Fw0R = crate::BitReader<Fw0>;
impl Fw0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fw0 {
        match self.bits {
            false => Fw0::Disabled,
            true => Fw0::Enabled,
        }
    }
    #[doc = "Filter working disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fw0::Disabled
    }
    #[doc = "Filter working enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fw0::Enabled
    }
}
#[doc = "Field `FW0` writer - Filter working"]
pub type Fw0W<'a, REG> = crate::BitWriter<'a, REG, Fw0>;
impl<'a, REG> Fw0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter working disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fw0::Disabled)
    }
    #[doc = "Filter working enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fw0::Enabled)
    }
}
#[doc = "Field `FW1` reader - Filter working"]
pub use Fw0R as Fw1R;
#[doc = "Field `FW2` reader - Filter working"]
pub use Fw0R as Fw2R;
#[doc = "Field `FW3` reader - Filter working"]
pub use Fw0R as Fw3R;
#[doc = "Field `FW4` reader - Filter working"]
pub use Fw0R as Fw4R;
#[doc = "Field `FW5` reader - Filter working"]
pub use Fw0R as Fw5R;
#[doc = "Field `FW6` reader - Filter working"]
pub use Fw0R as Fw6R;
#[doc = "Field `FW7` reader - Filter working"]
pub use Fw0R as Fw7R;
#[doc = "Field `FW8` reader - Filter working"]
pub use Fw0R as Fw8R;
#[doc = "Field `FW9` reader - Filter working"]
pub use Fw0R as Fw9R;
#[doc = "Field `FW10` reader - Filter working"]
pub use Fw0R as Fw10R;
#[doc = "Field `FW11` reader - Filter working"]
pub use Fw0R as Fw11R;
#[doc = "Field `FW12` reader - Filter working"]
pub use Fw0R as Fw12R;
#[doc = "Field `FW13` reader - Filter working"]
pub use Fw0R as Fw13R;
#[doc = "Field `FW14` reader - Filter working"]
pub use Fw0R as Fw14R;
#[doc = "Field `FW15` reader - Filter working"]
pub use Fw0R as Fw15R;
#[doc = "Field `FW16` reader - Filter working"]
pub use Fw0R as Fw16R;
#[doc = "Field `FW17` reader - Filter working"]
pub use Fw0R as Fw17R;
#[doc = "Field `FW18` reader - Filter working"]
pub use Fw0R as Fw18R;
#[doc = "Field `FW19` reader - Filter working"]
pub use Fw0R as Fw19R;
#[doc = "Field `FW20` reader - Filter working"]
pub use Fw0R as Fw20R;
#[doc = "Field `FW21` reader - Filter working"]
pub use Fw0R as Fw21R;
#[doc = "Field `FW22` reader - Filter working"]
pub use Fw0R as Fw22R;
#[doc = "Field `FW23` reader - Filter working"]
pub use Fw0R as Fw23R;
#[doc = "Field `FW24` reader - Filter working"]
pub use Fw0R as Fw24R;
#[doc = "Field `FW25` reader - Filter working"]
pub use Fw0R as Fw25R;
#[doc = "Field `FW26` reader - Filter working"]
pub use Fw0R as Fw26R;
#[doc = "Field `FW27` reader - Filter working"]
pub use Fw0R as Fw27R;
#[doc = "Field `FW1` writer - Filter working"]
pub use Fw0W as Fw1W;
#[doc = "Field `FW2` writer - Filter working"]
pub use Fw0W as Fw2W;
#[doc = "Field `FW3` writer - Filter working"]
pub use Fw0W as Fw3W;
#[doc = "Field `FW4` writer - Filter working"]
pub use Fw0W as Fw4W;
#[doc = "Field `FW5` writer - Filter working"]
pub use Fw0W as Fw5W;
#[doc = "Field `FW6` writer - Filter working"]
pub use Fw0W as Fw6W;
#[doc = "Field `FW7` writer - Filter working"]
pub use Fw0W as Fw7W;
#[doc = "Field `FW8` writer - Filter working"]
pub use Fw0W as Fw8W;
#[doc = "Field `FW9` writer - Filter working"]
pub use Fw0W as Fw9W;
#[doc = "Field `FW10` writer - Filter working"]
pub use Fw0W as Fw10W;
#[doc = "Field `FW11` writer - Filter working"]
pub use Fw0W as Fw11W;
#[doc = "Field `FW12` writer - Filter working"]
pub use Fw0W as Fw12W;
#[doc = "Field `FW13` writer - Filter working"]
pub use Fw0W as Fw13W;
#[doc = "Field `FW14` writer - Filter working"]
pub use Fw0W as Fw14W;
#[doc = "Field `FW15` writer - Filter working"]
pub use Fw0W as Fw15W;
#[doc = "Field `FW16` writer - Filter working"]
pub use Fw0W as Fw16W;
#[doc = "Field `FW17` writer - Filter working"]
pub use Fw0W as Fw17W;
#[doc = "Field `FW18` writer - Filter working"]
pub use Fw0W as Fw18W;
#[doc = "Field `FW19` writer - Filter working"]
pub use Fw0W as Fw19W;
#[doc = "Field `FW20` writer - Filter working"]
pub use Fw0W as Fw20W;
#[doc = "Field `FW21` writer - Filter working"]
pub use Fw0W as Fw21W;
#[doc = "Field `FW22` writer - Filter working"]
pub use Fw0W as Fw22W;
#[doc = "Field `FW23` writer - Filter working"]
pub use Fw0W as Fw23W;
#[doc = "Field `FW24` writer - Filter working"]
pub use Fw0W as Fw24W;
#[doc = "Field `FW25` writer - Filter working"]
pub use Fw0W as Fw25W;
#[doc = "Field `FW26` writer - Filter working"]
pub use Fw0W as Fw26W;
#[doc = "Field `FW27` writer - Filter working"]
pub use Fw0W as Fw27W;
impl R {
    #[doc = "Bit 0 - Filter working"]
    #[inline(always)]
    pub fn fw0(&self) -> Fw0R {
        Fw0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter working"]
    #[inline(always)]
    pub fn fw1(&self) -> Fw1R {
        Fw1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter working"]
    #[inline(always)]
    pub fn fw2(&self) -> Fw2R {
        Fw2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter working"]
    #[inline(always)]
    pub fn fw3(&self) -> Fw3R {
        Fw3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter working"]
    #[inline(always)]
    pub fn fw4(&self) -> Fw4R {
        Fw4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter working"]
    #[inline(always)]
    pub fn fw5(&self) -> Fw5R {
        Fw5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter working"]
    #[inline(always)]
    pub fn fw6(&self) -> Fw6R {
        Fw6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter working"]
    #[inline(always)]
    pub fn fw7(&self) -> Fw7R {
        Fw7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter working"]
    #[inline(always)]
    pub fn fw8(&self) -> Fw8R {
        Fw8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter working"]
    #[inline(always)]
    pub fn fw9(&self) -> Fw9R {
        Fw9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter working"]
    #[inline(always)]
    pub fn fw10(&self) -> Fw10R {
        Fw10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter working"]
    #[inline(always)]
    pub fn fw11(&self) -> Fw11R {
        Fw11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter working"]
    #[inline(always)]
    pub fn fw12(&self) -> Fw12R {
        Fw12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter working"]
    #[inline(always)]
    pub fn fw13(&self) -> Fw13R {
        Fw13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter working"]
    #[inline(always)]
    pub fn fw14(&self) -> Fw14R {
        Fw14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter working"]
    #[inline(always)]
    pub fn fw15(&self) -> Fw15R {
        Fw15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter working"]
    #[inline(always)]
    pub fn fw16(&self) -> Fw16R {
        Fw16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter working"]
    #[inline(always)]
    pub fn fw17(&self) -> Fw17R {
        Fw17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter working"]
    #[inline(always)]
    pub fn fw18(&self) -> Fw18R {
        Fw18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter working"]
    #[inline(always)]
    pub fn fw19(&self) -> Fw19R {
        Fw19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter working"]
    #[inline(always)]
    pub fn fw20(&self) -> Fw20R {
        Fw20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter working"]
    #[inline(always)]
    pub fn fw21(&self) -> Fw21R {
        Fw21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter working"]
    #[inline(always)]
    pub fn fw22(&self) -> Fw22R {
        Fw22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter working"]
    #[inline(always)]
    pub fn fw23(&self) -> Fw23R {
        Fw23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter working"]
    #[inline(always)]
    pub fn fw24(&self) -> Fw24R {
        Fw24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter working"]
    #[inline(always)]
    pub fn fw25(&self) -> Fw25R {
        Fw25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter working"]
    #[inline(always)]
    pub fn fw26(&self) -> Fw26R {
        Fw26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter working"]
    #[inline(always)]
    pub fn fw27(&self) -> Fw27R {
        Fw27R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw0(&mut self) -> Fw0W<FwSpec> {
        Fw0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw1(&mut self) -> Fw1W<FwSpec> {
        Fw1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw2(&mut self) -> Fw2W<FwSpec> {
        Fw2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw3(&mut self) -> Fw3W<FwSpec> {
        Fw3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw4(&mut self) -> Fw4W<FwSpec> {
        Fw4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw5(&mut self) -> Fw5W<FwSpec> {
        Fw5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw6(&mut self) -> Fw6W<FwSpec> {
        Fw6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw7(&mut self) -> Fw7W<FwSpec> {
        Fw7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw8(&mut self) -> Fw8W<FwSpec> {
        Fw8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw9(&mut self) -> Fw9W<FwSpec> {
        Fw9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw10(&mut self) -> Fw10W<FwSpec> {
        Fw10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw11(&mut self) -> Fw11W<FwSpec> {
        Fw11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw12(&mut self) -> Fw12W<FwSpec> {
        Fw12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw13(&mut self) -> Fw13W<FwSpec> {
        Fw13W::new(self, 13)
    }
    #[doc = "Bit 14 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw14(&mut self) -> Fw14W<FwSpec> {
        Fw14W::new(self, 14)
    }
    #[doc = "Bit 15 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw15(&mut self) -> Fw15W<FwSpec> {
        Fw15W::new(self, 15)
    }
    #[doc = "Bit 16 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw16(&mut self) -> Fw16W<FwSpec> {
        Fw16W::new(self, 16)
    }
    #[doc = "Bit 17 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw17(&mut self) -> Fw17W<FwSpec> {
        Fw17W::new(self, 17)
    }
    #[doc = "Bit 18 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw18(&mut self) -> Fw18W<FwSpec> {
        Fw18W::new(self, 18)
    }
    #[doc = "Bit 19 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw19(&mut self) -> Fw19W<FwSpec> {
        Fw19W::new(self, 19)
    }
    #[doc = "Bit 20 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw20(&mut self) -> Fw20W<FwSpec> {
        Fw20W::new(self, 20)
    }
    #[doc = "Bit 21 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw21(&mut self) -> Fw21W<FwSpec> {
        Fw21W::new(self, 21)
    }
    #[doc = "Bit 22 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw22(&mut self) -> Fw22W<FwSpec> {
        Fw22W::new(self, 22)
    }
    #[doc = "Bit 23 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw23(&mut self) -> Fw23W<FwSpec> {
        Fw23W::new(self, 23)
    }
    #[doc = "Bit 24 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw24(&mut self) -> Fw24W<FwSpec> {
        Fw24W::new(self, 24)
    }
    #[doc = "Bit 25 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw25(&mut self) -> Fw25W<FwSpec> {
        Fw25W::new(self, 25)
    }
    #[doc = "Bit 26 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw26(&mut self) -> Fw26W<FwSpec> {
        Fw26W::new(self, 26)
    }
    #[doc = "Bit 27 - Filter working"]
    #[inline(always)]
    #[must_use]
    pub fn fw27(&mut self) -> Fw27W<FwSpec> {
        Fw27W::new(self, 27)
    }
}
#[doc = "Filter working register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwSpec;
impl crate::RegisterSpec for FwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw::R`](R) reader structure"]
impl crate::Readable for FwSpec {}
#[doc = "`write(|w| ..)` method takes [`fw::W`](W) writer structure"]
impl crate::Writable for FwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW to value 0"]
impl crate::Resettable for FwSpec {
    const RESET_VALUE: u32 = 0;
}
