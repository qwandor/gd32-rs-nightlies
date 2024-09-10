#[doc = "Register `PTP_TSCTL` reader"]
pub type R = crate::R<PtpTsctlSpec>;
#[doc = "Register `PTP_TSCTL` writer"]
pub type W = crate::W<PtpTsctlSpec>;
#[doc = "Field `TMSEN` reader - Time stamp enable"]
pub type TmsenR = crate::BitReader;
#[doc = "Field `TMSEN` writer - Time stamp enable"]
pub type TmsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMSFCU` reader - Time stamp fine or coarse update"]
pub type TmsfcuR = crate::BitReader;
#[doc = "Field `TMSFCU` writer - Time stamp fine or coarse update"]
pub type TmsfcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMSSTI` reader - Time stamp system time initialize"]
pub type TmsstiR = crate::BitReader;
#[doc = "Field `TMSSTI` writer - Time stamp system time initialize"]
pub type TmsstiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMSSTU` reader - Time stamp system time update"]
pub type TmsstuR = crate::BitReader;
#[doc = "Field `TMSSTU` writer - Time stamp system time update"]
pub type TmsstuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMSITEN` reader - Time stamp interrupt trigger enable"]
pub type TmsitenR = crate::BitReader;
#[doc = "Field `TMSITEN` writer - Time stamp interrupt trigger enable"]
pub type TmsitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMSARU` reader - Time stamp addend register update"]
pub type TmsaruR = crate::BitReader;
#[doc = "Field `TMSARU` writer - Time stamp addend register update"]
pub type TmsaruW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    pub fn tmsen(&self) -> TmsenR {
        TmsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    pub fn tmsfcu(&self) -> TmsfcuR {
        TmsfcuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    pub fn tmssti(&self) -> TmsstiR {
        TmsstiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    pub fn tmsstu(&self) -> TmsstuR {
        TmsstuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tmsiten(&self) -> TmsitenR {
        TmsitenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    pub fn tmsaru(&self) -> TmsaruR {
        TmsaruR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmsen(&mut self) -> TmsenW<PtpTsctlSpec> {
        TmsenW::new(self, 0)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    #[must_use]
    pub fn tmsfcu(&mut self) -> TmsfcuW<PtpTsctlSpec> {
        TmsfcuW::new(self, 1)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    #[must_use]
    pub fn tmssti(&mut self) -> TmsstiW<PtpTsctlSpec> {
        TmsstiW::new(self, 2)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    #[must_use]
    pub fn tmsstu(&mut self) -> TmsstuW<PtpTsctlSpec> {
        TmsstuW::new(self, 3)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmsiten(&mut self) -> TmsitenW<PtpTsctlSpec> {
        TmsitenW::new(self, 4)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    #[must_use]
    pub fn tmsaru(&mut self) -> TmsaruW<PtpTsctlSpec> {
        TmsaruW::new(self, 5)
    }
}
#[doc = "Ethernet PTP time stamp control register (PTP_TSCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptp_tsctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptp_tsctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtpTsctlSpec;
impl crate::RegisterSpec for PtpTsctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptp_tsctl::R`](R) reader structure"]
impl crate::Readable for PtpTsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ptp_tsctl::W`](W) writer structure"]
impl crate::Writable for PtpTsctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTP_TSCTL to value 0"]
impl crate::Resettable for PtpTsctlSpec {
    const RESET_VALUE: u32 = 0;
}
