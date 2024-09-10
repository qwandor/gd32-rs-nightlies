#[doc = "Register `AFSEL1` reader"]
pub type R = crate::R<Afsel1Spec>;
#[doc = "Register `AFSEL1` writer"]
pub type W = crate::W<Afsel1Spec>;
#[doc = "Field `SEL8` reader - Port 8 alternate function selected"]
pub type Sel8R = crate::FieldReader;
#[doc = "Field `SEL8` writer - Port 8 alternate function selected"]
pub type Sel8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL9` reader - Port 9 alternate function selected"]
pub type Sel9R = crate::FieldReader;
#[doc = "Field `SEL9` writer - Port 9 alternate function selected"]
pub type Sel9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL10` reader - Port 10 alternate function selected"]
pub type Sel10R = crate::FieldReader;
#[doc = "Field `SEL10` writer - Port 10 alternate function selected"]
pub type Sel10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL11` reader - Port 11 alternate function selected"]
pub type Sel11R = crate::FieldReader;
#[doc = "Field `SEL11` writer - Port 11 alternate function selected"]
pub type Sel11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL12` reader - Port 12 alternate function selected"]
pub type Sel12R = crate::FieldReader;
#[doc = "Field `SEL12` writer - Port 12 alternate function selected"]
pub type Sel12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL13` reader - Port 13 alternate function selected"]
pub type Sel13R = crate::FieldReader;
#[doc = "Field `SEL13` writer - Port 13 alternate function selected"]
pub type Sel13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL14` reader - Port 14 alternate function selected"]
pub type Sel14R = crate::FieldReader;
#[doc = "Field `SEL14` writer - Port 14 alternate function selected"]
pub type Sel14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL15` reader - Port 15 alternate function selected"]
pub type Sel15R = crate::FieldReader;
#[doc = "Field `SEL15` writer - Port 15 alternate function selected"]
pub type Sel15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Port 8 alternate function selected"]
    #[inline(always)]
    pub fn sel8(&self) -> Sel8R {
        Sel8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Port 9 alternate function selected"]
    #[inline(always)]
    pub fn sel9(&self) -> Sel9R {
        Sel9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Port 10 alternate function selected"]
    #[inline(always)]
    pub fn sel10(&self) -> Sel10R {
        Sel10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Port 11 alternate function selected"]
    #[inline(always)]
    pub fn sel11(&self) -> Sel11R {
        Sel11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Port 12 alternate function selected"]
    #[inline(always)]
    pub fn sel12(&self) -> Sel12R {
        Sel12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Port 13 alternate function selected"]
    #[inline(always)]
    pub fn sel13(&self) -> Sel13R {
        Sel13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Port 14 alternate function selected"]
    #[inline(always)]
    pub fn sel14(&self) -> Sel14R {
        Sel14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Port 15 alternate function selected"]
    #[inline(always)]
    pub fn sel15(&self) -> Sel15R {
        Sel15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Port 8 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> Sel8W<Afsel1Spec> {
        Sel8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Port 9 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel9(&mut self) -> Sel9W<Afsel1Spec> {
        Sel9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Port 10 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel10(&mut self) -> Sel10W<Afsel1Spec> {
        Sel10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Port 11 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel11(&mut self) -> Sel11W<Afsel1Spec> {
        Sel11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Port 12 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> Sel12W<Afsel1Spec> {
        Sel12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Port 13 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel13(&mut self) -> Sel13W<Afsel1Spec> {
        Sel13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Port 14 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel14(&mut self) -> Sel14W<Afsel1Spec> {
        Sel14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Port 15 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel15(&mut self) -> Sel15W<Afsel1Spec> {
        Sel15W::new(self, 28)
    }
}
#[doc = "GPIO alternate function selected register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afsel1Spec;
impl crate::RegisterSpec for Afsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsel1::R`](R) reader structure"]
impl crate::Readable for Afsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`afsel1::W`](W) writer structure"]
impl crate::Writable for Afsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFSEL1 to value 0"]
impl crate::Resettable for Afsel1Spec {
    const RESET_VALUE: u32 = 0;
}
