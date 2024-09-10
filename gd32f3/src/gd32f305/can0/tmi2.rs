#[doc = "Register `TMI2` reader"]
pub type R = crate::R<Tmi2Spec>;
#[doc = "Register `TMI2` writer"]
pub type W = crate::W<Tmi2Spec>;
#[doc = "Transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ten {
    #[doc = "0: Transmit disabled"]
    Disabled = 0,
    #[doc = "1: Transmit enabled"]
    Enabled = 1,
}
impl From<Ten> for bool {
    #[inline(always)]
    fn from(variant: Ten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Transmit enable"]
pub type TenR = crate::BitReader<Ten>;
impl TenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ten {
        match self.bits {
            false => Ten::Disabled,
            true => Ten::Enabled,
        }
    }
    #[doc = "Transmit disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ten::Disabled
    }
    #[doc = "Transmit enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ten::Enabled
    }
}
#[doc = "Field `TEN` writer - Transmit enable"]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG, Ten>;
impl<'a, REG> TenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::Disabled)
    }
    #[doc = "Transmit enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::Enabled)
    }
}
#[doc = "Frame type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ft {
    #[doc = "0: Data frame"]
    Data = 0,
    #[doc = "1: Remote frame"]
    Remote = 1,
}
impl From<Ft> for bool {
    #[inline(always)]
    fn from(variant: Ft) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT` reader - Frame type"]
pub type FtR = crate::BitReader<Ft>;
impl FtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ft {
        match self.bits {
            false => Ft::Data,
            true => Ft::Remote,
        }
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Ft::Data
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == Ft::Remote
    }
}
#[doc = "Field `FT` writer - Frame type"]
pub type FtW<'a, REG> = crate::BitWriter<'a, REG, Ft>;
impl<'a, REG> FtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Ft::Data)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn remote(self) -> &'a mut crate::W<REG> {
        self.variant(Ft::Remote)
    }
}
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ff {
    #[doc = "0: Standard format frame"]
    Standard = 0,
    #[doc = "1: Extended format frame"]
    Extended = 1,
}
impl From<Ff> for bool {
    #[inline(always)]
    fn from(variant: Ff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FF` reader - Frame format"]
pub type FfR = crate::BitReader<Ff>;
impl FfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ff {
        match self.bits {
            false => Ff::Standard,
            true => Ff::Extended,
        }
    }
    #[doc = "Standard format frame"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Ff::Standard
    }
    #[doc = "Extended format frame"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == Ff::Extended
    }
}
#[doc = "Field `FF` writer - Frame format"]
pub type FfW<'a, REG> = crate::BitWriter<'a, REG, Ff>;
impl<'a, REG> FfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard format frame"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Ff::Standard)
    }
    #[doc = "Extended format frame"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(Ff::Extended)
    }
}
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EfidR = crate::FieldReader<u32>;
#[doc = "Field `EFID` writer - The frame identifier"]
pub type EfidW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SfidEfidR = crate::FieldReader<u16>;
#[doc = "Field `SFID_EFID` writer - The frame identifier"]
pub type SfidEfidW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FtR {
        FtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FfR {
        FfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EfidR {
        EfidR::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SfidEfidR {
        SfidEfidR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TenW<Tmi2Spec> {
        TenW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    #[must_use]
    pub fn ft(&mut self) -> FtW<Tmi2Spec> {
        FtW::new(self, 1)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FfW<Tmi2Spec> {
        FfW::new(self, 2)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    #[must_use]
    pub fn efid(&mut self) -> EfidW<Tmi2Spec> {
        EfidW::new(self, 3)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    #[must_use]
    pub fn sfid_efid(&mut self) -> SfidEfidW<Tmi2Spec> {
        SfidEfidW::new(self, 21)
    }
}
#[doc = "Transmit mailbox identifier register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmi2Spec;
impl crate::RegisterSpec for Tmi2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmi2::R`](R) reader structure"]
impl crate::Readable for Tmi2Spec {}
#[doc = "`write(|w| ..)` method takes [`tmi2::W`](W) writer structure"]
impl crate::Writable for Tmi2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMI2 to value 0"]
impl crate::Resettable for Tmi2Spec {
    const RESET_VALUE: u32 = 0;
}
