#[doc = "Register `FSCFG` reader"]
pub type R = crate::R<FscfgSpec>;
#[doc = "Register `FSCFG` writer"]
pub type W = crate::W<FscfgSpec>;
#[doc = "Filter scale configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fs0 {
    #[doc = "0: Filter with 16-bit scale"]
    Scale16 = 0,
    #[doc = "1: Filter with 32-bit scale"]
    Scale32 = 1,
}
impl From<Fs0> for bool {
    #[inline(always)]
    fn from(variant: Fs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FS0` reader - Filter scale configuration"]
pub type Fs0R = crate::BitReader<Fs0>;
impl Fs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fs0 {
        match self.bits {
            false => Fs0::Scale16,
            true => Fs0::Scale32,
        }
    }
    #[doc = "Filter with 16-bit scale"]
    #[inline(always)]
    pub fn is_scale16(&self) -> bool {
        *self == Fs0::Scale16
    }
    #[doc = "Filter with 32-bit scale"]
    #[inline(always)]
    pub fn is_scale32(&self) -> bool {
        *self == Fs0::Scale32
    }
}
#[doc = "Field `FS0` writer - Filter scale configuration"]
pub type Fs0W<'a, REG> = crate::BitWriter<'a, REG, Fs0>;
impl<'a, REG> Fs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter with 16-bit scale"]
    #[inline(always)]
    pub fn scale16(self) -> &'a mut crate::W<REG> {
        self.variant(Fs0::Scale16)
    }
    #[doc = "Filter with 32-bit scale"]
    #[inline(always)]
    pub fn scale32(self) -> &'a mut crate::W<REG> {
        self.variant(Fs0::Scale32)
    }
}
#[doc = "Field `FS1` reader - Filter scale configuration"]
pub use Fs0R as Fs1R;
#[doc = "Field `FS2` reader - Filter scale configuration"]
pub use Fs0R as Fs2R;
#[doc = "Field `FS3` reader - Filter scale configuration"]
pub use Fs0R as Fs3R;
#[doc = "Field `FS4` reader - Filter scale configuration"]
pub use Fs0R as Fs4R;
#[doc = "Field `FS5` reader - Filter scale configuration"]
pub use Fs0R as Fs5R;
#[doc = "Field `FS6` reader - Filter scale configuration"]
pub use Fs0R as Fs6R;
#[doc = "Field `FS7` reader - Filter scale configuration"]
pub use Fs0R as Fs7R;
#[doc = "Field `FS8` reader - Filter scale configuration"]
pub use Fs0R as Fs8R;
#[doc = "Field `FS9` reader - Filter scale configuration"]
pub use Fs0R as Fs9R;
#[doc = "Field `FS10` reader - Filter scale configuration"]
pub use Fs0R as Fs10R;
#[doc = "Field `FS11` reader - Filter scale configuration"]
pub use Fs0R as Fs11R;
#[doc = "Field `FS12` reader - Filter scale configuration"]
pub use Fs0R as Fs12R;
#[doc = "Field `FS13` reader - Filter scale configuration"]
pub use Fs0R as Fs13R;
#[doc = "Field `FS14` reader - Filter scale configuration"]
pub use Fs0R as Fs14R;
#[doc = "Field `FS15` reader - Filter scale configuration"]
pub use Fs0R as Fs15R;
#[doc = "Field `FS16` reader - Filter scale configuration"]
pub use Fs0R as Fs16R;
#[doc = "Field `FS17` reader - Filter scale configuration"]
pub use Fs0R as Fs17R;
#[doc = "Field `FS18` reader - Filter scale configuration"]
pub use Fs0R as Fs18R;
#[doc = "Field `FS19` reader - Filter scale configuration"]
pub use Fs0R as Fs19R;
#[doc = "Field `FS20` reader - Filter scale configuration"]
pub use Fs0R as Fs20R;
#[doc = "Field `FS21` reader - Filter scale configuration"]
pub use Fs0R as Fs21R;
#[doc = "Field `FS22` reader - Filter scale configuration"]
pub use Fs0R as Fs22R;
#[doc = "Field `FS23` reader - Filter scale configuration"]
pub use Fs0R as Fs23R;
#[doc = "Field `FS24` reader - Filter scale configuration"]
pub use Fs0R as Fs24R;
#[doc = "Field `FS25` reader - Filter scale configuration"]
pub use Fs0R as Fs25R;
#[doc = "Field `FS26` reader - Filter scale configuration"]
pub use Fs0R as Fs26R;
#[doc = "Field `FS27` reader - Filter scale configuration"]
pub use Fs0R as Fs27R;
#[doc = "Field `FS1` writer - Filter scale configuration"]
pub use Fs0W as Fs1W;
#[doc = "Field `FS2` writer - Filter scale configuration"]
pub use Fs0W as Fs2W;
#[doc = "Field `FS3` writer - Filter scale configuration"]
pub use Fs0W as Fs3W;
#[doc = "Field `FS4` writer - Filter scale configuration"]
pub use Fs0W as Fs4W;
#[doc = "Field `FS5` writer - Filter scale configuration"]
pub use Fs0W as Fs5W;
#[doc = "Field `FS6` writer - Filter scale configuration"]
pub use Fs0W as Fs6W;
#[doc = "Field `FS7` writer - Filter scale configuration"]
pub use Fs0W as Fs7W;
#[doc = "Field `FS8` writer - Filter scale configuration"]
pub use Fs0W as Fs8W;
#[doc = "Field `FS9` writer - Filter scale configuration"]
pub use Fs0W as Fs9W;
#[doc = "Field `FS10` writer - Filter scale configuration"]
pub use Fs0W as Fs10W;
#[doc = "Field `FS11` writer - Filter scale configuration"]
pub use Fs0W as Fs11W;
#[doc = "Field `FS12` writer - Filter scale configuration"]
pub use Fs0W as Fs12W;
#[doc = "Field `FS13` writer - Filter scale configuration"]
pub use Fs0W as Fs13W;
#[doc = "Field `FS14` writer - Filter scale configuration"]
pub use Fs0W as Fs14W;
#[doc = "Field `FS15` writer - Filter scale configuration"]
pub use Fs0W as Fs15W;
#[doc = "Field `FS16` writer - Filter scale configuration"]
pub use Fs0W as Fs16W;
#[doc = "Field `FS17` writer - Filter scale configuration"]
pub use Fs0W as Fs17W;
#[doc = "Field `FS18` writer - Filter scale configuration"]
pub use Fs0W as Fs18W;
#[doc = "Field `FS19` writer - Filter scale configuration"]
pub use Fs0W as Fs19W;
#[doc = "Field `FS20` writer - Filter scale configuration"]
pub use Fs0W as Fs20W;
#[doc = "Field `FS21` writer - Filter scale configuration"]
pub use Fs0W as Fs21W;
#[doc = "Field `FS22` writer - Filter scale configuration"]
pub use Fs0W as Fs22W;
#[doc = "Field `FS23` writer - Filter scale configuration"]
pub use Fs0W as Fs23W;
#[doc = "Field `FS24` writer - Filter scale configuration"]
pub use Fs0W as Fs24W;
#[doc = "Field `FS25` writer - Filter scale configuration"]
pub use Fs0W as Fs25W;
#[doc = "Field `FS26` writer - Filter scale configuration"]
pub use Fs0W as Fs26W;
#[doc = "Field `FS27` writer - Filter scale configuration"]
pub use Fs0W as Fs27W;
impl R {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs0(&self) -> Fs0R {
        Fs0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs1(&self) -> Fs1R {
        Fs1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs2(&self) -> Fs2R {
        Fs2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs3(&self) -> Fs3R {
        Fs3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs4(&self) -> Fs4R {
        Fs4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs5(&self) -> Fs5R {
        Fs5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs6(&self) -> Fs6R {
        Fs6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs7(&self) -> Fs7R {
        Fs7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs8(&self) -> Fs8R {
        Fs8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs9(&self) -> Fs9R {
        Fs9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs10(&self) -> Fs10R {
        Fs10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs11(&self) -> Fs11R {
        Fs11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs12(&self) -> Fs12R {
        Fs12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs13(&self) -> Fs13R {
        Fs13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs14(&self) -> Fs14R {
        Fs14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs15(&self) -> Fs15R {
        Fs15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs16(&self) -> Fs16R {
        Fs16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs17(&self) -> Fs17R {
        Fs17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs18(&self) -> Fs18R {
        Fs18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs19(&self) -> Fs19R {
        Fs19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs20(&self) -> Fs20R {
        Fs20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs21(&self) -> Fs21R {
        Fs21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs22(&self) -> Fs22R {
        Fs22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs23(&self) -> Fs23R {
        Fs23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs24(&self) -> Fs24R {
        Fs24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs25(&self) -> Fs25R {
        Fs25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs26(&self) -> Fs26R {
        Fs26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    pub fn fs27(&self) -> Fs27R {
        Fs27R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs0(&mut self) -> Fs0W<FscfgSpec> {
        Fs0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs1(&mut self) -> Fs1W<FscfgSpec> {
        Fs1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs2(&mut self) -> Fs2W<FscfgSpec> {
        Fs2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs3(&mut self) -> Fs3W<FscfgSpec> {
        Fs3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs4(&mut self) -> Fs4W<FscfgSpec> {
        Fs4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs5(&mut self) -> Fs5W<FscfgSpec> {
        Fs5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs6(&mut self) -> Fs6W<FscfgSpec> {
        Fs6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs7(&mut self) -> Fs7W<FscfgSpec> {
        Fs7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs8(&mut self) -> Fs8W<FscfgSpec> {
        Fs8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs9(&mut self) -> Fs9W<FscfgSpec> {
        Fs9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs10(&mut self) -> Fs10W<FscfgSpec> {
        Fs10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs11(&mut self) -> Fs11W<FscfgSpec> {
        Fs11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs12(&mut self) -> Fs12W<FscfgSpec> {
        Fs12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs13(&mut self) -> Fs13W<FscfgSpec> {
        Fs13W::new(self, 13)
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs14(&mut self) -> Fs14W<FscfgSpec> {
        Fs14W::new(self, 14)
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs15(&mut self) -> Fs15W<FscfgSpec> {
        Fs15W::new(self, 15)
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs16(&mut self) -> Fs16W<FscfgSpec> {
        Fs16W::new(self, 16)
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs17(&mut self) -> Fs17W<FscfgSpec> {
        Fs17W::new(self, 17)
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs18(&mut self) -> Fs18W<FscfgSpec> {
        Fs18W::new(self, 18)
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs19(&mut self) -> Fs19W<FscfgSpec> {
        Fs19W::new(self, 19)
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs20(&mut self) -> Fs20W<FscfgSpec> {
        Fs20W::new(self, 20)
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs21(&mut self) -> Fs21W<FscfgSpec> {
        Fs21W::new(self, 21)
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs22(&mut self) -> Fs22W<FscfgSpec> {
        Fs22W::new(self, 22)
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs23(&mut self) -> Fs23W<FscfgSpec> {
        Fs23W::new(self, 23)
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs24(&mut self) -> Fs24W<FscfgSpec> {
        Fs24W::new(self, 24)
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs25(&mut self) -> Fs25W<FscfgSpec> {
        Fs25W::new(self, 25)
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs26(&mut self) -> Fs26W<FscfgSpec> {
        Fs26W::new(self, 26)
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fs27(&mut self) -> Fs27W<FscfgSpec> {
        Fs27W::new(self, 27)
    }
}
#[doc = "Filter scale configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FscfgSpec;
impl crate::RegisterSpec for FscfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscfg::R`](R) reader structure"]
impl crate::Readable for FscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fscfg::W`](W) writer structure"]
impl crate::Writable for FscfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSCFG to value 0"]
impl crate::Resettable for FscfgSpec {
    const RESET_VALUE: u32 = 0;
}
