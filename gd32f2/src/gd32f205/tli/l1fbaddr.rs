#[doc = "Register `L1FBADDR` reader"]
pub type R = crate::R<L1fbaddrSpec>;
#[doc = "Register `L1FBADDR` writer"]
pub type W = crate::W<L1fbaddrSpec>;
#[doc = "Field `FBADD` reader - Frame Buffer base Address"]
pub type FbaddR = crate::FieldReader<u32>;
#[doc = "Field `FBADD` writer - Frame Buffer base Address"]
pub type FbaddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Frame Buffer base Address"]
    #[inline(always)]
    pub fn fbadd(&self) -> FbaddR {
        FbaddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frame Buffer base Address"]
    #[inline(always)]
    #[must_use]
    pub fn fbadd(&mut self) -> FbaddW<L1fbaddrSpec> {
        FbaddW::new(self, 0)
    }
}
#[doc = "Layer 1 frame base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1fbaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1fbaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1fbaddrSpec;
impl crate::RegisterSpec for L1fbaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1fbaddr::R`](R) reader structure"]
impl crate::Readable for L1fbaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1fbaddr::W`](W) writer structure"]
impl crate::Writable for L1fbaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1FBADDR to value 0"]
impl crate::Resettable for L1fbaddrSpec {
    const RESET_VALUE: u32 = 0;
}
