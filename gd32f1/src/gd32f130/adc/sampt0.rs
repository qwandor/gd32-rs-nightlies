#[doc = "Register `SAMPT0` reader"]
pub type R = crate::R<Sampt0Spec>;
#[doc = "Register `SAMPT0` writer"]
pub type W = crate::W<Sampt0Spec>;
#[doc = "Channel 10 sample time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spt10 {
    #[doc = "0: 1.5 ADC clock cycles"]
    Cycles1_5 = 0,
    #[doc = "1: 7.5 ADC clock cycles"]
    Cycles7_5 = 1,
    #[doc = "2: 13.5 ADC clock cycles"]
    Cycles13_5 = 2,
    #[doc = "3: 28.5 ADC clock cycles"]
    Cycles28_5 = 3,
    #[doc = "4: 41.5 ADC clock cycles"]
    Cycles41_5 = 4,
    #[doc = "5: 55.5 ADC clock cycles"]
    Cycles55_5 = 5,
    #[doc = "6: 71.5 ADC clock cycles"]
    Cycles71_5 = 6,
    #[doc = "7: 239.5 ADC clock cycles"]
    Cycles239_5 = 7,
}
impl From<Spt10> for u8 {
    #[inline(always)]
    fn from(variant: Spt10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spt10 {
    type Ux = u8;
}
#[doc = "Field `SPT10` reader - Channel 10 sample time selection"]
pub type Spt10R = crate::FieldReader<Spt10>;
impl Spt10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spt10 {
        match self.bits {
            0 => Spt10::Cycles1_5,
            1 => Spt10::Cycles7_5,
            2 => Spt10::Cycles13_5,
            3 => Spt10::Cycles28_5,
            4 => Spt10::Cycles41_5,
            5 => Spt10::Cycles55_5,
            6 => Spt10::Cycles71_5,
            7 => Spt10::Cycles239_5,
            _ => unreachable!(),
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == Spt10::Cycles1_5
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == Spt10::Cycles7_5
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == Spt10::Cycles13_5
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == Spt10::Cycles28_5
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == Spt10::Cycles41_5
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == Spt10::Cycles55_5
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == Spt10::Cycles71_5
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == Spt10::Cycles239_5
    }
}
#[doc = "Field `SPT10` writer - Channel 10 sample time selection"]
pub type Spt10W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Spt10>;
impl<'a, REG> Spt10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles1_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles7_5)
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles13_5)
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles28_5)
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles41_5)
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles55_5)
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles71_5)
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt10::Cycles239_5)
    }
}
#[doc = "Field `SPT11` reader - Channel 11 sample time selection"]
pub use Spt10R as Spt11R;
#[doc = "Field `SPT12` reader - Channel 12 sample time selection"]
pub use Spt10R as Spt12R;
#[doc = "Field `SPT13` reader - Channel 13 sample time selection"]
pub use Spt10R as Spt13R;
#[doc = "Field `SPT14` reader - Channel 14 sample time selection"]
pub use Spt10R as Spt14R;
#[doc = "Field `SPT15` reader - Channel 15 sample time selection"]
pub use Spt10R as Spt15R;
#[doc = "Field `SPT16` reader - Channel 16 sample time selection"]
pub use Spt10R as Spt16R;
#[doc = "Field `SPT17` reader - Channel 17 sample time selection"]
pub use Spt10R as Spt17R;
#[doc = "Field `SPT18` reader - Channel 18 sample time selection"]
pub use Spt10R as Spt18R;
#[doc = "Field `SPT11` writer - Channel 11 sample time selection"]
pub use Spt10W as Spt11W;
#[doc = "Field `SPT12` writer - Channel 12 sample time selection"]
pub use Spt10W as Spt12W;
#[doc = "Field `SPT13` writer - Channel 13 sample time selection"]
pub use Spt10W as Spt13W;
#[doc = "Field `SPT14` writer - Channel 14 sample time selection"]
pub use Spt10W as Spt14W;
#[doc = "Field `SPT15` writer - Channel 15 sample time selection"]
pub use Spt10W as Spt15W;
#[doc = "Field `SPT16` writer - Channel 16 sample time selection"]
pub use Spt10W as Spt16W;
#[doc = "Field `SPT17` writer - Channel 17 sample time selection"]
pub use Spt10W as Spt17W;
#[doc = "Field `SPT18` writer - Channel 18 sample time selection"]
pub use Spt10W as Spt18W;
impl R {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn spt10(&self) -> Spt10R {
        Spt10R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn spt11(&self) -> Spt11R {
        Spt11R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn spt12(&self) -> Spt12R {
        Spt12R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn spt13(&self) -> Spt13R {
        Spt13R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn spt14(&self) -> Spt14R {
        Spt14R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn spt15(&self) -> Spt15R {
        Spt15R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&self) -> Spt16R {
        Spt16R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&self) -> Spt17R {
        Spt17R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 18 sample time selection"]
    #[inline(always)]
    pub fn spt18(&self) -> Spt18R {
        Spt18R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt10(&mut self) -> Spt10W<Sampt0Spec> {
        Spt10W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt11(&mut self) -> Spt11W<Sampt0Spec> {
        Spt11W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt12(&mut self) -> Spt12W<Sampt0Spec> {
        Spt12W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt13(&mut self) -> Spt13W<Sampt0Spec> {
        Spt13W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt14(&mut self) -> Spt14W<Sampt0Spec> {
        Spt14W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt15(&mut self) -> Spt15W<Sampt0Spec> {
        Spt15W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt16(&mut self) -> Spt16W<Sampt0Spec> {
        Spt16W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt17(&mut self) -> Spt17W<Sampt0Spec> {
        Spt17W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel 18 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn spt18(&mut self) -> Spt18W<Sampt0Spec> {
        Spt18W::new(self, 24)
    }
}
#[doc = "Sampling time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sampt0Spec;
impl crate::RegisterSpec for Sampt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sampt0::R`](R) reader structure"]
impl crate::Readable for Sampt0Spec {}
#[doc = "`write(|w| ..)` method takes [`sampt0::W`](W) writer structure"]
impl crate::Writable for Sampt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAMPT0 to value 0"]
impl crate::Resettable for Sampt0Spec {
    const RESET_VALUE: u32 = 0;
}
