#[doc = "Register `CPU_IRQ_LAT` reader"]
pub type R = crate::R<CPU_IRQ_LAT_SPEC>;
#[doc = "Register `CPU_IRQ_LAT` writer"]
pub type W = crate::W<CPU_IRQ_LAT_SPEC>;
#[doc = "Field `IRQ_LATENCY` reader - specifies the minimum number of cycles between an interrupt"]
pub type IRQ_LATENCY_R = crate::FieldReader;
#[doc = "Field `IRQ_LATENCY` writer - specifies the minimum number of cycles between an interrupt"]
pub type IRQ_LATENCY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - specifies the minimum number of cycles between an interrupt"]
    #[inline(always)]
    pub fn irq_latency(&self) -> IRQ_LATENCY_R {
        IRQ_LATENCY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - specifies the minimum number of cycles between an interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn irq_latency(&mut self) -> IRQ_LATENCY_W<CPU_IRQ_LAT_SPEC, 0> {
        IRQ_LATENCY_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IRQ Latency register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_irq_lat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_irq_lat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_IRQ_LAT_SPEC;
impl crate::RegisterSpec for CPU_IRQ_LAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_irq_lat::R`](R) reader structure"]
impl crate::Readable for CPU_IRQ_LAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_irq_lat::W`](W) writer structure"]
impl crate::Writable for CPU_IRQ_LAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_IRQ_LAT to value 0"]
impl crate::Resettable for CPU_IRQ_LAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}