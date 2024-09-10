#[doc = "Register `FMPCFG` reader"]
pub type R = crate::R<FmpcfgSpec>;
#[doc = "Register `FMPCFG` writer"]
pub type W = crate::W<FmpcfgSpec>;
#[doc = "Field `FMPEN` reader - Fast-mode-plus enable"]
pub type FmpenR = crate::BitReader;
#[doc = "Field `FMPEN` writer - Fast-mode-plus enable"]
pub type FmpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fast-mode-plus enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FmpenR {
        FmpenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast-mode-plus enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpen(&mut self) -> FmpenW<FmpcfgSpec> {
        FmpenW::new(self, 0)
    }
}
#[doc = "Fast-mode-plus configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmpcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmpcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmpcfgSpec;
impl crate::RegisterSpec for FmpcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmpcfg::R`](R) reader structure"]
impl crate::Readable for FmpcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fmpcfg::W`](W) writer structure"]
impl crate::Writable for FmpcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMPCFG to value 0"]
impl crate::Resettable for FmpcfgSpec {
    const RESET_VALUE: u32 = 0;
}
