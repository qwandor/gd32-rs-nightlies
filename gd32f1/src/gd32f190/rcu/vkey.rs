#[doc = "Register `VKEY` reader"]
pub type R = crate::R<VkeySpec>;
#[doc = "Register `VKEY` writer"]
pub type W = crate::W<VkeySpec>;
#[doc = "The key of RCU_PDVSEL and RCU_DSV register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Keyw {
    #[doc = "439041101: Allow PDVSEL and DSV to be written"]
    Enable = 439041101,
}
impl From<Keyw> for u32 {
    #[inline(always)]
    fn from(variant: Keyw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyw {
    type Ux = u32;
}
#[doc = "Field `KEY` reader - The key of RCU_PDVSEL and RCU_DSV register"]
pub type KeyR = crate::FieldReader<Keyw>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyw> {
        match self.bits {
            439041101 => Some(Keyw::Enable),
            _ => None,
        }
    }
    #[doc = "Allow PDVSEL and DSV to be written"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Keyw::Enable
    }
}
#[doc = "Field `KEY` writer - The key of RCU_PDVSEL and RCU_DSV register"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, Keyw>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Allow PDVSEL and DSV to be written"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Keyw::Enable)
    }
}
impl R {
    #[doc = "Bits 0:31 - The key of RCU_PDVSEL and RCU_DSV register"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The key of RCU_PDVSEL and RCU_DSV register"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<VkeySpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Voltage key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vkey::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vkey::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VkeySpec;
impl crate::RegisterSpec for VkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vkey::R`](R) reader structure"]
impl crate::Readable for VkeySpec {}
#[doc = "`write(|w| ..)` method takes [`vkey::W`](W) writer structure"]
impl crate::Writable for VkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VKEY to value 0"]
impl crate::Resettable for VkeySpec {
    const RESET_VALUE: u32 = 0;
}
