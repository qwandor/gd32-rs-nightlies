#[doc = "Register `AHB2SPEN` reader"]
pub type R = crate::R<Ahb2spenSpec>;
#[doc = "Register `AHB2SPEN` writer"]
pub type W = crate::W<Ahb2spenSpec>;
#[doc = "Field `DCISPEN` reader - DCI clock enable when sleep mode"]
pub type DcispenR = crate::BitReader;
#[doc = "Field `DCISPEN` writer - DCI clock enable when sleep mode"]
pub type DcispenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNGSPEN` reader - TRNG clock enable when sleep mode"]
pub type TrngspenR = crate::BitReader;
#[doc = "Field `TRNGSPEN` writer - TRNG clock enable when sleep mode"]
pub type TrngspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBFSSPEN` reader - USBFS clock enable when sleep mode"]
pub type UsbfsspenR = crate::BitReader;
#[doc = "Field `USBFSSPEN` writer - USBFS clock enable when sleep mode"]
pub type UsbfsspenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCI clock enable when sleep mode"]
    #[inline(always)]
    pub fn dcispen(&self) -> DcispenR {
        DcispenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG clock enable when sleep mode"]
    #[inline(always)]
    pub fn trngspen(&self) -> TrngspenR {
        TrngspenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USBFS clock enable when sleep mode"]
    #[inline(always)]
    pub fn usbfsspen(&self) -> UsbfsspenR {
        UsbfsspenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dcispen(&mut self) -> DcispenW<Ahb2spenSpec> {
        DcispenW::new(self, 0)
    }
    #[doc = "Bit 6 - TRNG clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn trngspen(&mut self) -> TrngspenW<Ahb2spenSpec> {
        TrngspenW::new(self, 6)
    }
    #[doc = "Bit 7 - USBFS clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsspen(&mut self) -> UsbfsspenW<Ahb2spenSpec> {
        UsbfsspenW::new(self, 7)
    }
}
#[doc = "AHB2 sleep mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2spen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2spen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2spenSpec;
impl crate::RegisterSpec for Ahb2spenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2spen::R`](R) reader structure"]
impl crate::Readable for Ahb2spenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2spen::W`](W) writer structure"]
impl crate::Writable for Ahb2spenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2SPEN to value 0xc1"]
impl crate::Resettable for Ahb2spenSpec {
    const RESET_VALUE: u32 = 0xc1;
}
