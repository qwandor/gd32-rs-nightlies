#[doc = "Register `CPPOS` reader"]
pub type R = crate::R<CpposSpec>;
#[doc = "Field `VPOS` reader - Vertical position"]
pub type VposR = crate::FieldReader<u16>;
#[doc = "Field `HPOS` reader - Horizontal position"]
pub type HposR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Vertical position"]
    #[inline(always)]
    pub fn vpos(&self) -> VposR {
        VposR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Horizontal position"]
    #[inline(always)]
    pub fn hpos(&self) -> HposR {
        HposR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Current pixel position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cppos::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpposSpec;
impl crate::RegisterSpec for CpposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cppos::R`](R) reader structure"]
impl crate::Readable for CpposSpec {}
#[doc = "`reset()` method sets CPPOS to value 0"]
impl crate::Resettable for CpposSpec {
    const RESET_VALUE: u32 = 0;
}
