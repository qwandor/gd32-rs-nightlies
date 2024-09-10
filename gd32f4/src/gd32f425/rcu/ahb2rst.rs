#[doc = "Register `AHB2RST` reader"]
pub type R = crate::R<Ahb2rstSpec>;
#[doc = "Register `AHB2RST` writer"]
pub type W = crate::W<Ahb2rstSpec>;
#[doc = "Field `DCIRST` reader - DCI reset"]
pub type DcirstR = crate::BitReader;
#[doc = "Field `DCIRST` writer - DCI reset"]
pub type DcirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNGRST` reader - TRNG reset"]
pub type TrngrstR = crate::BitReader;
#[doc = "Field `TRNGRST` writer - TRNG reset"]
pub type TrngrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBFSRST` reader - USBFS reset"]
pub type UsbfsrstR = crate::BitReader;
#[doc = "Field `USBFSRST` writer - USBFS reset"]
pub type UsbfsrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCI reset"]
    #[inline(always)]
    pub fn dcirst(&self) -> DcirstR {
        DcirstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG reset"]
    #[inline(always)]
    pub fn trngrst(&self) -> TrngrstR {
        TrngrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> UsbfsrstR {
        UsbfsrstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcirst(&mut self) -> DcirstW<Ahb2rstSpec> {
        DcirstW::new(self, 0)
    }
    #[doc = "Bit 6 - TRNG reset"]
    #[inline(always)]
    #[must_use]
    pub fn trngrst(&mut self) -> TrngrstW<Ahb2rstSpec> {
        TrngrstW::new(self, 6)
    }
    #[doc = "Bit 7 - USBFS reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsrst(&mut self) -> UsbfsrstW<Ahb2rstSpec> {
        UsbfsrstW::new(self, 7)
    }
}
#[doc = "AHB2 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2rstSpec;
impl crate::RegisterSpec for Ahb2rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rst::R`](R) reader structure"]
impl crate::Readable for Ahb2rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2rst::W`](W) writer structure"]
impl crate::Writable for Ahb2rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2RST to value 0"]
impl crate::Resettable for Ahb2rstSpec {
    const RESET_VALUE: u32 = 0;
}
