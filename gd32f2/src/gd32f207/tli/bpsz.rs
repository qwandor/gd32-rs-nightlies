#[doc = "Register `BPSZ` reader"]
pub type R = crate::R<BpszSpec>;
#[doc = "Register `BPSZ` writer"]
pub type W = crate::W<BpszSpec>;
#[doc = "Field `VBPSZ` reader - Size of the vertical back porch plus synchronous pulse"]
pub type VbpszR = crate::FieldReader<u16>;
#[doc = "Field `VBPSZ` writer - Size of the vertical back porch plus synchronous pulse"]
pub type VbpszW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HBPSZ` reader - Size of the horizontal back porch plus synchronous pulse"]
pub type HbpszR = crate::FieldReader<u16>;
#[doc = "Field `HBPSZ` writer - Size of the horizontal back porch plus synchronous pulse"]
pub type HbpszW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Size of the vertical back porch plus synchronous pulse"]
    #[inline(always)]
    pub fn vbpsz(&self) -> VbpszR {
        VbpszR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Size of the horizontal back porch plus synchronous pulse"]
    #[inline(always)]
    pub fn hbpsz(&self) -> HbpszR {
        HbpszR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Size of the vertical back porch plus synchronous pulse"]
    #[inline(always)]
    #[must_use]
    pub fn vbpsz(&mut self) -> VbpszW<BpszSpec> {
        VbpszW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Size of the horizontal back porch plus synchronous pulse"]
    #[inline(always)]
    #[must_use]
    pub fn hbpsz(&mut self) -> HbpszW<BpszSpec> {
        HbpszW::new(self, 16)
    }
}
#[doc = "Back-porch size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpsz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpsz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpszSpec;
impl crate::RegisterSpec for BpszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpsz::R`](R) reader structure"]
impl crate::Readable for BpszSpec {}
#[doc = "`write(|w| ..)` method takes [`bpsz::W`](W) writer structure"]
impl crate::Writable for BpszSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPSZ to value 0"]
impl crate::Resettable for BpszSpec {
    const RESET_VALUE: u32 = 0;
}
