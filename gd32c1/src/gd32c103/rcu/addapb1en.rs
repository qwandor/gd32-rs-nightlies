#[doc = "Register `ADDAPB1EN` reader"]
pub type R = crate::R<Addapb1enSpec>;
#[doc = "Register `ADDAPB1EN` writer"]
pub type W = crate::W<Addapb1enSpec>;
#[doc = "CTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctcen {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<Ctcen> for bool {
    #[inline(always)]
    fn from(variant: Ctcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCEN` reader - CTC clock enable"]
pub type CtcenR = crate::BitReader<Ctcen>;
impl CtcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctcen {
        match self.bits {
            false => Ctcen::Disabled,
            true => Ctcen::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctcen::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctcen::Enabled
    }
}
#[doc = "Field `CTCEN` writer - CTC clock enable"]
pub type CtcenW<'a, REG> = crate::BitWriter<'a, REG, Ctcen>;
impl<'a, REG> CtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctcen::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctcen::Enabled)
    }
}
impl R {
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    pub fn ctcen(&self) -> CtcenR {
        CtcenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctcen(&mut self) -> CtcenW<Addapb1enSpec> {
        CtcenW::new(self, 27)
    }
}
#[doc = "APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addapb1enSpec;
impl crate::RegisterSpec for Addapb1enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1en::R`](R) reader structure"]
impl crate::Readable for Addapb1enSpec {}
#[doc = "`write(|w| ..)` method takes [`addapb1en::W`](W) writer structure"]
impl crate::Writable for Addapb1enSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDAPB1EN to value 0"]
impl crate::Resettable for Addapb1enSpec {
    const RESET_VALUE: u32 = 0;
}
