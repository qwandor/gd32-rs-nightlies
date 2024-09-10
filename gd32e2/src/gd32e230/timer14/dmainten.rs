#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DmaintenSpec>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DmaintenSpec>;
#[doc = "Capture/Compare 1 interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::Ch0ie;
#[doc = "Field `CH0IE` reader - Capture/Compare 1 interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::Ch0ieR;
#[doc = "Field `CH1IE` reader - Capture/Compare 2 interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::Ch0ieR as Ch1ieR;
#[doc = "Field `CH0IE` writer - Capture/Compare 1 interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::Ch0ieW;
#[doc = "Field `CH1IE` writer - Capture/Compare 2 interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::Ch0ieW as Ch1ieW;
#[doc = "COM interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::Cmtie;
#[doc = "Field `CMTIE` reader - COM interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::CmtieR;
#[doc = "Field `CMTIE` writer - COM interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::CmtieW;
#[doc = "Update interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::Upie;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::UpieR;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::UpieW;
#[doc = "Trigger interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgie {
    #[doc = "0: Trigger interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Trigger interrupt enabled"]
    Enabled = 1,
}
impl From<Trgie> for bool {
    #[inline(always)]
    fn from(variant: Trgie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGIE` reader - Trigger interrupt enable"]
pub type TrgieR = crate::BitReader<Trgie>;
impl TrgieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgie {
        match self.bits {
            false => Trgie::Disabled,
            true => Trgie::Enabled,
        }
    }
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trgie::Disabled
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trgie::Enabled
    }
}
#[doc = "Field `TRGIE` writer - Trigger interrupt enable"]
pub type TrgieW<'a, REG> = crate::BitWriter<'a, REG, Trgie>;
impl<'a, REG> TrgieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trgie::Disabled)
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trgie::Enabled)
    }
}
#[doc = "Break interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::Brkie;
#[doc = "Field `BRKIE` reader - Break interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::BrkieR;
#[doc = "Field `BRKIE` writer - Break interrupt enable"]
pub use crate::gd32e230::timer0::dmainten::BrkieW;
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upden {
    #[doc = "0: Update DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Update DMA request enabled"]
    Enabled = 1,
}
impl From<Upden> for bool {
    #[inline(always)]
    fn from(variant: Upden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UpdenR = crate::BitReader<Upden>;
impl UpdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upden {
        match self.bits {
            false => Upden::Disabled,
            true => Upden::Enabled,
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Upden::Disabled
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Upden::Enabled
    }
}
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UpdenW<'a, REG> = crate::BitWriter<'a, REG, Upden>;
impl<'a, REG> UpdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Upden::Disabled)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Upden::Enabled)
    }
}
#[doc = "Capture/Compare 0 DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0den {
    #[doc = "0: Capture/compare DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Capture/compare DMA request enabled"]
    Enabled = 1,
}
impl From<Ch0den> for bool {
    #[inline(always)]
    fn from(variant: Ch0den) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0DEN` reader - Capture/Compare 0 DMA request enable"]
pub type Ch0denR = crate::BitReader<Ch0den>;
impl Ch0denR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0den {
        match self.bits {
            false => Ch0den::Disabled,
            true => Ch0den::Enabled,
        }
    }
    #[doc = "Capture/compare DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0den::Disabled
    }
    #[doc = "Capture/compare DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0den::Enabled
    }
}
#[doc = "Field `CH0DEN` writer - Capture/Compare 0 DMA request enable"]
pub type Ch0denW<'a, REG> = crate::BitWriter<'a, REG, Ch0den>;
impl<'a, REG> Ch0denW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0den::Disabled)
    }
    #[doc = "Capture/compare DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0den::Enabled)
    }
}
#[doc = "Field `CH1DEN` reader - Capture/Compare 1 DMA request enable"]
pub use Ch0denR as Ch1denR;
#[doc = "Field `CH1DEN` writer - Capture/Compare 1 DMA request enable"]
pub use Ch0denW as Ch1denW;
#[doc = "Field `CMTDEN` reader - Commutation DMA request enable"]
pub type CmtdenR = crate::BitReader;
#[doc = "Field `CMTDEN` writer - Commutation DMA request enable"]
pub type CmtdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Trigger DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgden {
    #[doc = "0: Trigger DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Trigger DMA request enabled"]
    Enabled = 1,
}
impl From<Trgden> for bool {
    #[inline(always)]
    fn from(variant: Trgden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGDEN` reader - Trigger DMA request enable"]
pub type TrgdenR = crate::BitReader<Trgden>;
impl TrgdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgden {
        match self.bits {
            false => Trgden::Disabled,
            true => Trgden::Enabled,
        }
    }
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trgden::Disabled
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trgden::Enabled
    }
}
#[doc = "Field `TRGDEN` writer - Trigger DMA request enable"]
pub type TrgdenW<'a, REG> = crate::BitWriter<'a, REG, Trgden>;
impl<'a, REG> TrgdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trgden::Disabled)
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trgden::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> Ch0ieR {
        Ch0ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&self) -> Ch1ieR {
        Ch1ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&self) -> CmtieR {
        CmtieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&self) -> TrgieR {
        TrgieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&self) -> BrkieR {
        BrkieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UpdenR {
        UpdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 0 DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> Ch0denR {
        Ch0denR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&self) -> Ch1denR {
        Ch1denR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Commutation DMA request enable"]
    #[inline(always)]
    pub fn cmtden(&self) -> CmtdenR {
        CmtdenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&self) -> TrgdenR {
        TrgdenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UpieW<DmaintenSpec> {
        UpieW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ie(&mut self) -> Ch0ieW<DmaintenSpec> {
        Ch0ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ie(&mut self) -> Ch1ieW<DmaintenSpec> {
        Ch1ieW::new(self, 2)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtie(&mut self) -> CmtieW<DmaintenSpec> {
        CmtieW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgie(&mut self) -> TrgieW<DmaintenSpec> {
        TrgieW::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brkie(&mut self) -> BrkieW<DmaintenSpec> {
        BrkieW::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn upden(&mut self) -> UpdenW<DmaintenSpec> {
        UpdenW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 0 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0den(&mut self) -> Ch0denW<DmaintenSpec> {
        Ch0denW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1den(&mut self) -> Ch1denW<DmaintenSpec> {
        Ch1denW::new(self, 10)
    }
    #[doc = "Bit 13 - Commutation DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtden(&mut self) -> CmtdenW<DmaintenSpec> {
        CmtdenW::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgden(&mut self) -> TrgdenW<DmaintenSpec> {
        TrgdenW::new(self, 14)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmainten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintenSpec;
impl crate::RegisterSpec for DmaintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DmaintenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DmaintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DmaintenSpec {
    const RESET_VALUE: u32 = 0;
}
