#[doc = "Register `SINIT` reader"]
pub type R = crate::R<SinitSpec>;
#[doc = "Register `SINIT` writer"]
pub type W = crate::W<SinitSpec>;
#[doc = "Field `CMDBIT` reader - Bit number of SPI PSRAM command phase"]
pub type CmdbitR = crate::FieldReader;
#[doc = "Field `CMDBIT` writer - Bit number of SPI PSRAM command phase"]
pub type CmdbitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADRBIT` reader - Bit number of SPI PSRAM address phase"]
pub type AdrbitR = crate::FieldReader;
#[doc = "Field `ADRBIT` writer - Bit number of SPI PSRAM address phase"]
pub type AdrbitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IDL` reader - SPI PSRAM ID Length"]
pub type IdlR = crate::FieldReader;
#[doc = "Field `IDL` writer - SPI PSRAM ID Length"]
pub type IdlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `POL` reader - Read data sample polarity"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Read data sample polarity"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - Bit number of SPI PSRAM command phase"]
    #[inline(always)]
    pub fn cmdbit(&self) -> CmdbitR {
        CmdbitR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:28 - Bit number of SPI PSRAM address phase"]
    #[inline(always)]
    pub fn adrbit(&self) -> AdrbitR {
        AdrbitR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - SPI PSRAM ID Length"]
    #[inline(always)]
    pub fn idl(&self) -> IdlR {
        IdlR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Read data sample polarity"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Bit number of SPI PSRAM command phase"]
    #[inline(always)]
    #[must_use]
    pub fn cmdbit(&mut self) -> CmdbitW<SinitSpec> {
        CmdbitW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Bit number of SPI PSRAM address phase"]
    #[inline(always)]
    #[must_use]
    pub fn adrbit(&mut self) -> AdrbitW<SinitSpec> {
        AdrbitW::new(self, 24)
    }
    #[doc = "Bits 29:30 - SPI PSRAM ID Length"]
    #[inline(always)]
    #[must_use]
    pub fn idl(&mut self) -> IdlW<SinitSpec> {
        IdlW::new(self, 29)
    }
    #[doc = "Bit 31 - Read data sample polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> PolW<SinitSpec> {
        PolW::new(self, 31)
    }
}
#[doc = "SPI initialization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sinit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sinit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinitSpec;
impl crate::RegisterSpec for SinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sinit::R`](R) reader structure"]
impl crate::Readable for SinitSpec {}
#[doc = "`write(|w| ..)` method takes [`sinit::W`](W) writer structure"]
impl crate::Writable for SinitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SINIT to value 0x1801_0000"]
impl crate::Resettable for SinitSpec {
    const RESET_VALUE: u32 = 0x1801_0000;
}
