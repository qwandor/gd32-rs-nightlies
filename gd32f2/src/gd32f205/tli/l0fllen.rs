#[doc = "Register `L0FLLEN` reader"]
pub type R = crate::R<L0fllenSpec>;
#[doc = "Register `L0FLLEN` writer"]
pub type W = crate::W<L0fllenSpec>;
#[doc = "Field `FLL` reader - Frame Line Length"]
pub type FllR = crate::FieldReader<u16>;
#[doc = "Field `FLL` writer - Frame Line Length"]
pub type FllW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `STDOFF` reader - Frame Buffer Stride Offset"]
pub type StdoffR = crate::FieldReader<u16>;
#[doc = "Field `STDOFF` writer - Frame Buffer Stride Offset"]
pub type StdoffW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Frame Line Length"]
    #[inline(always)]
    pub fn fll(&self) -> FllR {
        FllR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Frame Buffer Stride Offset"]
    #[inline(always)]
    pub fn stdoff(&self) -> StdoffR {
        StdoffR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Line Length"]
    #[inline(always)]
    #[must_use]
    pub fn fll(&mut self) -> FllW<L0fllenSpec> {
        FllW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Frame Buffer Stride Offset"]
    #[inline(always)]
    #[must_use]
    pub fn stdoff(&mut self) -> StdoffW<L0fllenSpec> {
        StdoffW::new(self, 16)
    }
}
#[doc = "Layer 0 frame line length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l0fllen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l0fllen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0fllenSpec;
impl crate::RegisterSpec for L0fllenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0fllen::R`](R) reader structure"]
impl crate::Readable for L0fllenSpec {}
#[doc = "`write(|w| ..)` method takes [`l0fllen::W`](W) writer structure"]
impl crate::Writable for L0fllenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L0FLLEN to value 0"]
impl crate::Resettable for L0fllenSpec {
    const RESET_VALUE: u32 = 0;
}
