#[doc = "Register `I2SPSC` reader"]
pub type R = crate::R<I2spscSpec>;
#[doc = "Register `I2SPSC` writer"]
pub type W = crate::W<I2spscSpec>;
#[doc = "Field `DIV` reader - Dividing factor for the prescaler"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Dividing factor for the prescaler"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OF` reader - Odd factor for the prescaler"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - Odd factor for the prescaler"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOEN` reader - I2S_MCK output enable"]
pub type MckoenR = crate::BitReader;
#[doc = "Field `MCKOEN` writer - I2S_MCK output enable"]
pub type MckoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    pub fn mckoen(&self) -> MckoenR {
        MckoenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dividing factor for the prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<I2spscSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<I2spscSpec> {
        OfW::new(self, 8)
    }
    #[doc = "Bit 9 - I2S_MCK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckoen(&mut self) -> MckoenW<I2spscSpec> {
        MckoenW::new(self, 9)
    }
}
#[doc = "I2S prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2spsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2spsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2spscSpec;
impl crate::RegisterSpec for I2spscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2spsc::R`](R) reader structure"]
impl crate::Readable for I2spscSpec {}
#[doc = "`write(|w| ..)` method takes [`i2spsc::W`](W) writer structure"]
impl crate::Writable for I2spscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SPSC to value 0x02"]
impl crate::Resettable for I2spscSpec {
    const RESET_VALUE: u32 = 0x02;
}
