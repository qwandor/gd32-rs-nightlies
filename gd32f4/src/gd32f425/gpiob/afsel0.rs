#[doc = "Register `AFSEL0` reader"]
pub type R = crate::R<Afsel0Spec>;
#[doc = "Register `AFSEL0` writer"]
pub type W = crate::W<Afsel0Spec>;
#[doc = "Field `SEL0` reader - Port 0 alternate function selected"]
pub type Sel0R = crate::FieldReader;
#[doc = "Field `SEL0` writer - Port 0 alternate function selected"]
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL1` reader - Port 1 alternate function selected"]
pub type Sel1R = crate::FieldReader;
#[doc = "Field `SEL1` writer - Port 1 alternate function selected"]
pub type Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL2` reader - Port 2 alternate function selected"]
pub type Sel2R = crate::FieldReader;
#[doc = "Field `SEL2` writer - Port 2 alternate function selected"]
pub type Sel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL3` reader - Port 3 alternate function selected"]
pub type Sel3R = crate::FieldReader;
#[doc = "Field `SEL3` writer - Port 3 alternate function selected"]
pub type Sel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL4` reader - Port 4 alternate function selected"]
pub type Sel4R = crate::FieldReader;
#[doc = "Field `SEL4` writer - Port 4 alternate function selected"]
pub type Sel4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL5` reader - Port 5 alternate function selected"]
pub type Sel5R = crate::FieldReader;
#[doc = "Field `SEL5` writer - Port 5 alternate function selected"]
pub type Sel5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL6` reader - Port 6 alternate function selected"]
pub type Sel6R = crate::FieldReader;
#[doc = "Field `SEL6` writer - Port 6 alternate function selected"]
pub type Sel6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL7` reader - Port 7 alternate function selected"]
pub type Sel7R = crate::FieldReader;
#[doc = "Field `SEL7` writer - Port 7 alternate function selected"]
pub type Sel7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Port 0 alternate function selected"]
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Port 1 alternate function selected"]
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Port 2 alternate function selected"]
    #[inline(always)]
    pub fn sel2(&self) -> Sel2R {
        Sel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Port 3 alternate function selected"]
    #[inline(always)]
    pub fn sel3(&self) -> Sel3R {
        Sel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Port 4 alternate function selected"]
    #[inline(always)]
    pub fn sel4(&self) -> Sel4R {
        Sel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Port 5 alternate function selected"]
    #[inline(always)]
    pub fn sel5(&self) -> Sel5R {
        Sel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Port 6 alternate function selected"]
    #[inline(always)]
    pub fn sel6(&self) -> Sel6R {
        Sel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Port 7 alternate function selected"]
    #[inline(always)]
    pub fn sel7(&self) -> Sel7R {
        Sel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Port 0 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> Sel0W<Afsel0Spec> {
        Sel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Port 1 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> Sel1W<Afsel0Spec> {
        Sel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Port 2 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> Sel2W<Afsel0Spec> {
        Sel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Port 3 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> Sel3W<Afsel0Spec> {
        Sel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Port 4 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> Sel4W<Afsel0Spec> {
        Sel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Port 5 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel5(&mut self) -> Sel5W<Afsel0Spec> {
        Sel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Port 6 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> Sel6W<Afsel0Spec> {
        Sel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Port 7 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel7(&mut self) -> Sel7W<Afsel0Spec> {
        Sel7W::new(self, 28)
    }
}
#[doc = "GPIO alternate function selected register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afsel0Spec;
impl crate::RegisterSpec for Afsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsel0::R`](R) reader structure"]
impl crate::Readable for Afsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`afsel0::W`](W) writer structure"]
impl crate::Writable for Afsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFSEL0 to value 0"]
impl crate::Resettable for Afsel0Spec {
    const RESET_VALUE: u32 = 0;
}
