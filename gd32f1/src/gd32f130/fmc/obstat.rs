#[doc = "Register `OBSTAT` reader"]
pub type R = crate::R<ObstatSpec>;
#[doc = "Option byte read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oberr {
    #[doc = "0: No error with option bytes"]
    NoError = 0,
    #[doc = "1: Option bytes and complement bytes do not match"]
    Error = 1,
}
impl From<Oberr> for bool {
    #[inline(always)]
    fn from(variant: Oberr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBERR` reader - Option byte read error"]
pub type OberrR = crate::BitReader<Oberr>;
impl OberrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oberr {
        match self.bits {
            false => Oberr::NoError,
            true => Oberr::Error,
        }
    }
    #[doc = "No error with option bytes"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Oberr::NoError
    }
    #[doc = "Option bytes and complement bytes do not match"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Oberr::Error
    }
}
#[doc = "Security Protection level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plevel {
    #[doc = "0: No protection level"]
    None = 0,
    #[doc = "1: Low protection level"]
    Low = 1,
    #[doc = "3: High protection level"]
    High = 3,
}
impl From<Plevel> for u8 {
    #[inline(always)]
    fn from(variant: Plevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Plevel {
    type Ux = u8;
}
#[doc = "Field `PLEVEL` reader - Security Protection level"]
pub type PlevelR = crate::FieldReader<Plevel>;
impl PlevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Plevel> {
        match self.bits {
            0 => Some(Plevel::None),
            1 => Some(Plevel::Low),
            3 => Some(Plevel::High),
            _ => None,
        }
    }
    #[doc = "No protection level"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Plevel::None
    }
    #[doc = "Low protection level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Plevel::Low
    }
    #[doc = "High protection level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Plevel::High
    }
}
#[doc = "Field `OB_USER` reader - Store OB_USER byte of option byte block after system reset"]
pub type ObUserR = crate::FieldReader;
#[doc = "Field `OB_DATA` reader - Store OB_DATA\\[15:0\\]
of option byte block after system reset"]
pub type ObDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Option byte read error"]
    #[inline(always)]
    pub fn oberr(&self) -> OberrR {
        OberrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Security Protection level"]
    #[inline(always)]
    pub fn plevel(&self) -> PlevelR {
        PlevelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Store OB_USER byte of option byte block after system reset"]
    #[inline(always)]
    pub fn ob_user(&self) -> ObUserR {
        ObUserR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Store OB_DATA\\[15:0\\]
of option byte block after system reset"]
    #[inline(always)]
    pub fn ob_data(&self) -> ObDataR {
        ObDataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Option byte status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ObstatSpec;
impl crate::RegisterSpec for ObstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obstat::R`](R) reader structure"]
impl crate::Readable for ObstatSpec {}
#[doc = "`reset()` method sets OBSTAT to value 0"]
impl crate::Resettable for ObstatSpec {
    const RESET_VALUE: u32 = 0;
}
