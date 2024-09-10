#[doc = "Register `ADDCTL` reader"]
pub type R = crate::R<AddctlSpec>;
#[doc = "Register `ADDCTL` writer"]
pub type W = crate::W<AddctlSpec>;
#[doc = "Field `CK48MSEL` reader - 48MHz clock selection"]
pub type Ck48mselR = crate::BitReader;
#[doc = "Field `CK48MSEL` writer - 48MHz clock selection"]
pub type Ck48mselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL48MSEL` reader - PLL48M clock selection"]
pub type Pll48mselR = crate::BitReader;
#[doc = "Field `PLL48MSEL` writer - PLL48M clock selection"]
pub type Pll48mselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC48MEN` reader - Internal 48MHz RC oscillator enable"]
pub type Irc48menR = crate::BitReader;
#[doc = "Field `IRC48MEN` writer - Internal 48MHz RC oscillator enable"]
pub type Irc48menW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC48MSTB` reader - Internal 48MHz RC oscillator clock stabilization Flag"]
pub type Irc48mstbR = crate::BitReader;
#[doc = "Field `IRC48MCAL` reader - Internal 48MHz RC oscillator calibration value register"]
pub type Irc48mcalR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 48MHz clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> Ck48mselR {
        Ck48mselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL48M clock selection"]
    #[inline(always)]
    pub fn pll48msel(&self) -> Pll48mselR {
        Pll48mselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&self) -> Irc48menR {
        Irc48menR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal 48MHz RC oscillator clock stabilization Flag"]
    #[inline(always)]
    pub fn irc48mstb(&self) -> Irc48mstbR {
        Irc48mstbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Internal 48MHz RC oscillator calibration value register"]
    #[inline(always)]
    pub fn irc48mcal(&self) -> Irc48mcalR {
        Irc48mcalR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 48MHz clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn ck48msel(&mut self) -> Ck48mselW<AddctlSpec> {
        Ck48mselW::new(self, 0)
    }
    #[doc = "Bit 1 - PLL48M clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn pll48msel(&mut self) -> Pll48mselW<AddctlSpec> {
        Pll48mselW::new(self, 1)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc48men(&mut self) -> Irc48menW<AddctlSpec> {
        Irc48menW::new(self, 16)
    }
}
#[doc = "Additional clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddctlSpec;
impl crate::RegisterSpec for AddctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addctl::R`](R) reader structure"]
impl crate::Readable for AddctlSpec {}
#[doc = "`write(|w| ..)` method takes [`addctl::W`](W) writer structure"]
impl crate::Writable for AddctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDCTL to value 0"]
impl crate::Resettable for AddctlSpec {
    const RESET_VALUE: u32 = 0;
}
