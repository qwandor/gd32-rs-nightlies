#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `CAUDIR` reader - CAU direction"]
pub type CaudirR = crate::BitReader;
#[doc = "Field `CAUDIR` writer - CAU direction"]
pub type CaudirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGM` reader - Encryption/decryption algorithm mode"]
pub type AlgmR = crate::FieldReader;
#[doc = "Field `ALGM` writer - Encryption/decryption algorithm mode"]
pub type AlgmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATAM` reader - Data swapping type mode configuration"]
pub type DatamR = crate::FieldReader;
#[doc = "Field `DATAM` writer - Data swapping type mode configuration"]
pub type DatamW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYM` reader - AES key size mode configuration"]
pub type KeymR = crate::FieldReader;
#[doc = "Field `KEYM` writer - AES key size mode configuration"]
pub type KeymW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FFLUSH` writer - FIFO flush"]
pub type FflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAUEN` reader - Cryptographic module enable"]
pub type CauenR = crate::BitReader;
#[doc = "Field `CAUEN` writer - Cryptographic module enable"]
pub type CauenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - CAU direction"]
    #[inline(always)]
    pub fn caudir(&self) -> CaudirR {
        CaudirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Encryption/decryption algorithm mode"]
    #[inline(always)]
    pub fn algm(&self) -> AlgmR {
        AlgmR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Data swapping type mode configuration"]
    #[inline(always)]
    pub fn datam(&self) -> DatamR {
        DatamR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - AES key size mode configuration"]
    #[inline(always)]
    pub fn keym(&self) -> KeymR {
        KeymR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Cryptographic module enable"]
    #[inline(always)]
    pub fn cauen(&self) -> CauenR {
        CauenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - CAU direction"]
    #[inline(always)]
    #[must_use]
    pub fn caudir(&mut self) -> CaudirW<CtlSpec> {
        CaudirW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Encryption/decryption algorithm mode"]
    #[inline(always)]
    #[must_use]
    pub fn algm(&mut self) -> AlgmW<CtlSpec> {
        AlgmW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Data swapping type mode configuration"]
    #[inline(always)]
    #[must_use]
    pub fn datam(&mut self) -> DatamW<CtlSpec> {
        DatamW::new(self, 6)
    }
    #[doc = "Bits 8:9 - AES key size mode configuration"]
    #[inline(always)]
    #[must_use]
    pub fn keym(&mut self) -> KeymW<CtlSpec> {
        KeymW::new(self, 8)
    }
    #[doc = "Bit 14 - FIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FflushW<CtlSpec> {
        FflushW::new(self, 14)
    }
    #[doc = "Bit 15 - Cryptographic module enable"]
    #[inline(always)]
    #[must_use]
    pub fn cauen(&mut self) -> CauenW<CtlSpec> {
        CauenW::new(self, 15)
    }
}
#[doc = "CAU control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
