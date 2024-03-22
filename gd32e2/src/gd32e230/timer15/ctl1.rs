#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Capture/compare preloaded control"]
pub use crate::gd32e230::timer0::ctl1::Ccse;
#[doc = "Field `CCSE` reader - Capture/compare preloaded control"]
pub use crate::gd32e230::timer0::ctl1::CcseR;
#[doc = "Field `CCSE` writer - Capture/compare preloaded control"]
pub use crate::gd32e230::timer0::ctl1::CcseW;
#[doc = "Capture/compare control update selection"]
pub use crate::gd32e230::timer0::ctl1::Ccuc;
#[doc = "Field `CCUC` reader - Capture/compare control update selection"]
pub use crate::gd32e230::timer0::ctl1::CcucR;
#[doc = "Field `CCUC` writer - Capture/compare control update selection"]
pub use crate::gd32e230::timer0::ctl1::CcucW;
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmas {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    OnCompare = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    OnUpdate = 1,
}
impl From<Dmas> for bool {
    #[inline(always)]
    fn from(variant: Dmas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAS` reader - Capture/compare DMA selection"]
pub type DmasR = crate::BitReader<Dmas>;
impl DmasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmas {
        match self.bits {
            false => Dmas::OnCompare,
            true => Dmas::OnUpdate,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == Dmas::OnCompare
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == Dmas::OnUpdate
    }
}
#[doc = "Field `DMAS` writer - Capture/compare DMA selection"]
pub type DmasW<'a, REG> = crate::BitWriter<'a, REG, Dmas>;
impl<'a, REG> DmasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut crate::W<REG> {
        self.variant(Dmas::OnCompare)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut crate::W<REG> {
        self.variant(Dmas::OnUpdate)
    }
}
#[doc = "Output Idle state 0"]
pub use crate::gd32e230::timer0::ctl1::Iso0;
#[doc = "Field `ISO0` reader - Output Idle state 0"]
pub use crate::gd32e230::timer0::ctl1::Iso0R;
#[doc = "Field `ISO0` writer - Output Idle state 0"]
pub use crate::gd32e230::timer0::ctl1::Iso0W;
#[doc = "Output Idle state 0"]
pub use crate::gd32e230::timer0::ctl1::Iso0n;
#[doc = "Field `ISO0N` reader - Output Idle state 0"]
pub use crate::gd32e230::timer0::ctl1::Iso0nR;
#[doc = "Field `ISO0N` writer - Output Idle state 0"]
pub use crate::gd32e230::timer0::ctl1::Iso0nW;
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccse(&self) -> CcseR {
        CcseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccuc(&self) -> CcucR {
        CcucR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DmasR {
        DmasR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 0"]
    #[inline(always)]
    pub fn iso0(&self) -> Iso0R {
        Iso0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 0"]
    #[inline(always)]
    pub fn iso0n(&self) -> Iso0nR {
        Iso0nR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    #[must_use]
    pub fn ccse(&mut self) -> CcseW<Ctl1Spec> {
        CcseW::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccuc(&mut self) -> CcucW<Ctl1Spec> {
        CcucW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmas(&mut self) -> DmasW<Ctl1Spec> {
        DmasW::new(self, 3)
    }
    #[doc = "Bit 8 - Output Idle state 0"]
    #[inline(always)]
    #[must_use]
    pub fn iso0(&mut self) -> Iso0W<Ctl1Spec> {
        Iso0W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 0"]
    #[inline(always)]
    #[must_use]
    pub fn iso0n(&mut self) -> Iso0nW<Ctl1Spec> {
        Iso0nW::new(self, 9)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
