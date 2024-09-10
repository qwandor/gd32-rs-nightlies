#[doc = "Register `SWCMD` reader"]
pub type R = crate::R<SwcmdSpec>;
#[doc = "Register `SWCMD` writer"]
pub type W = crate::W<SwcmdSpec>;
#[doc = "Field `WCMD` reader - SPI Write Command for AHB write transfer"]
pub type WcmdR = crate::FieldReader<u16>;
#[doc = "Field `WCMD` writer - SPI Write Command for AHB write transfer"]
pub type WcmdW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `WWAITCYCLE` reader - SPI Write Wait Cycle number after address phase"]
pub type WwaitcycleR = crate::FieldReader;
#[doc = "Field `WWAITCYCLE` writer - SPI Write Wait Cycle number after address phase"]
pub type WwaitcycleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WMODE` reader - SPI PSRAM Write command mode"]
pub type WmodeR = crate::FieldReader;
#[doc = "Field `WMODE` writer - SPI PSRAM Write command mode"]
pub type WmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SC` reader - Send SPI Special Command which does not have address and data phase, command code and mode come from WCMD and WMODE"]
pub type ScR = crate::BitReader;
#[doc = "Field `SC` writer - Send SPI Special Command which does not have address and data phase, command code and mode come from WCMD and WMODE"]
pub type ScW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - SPI Write Command for AHB write transfer"]
    #[inline(always)]
    pub fn wcmd(&self) -> WcmdR {
        WcmdR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:19 - SPI Write Wait Cycle number after address phase"]
    #[inline(always)]
    pub fn wwaitcycle(&self) -> WwaitcycleR {
        WwaitcycleR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - SPI PSRAM Write command mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WmodeR {
        WmodeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 31 - Send SPI Special Command which does not have address and data phase, command code and mode come from WCMD and WMODE"]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - SPI Write Command for AHB write transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wcmd(&mut self) -> WcmdW<SwcmdSpec> {
        WcmdW::new(self, 0)
    }
    #[doc = "Bits 16:19 - SPI Write Wait Cycle number after address phase"]
    #[inline(always)]
    #[must_use]
    pub fn wwaitcycle(&mut self) -> WwaitcycleW<SwcmdSpec> {
        WwaitcycleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - SPI PSRAM Write command mode"]
    #[inline(always)]
    #[must_use]
    pub fn wmode(&mut self) -> WmodeW<SwcmdSpec> {
        WmodeW::new(self, 20)
    }
    #[doc = "Bit 31 - Send SPI Special Command which does not have address and data phase, command code and mode come from WCMD and WMODE"]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> ScW<SwcmdSpec> {
        ScW::new(self, 31)
    }
}
#[doc = "SPI write command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwcmdSpec;
impl crate::RegisterSpec for SwcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swcmd::R`](R) reader structure"]
impl crate::Readable for SwcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`swcmd::W`](W) writer structure"]
impl crate::Writable for SwcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWCMD to value 0"]
impl crate::Resettable for SwcmdSpec {
    const RESET_VALUE: u32 = 0;
}
