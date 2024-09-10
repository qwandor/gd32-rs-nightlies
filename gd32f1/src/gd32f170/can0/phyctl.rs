#[doc = "Register `PHYCTL` reader"]
pub type R = crate::R<PhyctlSpec>;
#[doc = "PHY enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phyen {
    #[doc = "0: CAN PHY disabled"]
    Disabled = 0,
    #[doc = "1: CAN PHY enabled"]
    Enabled = 1,
}
impl From<Phyen> for bool {
    #[inline(always)]
    fn from(variant: Phyen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYEN` reader - PHY enable bit"]
pub type PhyenR = crate::BitReader<Phyen>;
impl PhyenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Phyen {
        match self.bits {
            false => Phyen::Disabled,
            true => Phyen::Enabled,
        }
    }
    #[doc = "CAN PHY disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Phyen::Disabled
    }
    #[doc = "CAN PHY enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Phyen::Enabled
    }
}
#[doc = "CAN PHY output driver control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pomod {
    #[doc = "0: Low slope mode"]
    LowSlope = 0,
    #[doc = "1: Middle slope mode"]
    MiddleSlope = 1,
    #[doc = "2: High slope mode"]
    HighSlope = 2,
    #[doc = "3: High speed mode"]
    HighSpeed = 3,
}
impl From<Pomod> for u8 {
    #[inline(always)]
    fn from(variant: Pomod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pomod {
    type Ux = u8;
}
#[doc = "Field `POMOD` reader - CAN PHY output driver control"]
pub type PomodR = crate::FieldReader<Pomod>;
impl PomodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pomod {
        match self.bits {
            0 => Pomod::LowSlope,
            1 => Pomod::MiddleSlope,
            2 => Pomod::HighSlope,
            3 => Pomod::HighSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "Low slope mode"]
    #[inline(always)]
    pub fn is_low_slope(&self) -> bool {
        *self == Pomod::LowSlope
    }
    #[doc = "Middle slope mode"]
    #[inline(always)]
    pub fn is_middle_slope(&self) -> bool {
        *self == Pomod::MiddleSlope
    }
    #[doc = "High slope mode"]
    #[inline(always)]
    pub fn is_high_slope(&self) -> bool {
        *self == Pomod::HighSlope
    }
    #[doc = "High speed mode"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == Pomod::HighSpeed
    }
}
impl R {
    #[doc = "Bit 0 - PHY enable bit"]
    #[inline(always)]
    pub fn phyen(&self) -> PhyenR {
        PhyenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - CAN PHY output driver control"]
    #[inline(always)]
    pub fn pomod(&self) -> PomodR {
        PomodR::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phyctl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyctlSpec;
impl crate::RegisterSpec for PhyctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phyctl::R`](R) reader structure"]
impl crate::Readable for PhyctlSpec {}
#[doc = "`reset()` method sets PHYCTL to value 0x0300"]
impl crate::Resettable for PhyctlSpec {
    const RESET_VALUE: u32 = 0x0300;
}
