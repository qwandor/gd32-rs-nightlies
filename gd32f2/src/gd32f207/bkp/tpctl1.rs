#[doc = "Register `TPCTL1` reader"]
pub type R = crate::R<Tpctl1Spec>;
#[doc = "Register `TPCTL1` writer"]
pub type W = crate::W<Tpctl1Spec>;
#[doc = "Field `TPEN1` reader - TAMPER1 detection enable"]
pub type Tpen1R = crate::BitReader;
#[doc = "Field `TPEN1` writer - TAMPER1 detection enable"]
pub type Tpen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPAL1` reader - TAMPER1 pin active level"]
pub type Tpal1R = crate::BitReader;
#[doc = "Field `TPAL1` writer - TAMPER1 pin active level"]
pub type Tpal1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPM2` reader - The second waveform detection enable"]
pub type Tpm2R = crate::BitReader;
#[doc = "Field `TPM2` writer - The second waveform detection enable"]
pub type Tpm2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPM1` reader - The first waveform detection enable"]
pub type Tpm1R = crate::BitReader;
#[doc = "Field `TPM1` writer - The first waveform detection enable"]
pub type Tpm1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - TAMPER1 detection enable"]
    #[inline(always)]
    pub fn tpen1(&self) -> Tpen1R {
        Tpen1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TAMPER1 pin active level"]
    #[inline(always)]
    pub fn tpal1(&self) -> Tpal1R {
        Tpal1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - The second waveform detection enable"]
    #[inline(always)]
    pub fn tpm2(&self) -> Tpm2R {
        Tpm2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The first waveform detection enable"]
    #[inline(always)]
    pub fn tpm1(&self) -> Tpm1R {
        Tpm1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - TAMPER1 detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpen1(&mut self) -> Tpen1W<Tpctl1Spec> {
        Tpen1W::new(self, 8)
    }
    #[doc = "Bit 9 - TAMPER1 pin active level"]
    #[inline(always)]
    #[must_use]
    pub fn tpal1(&mut self) -> Tpal1W<Tpctl1Spec> {
        Tpal1W::new(self, 9)
    }
    #[doc = "Bit 14 - The second waveform detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpm2(&mut self) -> Tpm2W<Tpctl1Spec> {
        Tpm2W::new(self, 14)
    }
    #[doc = "Bit 15 - The first waveform detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpm1(&mut self) -> Tpm1W<Tpctl1Spec> {
        Tpm1W::new(self, 15)
    }
}
#[doc = "Tamper pin control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tpctl1Spec;
impl crate::RegisterSpec for Tpctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpctl1::R`](R) reader structure"]
impl crate::Readable for Tpctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`tpctl1::W`](W) writer structure"]
impl crate::Writable for Tpctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPCTL1 to value 0"]
impl crate::Resettable for Tpctl1Spec {
    const RESET_VALUE: u32 = 0;
}
