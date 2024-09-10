#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `PG` reader - main flash program command bit"]
pub type PgR = crate::BitReader;
#[doc = "Field `PG` writer - main flash program command bit"]
pub type PgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SER` reader - main flash sector erase command bit"]
pub type SerR = crate::BitReader;
#[doc = "Field `SER` writer - main flash sector erase command bit"]
pub type SerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER0` reader - main flash mass erase for bank0 command bit"]
pub type Mer0R = crate::BitReader;
#[doc = "Field `MER0` writer - main flash mass erase for bank0 command bit"]
pub type Mer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SN` reader - Select which sector number to be erased."]
pub type SnR = crate::FieldReader;
#[doc = "Field `SN` writer - Select which sector number to be erased."]
pub type SnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PSZ` reader - Program size bit"]
pub type PszR = crate::FieldReader;
#[doc = "Field `PSZ` writer - Program size bit"]
pub type PszW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MER1` reader - main flash mass erase for bank1command bit"]
pub type Mer1R = crate::BitReader;
#[doc = "Field `MER1` writer - main flash mass erase for bank1command bit"]
pub type Mer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - send erase command to FMC bit"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - send erase command to FMC bit"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDIE` reader - End of operation interrupt enable bit"]
pub type EndieR = crate::BitReader;
#[doc = "Field `ENDIE` writer - End of operation interrupt enable bit"]
pub type EndieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable bit"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable bit"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK` reader - FMC_CTL lock bit"]
pub type LkR = crate::BitReader;
#[doc = "Field `LK` writer - FMC_CTL lock bit"]
pub type LkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - main flash program command bit"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - main flash sector erase command bit"]
    #[inline(always)]
    pub fn ser(&self) -> SerR {
        SerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - main flash mass erase for bank0 command bit"]
    #[inline(always)]
    pub fn mer0(&self) -> Mer0R {
        Mer0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Select which sector number to be erased."]
    #[inline(always)]
    pub fn sn(&self) -> SnR {
        SnR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Program size bit"]
    #[inline(always)]
    pub fn psz(&self) -> PszR {
        PszR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - main flash mass erase for bank1command bit"]
    #[inline(always)]
    pub fn mer1(&self) -> Mer1R {
        Mer1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable bit"]
    #[inline(always)]
    pub fn endie(&self) -> EndieR {
        EndieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable bit"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - FMC_CTL lock bit"]
    #[inline(always)]
    pub fn lk(&self) -> LkR {
        LkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - main flash program command bit"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PgW<CtlSpec> {
        PgW::new(self, 0)
    }
    #[doc = "Bit 1 - main flash sector erase command bit"]
    #[inline(always)]
    #[must_use]
    pub fn ser(&mut self) -> SerW<CtlSpec> {
        SerW::new(self, 1)
    }
    #[doc = "Bit 2 - main flash mass erase for bank0 command bit"]
    #[inline(always)]
    #[must_use]
    pub fn mer0(&mut self) -> Mer0W<CtlSpec> {
        Mer0W::new(self, 2)
    }
    #[doc = "Bits 3:7 - Select which sector number to be erased."]
    #[inline(always)]
    #[must_use]
    pub fn sn(&mut self) -> SnW<CtlSpec> {
        SnW::new(self, 3)
    }
    #[doc = "Bits 8:9 - Program size bit"]
    #[inline(always)]
    #[must_use]
    pub fn psz(&mut self) -> PszW<CtlSpec> {
        PszW::new(self, 8)
    }
    #[doc = "Bit 15 - main flash mass erase for bank1command bit"]
    #[inline(always)]
    #[must_use]
    pub fn mer1(&mut self) -> Mer1W<CtlSpec> {
        Mer1W::new(self, 15)
    }
    #[doc = "Bit 16 - send erase command to FMC bit"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CtlSpec> {
        StartW::new(self, 16)
    }
    #[doc = "Bit 24 - End of operation interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn endie(&mut self) -> EndieW<CtlSpec> {
        EndieW::new(self, 24)
    }
    #[doc = "Bit 25 - Error interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<CtlSpec> {
        ErrieW::new(self, 25)
    }
    #[doc = "Bit 31 - FMC_CTL lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lk(&mut self) -> LkW<CtlSpec> {
        LkW::new(self, 31)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0x80"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x80;
}
