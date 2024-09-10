#[doc = "Register `OBCTL0` reader"]
pub type R = crate::R<Obctl0Spec>;
#[doc = "Register `OBCTL0` writer"]
pub type W = crate::W<Obctl0Spec>;
#[doc = "Field `OB_LK` reader - FMC_OBCTL0 lock bit"]
pub type ObLkR = crate::BitReader;
#[doc = "Field `OB_LK` writer - FMC_OBCTL0 lock bit"]
pub type ObLkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OB_START` reader - send option byte change command to FMC bit"]
pub type ObStartR = crate::BitReader;
#[doc = "Field `OB_START` writer - send option byte change command to FMC bit"]
pub type ObStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOR_TH` reader - option byte BOR threshold value"]
pub type BorThR = crate::FieldReader;
#[doc = "Field `BOR_TH` writer - option byte BOR threshold value"]
pub type BorThW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BB` reader - option byte boot bank value"]
pub type BbR = crate::BitReader;
#[doc = "Field `BB` writer - option byte boot bank value"]
pub type BbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nWDG_HW` reader - option byte watchdog value"]
pub type NWdgHwR = crate::BitReader;
#[doc = "Field `nWDG_HW` writer - option byte watchdog value"]
pub type NWdgHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_DPSLP` reader - option byte deepsleep reset value"]
pub type NRstDpslpR = crate::BitReader;
#[doc = "Field `nRST_DPSLP` writer - option byte deepsleep reset value"]
pub type NRstDpslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STDBY` reader - option byte standby reset value"]
pub type NRstStdbyR = crate::BitReader;
#[doc = "Field `nRST_STDBY` writer - option byte standby reset value"]
pub type NRstStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPC` reader - option byte Security Protection code"]
pub type SpcR = crate::FieldReader;
#[doc = "Field `SPC` writer - option byte Security Protection code"]
pub type SpcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WP0` reader - Erase/program protection of each sector when DRP is 0"]
pub type Wp0R = crate::FieldReader<u16>;
#[doc = "Field `WP0` writer - Erase/program protection of each sector when DRP is 0"]
pub type Wp0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DBS` reader - Double banks or single bank selection when flash size is 1M"]
pub type DbsR = crate::BitReader;
#[doc = "Field `DBS` writer - Double banks or single bank selection when flash size is 1M"]
pub type DbsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRP` reader - D-bus read protection bit"]
pub type DrpR = crate::BitReader;
#[doc = "Field `DRP` writer - D-bus read protection bit"]
pub type DrpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FMC_OBCTL0 lock bit"]
    #[inline(always)]
    pub fn ob_lk(&self) -> ObLkR {
        ObLkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - send option byte change command to FMC bit"]
    #[inline(always)]
    pub fn ob_start(&self) -> ObStartR {
        ObStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - option byte BOR threshold value"]
    #[inline(always)]
    pub fn bor_th(&self) -> BorThR {
        BorThR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - option byte boot bank value"]
    #[inline(always)]
    pub fn bb(&self) -> BbR {
        BbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - option byte watchdog value"]
    #[inline(always)]
    pub fn n_wdg_hw(&self) -> NWdgHwR {
        NWdgHwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - option byte deepsleep reset value"]
    #[inline(always)]
    pub fn n_rst_dpslp(&self) -> NRstDpslpR {
        NRstDpslpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - option byte standby reset value"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRstStdbyR {
        NRstStdbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - option byte Security Protection code"]
    #[inline(always)]
    pub fn spc(&self) -> SpcR {
        SpcR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:27 - Erase/program protection of each sector when DRP is 0"]
    #[inline(always)]
    pub fn wp0(&self) -> Wp0R {
        Wp0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - Double banks or single bank selection when flash size is 1M"]
    #[inline(always)]
    pub fn dbs(&self) -> DbsR {
        DbsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - D-bus read protection bit"]
    #[inline(always)]
    pub fn drp(&self) -> DrpR {
        DrpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMC_OBCTL0 lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn ob_lk(&mut self) -> ObLkW<Obctl0Spec> {
        ObLkW::new(self, 0)
    }
    #[doc = "Bit 1 - send option byte change command to FMC bit"]
    #[inline(always)]
    #[must_use]
    pub fn ob_start(&mut self) -> ObStartW<Obctl0Spec> {
        ObStartW::new(self, 1)
    }
    #[doc = "Bits 2:3 - option byte BOR threshold value"]
    #[inline(always)]
    #[must_use]
    pub fn bor_th(&mut self) -> BorThW<Obctl0Spec> {
        BorThW::new(self, 2)
    }
    #[doc = "Bit 4 - option byte boot bank value"]
    #[inline(always)]
    #[must_use]
    pub fn bb(&mut self) -> BbW<Obctl0Spec> {
        BbW::new(self, 4)
    }
    #[doc = "Bit 5 - option byte watchdog value"]
    #[inline(always)]
    #[must_use]
    pub fn n_wdg_hw(&mut self) -> NWdgHwW<Obctl0Spec> {
        NWdgHwW::new(self, 5)
    }
    #[doc = "Bit 6 - option byte deepsleep reset value"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_dpslp(&mut self) -> NRstDpslpW<Obctl0Spec> {
        NRstDpslpW::new(self, 6)
    }
    #[doc = "Bit 7 - option byte standby reset value"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> NRstStdbyW<Obctl0Spec> {
        NRstStdbyW::new(self, 7)
    }
    #[doc = "Bits 8:15 - option byte Security Protection code"]
    #[inline(always)]
    #[must_use]
    pub fn spc(&mut self) -> SpcW<Obctl0Spec> {
        SpcW::new(self, 8)
    }
    #[doc = "Bits 16:27 - Erase/program protection of each sector when DRP is 0"]
    #[inline(always)]
    #[must_use]
    pub fn wp0(&mut self) -> Wp0W<Obctl0Spec> {
        Wp0W::new(self, 16)
    }
    #[doc = "Bit 30 - Double banks or single bank selection when flash size is 1M"]
    #[inline(always)]
    #[must_use]
    pub fn dbs(&mut self) -> DbsW<Obctl0Spec> {
        DbsW::new(self, 30)
    }
    #[doc = "Bit 31 - D-bus read protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn drp(&mut self) -> DrpW<Obctl0Spec> {
        DrpW::new(self, 31)
    }
}
#[doc = "Option byte control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obctl0Spec;
impl crate::RegisterSpec for Obctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obctl0::R`](R) reader structure"]
impl crate::Readable for Obctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`obctl0::W`](W) writer structure"]
impl crate::Writable for Obctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OBCTL0 to value 0x2fff_aaed"]
impl crate::Resettable for Obctl0Spec {
    const RESET_VALUE: u32 = 0x2fff_aaed;
}
