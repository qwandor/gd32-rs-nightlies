#[doc = "Register `SADDR1` reader"]
pub type R = crate::R<Saddr1Spec>;
#[doc = "Register `SADDR1` writer"]
pub type W = crate::W<Saddr1Spec>;
#[doc = "Dual addressing mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duaden {
    #[doc = "0: Single addressing mode"]
    Single = 0,
    #[doc = "1: Dual addressing mode"]
    Dual = 1,
}
impl From<Duaden> for bool {
    #[inline(always)]
    fn from(variant: Duaden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUADEN` reader - Dual addressing mode enable"]
pub type DuadenR = crate::BitReader<Duaden>;
impl DuadenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Duaden {
        match self.bits {
            false => Duaden::Single,
            true => Duaden::Dual,
        }
    }
    #[doc = "Single addressing mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Duaden::Single
    }
    #[doc = "Dual addressing mode"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == Duaden::Dual
    }
}
#[doc = "Field `DUADEN` writer - Dual addressing mode enable"]
pub type DuadenW<'a, REG> = crate::BitWriter<'a, REG, Duaden>;
impl<'a, REG> DuadenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single addressing mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Duaden::Single)
    }
    #[doc = "Dual addressing mode"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(Duaden::Dual)
    }
}
#[doc = "Field `ADDRESS2` reader - Interface address"]
pub type Address2R = crate::FieldReader;
#[doc = "Field `ADDRESS2` writer - Interface address"]
pub type Address2W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn duaden(&self) -> DuadenR {
        DuadenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn address2(&self) -> Address2R {
        Address2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn duaden(&mut self) -> DuadenW<Saddr1Spec> {
        DuadenW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn address2(&mut self) -> Address2W<Saddr1Spec> {
        Address2W::new(self, 1)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saddr1Spec;
impl crate::RegisterSpec for Saddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr1::R`](R) reader structure"]
impl crate::Readable for Saddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`saddr1::W`](W) writer structure"]
impl crate::Writable for Saddr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SADDR1 to value 0"]
impl crate::Resettable for Saddr1Spec {
    const RESET_VALUE: u32 = 0;
}
