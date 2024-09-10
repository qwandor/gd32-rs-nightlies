#[doc = "Register `IRMP` reader"]
pub type R = crate::R<IrmpSpec>;
#[doc = "Register `IRMP` writer"]
pub type W = crate::W<IrmpSpec>;
#[doc = "Timer input 0 remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ci0Rmp {
    #[doc = "0: Channel 0 input is connected to GPIO"]
    Gpio = 0,
    #[doc = "1: Channel 0 input is connected to RTCCLK"]
    Rtcclk = 1,
    #[doc = "2: Channel 0 input is connected to HXTAL / 32"]
    Hxtal32 = 2,
    #[doc = "3: Channel 0 input is connected to CKOUTSEL"]
    Ckoutsel = 3,
}
impl From<Ci0Rmp> for u8 {
    #[inline(always)]
    fn from(variant: Ci0Rmp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ci0Rmp {
    type Ux = u8;
}
#[doc = "Field `CI0_RMP` reader - Timer input 0 remap"]
pub type Ci0RmpR = crate::FieldReader<Ci0Rmp>;
impl Ci0RmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ci0Rmp {
        match self.bits {
            0 => Ci0Rmp::Gpio,
            1 => Ci0Rmp::Rtcclk,
            2 => Ci0Rmp::Hxtal32,
            3 => Ci0Rmp::Ckoutsel,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 input is connected to GPIO"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Ci0Rmp::Gpio
    }
    #[doc = "Channel 0 input is connected to RTCCLK"]
    #[inline(always)]
    pub fn is_rtcclk(&self) -> bool {
        *self == Ci0Rmp::Rtcclk
    }
    #[doc = "Channel 0 input is connected to HXTAL / 32"]
    #[inline(always)]
    pub fn is_hxtal_32(&self) -> bool {
        *self == Ci0Rmp::Hxtal32
    }
    #[doc = "Channel 0 input is connected to CKOUTSEL"]
    #[inline(always)]
    pub fn is_ckoutsel(&self) -> bool {
        *self == Ci0Rmp::Ckoutsel
    }
}
#[doc = "Field `CI0_RMP` writer - Timer input 0 remap"]
pub type Ci0RmpW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ci0Rmp>;
impl<'a, REG> Ci0RmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 input is connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Ci0Rmp::Gpio)
    }
    #[doc = "Channel 0 input is connected to RTCCLK"]
    #[inline(always)]
    pub fn rtcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ci0Rmp::Rtcclk)
    }
    #[doc = "Channel 0 input is connected to HXTAL / 32"]
    #[inline(always)]
    pub fn hxtal_32(self) -> &'a mut crate::W<REG> {
        self.variant(Ci0Rmp::Hxtal32)
    }
    #[doc = "Channel 0 input is connected to CKOUTSEL"]
    #[inline(always)]
    pub fn ckoutsel(self) -> &'a mut crate::W<REG> {
        self.variant(Ci0Rmp::Ckoutsel)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer input 0 remap"]
    #[inline(always)]
    pub fn ci0_rmp(&self) -> Ci0RmpR {
        Ci0RmpR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer input 0 remap"]
    #[inline(always)]
    #[must_use]
    pub fn ci0_rmp(&mut self) -> Ci0RmpW<IrmpSpec> {
        Ci0RmpW::new(self, 0)
    }
}
#[doc = "channel input remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrmpSpec;
impl crate::RegisterSpec for IrmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irmp::R`](R) reader structure"]
impl crate::Readable for IrmpSpec {}
#[doc = "`write(|w| ..)` method takes [`irmp::W`](W) writer structure"]
impl crate::Writable for IrmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRMP to value 0"]
impl crate::Resettable for IrmpSpec {
    const RESET_VALUE: u32 = 0;
}
