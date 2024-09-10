#[doc = "Register `AHB2EN` reader"]
pub type R = crate::R<Ahb2enSpec>;
#[doc = "Register `AHB2EN` writer"]
pub type W = crate::W<Ahb2enSpec>;
#[doc = "Field `DCIEN` reader - DCI clock enable"]
pub type DcienR = crate::BitReader;
#[doc = "Field `DCIEN` writer - DCI clock enable"]
pub type DcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNGEN` reader - TRNG clock enable"]
pub type TrngenR = crate::BitReader;
#[doc = "Field `TRNGEN` writer - TRNG clock enable"]
pub type TrngenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBFSEN` reader - USBFS clock enable"]
pub type UsbfsenR = crate::BitReader;
#[doc = "Field `USBFSEN` writer - USBFS clock enable"]
pub type UsbfsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    pub fn dcien(&self) -> DcienR {
        DcienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    pub fn trngen(&self) -> TrngenR {
        TrngenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USBFS clock enable"]
    #[inline(always)]
    pub fn usbfsen(&self) -> UsbfsenR {
        UsbfsenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcien(&mut self) -> DcienW<Ahb2enSpec> {
        DcienW::new(self, 0)
    }
    #[doc = "Bit 6 - TRNG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn trngen(&mut self) -> TrngenW<Ahb2enSpec> {
        TrngenW::new(self, 6)
    }
    #[doc = "Bit 7 - USBFS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsen(&mut self) -> UsbfsenW<Ahb2enSpec> {
        UsbfsenW::new(self, 7)
    }
}
#[doc = "AHB2 enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2enSpec;
impl crate::RegisterSpec for Ahb2enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2en::R`](R) reader structure"]
impl crate::Readable for Ahb2enSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2en::W`](W) writer structure"]
impl crate::Writable for Ahb2enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2EN to value 0"]
impl crate::Resettable for Ahb2enSpec {
    const RESET_VALUE: u32 = 0;
}
