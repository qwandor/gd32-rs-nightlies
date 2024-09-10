#[doc = "Register `OCTL` reader"]
pub type R = crate::R<OctlSpec>;
#[doc = "Register `OCTL` writer"]
pub type W = crate::W<OctlSpec>;
#[doc = "Field `OCTL0` reader - Port output control (y = 0)"]
pub type Octl0R = crate::BitReader;
#[doc = "Field `OCTL0` writer - Port output control (y = 0)"]
pub type Octl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL1` reader - Port output control (y = 1)"]
pub type Octl1R = crate::BitReader;
#[doc = "Field `OCTL1` writer - Port output control (y = 1)"]
pub type Octl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL2` reader - Port output control (y = 2)"]
pub type Octl2R = crate::BitReader;
#[doc = "Field `OCTL2` writer - Port output control (y = 2)"]
pub type Octl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL3` reader - Port output control (y = 3)"]
pub type Octl3R = crate::BitReader;
#[doc = "Field `OCTL3` writer - Port output control (y = 3)"]
pub type Octl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL4` reader - Port output control (y = 4)"]
pub type Octl4R = crate::BitReader;
#[doc = "Field `OCTL4` writer - Port output control (y = 4)"]
pub type Octl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL5` reader - Port output control (y = 5)"]
pub type Octl5R = crate::BitReader;
#[doc = "Field `OCTL5` writer - Port output control (y = 5)"]
pub type Octl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL6` reader - Port output control (y = 6)"]
pub type Octl6R = crate::BitReader;
#[doc = "Field `OCTL6` writer - Port output control (y = 6)"]
pub type Octl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL7` reader - Port output control (y = 7)"]
pub type Octl7R = crate::BitReader;
#[doc = "Field `OCTL7` writer - Port output control (y = 7)"]
pub type Octl7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL8` reader - Port output control (y = 8)"]
pub type Octl8R = crate::BitReader;
#[doc = "Field `OCTL8` writer - Port output control (y = 8)"]
pub type Octl8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL9` reader - Port output control (y = 9)"]
pub type Octl9R = crate::BitReader;
#[doc = "Field `OCTL9` writer - Port output control (y = 9)"]
pub type Octl9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL10` reader - Port output control (y = 10)"]
pub type Octl10R = crate::BitReader;
#[doc = "Field `OCTL10` writer - Port output control (y = 10)"]
pub type Octl10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL11` reader - Port output control (y = 11)"]
pub type Octl11R = crate::BitReader;
#[doc = "Field `OCTL11` writer - Port output control (y = 11)"]
pub type Octl11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL12` reader - Port output control (y = 12)"]
pub type Octl12R = crate::BitReader;
#[doc = "Field `OCTL12` writer - Port output control (y = 12)"]
pub type Octl12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL13` reader - Port output control (y = 13)"]
pub type Octl13R = crate::BitReader;
#[doc = "Field `OCTL13` writer - Port output control (y = 13)"]
pub type Octl13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL14` reader - Port output control (y = 14)"]
pub type Octl14R = crate::BitReader;
#[doc = "Field `OCTL14` writer - Port output control (y = 14)"]
pub type Octl14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTL15` reader - Port output control (y = 15)"]
pub type Octl15R = crate::BitReader;
#[doc = "Field `OCTL15` writer - Port output control (y = 15)"]
pub type Octl15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port output control (y = 0)"]
    #[inline(always)]
    pub fn octl0(&self) -> Octl0R {
        Octl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output control (y = 1)"]
    #[inline(always)]
    pub fn octl1(&self) -> Octl1R {
        Octl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output control (y = 2)"]
    #[inline(always)]
    pub fn octl2(&self) -> Octl2R {
        Octl2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output control (y = 3)"]
    #[inline(always)]
    pub fn octl3(&self) -> Octl3R {
        Octl3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output control (y = 4)"]
    #[inline(always)]
    pub fn octl4(&self) -> Octl4R {
        Octl4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output control (y = 5)"]
    #[inline(always)]
    pub fn octl5(&self) -> Octl5R {
        Octl5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output control (y = 6)"]
    #[inline(always)]
    pub fn octl6(&self) -> Octl6R {
        Octl6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output control (y = 7)"]
    #[inline(always)]
    pub fn octl7(&self) -> Octl7R {
        Octl7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output control (y = 8)"]
    #[inline(always)]
    pub fn octl8(&self) -> Octl8R {
        Octl8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output control (y = 9)"]
    #[inline(always)]
    pub fn octl9(&self) -> Octl9R {
        Octl9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output control (y = 10)"]
    #[inline(always)]
    pub fn octl10(&self) -> Octl10R {
        Octl10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output control (y = 11)"]
    #[inline(always)]
    pub fn octl11(&self) -> Octl11R {
        Octl11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output control (y = 12)"]
    #[inline(always)]
    pub fn octl12(&self) -> Octl12R {
        Octl12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output control (y = 13)"]
    #[inline(always)]
    pub fn octl13(&self) -> Octl13R {
        Octl13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output control (y = 14)"]
    #[inline(always)]
    pub fn octl14(&self) -> Octl14R {
        Octl14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output control (y = 15)"]
    #[inline(always)]
    pub fn octl15(&self) -> Octl15R {
        Octl15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output control (y = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn octl0(&mut self) -> Octl0W<OctlSpec> {
        Octl0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port output control (y = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn octl1(&mut self) -> Octl1W<OctlSpec> {
        Octl1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port output control (y = 2)"]
    #[inline(always)]
    #[must_use]
    pub fn octl2(&mut self) -> Octl2W<OctlSpec> {
        Octl2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port output control (y = 3)"]
    #[inline(always)]
    #[must_use]
    pub fn octl3(&mut self) -> Octl3W<OctlSpec> {
        Octl3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port output control (y = 4)"]
    #[inline(always)]
    #[must_use]
    pub fn octl4(&mut self) -> Octl4W<OctlSpec> {
        Octl4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port output control (y = 5)"]
    #[inline(always)]
    #[must_use]
    pub fn octl5(&mut self) -> Octl5W<OctlSpec> {
        Octl5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port output control (y = 6)"]
    #[inline(always)]
    #[must_use]
    pub fn octl6(&mut self) -> Octl6W<OctlSpec> {
        Octl6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port output control (y = 7)"]
    #[inline(always)]
    #[must_use]
    pub fn octl7(&mut self) -> Octl7W<OctlSpec> {
        Octl7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port output control (y = 8)"]
    #[inline(always)]
    #[must_use]
    pub fn octl8(&mut self) -> Octl8W<OctlSpec> {
        Octl8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port output control (y = 9)"]
    #[inline(always)]
    #[must_use]
    pub fn octl9(&mut self) -> Octl9W<OctlSpec> {
        Octl9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port output control (y = 10)"]
    #[inline(always)]
    #[must_use]
    pub fn octl10(&mut self) -> Octl10W<OctlSpec> {
        Octl10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port output control (y = 11)"]
    #[inline(always)]
    #[must_use]
    pub fn octl11(&mut self) -> Octl11W<OctlSpec> {
        Octl11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port output control (y = 12)"]
    #[inline(always)]
    #[must_use]
    pub fn octl12(&mut self) -> Octl12W<OctlSpec> {
        Octl12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port output control (y = 13)"]
    #[inline(always)]
    #[must_use]
    pub fn octl13(&mut self) -> Octl13W<OctlSpec> {
        Octl13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port output control (y = 14)"]
    #[inline(always)]
    #[must_use]
    pub fn octl14(&mut self) -> Octl14W<OctlSpec> {
        Octl14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port output control (y = 15)"]
    #[inline(always)]
    #[must_use]
    pub fn octl15(&mut self) -> Octl15W<OctlSpec> {
        Octl15W::new(self, 15)
    }
}
#[doc = "GPIO port output control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OctlSpec;
impl crate::RegisterSpec for OctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octl::R`](R) reader structure"]
impl crate::Readable for OctlSpec {}
#[doc = "`write(|w| ..)` method takes [`octl::W`](W) writer structure"]
impl crate::Writable for OctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCTL to value 0"]
impl crate::Resettable for OctlSpec {
    const RESET_VALUE: u32 = 0;
}
