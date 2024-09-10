#[doc = "Register `ADDAPB2EN` reader"]
pub type R = crate::R<Addapb2enSpec>;
#[doc = "Register `ADDAPB2EN` writer"]
pub type W = crate::W<Addapb2enSpec>;
#[doc = "Field `USART5EN` reader - USART5 clock enable"]
pub type Usart5enR = crate::BitReader;
#[doc = "Field `USART5EN` writer - USART5 clock enable"]
pub type Usart5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLIEN` reader - TLI clock enable"]
pub type TlienR = crate::BitReader;
#[doc = "Field `TLIEN` writer - TLI clock enable"]
pub type TlienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHEN` reader - GPIO port H clock enable"]
pub type PhenR = crate::BitReader;
#[doc = "Field `PHEN` writer - GPIO port H clock enable"]
pub type PhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIEN` reader - GPIO port I clock enable"]
pub type PienR = crate::BitReader;
#[doc = "Field `PIEN` writer - GPIO port I clock enable"]
pub type PienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5en(&self) -> Usart5enR {
        Usart5enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TLI clock enable"]
    #[inline(always)]
    pub fn tlien(&self) -> TlienR {
        TlienR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO port H clock enable"]
    #[inline(always)]
    pub fn phen(&self) -> PhenR {
        PhenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO port I clock enable"]
    #[inline(always)]
    pub fn pien(&self) -> PienR {
        PienR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart5en(&mut self) -> Usart5enW<Addapb2enSpec> {
        Usart5enW::new(self, 24)
    }
    #[doc = "Bit 26 - TLI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tlien(&mut self) -> TlienW<Addapb2enSpec> {
        TlienW::new(self, 26)
    }
    #[doc = "Bit 30 - GPIO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn phen(&mut self) -> PhenW<Addapb2enSpec> {
        PhenW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO port I clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pien(&mut self) -> PienW<Addapb2enSpec> {
        PienW::new(self, 31)
    }
}
#[doc = "APB2 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb2en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb2en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addapb2enSpec;
impl crate::RegisterSpec for Addapb2enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb2en::R`](R) reader structure"]
impl crate::Readable for Addapb2enSpec {}
#[doc = "`write(|w| ..)` method takes [`addapb2en::W`](W) writer structure"]
impl crate::Writable for Addapb2enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDAPB2EN to value 0"]
impl crate::Resettable for Addapb2enSpec {
    const RESET_VALUE: u32 = 0;
}
