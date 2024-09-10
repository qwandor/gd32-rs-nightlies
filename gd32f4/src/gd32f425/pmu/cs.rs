#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WufR = crate::BitReader;
#[doc = "Field `STBF` reader - Standby flag"]
pub type StbfR = crate::BitReader;
#[doc = "Field `LVDF` reader - Low Voltage Detector Status Flag"]
pub type LvdfR = crate::BitReader;
#[doc = "Field `BLDORF` reader - Backup SRAM LDO ready flag"]
pub type BldorfR = crate::BitReader;
#[doc = "Field `WUPEN` reader - Enable WKUP pin"]
pub type WupenR = crate::BitReader;
#[doc = "Field `WUPEN` writer - Enable WKUP pin"]
pub type WupenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLDOON` reader - Backup SRAM LDO on"]
pub type BldoonR = crate::BitReader;
#[doc = "Field `BLDOON` writer - Backup SRAM LDO on"]
pub type BldoonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDOVSRF` reader - LDO voltage select ready flag"]
pub type LdovsrfR = crate::BitReader;
#[doc = "Field `HDRF` reader - High-driver ready flag"]
pub type HdrfR = crate::BitReader;
#[doc = "Field `HDSRF` reader - High-driver switch ready flag"]
pub type HdsrfR = crate::BitReader;
#[doc = "Field `LDRF` reader - Low-driver mode ready flag"]
pub type LdrfR = crate::FieldReader;
#[doc = "Field `LDRF` writer - Low-driver mode ready flag"]
pub type LdrfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn stbf(&self) -> StbfR {
        StbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Voltage Detector Status Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LvdfR {
        LvdfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Backup SRAM LDO ready flag"]
    #[inline(always)]
    pub fn bldorf(&self) -> BldorfR {
        BldorfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn wupen(&self) -> WupenR {
        WupenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Backup SRAM LDO on"]
    #[inline(always)]
    pub fn bldoon(&self) -> BldoonR {
        BldoonR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - LDO voltage select ready flag"]
    #[inline(always)]
    pub fn ldovsrf(&self) -> LdovsrfR {
        LdovsrfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - High-driver ready flag"]
    #[inline(always)]
    pub fn hdrf(&self) -> HdrfR {
        HdrfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High-driver switch ready flag"]
    #[inline(always)]
    pub fn hdsrf(&self) -> HdsrfR {
        HdsrfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Low-driver mode ready flag"]
    #[inline(always)]
    pub fn ldrf(&self) -> LdrfR {
        LdrfR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WupenW<CsSpec> {
        WupenW::new(self, 8)
    }
    #[doc = "Bit 9 - Backup SRAM LDO on"]
    #[inline(always)]
    #[must_use]
    pub fn bldoon(&mut self) -> BldoonW<CsSpec> {
        BldoonW::new(self, 9)
    }
    #[doc = "Bits 18:19 - Low-driver mode ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn ldrf(&mut self) -> LdrfW<CsSpec> {
        LdrfW::new(self, 18)
    }
}
#[doc = "power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsSpec;
impl crate::RegisterSpec for CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CsSpec {
    const RESET_VALUE: u32 = 0;
}
