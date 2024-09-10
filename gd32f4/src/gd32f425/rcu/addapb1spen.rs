#[doc = "Register `ADDAPB1SPEN` reader"]
pub type R = crate::R<Addapb1spenSpec>;
#[doc = "Register `ADDAPB1SPEN` writer"]
pub type W = crate::W<Addapb1spenSpec>;
#[doc = "Field `CTCSPEN` reader - CTC enable when sleep mode"]
pub type CtcspenR = crate::BitReader;
#[doc = "Field `CTCSPEN` writer - CTC enable when sleep mode"]
pub type CtcspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREFSPEN` reader - IREF enable when sleep mode"]
pub type IrefspenR = crate::BitReader;
#[doc = "Field `IREFSPEN` writer - IREF enable when sleep mode"]
pub type IrefspenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - CTC enable when sleep mode"]
    #[inline(always)]
    pub fn ctcspen(&self) -> CtcspenR {
        CtcspenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - IREF enable when sleep mode"]
    #[inline(always)]
    pub fn irefspen(&self) -> IrefspenR {
        IrefspenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctcspen(&mut self) -> CtcspenW<Addapb1spenSpec> {
        CtcspenW::new(self, 27)
    }
    #[doc = "Bit 31 - IREF enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn irefspen(&mut self) -> IrefspenW<Addapb1spenSpec> {
        IrefspenW::new(self, 31)
    }
}
#[doc = "APB1 additional sleep mode enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1spen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1spen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addapb1spenSpec;
impl crate::RegisterSpec for Addapb1spenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1spen::R`](R) reader structure"]
impl crate::Readable for Addapb1spenSpec {}
#[doc = "`write(|w| ..)` method takes [`addapb1spen::W`](W) writer structure"]
impl crate::Writable for Addapb1spenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDAPB1SPEN to value 0x8800_0000"]
impl crate::Resettable for Addapb1spenSpec {
    const RESET_VALUE: u32 = 0x8800_0000;
}
