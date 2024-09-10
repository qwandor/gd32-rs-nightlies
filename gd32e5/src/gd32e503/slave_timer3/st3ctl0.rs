#[doc = "Register `ST3CTL0` reader"]
pub type R = crate::R<St3ctl0Spec>;
#[doc = "Register `ST3CTL0` writer"]
pub type W = crate::W<St3ctl0Spec>;
#[doc = "Field `CNTCKDIV` reader - Counter clock division"]
pub type CntckdivR = crate::FieldReader;
#[doc = "Field `CNTCKDIV` writer - Counter clock division"]
pub type CntckdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTNM` reader - Continuous mode"]
pub type CtnmR = crate::BitReader;
#[doc = "Field `CTNM` writer - Continuous mode"]
pub type CtnmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTRSTM` reader - Counter reset mode"]
pub type CntrstmR = crate::BitReader;
#[doc = "Field `CNTRSTM` writer - Counter reset mode"]
pub type CntrstmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALFM` reader - Half mode"]
pub type HalfmR = crate::BitReader;
#[doc = "Field `HALFM` writer - Half mode"]
pub type HalfmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLNMEN` reader - Balanced mode enable"]
pub type BlnmenR = crate::BitReader;
#[doc = "Field `BLNMEN` writer - Balanced mode enable"]
pub type BlnmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNIRST` reader - Synchronization input reset counter"]
pub type SynirstR = crate::BitReader;
#[doc = "Field `SYNIRST` writer - Synchronization input reset counter"]
pub type SynirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNISTRT` reader - Synchronization input start counter"]
pub type SynistrtR = crate::BitReader;
#[doc = "Field `SYNISTRT` writer - Synchronization input start counter"]
pub type SynistrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELCMP1M` reader - Compare 1 delayed mode"]
pub type Delcmp1mR = crate::FieldReader;
#[doc = "Field `DELCMP1M` writer - Compare 1 delayed mode"]
pub type Delcmp1mW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELCMP3M` reader - Compare 3 delayed mode"]
pub type Delcmp3mR = crate::FieldReader;
#[doc = "Field `DELCMP3M` writer - Compare 3 delayed mode"]
pub type Delcmp3mW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UPREP` reader - Update event generated by repetition event"]
pub type UprepR = crate::BitReader;
#[doc = "Field `UPREP` writer - Update event generated by repetition event"]
pub type UprepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRST` reader - Update event generated by reset event"]
pub type UprstR = crate::BitReader;
#[doc = "Field `UPRST` writer - Update event generated by reset event"]
pub type UprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPBST0` reader - Update by Slave_TIMER0 update event"]
pub type Upbst0R = crate::BitReader;
#[doc = "Field `UPBST0` writer - Update by Slave_TIMER0 update event"]
pub type Upbst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPBST1` reader - Update by Slave_TIMER1 update event"]
pub type Upbst1R = crate::BitReader;
#[doc = "Field `UPBST1` writer - Update by Slave_TIMER1 update event"]
pub type Upbst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPBST2` reader - Update by Slave_TIMER2 update event"]
pub type Upbst2R = crate::BitReader;
#[doc = "Field `UPBST2` writer - Update by Slave_TIMER2 update event"]
pub type Upbst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPBST4` reader - Update by Slave_TIMER4 update event"]
pub type Upbst4R = crate::BitReader;
#[doc = "Field `UPBST4` writer - Update by Slave_TIMER4 update event"]
pub type Upbst4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPBMT` reader - Update by Master_TIMER update event"]
pub type UpbmtR = crate::BitReader;
#[doc = "Field `UPBMT` writer - Update by Master_TIMER update event"]
pub type UpbmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACTRGS` reader - Trigger source to DAC"]
pub type DactrgsR = crate::FieldReader;
#[doc = "Field `DACTRGS` writer - Trigger source to DAC"]
pub type DactrgsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SHWEN` reader - Shadow registers enable"]
pub type ShwenR = crate::BitReader;
#[doc = "Field `SHWEN` writer - Shadow registers enable"]
pub type ShwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPSEL` reader - Update event selection"]
pub type UpselR = crate::FieldReader;
#[doc = "Field `UPSEL` writer - Update event selection"]
pub type UpselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Counter clock division"]
    #[inline(always)]
    pub fn cntckdiv(&self) -> CntckdivR {
        CntckdivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn ctnm(&self) -> CtnmR {
        CtnmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter reset mode"]
    #[inline(always)]
    pub fn cntrstm(&self) -> CntrstmR {
        CntrstmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode"]
    #[inline(always)]
    pub fn halfm(&self) -> HalfmR {
        HalfmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Balanced mode enable"]
    #[inline(always)]
    pub fn blnmen(&self) -> BlnmenR {
        BlnmenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization input reset counter"]
    #[inline(always)]
    pub fn synirst(&self) -> SynirstR {
        SynirstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization input start counter"]
    #[inline(always)]
    pub fn synistrt(&self) -> SynistrtR {
        SynistrtR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Compare 1 delayed mode"]
    #[inline(always)]
    pub fn delcmp1m(&self) -> Delcmp1mR {
        Delcmp1mR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Compare 3 delayed mode"]
    #[inline(always)]
    pub fn delcmp3m(&self) -> Delcmp3mR {
        Delcmp3mR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Update event generated by repetition event"]
    #[inline(always)]
    pub fn uprep(&self) -> UprepR {
        UprepR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Update event generated by reset event"]
    #[inline(always)]
    pub fn uprst(&self) -> UprstR {
        UprstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Update by Slave_TIMER0 update event"]
    #[inline(always)]
    pub fn upbst0(&self) -> Upbst0R {
        Upbst0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Update by Slave_TIMER1 update event"]
    #[inline(always)]
    pub fn upbst1(&self) -> Upbst1R {
        Upbst1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Update by Slave_TIMER2 update event"]
    #[inline(always)]
    pub fn upbst2(&self) -> Upbst2R {
        Upbst2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Update by Slave_TIMER4 update event"]
    #[inline(always)]
    pub fn upbst4(&self) -> Upbst4R {
        Upbst4R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Update by Master_TIMER update event"]
    #[inline(always)]
    pub fn upbmt(&self) -> UpbmtR {
        UpbmtR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Trigger source to DAC"]
    #[inline(always)]
    pub fn dactrgs(&self) -> DactrgsR {
        DactrgsR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Shadow registers enable"]
    #[inline(always)]
    pub fn shwen(&self) -> ShwenR {
        ShwenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Update event selection"]
    #[inline(always)]
    pub fn upsel(&self) -> UpselR {
        UpselR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Counter clock division"]
    #[inline(always)]
    #[must_use]
    pub fn cntckdiv(&mut self) -> CntckdivW<St3ctl0Spec> {
        CntckdivW::new(self, 0)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctnm(&mut self) -> CtnmW<St3ctl0Spec> {
        CtnmW::new(self, 3)
    }
    #[doc = "Bit 4 - Counter reset mode"]
    #[inline(always)]
    #[must_use]
    pub fn cntrstm(&mut self) -> CntrstmW<St3ctl0Spec> {
        CntrstmW::new(self, 4)
    }
    #[doc = "Bit 5 - Half mode"]
    #[inline(always)]
    #[must_use]
    pub fn halfm(&mut self) -> HalfmW<St3ctl0Spec> {
        HalfmW::new(self, 5)
    }
    #[doc = "Bit 6 - Balanced mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn blnmen(&mut self) -> BlnmenW<St3ctl0Spec> {
        BlnmenW::new(self, 6)
    }
    #[doc = "Bit 10 - Synchronization input reset counter"]
    #[inline(always)]
    #[must_use]
    pub fn synirst(&mut self) -> SynirstW<St3ctl0Spec> {
        SynirstW::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronization input start counter"]
    #[inline(always)]
    #[must_use]
    pub fn synistrt(&mut self) -> SynistrtW<St3ctl0Spec> {
        SynistrtW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Compare 1 delayed mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp1m(&mut self) -> Delcmp1mW<St3ctl0Spec> {
        Delcmp1mW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Compare 3 delayed mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp3m(&mut self) -> Delcmp3mW<St3ctl0Spec> {
        Delcmp3mW::new(self, 14)
    }
    #[doc = "Bit 17 - Update event generated by repetition event"]
    #[inline(always)]
    #[must_use]
    pub fn uprep(&mut self) -> UprepW<St3ctl0Spec> {
        UprepW::new(self, 17)
    }
    #[doc = "Bit 18 - Update event generated by reset event"]
    #[inline(always)]
    #[must_use]
    pub fn uprst(&mut self) -> UprstW<St3ctl0Spec> {
        UprstW::new(self, 18)
    }
    #[doc = "Bit 19 - Update by Slave_TIMER0 update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbst0(&mut self) -> Upbst0W<St3ctl0Spec> {
        Upbst0W::new(self, 19)
    }
    #[doc = "Bit 20 - Update by Slave_TIMER1 update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbst1(&mut self) -> Upbst1W<St3ctl0Spec> {
        Upbst1W::new(self, 20)
    }
    #[doc = "Bit 21 - Update by Slave_TIMER2 update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbst2(&mut self) -> Upbst2W<St3ctl0Spec> {
        Upbst2W::new(self, 21)
    }
    #[doc = "Bit 23 - Update by Slave_TIMER4 update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbst4(&mut self) -> Upbst4W<St3ctl0Spec> {
        Upbst4W::new(self, 23)
    }
    #[doc = "Bit 24 - Update by Master_TIMER update event"]
    #[inline(always)]
    #[must_use]
    pub fn upbmt(&mut self) -> UpbmtW<St3ctl0Spec> {
        UpbmtW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Trigger source to DAC"]
    #[inline(always)]
    #[must_use]
    pub fn dactrgs(&mut self) -> DactrgsW<St3ctl0Spec> {
        DactrgsW::new(self, 25)
    }
    #[doc = "Bit 27 - Shadow registers enable"]
    #[inline(always)]
    #[must_use]
    pub fn shwen(&mut self) -> ShwenW<St3ctl0Spec> {
        ShwenW::new(self, 27)
    }
    #[doc = "Bits 28:31 - Update event selection"]
    #[inline(always)]
    #[must_use]
    pub fn upsel(&mut self) -> UpselW<St3ctl0Spec> {
        UpselW::new(self, 28)
    }
}
#[doc = "SHRTIMER Slave_TIMERx control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St3ctl0Spec;
impl crate::RegisterSpec for St3ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st3ctl0::R`](R) reader structure"]
impl crate::Readable for St3ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`st3ctl0::W`](W) writer structure"]
impl crate::Writable for St3ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST3CTL0 to value 0"]
impl crate::Resettable for St3ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
