#[doc = "Register `TPCS` reader"]
pub type R = crate::R<TpcsSpec>;
#[doc = "Register `TPCS` writer"]
pub type W = crate::W<TpcsSpec>;
#[doc = "Field `TER0` reader - Tamper0 event reset"]
pub type Ter0R = crate::BitReader;
#[doc = "Field `TER0` writer - Tamper0 event reset"]
pub type Ter0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIR0` reader - Tamper0 interrupt reset"]
pub type Tir0R = crate::BitReader;
#[doc = "Field `TIR0` writer - Tamper0 interrupt reset"]
pub type Tir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIE0` reader - Tamper0 interrupt enable"]
pub type Tpie0R = crate::BitReader;
#[doc = "Field `TPIE0` writer - Tamper0 interrupt enable"]
pub type Tpie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TER1` reader - Tamper1 event reset"]
pub type Ter1R = crate::BitReader;
#[doc = "Field `TER1` writer - Tamper1 event reset"]
pub type Ter1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIR1` reader - Tamper1 interrupt reset"]
pub type Tir1R = crate::BitReader;
#[doc = "Field `TIR1` writer - Tamper1 interrupt reset"]
pub type Tir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIE1` reader - Tamper1interrupt enable"]
pub type Tpie1R = crate::BitReader;
#[doc = "Field `TPIE1` writer - Tamper1interrupt enable"]
pub type Tpie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEF0` reader - Tamper0 event flag"]
pub type Tef0R = crate::BitReader;
#[doc = "Field `TEF0` writer - Tamper0 event flag"]
pub type Tef0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF0` reader - Tamper0 interrupt flag"]
pub type Tif0R = crate::BitReader;
#[doc = "Field `TIF0` writer - Tamper0 interrupt flag"]
pub type Tif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEF1` reader - Tamper1 event flag"]
pub type Tef1R = crate::BitReader;
#[doc = "Field `TEF1` writer - Tamper1 event flag"]
pub type Tef1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF1` reader - Tamper1 interrupt flag"]
pub type Tif1R = crate::BitReader;
#[doc = "Field `TIF1` writer - Tamper1 interrupt flag"]
pub type Tif1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper0 event reset"]
    #[inline(always)]
    pub fn ter0(&self) -> Ter0R {
        Ter0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper0 interrupt reset"]
    #[inline(always)]
    pub fn tir0(&self) -> Tir0R {
        Tir0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper0 interrupt enable"]
    #[inline(always)]
    pub fn tpie0(&self) -> Tpie0R {
        Tpie0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper1 event reset"]
    #[inline(always)]
    pub fn ter1(&self) -> Ter1R {
        Ter1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tamper1 interrupt reset"]
    #[inline(always)]
    pub fn tir1(&self) -> Tir1R {
        Tir1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper1interrupt enable"]
    #[inline(always)]
    pub fn tpie1(&self) -> Tpie1R {
        Tpie1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper0 event flag"]
    #[inline(always)]
    pub fn tef0(&self) -> Tef0R {
        Tef0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper0 interrupt flag"]
    #[inline(always)]
    pub fn tif0(&self) -> Tif0R {
        Tif0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper1 event flag"]
    #[inline(always)]
    pub fn tef1(&self) -> Tef1R {
        Tef1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tamper1 interrupt flag"]
    #[inline(always)]
    pub fn tif1(&self) -> Tif1R {
        Tif1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper0 event reset"]
    #[inline(always)]
    #[must_use]
    pub fn ter0(&mut self) -> Ter0W<TpcsSpec> {
        Ter0W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper0 interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn tir0(&mut self) -> Tir0W<TpcsSpec> {
        Tir0W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper0 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie0(&mut self) -> Tpie0W<TpcsSpec> {
        Tpie0W::new(self, 2)
    }
    #[doc = "Bit 5 - Tamper1 event reset"]
    #[inline(always)]
    #[must_use]
    pub fn ter1(&mut self) -> Ter1W<TpcsSpec> {
        Ter1W::new(self, 5)
    }
    #[doc = "Bit 6 - Tamper1 interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn tir1(&mut self) -> Tir1W<TpcsSpec> {
        Tir1W::new(self, 6)
    }
    #[doc = "Bit 7 - Tamper1interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpie1(&mut self) -> Tpie1W<TpcsSpec> {
        Tpie1W::new(self, 7)
    }
    #[doc = "Bit 8 - Tamper0 event flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef0(&mut self) -> Tef0W<TpcsSpec> {
        Tef0W::new(self, 8)
    }
    #[doc = "Bit 9 - Tamper0 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tif0(&mut self) -> Tif0W<TpcsSpec> {
        Tif0W::new(self, 9)
    }
    #[doc = "Bit 14 - Tamper1 event flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef1(&mut self) -> Tef1W<TpcsSpec> {
        Tef1W::new(self, 14)
    }
    #[doc = "Bit 15 - Tamper1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tif1(&mut self) -> Tif1W<TpcsSpec> {
        Tif1W::new(self, 15)
    }
}
#[doc = "Tamper control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpcsSpec;
impl crate::RegisterSpec for TpcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpcs::R`](R) reader structure"]
impl crate::Readable for TpcsSpec {}
#[doc = "`write(|w| ..)` method takes [`tpcs::W`](W) writer structure"]
impl crate::Writable for TpcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPCS to value 0"]
impl crate::Resettable for TpcsSpec {
    const RESET_VALUE: u32 = 0;
}
