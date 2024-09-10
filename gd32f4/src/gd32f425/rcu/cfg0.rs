#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Field `SCS` reader - System clock switch"]
pub type ScsR = crate::FieldReader;
#[doc = "Field `SCS` writer - System clock switch"]
pub type ScsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCSS` reader - System clock switch status"]
pub type ScssR = crate::FieldReader;
#[doc = "Field `AHBPSC` reader - AHB prescaler selection"]
pub type AhbpscR = crate::FieldReader;
#[doc = "Field `AHBPSC` writer - AHB prescaler selection"]
pub type AhbpscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `APB1PSC` reader - APB1 prescaler selection"]
pub type Apb1pscR = crate::FieldReader;
#[doc = "Field `APB1PSC` writer - APB1 prescaler selection"]
pub type Apb1pscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APB2PSC` reader - APB2 prescaler selection"]
pub type Apb2pscR = crate::FieldReader;
#[doc = "Field `APB2PSC` writer - APB2 prescaler selection"]
pub type Apb2pscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTCDIV` reader - RTC clock divider factor"]
pub type RtcdivR = crate::FieldReader;
#[doc = "Field `RTCDIV` writer - RTC clock divider factor"]
pub type RtcdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CKOUT0SEL` reader - CKOUT0 Clock Source Selection"]
pub type Ckout0selR = crate::FieldReader;
#[doc = "Field `CKOUT0SEL` writer - CKOUT0 Clock Source Selection"]
pub type Ckout0selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SSEL` reader - I2S Clock Source Selection"]
pub type I2sselR = crate::BitReader;
#[doc = "Field `I2SSEL` writer - I2S Clock Source Selection"]
pub type I2sselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKOUT0DIV` reader - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced"]
pub type Ckout0divR = crate::FieldReader;
#[doc = "Field `CKOUT0DIV` writer - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced"]
pub type Ckout0divW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKOUT1DIV` reader - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced"]
pub type Ckout1divR = crate::FieldReader;
#[doc = "Field `CKOUT1DIV` writer - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced"]
pub type Ckout1divW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKOUT1SEL` reader - CKOUT1 Clock Source Selection"]
pub type Ckout1selR = crate::FieldReader;
#[doc = "Field `CKOUT1SEL` writer - CKOUT1 Clock Source Selection"]
pub type Ckout1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&self) -> ScsR {
        ScsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn scss(&self) -> ScssR {
        ScssR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&self) -> AhbpscR {
        AhbpscR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&self) -> Apb1pscR {
        Apb1pscR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&self) -> Apb2pscR {
        Apb2pscR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - RTC clock divider factor"]
    #[inline(always)]
    pub fn rtcdiv(&self) -> RtcdivR {
        RtcdivR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout0sel(&self) -> Ckout0selR {
        Ckout0selR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - I2S Clock Source Selection"]
    #[inline(always)]
    pub fn i2ssel(&self) -> I2sselR {
        I2sselR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced"]
    #[inline(always)]
    pub fn ckout0div(&self) -> Ckout0divR {
        Ckout0divR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced"]
    #[inline(always)]
    pub fn ckout1div(&self) -> Ckout1divR {
        Ckout1divR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - CKOUT1 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout1sel(&self) -> Ckout1selR {
        Ckout1selR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> ScsW<Cfg0Spec> {
        ScsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn ahbpsc(&mut self) -> AhbpscW<Cfg0Spec> {
        AhbpscW::new(self, 4)
    }
    #[doc = "Bits 10:12 - APB1 prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn apb1psc(&mut self) -> Apb1pscW<Cfg0Spec> {
        Apb1pscW::new(self, 10)
    }
    #[doc = "Bits 13:15 - APB2 prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn apb2psc(&mut self) -> Apb2pscW<Cfg0Spec> {
        Apb2pscW::new(self, 13)
    }
    #[doc = "Bits 16:20 - RTC clock divider factor"]
    #[inline(always)]
    #[must_use]
    pub fn rtcdiv(&mut self) -> RtcdivW<Cfg0Spec> {
        RtcdivW::new(self, 16)
    }
    #[doc = "Bits 21:22 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckout0sel(&mut self) -> Ckout0selW<Cfg0Spec> {
        Ckout0selW::new(self, 21)
    }
    #[doc = "Bit 23 - I2S Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2ssel(&mut self) -> I2sselW<Cfg0Spec> {
        I2sselW::new(self, 23)
    }
    #[doc = "Bits 24:26 - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced"]
    #[inline(always)]
    #[must_use]
    pub fn ckout0div(&mut self) -> Ckout0divW<Cfg0Spec> {
        Ckout0divW::new(self, 24)
    }
    #[doc = "Bits 27:29 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1div(&mut self) -> Ckout1divW<Cfg0Spec> {
        Ckout1divW::new(self, 27)
    }
    #[doc = "Bits 30:31 - CKOUT1 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckout1sel(&mut self) -> Ckout1selW<Cfg0Spec> {
        Ckout1selW::new(self, 30)
    }
}
#[doc = "Clock configuration register 0 (RCU_CFG0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {
    const RESET_VALUE: u32 = 0;
}
