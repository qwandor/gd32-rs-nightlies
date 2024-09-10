#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `I2CCLK` reader - Peripheral clock frequency"]
pub type I2cclkR = crate::FieldReader;
#[doc = "Field `I2CCLK` writer - Peripheral clock frequency"]
pub type I2cclkW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Error interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt enabled"]
    Enabled = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader<Errie>;
impl ErrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errie {
        match self.bits {
            false => Errie::Disabled,
            true => Errie::Enabled,
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errie::Disabled
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errie::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Disabled)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Enabled)
    }
}
#[doc = "Event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evie {
    #[doc = "0: Event interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Event interrupt enabled"]
    Enabled = 1,
}
impl From<Evie> for bool {
    #[inline(always)]
    fn from(variant: Evie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVIE` reader - Event interrupt enable"]
pub type EvieR = crate::BitReader<Evie>;
impl EvieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evie {
        match self.bits {
            false => Evie::Disabled,
            true => Evie::Enabled,
        }
    }
    #[doc = "Event interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Evie::Disabled
    }
    #[doc = "Event interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Evie::Enabled
    }
}
#[doc = "Field `EVIE` writer - Event interrupt enable"]
pub type EvieW<'a, REG> = crate::BitWriter<'a, REG, Evie>;
impl<'a, REG> EvieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Evie::Disabled)
    }
    #[doc = "Event interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Evie::Enabled)
    }
}
#[doc = "Buffer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufie {
    #[doc = "0: TBE=1 or RBNE=1 does not generate any interrupt"]
    Disabled = 0,
    #[doc = "1: TBE=1 or RBNE=1 generates Event interrupt"]
    Enabled = 1,
}
impl From<Bufie> for bool {
    #[inline(always)]
    fn from(variant: Bufie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFIE` reader - Buffer interrupt enable"]
pub type BufieR = crate::BitReader<Bufie>;
impl BufieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufie {
        match self.bits {
            false => Bufie::Disabled,
            true => Bufie::Enabled,
        }
    }
    #[doc = "TBE=1 or RBNE=1 does not generate any interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bufie::Disabled
    }
    #[doc = "TBE=1 or RBNE=1 generates Event interrupt"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bufie::Enabled
    }
}
#[doc = "Field `BUFIE` writer - Buffer interrupt enable"]
pub type BufieW<'a, REG> = crate::BitWriter<'a, REG, Bufie>;
impl<'a, REG> BufieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TBE=1 or RBNE=1 does not generate any interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bufie::Disabled)
    }
    #[doc = "TBE=1 or RBNE=1 generates Event interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bufie::Enabled)
    }
}
#[doc = "DMA mode switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaon {
    #[doc = "0: DMA requests disabled"]
    Disabled = 0,
    #[doc = "1: DMA requests enabled"]
    Enabled = 1,
}
impl From<Dmaon> for bool {
    #[inline(always)]
    fn from(variant: Dmaon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAON` reader - DMA mode switch"]
pub type DmaonR = crate::BitReader<Dmaon>;
impl DmaonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaon {
        match self.bits {
            false => Dmaon::Disabled,
            true => Dmaon::Enabled,
        }
    }
    #[doc = "DMA requests disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaon::Disabled
    }
    #[doc = "DMA requests enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaon::Enabled
    }
}
#[doc = "Field `DMAON` writer - DMA mode switch"]
pub type DmaonW<'a, REG> = crate::BitWriter<'a, REG, Dmaon>;
impl<'a, REG> DmaonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA requests disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaon::Disabled)
    }
    #[doc = "DMA requests enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaon::Enabled)
    }
}
#[doc = "Flag indicating DMA last transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmalst {
    #[doc = "0: Next DMA EOT is not the last transfer"]
    NotLast = 0,
    #[doc = "1: Next DMA EOT is the last transfer"]
    Last = 1,
}
impl From<Dmalst> for bool {
    #[inline(always)]
    fn from(variant: Dmalst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMALST` reader - Flag indicating DMA last transfer"]
pub type DmalstR = crate::BitReader<Dmalst>;
impl DmalstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmalst {
        match self.bits {
            false => Dmalst::NotLast,
            true => Dmalst::Last,
        }
    }
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == Dmalst::NotLast
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == Dmalst::Last
    }
}
#[doc = "Field `DMALST` writer - Flag indicating DMA last transfer"]
pub type DmalstW<'a, REG> = crate::BitWriter<'a, REG, Dmalst>;
impl<'a, REG> DmalstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline(always)]
    pub fn not_last(self) -> &'a mut crate::W<REG> {
        self.variant(Dmalst::NotLast)
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(Dmalst::Last)
    }
}
impl R {
    #[doc = "Bits 0:6 - Peripheral clock frequency"]
    #[inline(always)]
    pub fn i2cclk(&self) -> I2cclkR {
        I2cclkR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evie(&self) -> EvieR {
        EvieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn bufie(&self) -> BufieR {
        BufieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    pub fn dmaon(&self) -> DmaonR {
        DmaonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    pub fn dmalst(&self) -> DmalstR {
        DmalstR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Peripheral clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn i2cclk(&mut self) -> I2cclkW<Ctl1Spec> {
        I2cclkW::new(self, 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ErrieW<Ctl1Spec> {
        ErrieW::new(self, 8)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn evie(&mut self) -> EvieW<Ctl1Spec> {
        EvieW::new(self, 9)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufie(&mut self) -> BufieW<Ctl1Spec> {
        BufieW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    #[must_use]
    pub fn dmaon(&mut self) -> DmaonW<Ctl1Spec> {
        DmaonW::new(self, 11)
    }
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dmalst(&mut self) -> DmalstW<Ctl1Spec> {
        DmalstW::new(self, 12)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}
