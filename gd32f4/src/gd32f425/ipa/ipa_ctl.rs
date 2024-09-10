#[doc = "Register `IPA_CTL` reader"]
pub type R = crate::R<IpaCtlSpec>;
#[doc = "Register `IPA_CTL` writer"]
pub type W = crate::W<IpaCtlSpec>;
#[doc = "Field `TEN` reader - Transfer enable"]
pub type TenR = crate::BitReader;
#[doc = "Field `TEN` writer - Transfer enable"]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THU` reader - Transfer hang up"]
pub type ThuR = crate::BitReader;
#[doc = "Field `THU` writer - Transfer hang up"]
pub type ThuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST` reader - Transfer stop"]
pub type TstR = crate::BitReader;
#[doc = "Field `TST` writer - Transfer stop"]
pub type TstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEIE` reader - Enable bit for transfer access error interrupt"]
pub type TaeieR = crate::BitReader;
#[doc = "Field `TAEIE` writer - Enable bit for transfer access error interrupt"]
pub type TaeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIE` reader - Enable bit for full transfer finish interrupt"]
pub type FtfieR = crate::BitReader;
#[doc = "Field `FTFIE` writer - Enable bit for full transfer finish interrupt"]
pub type FtfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLMIE` reader - Enable bit for transfer line mark interrupt"]
pub type TlmieR = crate::BitReader;
#[doc = "Field `TLMIE` writer - Enable bit for transfer line mark interrupt"]
pub type TlmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LACIE` reader - Enable bit for LUT access conflict interrupt"]
pub type LacieR = crate::BitReader;
#[doc = "Field `LACIE` writer - Enable bit for LUT access conflict interrupt"]
pub type LacieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLFIE` reader - Enable bit for LUT loading finish interrupt"]
pub type LlfieR = crate::BitReader;
#[doc = "Field `LLFIE` writer - Enable bit for LUT loading finish interrupt"]
pub type LlfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCFIE` reader - Enable bit for wrong configuration interrupt"]
pub type WcfieR = crate::BitReader;
#[doc = "Field `WCFIE` writer - Enable bit for wrong configuration interrupt"]
pub type WcfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFCM` reader - Pixel format convert mode"]
pub type PfcmR = crate::FieldReader;
#[doc = "Field `PFCM` writer - Pixel format convert mode"]
pub type PfcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Transfer enable"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer hang up"]
    #[inline(always)]
    pub fn thu(&self) -> ThuR {
        ThuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer stop"]
    #[inline(always)]
    pub fn tst(&self) -> TstR {
        TstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable bit for transfer access error interrupt"]
    #[inline(always)]
    pub fn taeie(&self) -> TaeieR {
        TaeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable bit for full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&self) -> FtfieR {
        FtfieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable bit for transfer line mark interrupt"]
    #[inline(always)]
    pub fn tlmie(&self) -> TlmieR {
        TlmieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable bit for LUT access conflict interrupt"]
    #[inline(always)]
    pub fn lacie(&self) -> LacieR {
        LacieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable bit for LUT loading finish interrupt"]
    #[inline(always)]
    pub fn llfie(&self) -> LlfieR {
        LlfieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable bit for wrong configuration interrupt"]
    #[inline(always)]
    pub fn wcfie(&self) -> WcfieR {
        WcfieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Pixel format convert mode"]
    #[inline(always)]
    pub fn pfcm(&self) -> PfcmR {
        PfcmR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TenW<IpaCtlSpec> {
        TenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer hang up"]
    #[inline(always)]
    #[must_use]
    pub fn thu(&mut self) -> ThuW<IpaCtlSpec> {
        ThuW::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer stop"]
    #[inline(always)]
    #[must_use]
    pub fn tst(&mut self) -> TstW<IpaCtlSpec> {
        TstW::new(self, 2)
    }
    #[doc = "Bit 8 - Enable bit for transfer access error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn taeie(&mut self) -> TaeieW<IpaCtlSpec> {
        TaeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable bit for full transfer finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ftfie(&mut self) -> FtfieW<IpaCtlSpec> {
        FtfieW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable bit for transfer line mark interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tlmie(&mut self) -> TlmieW<IpaCtlSpec> {
        TlmieW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable bit for LUT access conflict interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lacie(&mut self) -> LacieW<IpaCtlSpec> {
        LacieW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable bit for LUT loading finish interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn llfie(&mut self) -> LlfieW<IpaCtlSpec> {
        LlfieW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable bit for wrong configuration interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wcfie(&mut self) -> WcfieW<IpaCtlSpec> {
        WcfieW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Pixel format convert mode"]
    #[inline(always)]
    #[must_use]
    pub fn pfcm(&mut self) -> PfcmW<IpaCtlSpec> {
        PfcmW::new(self, 16)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaCtlSpec;
impl crate::RegisterSpec for IpaCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_ctl::R`](R) reader structure"]
impl crate::Readable for IpaCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_ctl::W`](W) writer structure"]
impl crate::Writable for IpaCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_CTL to value 0"]
impl crate::Resettable for IpaCtlSpec {
    const RESET_VALUE: u32 = 0;
}
