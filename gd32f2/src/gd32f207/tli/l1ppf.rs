#[doc = "Register `L1PPF` reader"]
pub type R = crate::R<L1ppfSpec>;
#[doc = "Register `L1PPF` writer"]
pub type W = crate::W<L1ppfSpec>;
#[doc = "Field `PPF` reader - Packeted Pixel Format"]
pub type PpfR = crate::FieldReader;
#[doc = "Field `PPF` writer - Packeted Pixel Format"]
pub type PpfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Packeted Pixel Format"]
    #[inline(always)]
    pub fn ppf(&self) -> PpfR {
        PpfR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Packeted Pixel Format"]
    #[inline(always)]
    #[must_use]
    pub fn ppf(&mut self) -> PpfW<L1ppfSpec> {
        PpfW::new(self, 0)
    }
}
#[doc = "Layer 1 packeted pixel format register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1ppf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1ppf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1ppfSpec;
impl crate::RegisterSpec for L1ppfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1ppf::R`](R) reader structure"]
impl crate::Readable for L1ppfSpec {}
#[doc = "`write(|w| ..)` method takes [`l1ppf::W`](W) writer structure"]
impl crate::Writable for L1ppfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1PPF to value 0"]
impl crate::Resettable for L1ppfSpec {
    const RESET_VALUE: u32 = 0;
}
