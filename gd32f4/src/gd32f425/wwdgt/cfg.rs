#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `WIN` reader - 7-bit window value"]
pub type WinR = crate::FieldReader;
#[doc = "Field `WIN` writer - 7-bit window value"]
pub type WinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PSC` reader - Prescaler"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EWIE` reader - Early wakeup interrupt"]
pub type EwieR = crate::BitReader;
#[doc = "Field `EWIE` writer - Early wakeup interrupt"]
pub type EwieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewie(&self) -> EwieR {
        EwieR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WinW<CfgSpec> {
        WinW::new(self, 0)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PscW<CfgSpec> {
        PscW::new(self, 7)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EwieW<CfgSpec> {
        EwieW::new(self, 9)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0x7f"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x7f;
}
