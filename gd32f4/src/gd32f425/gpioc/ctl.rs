#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `CTL0` reader - Port x configuration bits (x = 0)"]
pub type Ctl0R = crate::FieldReader;
#[doc = "Field `CTL0` writer - Port x configuration bits (x = 0)"]
pub type Ctl0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL1` reader - Port x configuration bits (x = 1)"]
pub type Ctl1R = crate::FieldReader;
#[doc = "Field `CTL1` writer - Port x configuration bits (x = 1)"]
pub type Ctl1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL2` reader - Port x configuration bits (x = 2)"]
pub type Ctl2R = crate::FieldReader;
#[doc = "Field `CTL2` writer - Port x configuration bits (x = 2)"]
pub type Ctl2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL3` reader - Port x configuration bits (x = 3)"]
pub type Ctl3R = crate::FieldReader;
#[doc = "Field `CTL3` writer - Port x configuration bits (x = 3)"]
pub type Ctl3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL4` reader - Port x configuration bits (x = 4 )"]
pub type Ctl4R = crate::FieldReader;
#[doc = "Field `CTL4` writer - Port x configuration bits (x = 4 )"]
pub type Ctl4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL5` reader - Port x configuration bits (x = 5)"]
pub type Ctl5R = crate::FieldReader;
#[doc = "Field `CTL5` writer - Port x configuration bits (x = 5)"]
pub type Ctl5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL6` reader - Port x configuration bits (x = 6 )"]
pub type Ctl6R = crate::FieldReader;
#[doc = "Field `CTL6` writer - Port x configuration bits (x = 6 )"]
pub type Ctl6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL7` reader - Port x configuration bits (x = 7)"]
pub type Ctl7R = crate::FieldReader;
#[doc = "Field `CTL7` writer - Port x configuration bits (x = 7)"]
pub type Ctl7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL8` reader - Port x configuration bits (x = 8)"]
pub type Ctl8R = crate::FieldReader;
#[doc = "Field `CTL8` writer - Port x configuration bits (x = 8)"]
pub type Ctl8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL9` reader - Port x configuration bits (x = 9)"]
pub type Ctl9R = crate::FieldReader;
#[doc = "Field `CTL9` writer - Port x configuration bits (x = 9)"]
pub type Ctl9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL10` reader - Port x configuration bits (x = 10)"]
pub type Ctl10R = crate::FieldReader;
#[doc = "Field `CTL10` writer - Port x configuration bits (x = 10)"]
pub type Ctl10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL11` reader - Port x configuration bits (x = 11)"]
pub type Ctl11R = crate::FieldReader;
#[doc = "Field `CTL11` writer - Port x configuration bits (x = 11)"]
pub type Ctl11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL12` reader - Port x configuration bits (x = 12)"]
pub type Ctl12R = crate::FieldReader;
#[doc = "Field `CTL12` writer - Port x configuration bits (x = 12)"]
pub type Ctl12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL13` reader - Port x configuration bits (x = 13)"]
pub type Ctl13R = crate::FieldReader;
#[doc = "Field `CTL13` writer - Port x configuration bits (x = 13)"]
pub type Ctl13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL14` reader - Port x configuration bits (x = 14)"]
pub type Ctl14R = crate::FieldReader;
#[doc = "Field `CTL14` writer - Port x configuration bits (x = 14)"]
pub type Ctl14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL15` reader - Port x configuration bits (x = 15)"]
pub type Ctl15R = crate::FieldReader;
#[doc = "Field `CTL15` writer - Port x configuration bits (x = 15)"]
pub type Ctl15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    pub fn ctl0(&self) -> Ctl0R {
        Ctl0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    pub fn ctl1(&self) -> Ctl1R {
        Ctl1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    pub fn ctl2(&self) -> Ctl2R {
        Ctl2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    pub fn ctl3(&self) -> Ctl3R {
        Ctl3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (x = 4 )"]
    #[inline(always)]
    pub fn ctl4(&self) -> Ctl4R {
        Ctl4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    pub fn ctl5(&self) -> Ctl5R {
        Ctl5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (x = 6 )"]
    #[inline(always)]
    pub fn ctl6(&self) -> Ctl6R {
        Ctl6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    pub fn ctl7(&self) -> Ctl7R {
        Ctl7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&self) -> Ctl8R {
        Ctl8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&self) -> Ctl9R {
        Ctl9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&self) -> Ctl10R {
        Ctl10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&self) -> Ctl11R {
        Ctl11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&self) -> Ctl12R {
        Ctl12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&self) -> Ctl13R {
        Ctl13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&self) -> Ctl14R {
        Ctl14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&self) -> Ctl15R {
        Ctl15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl0(&mut self) -> Ctl0W<CtlSpec> {
        Ctl0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl1(&mut self) -> Ctl1W<CtlSpec> {
        Ctl1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl2(&mut self) -> Ctl2W<CtlSpec> {
        Ctl2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl3(&mut self) -> Ctl3W<CtlSpec> {
        Ctl3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (x = 4 )"]
    #[inline(always)]
    #[must_use]
    pub fn ctl4(&mut self) -> Ctl4W<CtlSpec> {
        Ctl4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl5(&mut self) -> Ctl5W<CtlSpec> {
        Ctl5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (x = 6 )"]
    #[inline(always)]
    #[must_use]
    pub fn ctl6(&mut self) -> Ctl6W<CtlSpec> {
        Ctl6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl7(&mut self) -> Ctl7W<CtlSpec> {
        Ctl7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl8(&mut self) -> Ctl8W<CtlSpec> {
        Ctl8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl9(&mut self) -> Ctl9W<CtlSpec> {
        Ctl9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl10(&mut self) -> Ctl10W<CtlSpec> {
        Ctl10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl11(&mut self) -> Ctl11W<CtlSpec> {
        Ctl11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl12(&mut self) -> Ctl12W<CtlSpec> {
        Ctl12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl13(&mut self) -> Ctl13W<CtlSpec> {
        Ctl13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl14(&mut self) -> Ctl14W<CtlSpec> {
        Ctl14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    #[must_use]
    pub fn ctl15(&mut self) -> Ctl15W<CtlSpec> {
        Ctl15W::new(self, 30)
    }
}
#[doc = "GPIO port control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
