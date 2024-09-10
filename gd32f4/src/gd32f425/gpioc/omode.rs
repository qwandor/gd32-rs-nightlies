#[doc = "Register `OMODE` reader"]
pub type R = crate::R<OmodeSpec>;
#[doc = "Register `OMODE` writer"]
pub type W = crate::W<OmodeSpec>;
#[doc = "Field `OM0` reader - Port 0 output mode bit"]
pub type Om0R = crate::BitReader;
#[doc = "Field `OM0` writer - Port 0 output mode bit"]
pub type Om0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM1` reader - Port 1 output mode bit"]
pub type Om1R = crate::BitReader;
#[doc = "Field `OM1` writer - Port 1 output mode bit"]
pub type Om1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM2` reader - Port 2 output mode bit"]
pub type Om2R = crate::BitReader;
#[doc = "Field `OM2` writer - Port 2 output mode bit"]
pub type Om2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM3` reader - Port 3 output mode bit"]
pub type Om3R = crate::BitReader;
#[doc = "Field `OM3` writer - Port 3 output mode bit"]
pub type Om3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM4` reader - Port 4 output mode bit"]
pub type Om4R = crate::BitReader;
#[doc = "Field `OM4` writer - Port 4 output mode bit"]
pub type Om4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM5` reader - Port 5 output mode bit"]
pub type Om5R = crate::BitReader;
#[doc = "Field `OM5` writer - Port 5 output mode bit"]
pub type Om5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM6` reader - Port 6 output mode bit"]
pub type Om6R = crate::BitReader;
#[doc = "Field `OM6` writer - Port 6 output mode bit"]
pub type Om6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM7` reader - Port 7 output mode bit"]
pub type Om7R = crate::BitReader;
#[doc = "Field `OM7` writer - Port 7 output mode bit"]
pub type Om7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM8` reader - Port 8 output mode bit"]
pub type Om8R = crate::BitReader;
#[doc = "Field `OM8` writer - Port 8 output mode bit"]
pub type Om8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM9` reader - Port 9 output mode bit"]
pub type Om9R = crate::BitReader;
#[doc = "Field `OM9` writer - Port 9 output mode bit"]
pub type Om9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM10` reader - Port 10 output mode bit"]
pub type Om10R = crate::BitReader;
#[doc = "Field `OM10` writer - Port 10 output mode bit"]
pub type Om10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM11` reader - Port 11 output mode bit"]
pub type Om11R = crate::BitReader;
#[doc = "Field `OM11` writer - Port 11 output mode bit"]
pub type Om11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM12` reader - Port 12 output mode bit"]
pub type Om12R = crate::BitReader;
#[doc = "Field `OM12` writer - Port 12 output mode bit"]
pub type Om12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM13` reader - Port 13 output mode bit"]
pub type Om13R = crate::BitReader;
#[doc = "Field `OM13` writer - Port 13 output mode bit"]
pub type Om13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM14` reader - Port 14 output mode bit"]
pub type Om14R = crate::BitReader;
#[doc = "Field `OM14` writer - Port 14 output mode bit"]
pub type Om14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OM15` reader - Port 15 output mode bit"]
pub type Om15R = crate::BitReader;
#[doc = "Field `OM15` writer - Port 15 output mode bit"]
pub type Om15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port 0 output mode bit"]
    #[inline(always)]
    pub fn om0(&self) -> Om0R {
        Om0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 1 output mode bit"]
    #[inline(always)]
    pub fn om1(&self) -> Om1R {
        Om1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 2 output mode bit"]
    #[inline(always)]
    pub fn om2(&self) -> Om2R {
        Om2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 3 output mode bit"]
    #[inline(always)]
    pub fn om3(&self) -> Om3R {
        Om3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 4 output mode bit"]
    #[inline(always)]
    pub fn om4(&self) -> Om4R {
        Om4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 5 output mode bit"]
    #[inline(always)]
    pub fn om5(&self) -> Om5R {
        Om5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 6 output mode bit"]
    #[inline(always)]
    pub fn om6(&self) -> Om6R {
        Om6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 7 output mode bit"]
    #[inline(always)]
    pub fn om7(&self) -> Om7R {
        Om7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 8 output mode bit"]
    #[inline(always)]
    pub fn om8(&self) -> Om8R {
        Om8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 9 output mode bit"]
    #[inline(always)]
    pub fn om9(&self) -> Om9R {
        Om9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 10 output mode bit"]
    #[inline(always)]
    pub fn om10(&self) -> Om10R {
        Om10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 11 output mode bit"]
    #[inline(always)]
    pub fn om11(&self) -> Om11R {
        Om11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port 12 output mode bit"]
    #[inline(always)]
    pub fn om12(&self) -> Om12R {
        Om12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port 13 output mode bit"]
    #[inline(always)]
    pub fn om13(&self) -> Om13R {
        Om13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port 14 output mode bit"]
    #[inline(always)]
    pub fn om14(&self) -> Om14R {
        Om14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port 15 output mode bit"]
    #[inline(always)]
    pub fn om15(&self) -> Om15R {
        Om15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 0 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om0(&mut self) -> Om0W<OmodeSpec> {
        Om0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port 1 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om1(&mut self) -> Om1W<OmodeSpec> {
        Om1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port 2 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om2(&mut self) -> Om2W<OmodeSpec> {
        Om2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port 3 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om3(&mut self) -> Om3W<OmodeSpec> {
        Om3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port 4 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om4(&mut self) -> Om4W<OmodeSpec> {
        Om4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port 5 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om5(&mut self) -> Om5W<OmodeSpec> {
        Om5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port 6 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om6(&mut self) -> Om6W<OmodeSpec> {
        Om6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port 7 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om7(&mut self) -> Om7W<OmodeSpec> {
        Om7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port 8 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om8(&mut self) -> Om8W<OmodeSpec> {
        Om8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port 9 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om9(&mut self) -> Om9W<OmodeSpec> {
        Om9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port 10 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om10(&mut self) -> Om10W<OmodeSpec> {
        Om10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port 11 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om11(&mut self) -> Om11W<OmodeSpec> {
        Om11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port 12 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om12(&mut self) -> Om12W<OmodeSpec> {
        Om12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port 13 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om13(&mut self) -> Om13W<OmodeSpec> {
        Om13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port 14 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om14(&mut self) -> Om14W<OmodeSpec> {
        Om14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port 15 output mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn om15(&mut self) -> Om15W<OmodeSpec> {
        Om15W::new(self, 15)
    }
}
#[doc = "GPIO port output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
