#[doc = "Register `DADDR` reader"]
pub type R = crate::R<DaddrSpec>;
#[doc = "Register `DADDR` writer"]
pub type W = crate::W<DaddrSpec>;
#[doc = "Field `USBADDR` reader - USB device address"]
pub type UsbaddrR = crate::FieldReader;
#[doc = "Field `USBADDR` writer - USB device address"]
pub type UsbaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `USBEN` reader - USB device enable"]
pub type UsbenR = crate::BitReader;
#[doc = "Field `USBEN` writer - USB device enable"]
pub type UsbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - USB device address"]
    #[inline(always)]
    pub fn usbaddr(&self) -> UsbaddrR {
        UsbaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    pub fn usben(&self) -> UsbenR {
        UsbenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB device address"]
    #[inline(always)]
    #[must_use]
    pub fn usbaddr(&mut self) -> UsbaddrW<DaddrSpec> {
        UsbaddrW::new(self, 0)
    }
    #[doc = "Bit 7 - USB device enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> UsbenW<DaddrSpec> {
        UsbenW::new(self, 7)
    }
}
#[doc = "device address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaddrSpec;
impl crate::RegisterSpec for DaddrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`daddr::R`](R) reader structure"]
impl crate::Readable for DaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`daddr::W`](W) writer structure"]
impl crate::Writable for DaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DaddrSpec {
    const RESET_VALUE: u16 = 0;
}
