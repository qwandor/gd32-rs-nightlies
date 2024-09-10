#[doc = "Register `SNWTCFG` reader"]
pub type R = crate::R<SnwtcfgSpec>;
#[doc = "Register `SNWTCFG` writer"]
pub type W = crate::W<SnwtcfgSpec>;
#[doc = "Field `WASET` reader - address setup time"]
pub type WasetR = crate::FieldReader;
#[doc = "Field `WASET` writer - address setup time"]
pub type WasetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAHLD` reader - address hold time"]
pub type WahldR = crate::FieldReader;
#[doc = "Field `WAHLD` writer - address hold time"]
pub type WahldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WDSET` reader - data setup time"]
pub type WdsetR = crate::FieldReader;
#[doc = "Field `WDSET` writer - data setup time"]
pub type WdsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WBUSLAT` reader - bus latency"]
pub type WbuslatR = crate::FieldReader;
#[doc = "Field `WBUSLAT` writer - bus latency"]
pub type WbuslatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WASYNCMOD` reader - asynchronous access mode"]
pub type WasyncmodR = crate::FieldReader;
#[doc = "Field `WASYNCMOD` writer - asynchronous access mode"]
pub type WasyncmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - address setup time"]
    #[inline(always)]
    pub fn waset(&self) -> WasetR {
        WasetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - address hold time"]
    #[inline(always)]
    pub fn wahld(&self) -> WahldR {
        WahldR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - data setup time"]
    #[inline(always)]
    pub fn wdset(&self) -> WdsetR {
        WdsetR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - bus latency"]
    #[inline(always)]
    pub fn wbuslat(&self) -> WbuslatR {
        WbuslatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - asynchronous access mode"]
    #[inline(always)]
    pub fn wasyncmod(&self) -> WasyncmodR {
        WasyncmodR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn waset(&mut self) -> WasetW<SnwtcfgSpec> {
        WasetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - address hold time"]
    #[inline(always)]
    #[must_use]
    pub fn wahld(&mut self) -> WahldW<SnwtcfgSpec> {
        WahldW::new(self, 4)
    }
    #[doc = "Bits 8:15 - data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn wdset(&mut self) -> WdsetW<SnwtcfgSpec> {
        WdsetW::new(self, 8)
    }
    #[doc = "Bits 16:19 - bus latency"]
    #[inline(always)]
    #[must_use]
    pub fn wbuslat(&mut self) -> WbuslatW<SnwtcfgSpec> {
        WbuslatW::new(self, 16)
    }
    #[doc = "Bits 28:29 - asynchronous access mode"]
    #[inline(always)]
    #[must_use]
    pub fn wasyncmod(&mut self) -> WasyncmodW<SnwtcfgSpec> {
        WasyncmodW::new(self, 28)
    }
}
#[doc = "SRAM/NOR write timing configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snwtcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snwtcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnwtcfgSpec;
impl crate::RegisterSpec for SnwtcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snwtcfg::R`](R) reader structure"]
impl crate::Readable for SnwtcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`snwtcfg::W`](W) writer structure"]
impl crate::Writable for SnwtcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNWTCFG to value 0x0fff_ffff"]
impl crate::Resettable for SnwtcfgSpec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
