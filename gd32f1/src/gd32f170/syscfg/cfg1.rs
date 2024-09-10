#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `SLCD_DECA` reader - Decoupling capacitance connection for SLCD"]
pub type SlcdDecaR = crate::FieldReader;
#[doc = "Field `SLCD_DECA` writer - Decoupling capacitance connection for SLCD"]
pub type SlcdDecaW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bits 1:3 - Decoupling capacitance connection for SLCD"]
    #[inline(always)]
    pub fn slcd_deca(&self) -> SlcdDecaR {
        SlcdDecaR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Decoupling capacitance connection for SLCD"]
    #[inline(always)]
    #[must_use]
    pub fn slcd_deca(&mut self) -> SlcdDecaW<Cfg1Spec> {
        SlcdDecaW::new(self, 1)
    }
}
#[doc = "System configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
