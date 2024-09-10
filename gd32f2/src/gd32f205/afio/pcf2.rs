#[doc = "Register `PCF2` reader"]
pub type R = crate::R<Pcf2Spec>;
#[doc = "Register `PCF2` writer"]
pub type W = crate::W<Pcf2Spec>;
#[doc = "Field `DCI_VSYNC_REMAP` reader - DCI_VSYNC remapping"]
pub type DciVsyncRemapR = crate::FieldReader;
#[doc = "Field `DCI_VSYNC_REMAP` writer - DCI_VSYNC remapping"]
pub type DciVsyncRemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D0_REMAP` reader - DCI_D0 remapping"]
pub type DciD0RemapR = crate::FieldReader;
#[doc = "Field `DCI_D0_REMAP` writer - DCI_D0 remapping"]
pub type DciD0RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D1_REMAP` reader - DCI_D1 remapping"]
pub type DciD1RemapR = crate::FieldReader;
#[doc = "Field `DCI_D1_REMAP` writer - DCI_D1 remapping"]
pub type DciD1RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D2_REMAP` reader - DCI_D2 remapping"]
pub type DciD2RemapR = crate::FieldReader;
#[doc = "Field `DCI_D2_REMAP` writer - DCI_D2 remapping"]
pub type DciD2RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D3_REMAP` reader - DCI_D3 remapping"]
pub type DciD3RemapR = crate::FieldReader;
#[doc = "Field `DCI_D3_REMAP` writer - DCI_D3 remapping"]
pub type DciD3RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D4_REMAP` reader - DCI_D4 remapping"]
pub type DciD4RemapR = crate::FieldReader;
#[doc = "Field `DCI_D4_REMAP` writer - DCI_D4 remapping"]
pub type DciD4RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D5_REMAP` reader - DCI_D5 remapping"]
pub type DciD5RemapR = crate::FieldReader;
#[doc = "Field `DCI_D5_REMAP` writer - DCI_D5 remapping"]
pub type DciD5RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D6_REMAP` reader - DCI_D6 remapping"]
pub type DciD6RemapR = crate::FieldReader;
#[doc = "Field `DCI_D6_REMAP` writer - DCI_D6 remapping"]
pub type DciD6RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D7_REMAP` reader - DCI_D7 remapping"]
pub type DciD7RemapR = crate::FieldReader;
#[doc = "Field `DCI_D7_REMAP` writer - DCI_D7 remapping"]
pub type DciD7RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D8_REMAP` reader - DCI_D8 remapping"]
pub type DciD8RemapR = crate::FieldReader;
#[doc = "Field `DCI_D8_REMAP` writer - DCI_D8 remapping"]
pub type DciD8RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D9_REMAP` reader - DCI_D9 remapping"]
pub type DciD9RemapR = crate::FieldReader;
#[doc = "Field `DCI_D9_REMAP` writer - DCI_D9 remapping"]
pub type DciD9RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D10_REMAP` reader - DCI_D10 remapping"]
pub type DciD10RemapR = crate::FieldReader;
#[doc = "Field `DCI_D10_REMAP` writer - DCI_D10 remapping"]
pub type DciD10RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D11_REMAP` reader - DCI_D11 remapping"]
pub type DciD11RemapR = crate::FieldReader;
#[doc = "Field `DCI_D11_REMAP` writer - DCI_D11 remapping"]
pub type DciD11RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_D12_REMAP` reader - DCI_D12 remapping"]
pub type DciD12RemapR = crate::BitReader;
#[doc = "Field `DCI_D12_REMAP` writer - DCI_D12 remapping"]
pub type DciD12RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCI_D13_REMAP` reader - DCI_D13 remapping"]
pub type DciD13RemapR = crate::FieldReader;
#[doc = "Field `DCI_D13_REMAP` writer - DCI_D13 remapping"]
pub type DciD13RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCI_HSYNC_REMAP` reader - DCI_HSYNC remapping"]
pub type DciHsyncRemapR = crate::BitReader;
#[doc = "Field `DCI_HSYNC_REMAP` writer - DCI_HSYNC remapping"]
pub type DciHsyncRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH01_REMAP` reader - PH0/PH1 remapping"]
pub type Ph01RemapR = crate::BitReader;
#[doc = "Field `PH01_REMAP` writer - PH0/PH1 remapping"]
pub type Ph01RemapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - DCI_VSYNC remapping"]
    #[inline(always)]
    pub fn dci_vsync_remap(&self) -> DciVsyncRemapR {
        DciVsyncRemapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DCI_D0 remapping"]
    #[inline(always)]
    pub fn dci_d0_remap(&self) -> DciD0RemapR {
        DciD0RemapR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DCI_D1 remapping"]
    #[inline(always)]
    pub fn dci_d1_remap(&self) -> DciD1RemapR {
        DciD1RemapR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DCI_D2 remapping"]
    #[inline(always)]
    pub fn dci_d2_remap(&self) -> DciD2RemapR {
        DciD2RemapR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DCI_D3 remapping"]
    #[inline(always)]
    pub fn dci_d3_remap(&self) -> DciD3RemapR {
        DciD3RemapR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DCI_D4 remapping"]
    #[inline(always)]
    pub fn dci_d4_remap(&self) -> DciD4RemapR {
        DciD4RemapR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - DCI_D5 remapping"]
    #[inline(always)]
    pub fn dci_d5_remap(&self) -> DciD5RemapR {
        DciD5RemapR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DCI_D6 remapping"]
    #[inline(always)]
    pub fn dci_d6_remap(&self) -> DciD6RemapR {
        DciD6RemapR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DCI_D7 remapping"]
    #[inline(always)]
    pub fn dci_d7_remap(&self) -> DciD7RemapR {
        DciD7RemapR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - DCI_D8 remapping"]
    #[inline(always)]
    pub fn dci_d8_remap(&self) -> DciD8RemapR {
        DciD8RemapR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - DCI_D9 remapping"]
    #[inline(always)]
    pub fn dci_d9_remap(&self) -> DciD9RemapR {
        DciD9RemapR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - DCI_D10 remapping"]
    #[inline(always)]
    pub fn dci_d10_remap(&self) -> DciD10RemapR {
        DciD10RemapR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DCI_D11 remapping"]
    #[inline(always)]
    pub fn dci_d11_remap(&self) -> DciD11RemapR {
        DciD11RemapR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - DCI_D12 remapping"]
    #[inline(always)]
    pub fn dci_d12_remap(&self) -> DciD12RemapR {
        DciD12RemapR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - DCI_D13 remapping"]
    #[inline(always)]
    pub fn dci_d13_remap(&self) -> DciD13RemapR {
        DciD13RemapR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - DCI_HSYNC remapping"]
    #[inline(always)]
    pub fn dci_hsync_remap(&self) -> DciHsyncRemapR {
        DciHsyncRemapR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - PH0/PH1 remapping"]
    #[inline(always)]
    pub fn ph01_remap(&self) -> Ph01RemapR {
        Ph01RemapR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DCI_VSYNC remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_vsync_remap(&mut self) -> DciVsyncRemapW<Pcf2Spec> {
        DciVsyncRemapW::new(self, 0)
    }
    #[doc = "Bits 2:3 - DCI_D0 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d0_remap(&mut self) -> DciD0RemapW<Pcf2Spec> {
        DciD0RemapW::new(self, 2)
    }
    #[doc = "Bits 4:5 - DCI_D1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d1_remap(&mut self) -> DciD1RemapW<Pcf2Spec> {
        DciD1RemapW::new(self, 4)
    }
    #[doc = "Bits 6:7 - DCI_D2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d2_remap(&mut self) -> DciD2RemapW<Pcf2Spec> {
        DciD2RemapW::new(self, 6)
    }
    #[doc = "Bits 8:9 - DCI_D3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d3_remap(&mut self) -> DciD3RemapW<Pcf2Spec> {
        DciD3RemapW::new(self, 8)
    }
    #[doc = "Bits 10:11 - DCI_D4 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d4_remap(&mut self) -> DciD4RemapW<Pcf2Spec> {
        DciD4RemapW::new(self, 10)
    }
    #[doc = "Bits 12:13 - DCI_D5 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d5_remap(&mut self) -> DciD5RemapW<Pcf2Spec> {
        DciD5RemapW::new(self, 12)
    }
    #[doc = "Bits 14:15 - DCI_D6 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d6_remap(&mut self) -> DciD6RemapW<Pcf2Spec> {
        DciD6RemapW::new(self, 14)
    }
    #[doc = "Bits 16:17 - DCI_D7 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d7_remap(&mut self) -> DciD7RemapW<Pcf2Spec> {
        DciD7RemapW::new(self, 16)
    }
    #[doc = "Bits 18:19 - DCI_D8 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d8_remap(&mut self) -> DciD8RemapW<Pcf2Spec> {
        DciD8RemapW::new(self, 18)
    }
    #[doc = "Bits 20:21 - DCI_D9 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d9_remap(&mut self) -> DciD9RemapW<Pcf2Spec> {
        DciD9RemapW::new(self, 20)
    }
    #[doc = "Bits 22:23 - DCI_D10 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d10_remap(&mut self) -> DciD10RemapW<Pcf2Spec> {
        DciD10RemapW::new(self, 22)
    }
    #[doc = "Bits 24:25 - DCI_D11 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d11_remap(&mut self) -> DciD11RemapW<Pcf2Spec> {
        DciD11RemapW::new(self, 24)
    }
    #[doc = "Bit 26 - DCI_D12 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d12_remap(&mut self) -> DciD12RemapW<Pcf2Spec> {
        DciD12RemapW::new(self, 26)
    }
    #[doc = "Bits 27:28 - DCI_D13 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_d13_remap(&mut self) -> DciD13RemapW<Pcf2Spec> {
        DciD13RemapW::new(self, 27)
    }
    #[doc = "Bit 29 - DCI_HSYNC remapping"]
    #[inline(always)]
    #[must_use]
    pub fn dci_hsync_remap(&mut self) -> DciHsyncRemapW<Pcf2Spec> {
        DciHsyncRemapW::new(self, 29)
    }
    #[doc = "Bit 31 - PH0/PH1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn ph01_remap(&mut self) -> Ph01RemapW<Pcf2Spec> {
        Ph01RemapW::new(self, 31)
    }
}
#[doc = "AFIO port configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcf2Spec;
impl crate::RegisterSpec for Pcf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcf2::R`](R) reader structure"]
impl crate::Readable for Pcf2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcf2::W`](W) writer structure"]
impl crate::Writable for Pcf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCF2 to value 0"]
impl crate::Resettable for Pcf2Spec {
    const RESET_VALUE: u32 = 0;
}
