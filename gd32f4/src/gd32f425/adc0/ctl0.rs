#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `WDCHSEL` reader - Analog watchdog channel select"]
pub type WdchselR = crate::FieldReader;
#[doc = "Field `WDCHSEL` writer - Analog watchdog channel select"]
pub type WdchselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EocieR = crate::BitReader;
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDEIE` reader - Analog watchdog WDE"]
pub type WdeieR = crate::BitReader;
#[doc = "Field `WDEIE` writer - Analog watchdog WDE"]
pub type WdeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOICIE` reader - Interrupt enable for EOIC"]
pub type EoicieR = crate::BitReader;
#[doc = "Field `EOICIE` writer - Interrupt enable for EOIC"]
pub type EoicieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM` reader - Scan mode"]
pub type SmR = crate::BitReader;
#[doc = "Field `SM` writer - Scan mode"]
pub type SmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDSC` reader - When in scan mode, analog watchdog is effective on a single channel"]
pub type WdscR = crate::BitReader;
#[doc = "Field `WDSC` writer - When in scan mode, analog watchdog is effective on a single channel"]
pub type WdscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICA` reader - Inserted channel group convert automatically"]
pub type IcaR = crate::BitReader;
#[doc = "Field `ICA` writer - Inserted channel group convert automatically"]
pub type IcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISRC` reader - Discontinuous mode on regular channels"]
pub type DisrcR = crate::BitReader;
#[doc = "Field `DISRC` writer - Discontinuous mode on regular channels"]
pub type DisrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISIC` reader - Discontinuous mode on inserted channels"]
pub type DisicR = crate::BitReader;
#[doc = "Field `DISIC` writer - Discontinuous mode on inserted channels"]
pub type DisicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISNUM` reader - Number of conversions in discontinuous mode"]
pub type DisnumR = crate::FieldReader;
#[doc = "Field `DISNUM` writer - Number of conversions in discontinuous mode"]
pub type DisnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IWDEN` reader - Inserted channel analog watchdog enable"]
pub type IwdenR = crate::BitReader;
#[doc = "Field `IWDEN` writer - Inserted channel analog watchdog enable"]
pub type IwdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWDEN` reader - Regular channel analog watchdog enable"]
pub type RwdenR = crate::BitReader;
#[doc = "Field `RWDEN` writer - Regular channel analog watchdog enable"]
pub type RwdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRES` reader - ADC data resolution"]
pub type DresR = crate::FieldReader;
#[doc = "Field `DRES` writer - ADC data resolution"]
pub type DresW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ROVFIE` reader - Interrupt enable for ROVF"]
pub type RovfieR = crate::BitReader;
#[doc = "Field `ROVFIE` writer - Interrupt enable for ROVF"]
pub type RovfieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    pub fn wdchsel(&self) -> WdchselR {
        WdchselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog WDE"]
    #[inline(always)]
    pub fn wdeie(&self) -> WdeieR {
        WdeieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&self) -> EoicieR {
        EoicieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    pub fn wdsc(&self) -> WdscR {
        WdscR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    pub fn ica(&self) -> IcaR {
        IcaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn disrc(&self) -> DisrcR {
        DisrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    pub fn disic(&self) -> DisicR {
        DisicR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&self) -> DisnumR {
        DisnumR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    pub fn iwden(&self) -> IwdenR {
        IwdenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    pub fn rwden(&self) -> RwdenR {
        RwdenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - ADC data resolution"]
    #[inline(always)]
    pub fn dres(&self) -> DresR {
        DresR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Interrupt enable for ROVF"]
    #[inline(always)]
    pub fn rovfie(&self) -> RovfieR {
        RovfieR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    #[must_use]
    pub fn wdchsel(&mut self) -> WdchselW<Ctl0Spec> {
        WdchselW::new(self, 0)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EocieW<Ctl0Spec> {
        EocieW::new(self, 5)
    }
    #[doc = "Bit 6 - Analog watchdog WDE"]
    #[inline(always)]
    #[must_use]
    pub fn wdeie(&mut self) -> WdeieW<Ctl0Spec> {
        WdeieW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    #[must_use]
    pub fn eoicie(&mut self) -> EoicieW<Ctl0Spec> {
        EoicieW::new(self, 7)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SmW<Ctl0Spec> {
        SmW::new(self, 8)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    #[must_use]
    pub fn wdsc(&mut self) -> WdscW<Ctl0Spec> {
        WdscW::new(self, 9)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    #[must_use]
    pub fn ica(&mut self) -> IcaW<Ctl0Spec> {
        IcaW::new(self, 10)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn disrc(&mut self) -> DisrcW<Ctl0Spec> {
        DisrcW::new(self, 11)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    #[must_use]
    pub fn disic(&mut self) -> DisicW<Ctl0Spec> {
        DisicW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn disnum(&mut self) -> DisnumW<Ctl0Spec> {
        DisnumW::new(self, 13)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn iwden(&mut self) -> IwdenW<Ctl0Spec> {
        IwdenW::new(self, 22)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwden(&mut self) -> RwdenW<Ctl0Spec> {
        RwdenW::new(self, 23)
    }
    #[doc = "Bits 24:25 - ADC data resolution"]
    #[inline(always)]
    #[must_use]
    pub fn dres(&mut self) -> DresW<Ctl0Spec> {
        DresW::new(self, 24)
    }
    #[doc = "Bit 26 - Interrupt enable for ROVF"]
    #[inline(always)]
    #[must_use]
    pub fn rovfie(&mut self) -> RovfieW<Ctl0Spec> {
        RovfieW::new(self, 26)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
