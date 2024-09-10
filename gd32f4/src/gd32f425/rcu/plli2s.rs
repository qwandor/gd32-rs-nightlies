#[doc = "Register `PLLI2S` reader"]
pub type R = crate::R<Plli2sSpec>;
#[doc = "Register `PLLI2S` writer"]
pub type W = crate::W<Plli2sSpec>;
#[doc = "Field `PLLI2SPSC` reader - The PLLI2S VCO source clock prescaler"]
pub type Plli2spscR = crate::FieldReader;
#[doc = "Field `PLLI2SPSC` writer - The PLLI2S VCO source clock prescaler"]
pub type Plli2spscW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLLI2SN` reader - The PLLI2S VCO clock multi factor"]
pub type Plli2snR = crate::FieldReader<u16>;
#[doc = "Field `PLLI2SN` writer - The PLLI2S VCO clock multi factor"]
pub type Plli2snW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLLI2SQ` reader - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
pub type Plli2sqR = crate::FieldReader;
#[doc = "Field `PLLI2SQ` writer - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
pub type Plli2sqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLI2SR` reader - The PLLI2S R output frequency division factor from PLLI2S VCO clock"]
pub type Plli2srR = crate::FieldReader;
#[doc = "Field `PLLI2SR` writer - The PLLI2S R output frequency division factor from PLLI2S VCO clock"]
pub type Plli2srW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - The PLLI2S VCO source clock prescaler"]
    #[inline(always)]
    pub fn plli2spsc(&self) -> Plli2spscR {
        Plli2spscR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - The PLLI2S VCO clock multi factor"]
    #[inline(always)]
    pub fn plli2sn(&self) -> Plli2snR {
        Plli2snR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:27 - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    pub fn plli2sq(&self) -> Plli2sqR {
        Plli2sqR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - The PLLI2S R output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    pub fn plli2sr(&self) -> Plli2srR {
        Plli2srR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The PLLI2S VCO source clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn plli2spsc(&mut self) -> Plli2spscW<Plli2sSpec> {
        Plli2spscW::new(self, 0)
    }
    #[doc = "Bits 6:14 - The PLLI2S VCO clock multi factor"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sn(&mut self) -> Plli2snW<Plli2sSpec> {
        Plli2snW::new(self, 6)
    }
    #[doc = "Bits 24:27 - The PLLI2S Q output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sq(&mut self) -> Plli2sqW<Plli2sSpec> {
        Plli2sqW::new(self, 24)
    }
    #[doc = "Bits 28:30 - The PLLI2S R output frequency division factor from PLLI2S VCO clock"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sr(&mut self) -> Plli2srW<Plli2sSpec> {
        Plli2srW::new(self, 28)
    }
}
#[doc = "PLLI2S register (RCU_PLLI2S)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plli2s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plli2s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Plli2sSpec;
impl crate::RegisterSpec for Plli2sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plli2s::R`](R) reader structure"]
impl crate::Readable for Plli2sSpec {}
#[doc = "`write(|w| ..)` method takes [`plli2s::W`](W) writer structure"]
impl crate::Writable for Plli2sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLI2S to value 0x016e_41b8"]
impl crate::Resettable for Plli2sSpec {
    const RESET_VALUE: u32 = 0x016e_41b8;
}
