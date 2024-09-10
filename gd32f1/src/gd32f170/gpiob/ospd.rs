#[doc = "Register `OSPD` reader"]
pub type R = crate::R<OspdSpec>;
#[doc = "Register `OSPD` writer"]
pub type W = crate::W<OspdSpec>;
#[doc = "Pin 0 output max speed bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ospd0 {
    #[doc = "0: Max output speed 2 MHz"]
    Speed2m = 0,
    #[doc = "1: Max output speed 10 MHz"]
    Speed10m = 1,
    #[doc = "3: Max output speed 50 MHz"]
    Speed50m = 3,
}
impl From<Ospd0> for u8 {
    #[inline(always)]
    fn from(variant: Ospd0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ospd0 {
    type Ux = u8;
}
#[doc = "Field `OSPD0` reader - Pin 0 output max speed bits"]
pub type Ospd0R = crate::FieldReader<Ospd0>;
impl Ospd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ospd0> {
        match self.bits {
            0 => Some(Ospd0::Speed2m),
            1 => Some(Ospd0::Speed10m),
            3 => Some(Ospd0::Speed50m),
            _ => None,
        }
    }
    #[doc = "Max output speed 2 MHz"]
    #[inline(always)]
    pub fn is_speed2m(&self) -> bool {
        *self == Ospd0::Speed2m
    }
    #[doc = "Max output speed 10 MHz"]
    #[inline(always)]
    pub fn is_speed10m(&self) -> bool {
        *self == Ospd0::Speed10m
    }
    #[doc = "Max output speed 50 MHz"]
    #[inline(always)]
    pub fn is_speed50m(&self) -> bool {
        *self == Ospd0::Speed50m
    }
}
#[doc = "Field `OSPD0` writer - Pin 0 output max speed bits"]
pub type Ospd0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ospd0>;
impl<'a, REG> Ospd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Max output speed 2 MHz"]
    #[inline(always)]
    pub fn speed2m(self) -> &'a mut crate::W<REG> {
        self.variant(Ospd0::Speed2m)
    }
    #[doc = "Max output speed 10 MHz"]
    #[inline(always)]
    pub fn speed10m(self) -> &'a mut crate::W<REG> {
        self.variant(Ospd0::Speed10m)
    }
    #[doc = "Max output speed 50 MHz"]
    #[inline(always)]
    pub fn speed50m(self) -> &'a mut crate::W<REG> {
        self.variant(Ospd0::Speed50m)
    }
}
#[doc = "Field `OSPD1` reader - Pin 1 output max speed bits"]
pub use Ospd0R as Ospd1R;
#[doc = "Field `OSPD2` reader - Pin 2 output max speed bits"]
pub use Ospd0R as Ospd2R;
#[doc = "Field `OSPD3` reader - Pin 3 output max speed bits"]
pub use Ospd0R as Ospd3R;
#[doc = "Field `OSPD4` reader - Pin 4 output max speed bits"]
pub use Ospd0R as Ospd4R;
#[doc = "Field `OSPD5` reader - Pin 5 output max speed bits"]
pub use Ospd0R as Ospd5R;
#[doc = "Field `OSPD6` reader - Pin 6 output max speed bits"]
pub use Ospd0R as Ospd6R;
#[doc = "Field `OSPD7` reader - Pin 7 output max speed bits"]
pub use Ospd0R as Ospd7R;
#[doc = "Field `OSPD8` reader - Pin 8 output max speed bits"]
pub use Ospd0R as Ospd8R;
#[doc = "Field `OSPD9` reader - Pin 9 output max speed bits"]
pub use Ospd0R as Ospd9R;
#[doc = "Field `OSPD10` reader - Pin 10 output max speed bits"]
pub use Ospd0R as Ospd10R;
#[doc = "Field `OSPD11` reader - Pin 11 output max speed bits"]
pub use Ospd0R as Ospd11R;
#[doc = "Field `OSPD12` reader - Pin 12 output max speed bits"]
pub use Ospd0R as Ospd12R;
#[doc = "Field `OSPD13` reader - Pin 13 output max speed bits"]
pub use Ospd0R as Ospd13R;
#[doc = "Field `OSPD14` reader - Pin 14 output max speed bits"]
pub use Ospd0R as Ospd14R;
#[doc = "Field `OSPD15` reader - Pin 15 output max speed bits"]
pub use Ospd0R as Ospd15R;
#[doc = "Field `OSPD1` writer - Pin 1 output max speed bits"]
pub use Ospd0W as Ospd1W;
#[doc = "Field `OSPD2` writer - Pin 2 output max speed bits"]
pub use Ospd0W as Ospd2W;
#[doc = "Field `OSPD3` writer - Pin 3 output max speed bits"]
pub use Ospd0W as Ospd3W;
#[doc = "Field `OSPD4` writer - Pin 4 output max speed bits"]
pub use Ospd0W as Ospd4W;
#[doc = "Field `OSPD5` writer - Pin 5 output max speed bits"]
pub use Ospd0W as Ospd5W;
#[doc = "Field `OSPD6` writer - Pin 6 output max speed bits"]
pub use Ospd0W as Ospd6W;
#[doc = "Field `OSPD7` writer - Pin 7 output max speed bits"]
pub use Ospd0W as Ospd7W;
#[doc = "Field `OSPD8` writer - Pin 8 output max speed bits"]
pub use Ospd0W as Ospd8W;
#[doc = "Field `OSPD9` writer - Pin 9 output max speed bits"]
pub use Ospd0W as Ospd9W;
#[doc = "Field `OSPD10` writer - Pin 10 output max speed bits"]
pub use Ospd0W as Ospd10W;
#[doc = "Field `OSPD11` writer - Pin 11 output max speed bits"]
pub use Ospd0W as Ospd11W;
#[doc = "Field `OSPD12` writer - Pin 12 output max speed bits"]
pub use Ospd0W as Ospd12W;
#[doc = "Field `OSPD13` writer - Pin 13 output max speed bits"]
pub use Ospd0W as Ospd13W;
#[doc = "Field `OSPD14` writer - Pin 14 output max speed bits"]
pub use Ospd0W as Ospd14W;
#[doc = "Field `OSPD15` writer - Pin 15 output max speed bits"]
pub use Ospd0W as Ospd15W;
impl R {
    #[doc = "Bits 0:1 - Pin 0 output max speed bits"]
    #[inline(always)]
    pub fn ospd0(&self) -> Ospd0R {
        Ospd0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin 1 output max speed bits"]
    #[inline(always)]
    pub fn ospd1(&self) -> Ospd1R {
        Ospd1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin 2 output max speed bits"]
    #[inline(always)]
    pub fn ospd2(&self) -> Ospd2R {
        Ospd2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin 3 output max speed bits"]
    #[inline(always)]
    pub fn ospd3(&self) -> Ospd3R {
        Ospd3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin 4 output max speed bits"]
    #[inline(always)]
    pub fn ospd4(&self) -> Ospd4R {
        Ospd4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin 5 output max speed bits"]
    #[inline(always)]
    pub fn ospd5(&self) -> Ospd5R {
        Ospd5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin 6 output max speed bits"]
    #[inline(always)]
    pub fn ospd6(&self) -> Ospd6R {
        Ospd6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin 7 output max speed bits"]
    #[inline(always)]
    pub fn ospd7(&self) -> Ospd7R {
        Ospd7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin 8 output max speed bits"]
    #[inline(always)]
    pub fn ospd8(&self) -> Ospd8R {
        Ospd8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin 9 output max speed bits"]
    #[inline(always)]
    pub fn ospd9(&self) -> Ospd9R {
        Ospd9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin 10 output max speed bits"]
    #[inline(always)]
    pub fn ospd10(&self) -> Ospd10R {
        Ospd10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin 11 output max speed bits"]
    #[inline(always)]
    pub fn ospd11(&self) -> Ospd11R {
        Ospd11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin 12 output max speed bits"]
    #[inline(always)]
    pub fn ospd12(&self) -> Ospd12R {
        Ospd12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin 13 output max speed bits"]
    #[inline(always)]
    pub fn ospd13(&self) -> Ospd13R {
        Ospd13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Pin 14 output max speed bits"]
    #[inline(always)]
    pub fn ospd14(&self) -> Ospd14R {
        Ospd14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Pin 15 output max speed bits"]
    #[inline(always)]
    pub fn ospd15(&self) -> Ospd15R {
        Ospd15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin 0 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd0(&mut self) -> Ospd0W<OspdSpec> {
        Ospd0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pin 1 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd1(&mut self) -> Ospd1W<OspdSpec> {
        Ospd1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Pin 2 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd2(&mut self) -> Ospd2W<OspdSpec> {
        Ospd2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Pin 3 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd3(&mut self) -> Ospd3W<OspdSpec> {
        Ospd3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pin 4 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd4(&mut self) -> Ospd4W<OspdSpec> {
        Ospd4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pin 5 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd5(&mut self) -> Ospd5W<OspdSpec> {
        Ospd5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Pin 6 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd6(&mut self) -> Ospd6W<OspdSpec> {
        Ospd6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Pin 7 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd7(&mut self) -> Ospd7W<OspdSpec> {
        Ospd7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Pin 8 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd8(&mut self) -> Ospd8W<OspdSpec> {
        Ospd8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Pin 9 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd9(&mut self) -> Ospd9W<OspdSpec> {
        Ospd9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Pin 10 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd10(&mut self) -> Ospd10W<OspdSpec> {
        Ospd10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Pin 11 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd11(&mut self) -> Ospd11W<OspdSpec> {
        Ospd11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Pin 12 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd12(&mut self) -> Ospd12W<OspdSpec> {
        Ospd12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Pin 13 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd13(&mut self) -> Ospd13W<OspdSpec> {
        Ospd13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Pin 14 output max speed bits"]
    #[inline(always)]
    #[must_use]
    pub fn ospd14(&mut self) -> Ospd14W<OspdSpec> {
        Ospd14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Pin 15 output max speed bits"]
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
#[doc = "`reset()` method sets OSPD to value 0"]
impl crate::Resettable for OspdSpec {
    const RESET_VALUE: u32 = 0;
}
