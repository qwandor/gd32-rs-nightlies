#[doc = "Register `SRCMD` reader"]
pub type R = crate::R<SrcmdSpec>;
#[doc = "Register `SRCMD` writer"]
pub type W = crate::W<SrcmdSpec>;
#[doc = "Field `RCMD` reader - SPI Read Command for AHB read transfer"]
pub type RcmdR = crate::FieldReader<u16>;
#[doc = "Field `RCMD` writer - SPI Read Command for AHB read transfer"]
pub type RcmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RWAITCYCLE` reader - SPI Read Wait Cycle number after address phase"]
pub type RwaitcycleR = crate::FieldReader;
#[doc = "Field `RWAITCYCLE` writer - SPI Read Wait Cycle number after address phase"]
pub type RwaitcycleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RMODE` reader - SPI PSRAM Read command mode"]
pub type RmodeR = crate::FieldReader;
#[doc = "Field `RMODE` writer - SPI PSRAM Read command mode"]
pub type RmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RDID` reader - Send SPI Read ID Command"]
pub type RdidR = crate::BitReader;
#[doc = "Field `RDID` writer - Send SPI Read ID Command"]
pub type RdidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - SPI Read Command for AHB read transfer"]
    #[inline(always)]
    pub fn rcmd(&self) -> RcmdR {
        RcmdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - SPI Read Wait Cycle number after address phase"]
    #[inline(always)]
    pub fn rwaitcycle(&self) -> RwaitcycleR {
        RwaitcycleR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - SPI PSRAM Read command mode"]
    #[inline(always)]
    pub fn rmode(&self) -> RmodeR {
        RmodeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 31 - Send SPI Read ID Command"]
    #[inline(always)]
    pub fn rdid(&self) -> RdidR {
        RdidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPI Read Command for AHB read transfer"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd(&mut self) -> RcmdW<SrcmdSpec> {
        RcmdW::new(self, 0)
    }
    #[doc = "Bits 16:19 - SPI Read Wait Cycle number after address phase"]
    #[inline(always)]
    #[must_use]
    pub fn rwaitcycle(&mut self) -> RwaitcycleW<SrcmdSpec> {
        RwaitcycleW::new(self, 16)
    }
    #[doc = "Bits 20:21 - SPI PSRAM Read command mode"]
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RmodeW<SrcmdSpec> {
        RmodeW::new(self, 20)
    }
    #[doc = "Bit 31 - Send SPI Read ID Command"]
    #[inline(always)]
    #[must_use]
    pub fn rdid(&mut self) -> RdidW<SrcmdSpec> {
        RdidW::new(self, 31)
    }
}
#[doc = "SPI read command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcmdSpec;
impl crate::RegisterSpec for SrcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcmd::R`](R) reader structure"]
impl crate::Readable for SrcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`srcmd::W`](W) writer structure"]
impl crate::Writable for SrcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCMD to value 0"]
impl crate::Resettable for SrcmdSpec {
    const RESET_VALUE: u32 = 0;
}
