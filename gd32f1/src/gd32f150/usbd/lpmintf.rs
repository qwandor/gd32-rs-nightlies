#[doc = "Register `LPMINTF` reader"]
pub type R = crate::R<LpmintfSpec>;
#[doc = "Register `LPMINTF` writer"]
pub type W = crate::W<LpmintfSpec>;
#[doc = "Field `LPMSTIF` reader - LPM token Correct transfer interrupt flag"]
pub type LpmstifR = crate::BitReader;
#[doc = "Field `LPMSTIF` writer - LPM token Correct transfer interrupt flag"]
pub type LpmstifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - LPM token Correct transfer interrupt flag"]
    #[inline(always)]
    pub fn lpmstif(&self) -> LpmstifR {
        LpmstifR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - LPM token Correct transfer interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn lpmstif(&mut self) -> LpmstifW<LpmintfSpec> {
        LpmstifW::new(self, 15)
    }
}
#[doc = "USB LPM interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmintf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmintf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmintfSpec;
impl crate::RegisterSpec for LpmintfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lpmintf::R`](R) reader structure"]
impl crate::Readable for LpmintfSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmintf::W`](W) writer structure"]
impl crate::Writable for LpmintfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets LPMINTF to value 0"]
impl crate::Resettable for LpmintfSpec {
    const RESET_VALUE: u16 = 0;
}
