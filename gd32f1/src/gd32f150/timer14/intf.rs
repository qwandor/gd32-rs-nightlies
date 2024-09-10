#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<IntfSpec>;
#[doc = "Channel 0 interrupt enable"]
pub use crate::gd32f150::timer0::intf::Ch0if;
#[doc = "Field `CH0IF` reader - Channel 0 interrupt enable"]
pub use crate::gd32f150::timer0::intf::Ch0ifR;
#[doc = "Field `CH1IF` reader - Channel 1 interrupt enable"]
pub use crate::gd32f150::timer0::intf::Ch0ifR as Ch1ifR;
#[doc = "Field `CH0IF` writer - Channel 0 interrupt enable"]
pub use crate::gd32f150::timer0::intf::Ch0ifW;
#[doc = "Field `CH1IF` writer - Channel 1 interrupt enable"]
pub use crate::gd32f150::timer0::intf::Ch0ifW as Ch1ifW;
#[doc = "Channel commutation interrupt flag"]
pub use crate::gd32f150::timer0::intf::Cmtif;
#[doc = "Field `CMTIF` reader - Channel commutation interrupt flag"]
pub use crate::gd32f150::timer0::intf::CmtifR;
#[doc = "Field `CMTIF` writer - Channel commutation interrupt flag"]
pub use crate::gd32f150::timer0::intf::CmtifW;
#[doc = "Update interrupt flag"]
pub use crate::gd32f150::timer0::intf::Upif;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub use crate::gd32f150::timer0::intf::UpifR;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub use crate::gd32f150::timer0::intf::UpifW;
#[doc = "Trigger interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgif {
    #[doc = "0: No trigger event occured"]
    Clear = 0,
    #[doc = "1: Trigger event occurred"]
    Triggered = 1,
}
impl From<Trgif> for bool {
    #[inline(always)]
    fn from(variant: Trgif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TrgifR = crate::BitReader<Trgif>;
impl TrgifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgif {
        match self.bits {
            false => Trgif::Clear,
            true => Trgif::Triggered,
        }
    }
    #[doc = "No trigger event occured"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Trgif::Clear
    }
    #[doc = "Trigger event occurred"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == Trgif::Triggered
    }
}
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TrgifW<'a, REG> = crate::BitWriter<'a, REG, Trgif>;
impl<'a, REG> TrgifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger event occured"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Trgif::Clear)
    }
    #[doc = "Trigger event occurred"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(Trgif::Triggered)
    }
}
#[doc = "Break interrupt flag"]
pub use crate::gd32f150::timer0::intf::Brkif;
#[doc = "Field `BRKIF` reader - Break interrupt flag"]
pub use crate::gd32f150::timer0::intf::BrkifR;
#[doc = "Field `BRKIF` writer - Break interrupt flag"]
pub use crate::gd32f150::timer0::intf::BrkifW;
#[doc = "Channel 0 Capture overflow flag"]
pub use crate::gd32f150::timer0::intf::Ch0of;
#[doc = "Field `CH0OF` reader - Channel 0 Capture overflow flag"]
pub use crate::gd32f150::timer0::intf::Ch0ofR;
#[doc = "Field `CH1OF` reader - Channel 1 Capture overflow flag"]
pub use crate::gd32f150::timer0::intf::Ch0ofR as Ch1ofR;
#[doc = "Field `CH0OF` writer - Channel 0 Capture overflow flag"]
pub use crate::gd32f150::timer0::intf::Ch0ofW;
#[doc = "Field `CH1OF` writer - Channel 1 Capture overflow flag"]
pub use crate::gd32f150::timer0::intf::Ch0ofW as Ch1ofW;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UpifR {
        UpifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 interrupt enable"]
    #[inline(always)]
    pub fn ch0if(&self) -> Ch0ifR {
        Ch0ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn ch1if(&self) -> Ch1ifR {
        Ch1ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&self) -> CmtifR {
        CmtifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TrgifR {
        TrgifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn brkif(&self) -> BrkifR {
        BrkifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> Ch0ofR {
        Ch0ofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 Capture overflow flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> Ch1ofR {
        Ch1ofR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn upif(&mut self) -> UpifW<IntfSpec> {
        UpifW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0if(&mut self) -> Ch0ifW<IntfSpec> {
        Ch0ifW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1if(&mut self) -> Ch1ifW<IntfSpec> {
        Ch1ifW::new(self, 2)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmtif(&mut self) -> CmtifW<IntfSpec> {
        CmtifW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgif(&mut self) -> TrgifW<IntfSpec> {
        TrgifW::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn brkif(&mut self) -> BrkifW<IntfSpec> {
        BrkifW::new(self, 7)
    }
    #[doc = "Bit 9 - Channel 0 Capture overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> Ch0ofW<IntfSpec> {
        Ch0ofW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 1 Capture overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1of(&mut self) -> Ch1ofW<IntfSpec> {
        Ch1ofW::new(self, 10)
    }
}
#[doc = "interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for IntfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {
    const RESET_VALUE: u16 = 0;
}
