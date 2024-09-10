#[doc = "Register `ADDRST` reader"]
pub type R = crate::R<AddrstSpec>;
#[doc = "Register `ADDRST` writer"]
pub type W = crate::W<AddrstSpec>;
#[doc = "I2C2 unit reset\n\nValue on reset: 0"]
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
#[doc = "Field `I2C2RST` reader - I2C2 unit reset"]
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
#[doc = "Field `I2C2RST` writer - I2C2 unit reset"]
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
impl R {
    #[doc = "Bit 0 - I2C2 unit reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C2 unit reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2c2rstW<AddrstSpec> {
        I2c2rstW::new(self, 0)
    }
}
#[doc = "Additional reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrstSpec;
impl crate::RegisterSpec for AddrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrst::R`](R) reader structure"]
impl crate::Readable for AddrstSpec {}
#[doc = "`write(|w| ..)` method takes [`addrst::W`](W) writer structure"]
impl crate::Writable for AddrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRST to value 0"]
impl crate::Resettable for AddrstSpec {
    const RESET_VALUE: u32 = 0;
}
