#[doc = "Register `SAMPT0` reader"]
pub type R = crate::R<Sampt0Spec>;
#[doc = "Register `SAMPT0` writer"]
pub type W = crate::W<Sampt0Spec>;
#[doc = "Channel 16 sample time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spt16 {
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
impl From<Spt16> for u8 {
    #[inline(always)]
    fn from(variant: Spt16) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spt16 {
    type Ux = u8;
}
#[doc = "Field `SPT16` reader - Channel 16 sample time selection"]
pub type Spt16R = crate::FieldReader<Spt16>;
impl Spt16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spt16 {
        match self.bits {
            0 => Spt16::Cycles1_5,
            1 => Spt16::Cycles7_5,
            2 => Spt16::Cycles13_5,
            3 => Spt16::Cycles28_5,
            4 => Spt16::Cycles41_5,
            5 => Spt16::Cycles55_5,
            6 => Spt16::Cycles71_5,
            7 => Spt16::Cycles239_5,
            _ => unreachable!(),
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == Spt16::Cycles1_5
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == Spt16::Cycles7_5
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == Spt16::Cycles13_5
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == Spt16::Cycles28_5
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == Spt16::Cycles41_5
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == Spt16::Cycles55_5
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == Spt16::Cycles71_5
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == Spt16::Cycles239_5
    }
}
#[doc = "Field `SPT16` writer - Channel 16 sample time selection"]
pub type Spt16W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Spt16>;
impl<'a, REG> Spt16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt16::Cycles1_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt16::Cycles7_5)
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt16::Cycles13_5)
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt16::Cycles28_5)
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt16::Cycles41_5)
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt16::Cycles55_5)
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt16::Cycles71_5)
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(Spt16::Cycles239_5)
    }
}
#[doc = "Field `SPT17` reader - Channel 17 sample time selection"]
pub use Spt16R as Spt17R;
#[doc = "Field `SPT17` writer - Channel 17 sample time selection"]
pub use Spt16W as Spt17W;
impl R {
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
}
impl W {
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
}
#[doc = "Sampling time register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
