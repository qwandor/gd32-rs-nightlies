#[doc = "Register `FMPCFG` reader"]
pub type R = crate::R<FmpcfgSpec>;
#[doc = "Register `FMPCFG` writer"]
pub type W = crate::W<FmpcfgSpec>;
#[doc = "Fast-mode-plus enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmpen {
    #[doc = "0: Fast mode plus disabled"]
    Disabled = 0,
    #[doc = "1: Fast mode plus (1MHz max) enabled"]
    Enabled = 1,
}
impl From<Fmpen> for bool {
    #[inline(always)]
    fn from(variant: Fmpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMPEN` reader - Fast-mode-plus enable"]
pub type FmpenR = crate::BitReader<Fmpen>;
impl FmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmpen {
        match self.bits {
            false => Fmpen::Disabled,
            true => Fmpen::Enabled,
        }
    }
    #[doc = "Fast mode plus disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fmpen::Disabled
    }
    #[doc = "Fast mode plus (1MHz max) enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fmpen::Enabled
    }
}
#[doc = "Field `FMPEN` writer - Fast-mode-plus enable"]
pub type FmpenW<'a, REG> = crate::BitWriter<'a, REG, Fmpen>;
impl<'a, REG> FmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast mode plus disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fmpen::Disabled)
    }
    #[doc = "Fast mode plus (1MHz max) enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fmpen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Fast-mode-plus enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FmpenR {
        FmpenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast-mode-plus enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpen(&mut self) -> FmpenW<FmpcfgSpec> {
        FmpenW::new(self, 0)
    }
}
#[doc = "Fast mode plus configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmpcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmpcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmpcfgSpec;
impl crate::RegisterSpec for FmpcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmpcfg::R`](R) reader structure"]
impl crate::Readable for FmpcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fmpcfg::W`](W) writer structure"]
impl crate::Writable for FmpcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMPCFG to value 0"]
impl crate::Resettable for FmpcfgSpec {
    const RESET_VALUE: u32 = 0;
}
