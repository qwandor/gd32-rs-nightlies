#[doc = "Register `GP` reader"]
pub type R = crate::R<GpSpec>;
#[doc = "Register `GP` writer"]
pub type W = crate::W<GpSpec>;
#[doc = "Field `PSC` reader - specify the division factor that is used to divide the peripheral clock (PCLK1/PCLK2) to generate the low-power frequency"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - specify the division factor that is used to divide the peripheral clock (PCLK1/PCLK2) to generate the low-power frequency"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - specify the division factor that is used to divide the peripheral clock (PCLK1/PCLK2) to generate the low-power frequency"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - specify the division factor that is used to divide the peripheral clock (PCLK1/PCLK2) to generate the low-power frequency"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PscW<GpSpec> {
        PscW::new(self, 0)
    }
}
#[doc = "Guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpSpec;
impl crate::RegisterSpec for GpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp::R`](R) reader structure"]
impl crate::Readable for GpSpec {}
#[doc = "`write(|w| ..)` method takes [`gp::W`](W) writer structure"]
impl crate::Writable for GpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GP to value 0"]
impl crate::Resettable for GpSpec {
    const RESET_VALUE: u32 = 0;
}
