#[doc = "Register `CPCPOLY` reader"]
pub type R = crate::R<CpcpolySpec>;
#[doc = "Register `CPCPOLY` writer"]
pub type W = crate::W<CpcpolySpec>;
#[doc = "Field `CPR` reader - CRC polynomial register"]
pub type CprR = crate::FieldReader<u16>;
#[doc = "Field `CPR` writer - CRC polynomial register"]
pub type CprW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    pub fn cpr(&self) -> CprR {
        CprR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CprW<CpcpolySpec> {
        CprW::new(self, 0)
    }
}
#[doc = "CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpcpoly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpcpoly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpcpolySpec;
impl crate::RegisterSpec for CpcpolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpcpoly::R`](R) reader structure"]
impl crate::Readable for CpcpolySpec {}
#[doc = "`write(|w| ..)` method takes [`cpcpoly::W`](W) writer structure"]
impl crate::Writable for CpcpolySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPCPOLY to value 0x07"]
impl crate::Resettable for CpcpolySpec {
    const RESET_VALUE: u32 = 0x07;
}
