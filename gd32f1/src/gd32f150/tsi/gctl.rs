#[doc = "Register `GCTL` reader"]
pub type R = crate::R<GctlSpec>;
#[doc = "Register `GCTL` writer"]
pub type W = crate::W<GctlSpec>;
#[doc = "Field `GE0` reader - Analog I/O group 0 enable"]
pub type Ge0R = crate::BitReader;
#[doc = "Field `GE0` writer - Analog I/O group 0 enable"]
pub type Ge0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE1` reader - Analog I/O group 1 enable"]
pub type Ge1R = crate::BitReader;
#[doc = "Field `GE1` writer - Analog I/O group 1 enable"]
pub type Ge1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE2` reader - Analog I/O group 2 enable"]
pub type Ge2R = crate::BitReader;
#[doc = "Field `GE2` writer - Analog I/O group 2 enable"]
pub type Ge2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE3` reader - Analog I/O group 3 enable"]
pub type Ge3R = crate::BitReader;
#[doc = "Field `GE3` writer - Analog I/O group 3 enable"]
pub type Ge3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE4` reader - Analog I/O group 4 enable"]
pub type Ge4R = crate::BitReader;
#[doc = "Field `GE4` writer - Analog I/O group 4 enable"]
pub type Ge4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE5` reader - Analog I/O group 5 enable"]
pub type Ge5R = crate::BitReader;
#[doc = "Field `GE5` writer - Analog I/O group 5 enable"]
pub type Ge5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC0` reader - Analog I/O group 0 status"]
pub type Gc0R = crate::BitReader;
#[doc = "Field `GC1` reader - Analog I/O group 1 status"]
pub type Gc1R = crate::BitReader;
#[doc = "Field `GC2` reader - Analog I/O group 2 status"]
pub type Gc2R = crate::BitReader;
#[doc = "Field `GC3` reader - Analog I/O group 3 status"]
pub type Gc3R = crate::BitReader;
#[doc = "Field `GC4` reader - Analog I/O group 4 status"]
pub type Gc4R = crate::BitReader;
#[doc = "Field `GC5` reader - Analog I/O group 5 status"]
pub type Gc5R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog I/O group 0 enable"]
    #[inline(always)]
    pub fn ge0(&self) -> Ge0R {
        Ge0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog I/O group 1 enable"]
    #[inline(always)]
    pub fn ge1(&self) -> Ge1R {
        Ge1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog I/O group 2 enable"]
    #[inline(always)]
    pub fn ge2(&self) -> Ge2R {
        Ge2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog I/O group 3 enable"]
    #[inline(always)]
    pub fn ge3(&self) -> Ge3R {
        Ge3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog I/O group 4 enable"]
    #[inline(always)]
    pub fn ge4(&self) -> Ge4R {
        Ge4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog I/O group 5 enable"]
    #[inline(always)]
    pub fn ge5(&self) -> Ge5R {
        Ge5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog I/O group 0 status"]
    #[inline(always)]
    pub fn gc0(&self) -> Gc0R {
        Gc0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog I/O group 1 status"]
    #[inline(always)]
    pub fn gc1(&self) -> Gc1R {
        Gc1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog I/O group 2 status"]
    #[inline(always)]
    pub fn gc2(&self) -> Gc2R {
        Gc2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog I/O group 3 status"]
    #[inline(always)]
    pub fn gc3(&self) -> Gc3R {
        Gc3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog I/O group 4 status"]
    #[inline(always)]
    pub fn gc4(&self) -> Gc4R {
        Gc4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog I/O group 5 status"]
    #[inline(always)]
    pub fn gc5(&self) -> Gc5R {
        Gc5R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog I/O group 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge0(&mut self) -> Ge0W<GctlSpec> {
        Ge0W::new(self, 0)
    }
    #[doc = "Bit 1 - Analog I/O group 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge1(&mut self) -> Ge1W<GctlSpec> {
        Ge1W::new(self, 1)
    }
    #[doc = "Bit 2 - Analog I/O group 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge2(&mut self) -> Ge2W<GctlSpec> {
        Ge2W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog I/O group 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge3(&mut self) -> Ge3W<GctlSpec> {
        Ge3W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog I/O group 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge4(&mut self) -> Ge4W<GctlSpec> {
        Ge4W::new(self, 4)
    }
    #[doc = "Bit 5 - Analog I/O group 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge5(&mut self) -> Ge5W<GctlSpec> {
        Ge5W::new(self, 5)
    }
}
#[doc = "I/O group control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GctlSpec;
impl crate::RegisterSpec for GctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gctl::R`](R) reader structure"]
impl crate::Readable for GctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gctl::W`](W) writer structure"]
impl crate::Writable for GctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCTL to value 0"]
impl crate::Resettable for GctlSpec {
    const RESET_VALUE: u32 = 0;
}
