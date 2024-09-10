#[doc = "Register `LPMCTL` reader"]
pub type R = crate::R<LpmctlSpec>;
#[doc = "Register `LPMCTL` writer"]
pub type W = crate::W<LpmctlSpec>;
#[doc = "Field `LPMSTIE` reader - LPM token successful transfer interrupt enable"]
pub type LpmstieR = crate::BitReader;
#[doc = "Field `LPMSTIE` writer - LPM token successful transfer interrupt enable"]
pub type LpmstieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - LPM token successful transfer interrupt enable"]
    #[inline(always)]
    pub fn lpmstie(&self) -> LpmstieR {
        LpmstieR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - LPM token successful transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmstie(&mut self) -> LpmstieW<LpmctlSpec> {
        LpmstieW::new(self, 15)
    }
}
#[doc = "USB LPM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmctlSpec;
impl crate::RegisterSpec for LpmctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lpmctl::R`](R) reader structure"]
impl crate::Readable for LpmctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmctl::W`](W) writer structure"]
impl crate::Writable for LpmctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets LPMCTL to value 0"]
impl crate::Resettable for LpmctlSpec {
    const RESET_VALUE: u16 = 0;
}
