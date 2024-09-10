#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "DAC0 DMA underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddudr0 {
    #[doc = "0: No DMA underrun error condition occurred"]
    NoUnderrun = 0,
    #[doc = "1: DMA underrun error condition occurred"]
    Underrun = 1,
}
impl From<Ddudr0> for bool {
    #[inline(always)]
    fn from(variant: Ddudr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDUDR0` reader - DAC0 DMA underrun flag"]
pub type Ddudr0R = crate::BitReader<Ddudr0>;
impl Ddudr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddudr0 {
        match self.bits {
            false => Ddudr0::NoUnderrun,
            true => Ddudr0::Underrun,
        }
    }
    #[doc = "No DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == Ddudr0::NoUnderrun
    }
    #[doc = "DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == Ddudr0::Underrun
    }
}
#[doc = "Field `DDUDR0` writer - DAC0 DMA underrun flag"]
pub type Ddudr0W<'a, REG> = crate::BitWriter<'a, REG, Ddudr0>;
impl<'a, REG> Ddudr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut crate::W<REG> {
        self.variant(Ddudr0::NoUnderrun)
    }
    #[doc = "DMA underrun error condition occurred"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut crate::W<REG> {
        self.variant(Ddudr0::Underrun)
    }
}
impl R {
    #[doc = "Bit 13 - DAC0 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr0(&self) -> Ddudr0R {
        Ddudr0R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC0 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn ddudr0(&mut self) -> Ddudr0W<StatSpec> {
        Ddudr0W::new(self, 13)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
