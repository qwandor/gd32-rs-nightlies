#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AhbrstSpec>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AhbrstSpec>;
#[doc = "USBFS reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbfsrst {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<Usbfsrst> for bool {
    #[inline(always)]
    fn from(variant: Usbfsrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBFSRST` reader - USBFS reset"]
pub type UsbfsrstR = crate::BitReader<Usbfsrst>;
impl UsbfsrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Usbfsrst> {
        match self.bits {
            true => Some(Usbfsrst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Usbfsrst::Reset
    }
}
#[doc = "Field `USBFSRST` writer - USBFS reset"]
pub type UsbfsrstW<'a, REG> = crate::BitWriter<'a, REG, Usbfsrst>;
impl<'a, REG> UsbfsrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsrst::Reset)
    }
}
impl R {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> UsbfsrstR {
        UsbfsrstR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsrst(&mut self) -> UsbfsrstW<AhbrstSpec> {
        UsbfsrstW::new(self, 12)
    }
}
#[doc = "AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstSpec;
impl crate::RegisterSpec for AhbrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AhbrstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AhbrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AhbrstSpec {
    const RESET_VALUE: u32 = 0;
}
