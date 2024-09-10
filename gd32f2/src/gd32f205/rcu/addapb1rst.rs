#[doc = "Register `ADDAPB1RST` reader"]
pub type R = crate::R<Addapb1rstSpec>;
#[doc = "Register `ADDAPB1RST` writer"]
pub type W = crate::W<Addapb1rstSpec>;
#[doc = "I2C2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2rst {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<I2c2rst> for bool {
    #[inline(always)]
    fn from(variant: I2c2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2c2rstR = crate::BitReader<I2c2rst>;
impl I2c2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2c2rst> {
        match self.bits {
            true => Some(I2c2rst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2c2rst::Reset
    }
}
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG, I2c2rst>;
impl<'a, REG> I2c2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2rst::Reset)
    }
}
#[doc = "Field `UART6RST` reader - UART6 reset"]
pub use I2c2rstR as Uart6rstR;
#[doc = "Field `UART7RST` reader - UART7 reset"]
pub use I2c2rstR as Uart7rstR;
#[doc = "Field `UART6RST` writer - UART6 reset"]
pub use I2c2rstW as Uart6rstW;
#[doc = "Field `UART7RST` writer - UART7 reset"]
pub use I2c2rstW as Uart7rstW;
impl R {
    #[doc = "Bit 23 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - UART6 reset"]
    #[inline(always)]
    pub fn uart6rst(&self) -> Uart6rstR {
        Uart6rstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART7 reset"]
    #[inline(always)]
    pub fn uart7rst(&self) -> Uart7rstR {
        Uart7rstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2c2rstW<Addapb1rstSpec> {
        I2c2rstW::new(self, 23)
    }
    #[doc = "Bit 30 - UART6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart6rst(&mut self) -> Uart6rstW<Addapb1rstSpec> {
        Uart6rstW::new(self, 30)
    }
    #[doc = "Bit 31 - UART7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart7rst(&mut self) -> Uart7rstW<Addapb1rstSpec> {
        Uart7rstW::new(self, 31)
    }
}
#[doc = "APB1 additional enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addapb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addapb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addapb1rstSpec;
impl crate::RegisterSpec for Addapb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1rst::R`](R) reader structure"]
impl crate::Readable for Addapb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`addapb1rst::W`](W) writer structure"]
impl crate::Writable for Addapb1rstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDAPB1RST to value 0"]
impl crate::Resettable for Addapb1rstSpec {
    const RESET_VALUE: u32 = 0;
}
