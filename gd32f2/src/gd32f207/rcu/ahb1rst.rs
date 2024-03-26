#[doc = "Register `AHB1RST` reader"]
pub type R = crate::R<Ahb1rstSpec>;
#[doc = "Register `AHB1RST` writer"]
pub type W = crate::W<Ahb1rstSpec>;
#[doc = "Field `USBFSRST` reader - USBFS reset"]
pub type UsbfsrstR = crate::BitReader;
#[doc = "Field `USBFSRST` writer - USBFS reset"]
pub type UsbfsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETRST` reader - ENET reset"]
pub type EnetrstR = crate::BitReader;
#[doc = "Field `ENETRST` writer - ENET reset"]
pub type EnetrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> UsbfsrstR {
        UsbfsrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - ENET reset"]
    #[inline(always)]
    pub fn enetrst(&self) -> EnetrstR {
        EnetrstR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsrst(&mut self) -> UsbfsrstW<Ahb1rstSpec> {
        UsbfsrstW::new(self, 12)
    }
    #[doc = "Bit 14 - ENET reset"]
    #[inline(always)]
    #[must_use]
    pub fn enetrst(&mut self) -> EnetrstW<Ahb1rstSpec> {
        EnetrstW::new(self, 14)
    }
}
#[doc = "AHB1 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1rstSpec;
impl crate::RegisterSpec for Ahb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rst::R`](R) reader structure"]
impl crate::Readable for Ahb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1rst::W`](W) writer structure"]
impl crate::Writable for Ahb1rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1RST to value 0"]
impl crate::Resettable for Ahb1rstSpec {
    const RESET_VALUE: u32 = 0;
}
