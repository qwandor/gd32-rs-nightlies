#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<IntfSpec>;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub type UpifR = crate::BitReader;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub type UpifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UpifR {
        UpifR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn upif(&mut self) -> UpifW<IntfSpec> {
        UpifW::new(self, 0)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for IntfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {
    const RESET_VALUE: u32 = 0;
}
