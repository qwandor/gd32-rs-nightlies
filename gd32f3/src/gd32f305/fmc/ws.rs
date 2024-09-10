#[doc = "Register `WS` reader"]
pub type R = crate::R<WsSpec>;
#[doc = "Register `WS` writer"]
pub type W = crate::W<WsSpec>;
#[doc = "wait state counter register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wscnt {
    #[doc = "0: 0 wait states added"]
    Ws0 = 0,
    #[doc = "1: 1 wait state added"]
    Ws1 = 1,
    #[doc = "2: 2 wait states added"]
    Ws2 = 2,
}
impl From<Wscnt> for u8 {
    #[inline(always)]
    fn from(variant: Wscnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wscnt {
    type Ux = u8;
}
#[doc = "Field `WSCNT` reader - wait state counter register"]
pub type WscntR = crate::FieldReader<Wscnt>;
impl WscntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wscnt> {
        match self.bits {
            0 => Some(Wscnt::Ws0),
            1 => Some(Wscnt::Ws1),
            2 => Some(Wscnt::Ws2),
            _ => None,
        }
    }
    #[doc = "0 wait states added"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == Wscnt::Ws0
    }
    #[doc = "1 wait state added"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == Wscnt::Ws1
    }
    #[doc = "2 wait states added"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == Wscnt::Ws2
    }
}
#[doc = "Field `WSCNT` writer - wait state counter register"]
pub type WscntW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wscnt>;
impl<'a, REG> WscntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 wait states added"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(Wscnt::Ws0)
    }
    #[doc = "1 wait state added"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(Wscnt::Ws1)
    }
    #[doc = "2 wait states added"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(Wscnt::Ws2)
    }
}
impl R {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WscntR {
        WscntR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    #[must_use]
    pub fn wscnt(&mut self) -> WscntW<WsSpec> {
        WscntW::new(self, 0)
    }
}
#[doc = "wait state counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ws::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ws::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsSpec;
impl crate::RegisterSpec for WsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ws::R`](R) reader structure"]
impl crate::Readable for WsSpec {}
#[doc = "`write(|w| ..)` method takes [`ws::W`](W) writer structure"]
impl crate::Writable for WsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WS to value 0"]
impl crate::Resettable for WsSpec {
    const RESET_VALUE: u32 = 0;
}
