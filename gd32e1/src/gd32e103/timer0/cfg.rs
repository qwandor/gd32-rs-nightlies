#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "The output value selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outsel {
    #[doc = "0: Normal behaviour"]
    Normal = 0,
    #[doc = "1: If POEN and IOS is 0 the output is disabled"]
    Disabled = 1,
}
impl From<Outsel> for bool {
    #[inline(always)]
    fn from(variant: Outsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTSEL` reader - The output value selection"]
pub type OutselR = crate::BitReader<Outsel>;
impl OutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outsel {
        match self.bits {
            false => Outsel::Normal,
            true => Outsel::Disabled,
        }
    }
    #[doc = "Normal behaviour"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Outsel::Normal
    }
    #[doc = "If POEN and IOS is 0 the output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outsel::Disabled
    }
}
#[doc = "Field `OUTSEL` writer - The output value selection"]
pub type OutselW<'a, REG> = crate::BitWriter<'a, REG, Outsel>;
impl<'a, REG> OutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal behaviour"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Outsel::Normal)
    }
    #[doc = "If POEN and IOS is 0 the output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outsel::Disabled)
    }
}
#[doc = "Write CHxVAL register selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chvsel {
    #[doc = "0: Normal behaviour"]
    Normal = 0,
    #[doc = "1: Duplicate writes to CHxVAL are ignored"]
    IgnoreSame = 1,
}
impl From<Chvsel> for bool {
    #[inline(always)]
    fn from(variant: Chvsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHVSEL` reader - Write CHxVAL register selection"]
pub type ChvselR = crate::BitReader<Chvsel>;
impl ChvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chvsel {
        match self.bits {
            false => Chvsel::Normal,
            true => Chvsel::IgnoreSame,
        }
    }
    #[doc = "Normal behaviour"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Chvsel::Normal
    }
    #[doc = "Duplicate writes to CHxVAL are ignored"]
    #[inline(always)]
    pub fn is_ignore_same(&self) -> bool {
        *self == Chvsel::IgnoreSame
    }
}
#[doc = "Field `CHVSEL` writer - Write CHxVAL register selection"]
pub type ChvselW<'a, REG> = crate::BitWriter<'a, REG, Chvsel>;
impl<'a, REG> ChvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal behaviour"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Chvsel::Normal)
    }
    #[doc = "Duplicate writes to CHxVAL are ignored"]
    #[inline(always)]
    pub fn ignore_same(self) -> &'a mut crate::W<REG> {
        self.variant(Chvsel::IgnoreSame)
    }
}
impl R {
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OutselR {
        OutselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    pub fn chvsel(&self) -> ChvselR {
        ChvselR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The output value selection"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OutselW<CfgSpec> {
        OutselW::new(self, 0)
    }
    #[doc = "Bit 1 - Write CHxVAL register selection"]
    #[inline(always)]
    #[must_use]
    pub fn chvsel(&mut self) -> ChvselW<CfgSpec> {
        ChvselW::new(self, 1)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
