#[doc = "Register `L1FTLN` reader"]
pub type R = crate::R<L1ftlnSpec>;
#[doc = "Register `L1FTLN` writer"]
pub type W = crate::W<L1ftlnSpec>;
#[doc = "Field `FTLN` reader - Frame Total Line Number"]
pub type FtlnR = crate::FieldReader<u16>;
#[doc = "Field `FTLN` writer - Frame Total Line Number"]
pub type FtlnW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Frame Total Line Number"]
    #[inline(always)]
    pub fn ftln(&self) -> FtlnR {
        FtlnR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Total Line Number"]
    #[inline(always)]
    #[must_use]
    pub fn ftln(&mut self) -> FtlnW<L1ftlnSpec> {
        FtlnW::new(self, 0)
    }
}
#[doc = "Layer 1 frame total line number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ftln::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ftln::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1ftlnSpec;
impl crate::RegisterSpec for L1ftlnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1ftln::R`](R) reader structure"]
impl crate::Readable for L1ftlnSpec {}
#[doc = "`write(|w| ..)` method takes [`l1ftln::W`](W) writer structure"]
impl crate::Writable for L1ftlnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1FTLN to value 0"]
impl crate::Resettable for L1ftlnSpec {
    const RESET_VALUE: u32 = 0;
}
