#[doc = "Register `OSPD` reader"]
pub type R = crate::R<OspdSpec>;
#[doc = "Register `OSPD` writer"]
pub type W = crate::W<OspdSpec>;
#[doc = "Field `OSPD0` reader - Port 0 output max speed bits"]
pub type Ospd0R = crate::FieldReader;
#[doc = "Field `OSPD0` writer - Port 0 output max speed bits"]
pub type Ospd0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD1` reader - Port 1 output max speed bits"]
pub type Ospd1R = crate::FieldReader;
#[doc = "Field `OSPD1` writer - Port 1 output max speed bits"]
pub type Ospd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD2` reader - Port 2 output max speed bits"]
pub type Ospd2R = crate::FieldReader;
#[doc = "Field `OSPD2` writer - Port 2 output max speed bits"]
pub type Ospd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD3` reader - Port 3 output max speed bits"]
pub type Ospd3R = crate::FieldReader;
#[doc = "Field `OSPD3` writer - Port 3 output max speed bits"]
pub type Ospd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD4` reader - Port 4 output max speed bits"]
pub type Ospd4R = crate::FieldReader;
#[doc = "Field `OSPD4` writer - Port 4 output max speed bits"]
pub type Ospd4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD5` reader - Port 5 output max speed bits"]
pub type Ospd5R = crate::FieldReader;
#[doc = "Field `OSPD5` writer - Port 5 output max speed bits"]
pub type Ospd5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD6` reader - Port 6 output max speed bits"]
pub type Ospd6R = crate::FieldReader;
#[doc = "Field `OSPD6` writer - Port 6 output max speed bits"]
pub type Ospd6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD7` reader - Port 7 output max speed bits"]
pub type Ospd7R = crate::FieldReader;
#[doc = "Field `OSPD7` writer - Port 7 output max speed bits"]
pub type Ospd7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD8` reader - Port 8 output max speed bits"]
pub type Ospd8R = crate::FieldReader;
#[doc = "Field `OSPD8` writer - Port 8 output max speed bits"]
pub type Ospd8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD9` reader - Port 9 output max speed bits"]
pub type Ospd9R = crate::FieldReader;
#[doc = "Field `OSPD9` writer - Port 9 output max speed bits"]
pub type Ospd9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD10` reader - Port 10 output max speed bits"]
pub type Ospd10R = crate::FieldReader;
#[doc = "Field `OSPD10` writer - Port 10 output max speed bits"]
pub type Ospd10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD11` reader - Port 11 output max speed bits"]
pub type Ospd11R = crate::FieldReader;
#[doc = "Field `OSPD11` writer - Port 11 output max speed bits"]
pub type Ospd11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD12` reader - Port 12 output max speed bits"]
pub type Ospd12R = crate::FieldReader;
#[doc = "Field `OSPD12` writer - Port 12 output max speed bits"]
pub type Ospd12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD13` reader - Port 13 output max speed bits"]
pub type Ospd13R = crate::FieldReader;
#[doc = "Field `OSPD13` writer - Port 13 output max speed bits"]
pub type Ospd13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD14` reader - Port 14 output max speed bits"]
pub type Ospd14R = crate::FieldReader;
#[doc = "Field `OSPD14` writer - Port 14 output max speed bits"]
pub type Ospd14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPD15` reader - Port 15 output max speed bits"]
pub type Ospd15R = crate::FieldReader;
#[doc = "Field `OSPD15` writer - Port 15 output max speed bits"]
pub type Ospd15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port 0 output max speed bits"]
    #[inline(always)]
    pub fn ospd0(&self) -> Ospd0R {
        Ospd0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 output max speed bits"]
    #[inline(always)]
    pub fn ospd1(&self) -> Ospd1R {
        Ospd1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 2 output max speed bits"]
    #[inline(always)]
    pub fn ospd2(&self) -> Ospd2R {
        Ospd2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 3 output max speed bits"]
    #[inline(always)]
    pub fn ospd3(&self) -> Ospd3R {
        Ospd3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 4 output max speed bits"]
    #[inline(always)]
    pub fn ospd4(&self) -> Ospd4R {
        Ospd4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 5 output max speed bits"]
    #[inline(always)]
    pub fn ospd5(&self) -> Ospd5R {
        Ospd5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 6 output max speed bits"]
    #[inline(always)]
    pub fn ospd6(&self) -> Ospd6R {
        Ospd6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 7 output max speed bits"]
    #[inline(always)]
    pub fn ospd7(&self) -> Ospd7R {
        Ospd7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 8 output max speed bits"]
    #[inline(always)]
    pub fn ospd8(&self) -> Ospd8R {
        Ospd8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 9 output max speed bits"]
    #[inline(always)]
    pub fn ospd9(&self) -> Ospd9R {
        Ospd9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 10 output max speed bits"]
    #[inline(always)]
    pub fn ospd10(&self) -> Ospd10R {
        Ospd10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 11 output max speed bits"]
    #[inline(always)]
    pub fn ospd11(&self) -> Ospd11R {
        Ospd11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 12 output max speed bits"]
    #[inline(always)]
    pub fn ospd12(&self) -> Ospd12R {
        Ospd12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 13 output max speed bits"]
    #[inline(always)]
    pub fn ospd13(&self) -> Ospd13R {
        Ospd13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 14 output max speed bits"]
    #[inline(always)]
    pub fn ospd14(&self) -> Ospd14R {
        Ospd14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 15 output max speed bits"]
    #[inline(always)]
    pub fn ospd15(&self) -> Ospd15R {
        Ospd15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 0 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd0(&mut self) -> Ospd0W<OspdSpec> {
        Ospd0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 1 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd1(&mut self) -> Ospd1W<OspdSpec> {
        Ospd1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 2 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd2(&mut self) -> Ospd2W<OspdSpec> {
        Ospd2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 3 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd3(&mut self) -> Ospd3W<OspdSpec> {
        Ospd3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 4 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd4(&mut self) -> Ospd4W<OspdSpec> {
        Ospd4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 5 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd5(&mut self) -> Ospd5W<OspdSpec> {
        Ospd5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 6 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd6(&mut self) -> Ospd6W<OspdSpec> {
        Ospd6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 7 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd7(&mut self) -> Ospd7W<OspdSpec> {
        Ospd7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 8 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd8(&mut self) -> Ospd8W<OspdSpec> {
        Ospd8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 9 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd9(&mut self) -> Ospd9W<OspdSpec> {
        Ospd9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 10 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd10(&mut self) -> Ospd10W<OspdSpec> {
        Ospd10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 11 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd11(&mut self) -> Ospd11W<OspdSpec> {
        Ospd11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port 12 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd12(&mut self) -> Ospd12W<OspdSpec> {
        Ospd12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port 13 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd13(&mut self) -> Ospd13W<OspdSpec> {
        Ospd13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port 14 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd14(&mut self) -> Ospd14W<OspdSpec> {
        Ospd14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port 15 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd15(&mut self) -> Ospd15W<OspdSpec> {
        Ospd15W::new(self, 30)
    }
}
#[doc = "GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OspdSpec;
impl crate::RegisterSpec for OspdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospd::R`](R) reader structure"]
impl crate::Readable for OspdSpec {}
#[doc = "`write(|w| ..)` method takes [`ospd::W`](W) writer structure"]
impl crate::Writable for OspdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPD to value 0x0c00_0000"]
impl crate::Resettable for OspdSpec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
