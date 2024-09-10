#[doc = "Register `OMODE` reader"]
pub type R = crate::R<OmodeSpec>;
#[doc = "Register `OMODE` writer"]
pub type W = crate::W<OmodeSpec>;
#[doc = "Pin 0 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Om0 {
    #[doc = "0: Output push-pull (reset state)"]
    PushPull = 0,
    #[doc = "1: Output open-drain"]
    OpenDrain = 1,
}
impl From<Om0> for bool {
    #[inline(always)]
    fn from(variant: Om0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OM0` reader - Pin 0 output mode"]
pub type Om0R = crate::BitReader<Om0>;
impl Om0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Om0 {
        match self.bits {
            false => Om0::PushPull,
            true => Om0::OpenDrain,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == Om0::PushPull
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Om0::OpenDrain
    }
}
#[doc = "Field `OM0` writer - Pin 0 output mode"]
pub type Om0W<'a, REG> = crate::BitWriter<'a, REG, Om0>;
impl<'a, REG> Om0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(Om0::PushPull)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Om0::OpenDrain)
    }
}
#[doc = "Field `OM1` reader - Pin 1 output mode"]
pub use Om0R as Om1R;
#[doc = "Field `OM2` reader - Pin 2 output mode"]
pub use Om0R as Om2R;
#[doc = "Field `OM3` reader - Pin 3 output mode"]
pub use Om0R as Om3R;
#[doc = "Field `OM4` reader - Pin 4 output mode"]
pub use Om0R as Om4R;
#[doc = "Field `OM5` reader - Pin 5 output mode"]
pub use Om0R as Om5R;
#[doc = "Field `OM6` reader - Pin 6 output mode"]
pub use Om0R as Om6R;
#[doc = "Field `OM7` reader - Pin 7 output mode"]
pub use Om0R as Om7R;
#[doc = "Field `OM8` reader - Pin 8 output mode"]
pub use Om0R as Om8R;
#[doc = "Field `OM9` reader - Pin 9 output mode"]
pub use Om0R as Om9R;
#[doc = "Field `OM10` reader - Pin 10 output mode"]
pub use Om0R as Om10R;
#[doc = "Field `OM11` reader - Pin 11 output mode"]
pub use Om0R as Om11R;
#[doc = "Field `OM12` reader - Pin 12 output mode"]
pub use Om0R as Om12R;
#[doc = "Field `OM13` reader - Pin 13 output mode"]
pub use Om0R as Om13R;
#[doc = "Field `OM14` reader - Pin 14 output mode"]
pub use Om0R as Om14R;
#[doc = "Field `OM15` reader - Pin 15 output mode"]
pub use Om0R as Om15R;
#[doc = "Field `OM1` writer - Pin 1 output mode"]
pub use Om0W as Om1W;
#[doc = "Field `OM2` writer - Pin 2 output mode"]
pub use Om0W as Om2W;
#[doc = "Field `OM3` writer - Pin 3 output mode"]
pub use Om0W as Om3W;
#[doc = "Field `OM4` writer - Pin 4 output mode"]
pub use Om0W as Om4W;
#[doc = "Field `OM5` writer - Pin 5 output mode"]
pub use Om0W as Om5W;
#[doc = "Field `OM6` writer - Pin 6 output mode"]
pub use Om0W as Om6W;
#[doc = "Field `OM7` writer - Pin 7 output mode"]
pub use Om0W as Om7W;
#[doc = "Field `OM8` writer - Pin 8 output mode"]
pub use Om0W as Om8W;
#[doc = "Field `OM9` writer - Pin 9 output mode"]
pub use Om0W as Om9W;
#[doc = "Field `OM10` writer - Pin 10 output mode"]
pub use Om0W as Om10W;
#[doc = "Field `OM11` writer - Pin 11 output mode"]
pub use Om0W as Om11W;
#[doc = "Field `OM12` writer - Pin 12 output mode"]
pub use Om0W as Om12W;
#[doc = "Field `OM13` writer - Pin 13 output mode"]
pub use Om0W as Om13W;
#[doc = "Field `OM14` writer - Pin 14 output mode"]
pub use Om0W as Om14W;
#[doc = "Field `OM15` writer - Pin 15 output mode"]
pub use Om0W as Om15W;
impl R {
    #[doc = "Bit 0 - Pin 0 output mode"]
    #[inline(always)]
    pub fn om0(&self) -> Om0R {
        Om0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1 output mode"]
    #[inline(always)]
    pub fn om1(&self) -> Om1R {
        Om1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2 output mode"]
    #[inline(always)]
    pub fn om2(&self) -> Om2R {
        Om2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3 output mode"]
    #[inline(always)]
    pub fn om3(&self) -> Om3R {
        Om3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4 output mode"]
    #[inline(always)]
    pub fn om4(&self) -> Om4R {
        Om4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5 output mode"]
    #[inline(always)]
    pub fn om5(&self) -> Om5R {
        Om5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6 output mode"]
    #[inline(always)]
    pub fn om6(&self) -> Om6R {
        Om6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7 output mode"]
    #[inline(always)]
    pub fn om7(&self) -> Om7R {
        Om7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8 output mode"]
    #[inline(always)]
    pub fn om8(&self) -> Om8R {
        Om8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9 output mode"]
    #[inline(always)]
    pub fn om9(&self) -> Om9R {
        Om9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10 output mode"]
    #[inline(always)]
    pub fn om10(&self) -> Om10R {
        Om10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11 output mode"]
    #[inline(always)]
    pub fn om11(&self) -> Om11R {
        Om11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12 output mode"]
    #[inline(always)]
    pub fn om12(&self) -> Om12R {
        Om12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13 output mode"]
    #[inline(always)]
    pub fn om13(&self) -> Om13R {
        Om13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14 output mode"]
    #[inline(always)]
    pub fn om14(&self) -> Om14R {
        Om14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15 output mode"]
    #[inline(always)]
    pub fn om15(&self) -> Om15R {
        Om15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin 0 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om0(&mut self) -> Om0W<OmodeSpec> {
        Om0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin 1 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om1(&mut self) -> Om1W<OmodeSpec> {
        Om1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin 2 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om2(&mut self) -> Om2W<OmodeSpec> {
        Om2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin 3 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om3(&mut self) -> Om3W<OmodeSpec> {
        Om3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin 4 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om4(&mut self) -> Om4W<OmodeSpec> {
        Om4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin 5 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om5(&mut self) -> Om5W<OmodeSpec> {
        Om5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin 6 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om6(&mut self) -> Om6W<OmodeSpec> {
        Om6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin 7 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om7(&mut self) -> Om7W<OmodeSpec> {
        Om7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin 8 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om8(&mut self) -> Om8W<OmodeSpec> {
        Om8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin 9 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om9(&mut self) -> Om9W<OmodeSpec> {
        Om9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin 10 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om10(&mut self) -> Om10W<OmodeSpec> {
        Om10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin 11 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om11(&mut self) -> Om11W<OmodeSpec> {
        Om11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin 12 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om12(&mut self) -> Om12W<OmodeSpec> {
        Om12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin 13 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om13(&mut self) -> Om13W<OmodeSpec> {
        Om13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin 14 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om14(&mut self) -> Om14W<OmodeSpec> {
        Om14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin 15 output mode"]
    #[inline(always)]
    #[must_use]
    pub fn om15(&mut self) -> Om15W<OmodeSpec> {
        Om15W::new(self, 15)
    }
}
#[doc = "GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OmodeSpec;
impl crate::RegisterSpec for OmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`omode::R`](R) reader structure"]
impl crate::Readable for OmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`omode::W`](W) writer structure"]
impl crate::Writable for OmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OMODE to value 0"]
impl crate::Resettable for OmodeSpec {
    const RESET_VALUE: u32 = 0;
}
