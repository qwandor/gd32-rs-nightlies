#[doc = "Register `SEP0` reader"]
pub type R = crate::R<Sep0Spec>;
#[doc = "Register `SEP0` writer"]
pub type W = crate::W<Sep0Spec>;
#[doc = "Field `SUBPID_ATTR` reader - LPM Token bmAttribute Field."]
pub type SubpidAttrR = crate::FieldReader<u16>;
#[doc = "Field `SUBPID_ATTR` writer - LPM Token bmAttribute Field."]
pub type SubpidAttrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SUB_STA` reader - Status bits, for the handshake of receiving subpid LPM"]
pub type SubStaR = crate::FieldReader;
#[doc = "Field `SUB_STA` writer - Status bits, for the handshake of receiving subpid LPM"]
pub type SubStaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SUB_ST` reader - Successful Receive for LPM Token"]
pub type SubStR = crate::BitReader;
#[doc = "Field `SUB_ST` writer - Successful Receive for LPM Token"]
pub type SubStW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - LPM Token bmAttribute Field."]
    #[inline(always)]
    pub fn subpid_attr(&self) -> SubpidAttrR {
        SubpidAttrR::new(self.bits & 0x07ff)
    }
    #[doc = "Bits 12:13 - Status bits, for the handshake of receiving subpid LPM"]
    #[inline(always)]
    pub fn sub_sta(&self) -> SubStaR {
        SubStaR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Successful Receive for LPM Token"]
    #[inline(always)]
    pub fn sub_st(&self) -> SubStR {
        SubStR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - LPM Token bmAttribute Field."]
    #[inline(always)]
    #[must_use]
    pub fn subpid_attr(&mut self) -> SubpidAttrW<Sep0Spec> {
        SubpidAttrW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Status bits, for the handshake of receiving subpid LPM"]
    #[inline(always)]
    #[must_use]
    pub fn sub_sta(&mut self) -> SubStaW<Sep0Spec> {
        SubStaW::new(self, 12)
    }
    #[doc = "Bit 15 - Successful Receive for LPM Token"]
    #[inline(always)]
    #[must_use]
    pub fn sub_st(&mut self) -> SubStW<Sep0Spec> {
        SubStW::new(self, 15)
    }
}
#[doc = "USB sub-endpoint 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sep0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sep0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sep0Spec;
impl crate::RegisterSpec for Sep0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sep0::R`](R) reader structure"]
impl crate::Readable for Sep0Spec {}
#[doc = "`write(|w| ..)` method takes [`sep0::W`](W) writer structure"]
impl crate::Writable for Sep0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SEP0 to value 0"]
impl crate::Resettable for Sep0Spec {
    const RESET_VALUE: u16 = 0;
}
