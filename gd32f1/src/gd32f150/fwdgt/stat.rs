#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Free watchdog timer prescaler value update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pud {
    #[doc = "0: The value read from the PSC register is valid"]
    Valid = 0,
    #[doc = "1: A write operation to to the PSC register is ongoing, so the value read is invalid"]
    Ongoing = 1,
}
impl From<Pud> for bool {
    #[inline(always)]
    fn from(variant: Pud) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUD` reader - Free watchdog timer prescaler value update"]
pub type PudR = crate::BitReader<Pud>;
impl PudR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pud {
        match self.bits {
            false => Pud::Valid,
            true => Pud::Ongoing,
        }
    }
    #[doc = "The value read from the PSC register is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Pud::Valid
    }
    #[doc = "A write operation to to the PSC register is ongoing, so the value read is invalid"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == Pud::Ongoing
    }
}
#[doc = "Free watchdog timer counter reload value update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rud {
    #[doc = "0: The value read from the RLD register is valid"]
    Valid = 0,
    #[doc = "1: A write operation to to the RLD register is ongoing, so the value read is invalid"]
    Ongoing = 1,
}
impl From<Rud> for bool {
    #[inline(always)]
    fn from(variant: Rud) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUD` reader - Free watchdog timer counter reload value update"]
pub type RudR = crate::BitReader<Rud>;
impl RudR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rud {
        match self.bits {
            false => Rud::Valid,
            true => Rud::Ongoing,
        }
    }
    #[doc = "The value read from the RLD register is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Rud::Valid
    }
    #[doc = "A write operation to to the RLD register is ongoing, so the value read is invalid"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == Rud::Ongoing
    }
}
#[doc = "Watchdog counter window value update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wud {
    #[doc = "0: The value read from the WND register is valid"]
    Valid = 0,
    #[doc = "1: A write operation to to the WND register is ongoing, so the value read is invalid"]
    Ongoing = 1,
}
impl From<Wud> for bool {
    #[inline(always)]
    fn from(variant: Wud) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUD` reader - Watchdog counter window value update"]
pub type WudR = crate::BitReader<Wud>;
impl WudR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wud {
        match self.bits {
            false => Wud::Valid,
            true => Wud::Ongoing,
        }
    }
    #[doc = "The value read from the WND register is valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Wud::Valid
    }
    #[doc = "A write operation to to the WND register is ongoing, so the value read is invalid"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == Wud::Ongoing
    }
}
impl R {
    #[doc = "Bit 0 - Free watchdog timer prescaler value update"]
    #[inline(always)]
    pub fn pud(&self) -> PudR {
        PudR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free watchdog timer counter reload value update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog counter window value update"]
    #[inline(always)]
    pub fn wud(&self) -> WudR {
        WudR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
