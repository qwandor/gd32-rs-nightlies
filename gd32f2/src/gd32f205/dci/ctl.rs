#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `CAP` reader - Capture Enable"]
pub type CapR = crate::BitReader;
#[doc = "Field `CAP` writer - Capture Enable"]
pub type CapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAP` reader - Snapshot mode"]
pub type SnapR = crate::BitReader;
#[doc = "Field `SNAP` writer - Snapshot mode"]
pub type SnapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDEN` reader - Window Enable"]
pub type WdenR = crate::BitReader;
#[doc = "Field `WDEN` writer - Window Enable"]
pub type WdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JM` reader - JPEG mode"]
pub type JmR = crate::BitReader;
#[doc = "Field `JM` writer - JPEG mode"]
pub type JmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESM` reader - Embedded synchronous mode"]
pub type EsmR = crate::BitReader;
#[doc = "Field `ESM` writer - Embedded synchronous mode"]
pub type EsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKS` reader - Clock Polarity Selection"]
pub type CksR = crate::BitReader;
#[doc = "Field `CKS` writer - Clock Polarity Selection"]
pub type CksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPS` reader - Horizontal Polarity Selection"]
pub type HpsR = crate::BitReader;
#[doc = "Field `HPS` writer - Horizontal Polarity Selection"]
pub type HpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPS` reader - Vertical Polarity Selection"]
pub type VpsR = crate::BitReader;
#[doc = "Field `VPS` writer - Vertical Polarity Selection"]
pub type VpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FR` reader - Frame rate"]
pub type FrR = crate::FieldReader;
#[doc = "Field `FR` writer - Frame rate"]
pub type FrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCIF` reader - Digital camera interface format"]
pub type DcifR = crate::FieldReader;
#[doc = "Field `DCIF` writer - Digital camera interface format"]
pub type DcifW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCIEN` reader - DCI Enable"]
pub type DcienR = crate::BitReader;
#[doc = "Field `DCIEN` writer - DCI Enable"]
pub type DcienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture Enable"]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Snapshot mode"]
    #[inline(always)]
    pub fn snap(&self) -> SnapR {
        SnapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Enable"]
    #[inline(always)]
    pub fn wden(&self) -> WdenR {
        WdenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG mode"]
    #[inline(always)]
    pub fn jm(&self) -> JmR {
        JmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Embedded synchronous mode"]
    #[inline(always)]
    pub fn esm(&self) -> EsmR {
        EsmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Polarity Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal Polarity Selection"]
    #[inline(always)]
    pub fn hps(&self) -> HpsR {
        HpsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Vertical Polarity Selection"]
    #[inline(always)]
    pub fn vps(&self) -> VpsR {
        VpsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Frame rate"]
    #[inline(always)]
    pub fn fr(&self) -> FrR {
        FrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Digital camera interface format"]
    #[inline(always)]
    pub fn dcif(&self) -> DcifR {
        DcifR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - DCI Enable"]
    #[inline(always)]
    pub fn dcien(&self) -> DcienR {
        DcienR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap(&mut self) -> CapW<CtlSpec> {
        CapW::new(self, 0)
    }
    #[doc = "Bit 1 - Snapshot mode"]
    #[inline(always)]
    #[must_use]
    pub fn snap(&mut self) -> SnapW<CtlSpec> {
        SnapW::new(self, 1)
    }
    #[doc = "Bit 2 - Window Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wden(&mut self) -> WdenW<CtlSpec> {
        WdenW::new(self, 2)
    }
    #[doc = "Bit 3 - JPEG mode"]
    #[inline(always)]
    #[must_use]
    pub fn jm(&mut self) -> JmW<CtlSpec> {
        JmW::new(self, 3)
    }
    #[doc = "Bit 4 - Embedded synchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn esm(&mut self) -> EsmW<CtlSpec> {
        EsmW::new(self, 4)
    }
    #[doc = "Bit 5 - Clock Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CksW<CtlSpec> {
        CksW::new(self, 5)
    }
    #[doc = "Bit 6 - Horizontal Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn hps(&mut self) -> HpsW<CtlSpec> {
        HpsW::new(self, 6)
    }
    #[doc = "Bit 7 - Vertical Polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn vps(&mut self) -> VpsW<CtlSpec> {
        VpsW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Frame rate"]
    #[inline(always)]
    #[must_use]
    pub fn fr(&mut self) -> FrW<CtlSpec> {
        FrW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Digital camera interface format"]
    #[inline(always)]
    #[must_use]
    pub fn dcif(&mut self) -> DcifW<CtlSpec> {
        DcifW::new(self, 10)
    }
    #[doc = "Bit 14 - DCI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcien(&mut self) -> DcienW<CtlSpec> {
        DcienW::new(self, 14)
    }
}
#[doc = "DCI Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
