#[doc = "Register `L1LUT` reader"]
pub type R = crate::R<L1lutSpec>;
#[doc = "Register `L1LUT` writer"]
pub type W = crate::W<L1lutSpec>;
#[doc = "Field `TB` reader - Blue channel of a LUT entry"]
pub type TbR = crate::FieldReader;
#[doc = "Field `TB` writer - Blue channel of a LUT entry"]
pub type TbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TG` reader - Green channel of a LUT entry"]
pub type TgR = crate::FieldReader;
#[doc = "Field `TG` writer - Green channel of a LUT entry"]
pub type TgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TR` reader - Red channel of a LUT entry"]
pub type TrR = crate::FieldReader;
#[doc = "Field `TR` writer - Red channel of a LUT entry"]
pub type TrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TADD` reader - Look Up Table Write Address"]
pub type TaddR = crate::FieldReader;
#[doc = "Field `TADD` writer - Look Up Table Write Address"]
pub type TaddW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Blue channel of a LUT entry"]
    #[inline(always)]
    pub fn tb(&self) -> TbR {
        TbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green channel of a LUT entry"]
    #[inline(always)]
    pub fn tg(&self) -> TgR {
        TgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Red channel of a LUT entry"]
    #[inline(always)]
    pub fn tr(&self) -> TrR {
        TrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Look Up Table Write Address"]
    #[inline(always)]
    pub fn tadd(&self) -> TaddR {
        TaddR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blue channel of a LUT entry"]
    #[inline(always)]
    #[must_use]
    pub fn tb(&mut self) -> TbW<L1lutSpec> {
        TbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green channel of a LUT entry"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TgW<L1lutSpec> {
        TgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Red channel of a LUT entry"]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TrW<L1lutSpec> {
        TrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Look Up Table Write Address"]
    #[inline(always)]
    #[must_use]
    pub fn tadd(&mut self) -> TaddW<L1lutSpec> {
        TaddW::new(self, 24)
    }
}
#[doc = "Layer 1 look up table register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1lut::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1lut::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1lutSpec;
impl crate::RegisterSpec for L1lutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1lut::R`](R) reader structure"]
impl crate::Readable for L1lutSpec {}
#[doc = "`write(|w| ..)` method takes [`l1lut::W`](W) writer structure"]
impl crate::Writable for L1lutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1LUT to value 0"]
impl crate::Resettable for L1lutSpec {
    const RESET_VALUE: u32 = 0;
}
