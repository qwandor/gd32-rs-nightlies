#[doc = "Register `ERR` reader"]
pub type R = crate::R<ErrSpec>;
#[doc = "Register `ERR` writer"]
pub type W = crate::W<ErrSpec>;
#[doc = "Warning error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Werr {
    #[doc = "0: No warning error"]
    NoError = 0,
    #[doc = "1: Warning error"]
    Error = 1,
}
impl From<Werr> for bool {
    #[inline(always)]
    fn from(variant: Werr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WERR` reader - Warning error"]
pub type WerrR = crate::BitReader<Werr>;
impl WerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Werr {
        match self.bits {
            false => Werr::NoError,
            true => Werr::Error,
        }
    }
    #[doc = "No warning error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Werr::NoError
    }
    #[doc = "Warning error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Werr::Error
    }
}
#[doc = "Passive error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perr {
    #[doc = "0: No passive error"]
    NoError = 0,
    #[doc = "1: Passive error"]
    Error = 1,
}
impl From<Perr> for bool {
    #[inline(always)]
    fn from(variant: Perr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERR` reader - Passive error"]
pub type PerrR = crate::BitReader<Perr>;
impl PerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perr {
        match self.bits {
            false => Perr::NoError,
            true => Perr::Error,
        }
    }
    #[doc = "No passive error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Perr::NoError
    }
    #[doc = "Passive error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Perr::Error
    }
}
#[doc = "Bus-off error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boerr {
    #[doc = "0: No bus-off error"]
    NoError = 0,
    #[doc = "1: Bus-off error"]
    Error = 1,
}
impl From<Boerr> for bool {
    #[inline(always)]
    fn from(variant: Boerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOERR` reader - Bus-off error"]
pub type BoerrR = crate::BitReader<Boerr>;
impl BoerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boerr {
        match self.bits {
            false => Boerr::NoError,
            true => Boerr::Error,
        }
    }
    #[doc = "No bus-off error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Boerr::NoError
    }
    #[doc = "Bus-off error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Boerr::Error
    }
}
#[doc = "Error number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Errn {
    #[doc = "0: No Error"]
    NoError = 0,
    #[doc = "1: Stuff Error"]
    Stuff = 1,
    #[doc = "2: Form Error"]
    Form = 2,
    #[doc = "3: Acknowledgment Error"]
    Ack = 3,
    #[doc = "4: Bit recessive Error"]
    BitRecessive = 4,
    #[doc = "5: Bit dominant Error"]
    BitDominant = 5,
    #[doc = "6: CRC Error"]
    Crc = 6,
    #[doc = "7: Set by software"]
    Custom = 7,
}
impl From<Errn> for u8 {
    #[inline(always)]
    fn from(variant: Errn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Errn {
    type Ux = u8;
}
#[doc = "Field `ERRN` reader - Error number"]
pub type ErrnR = crate::FieldReader<Errn>;
impl ErrnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errn {
        match self.bits {
            0 => Errn::NoError,
            1 => Errn::Stuff,
            2 => Errn::Form,
            3 => Errn::Ack,
            4 => Errn::BitRecessive,
            5 => Errn::BitDominant,
            6 => Errn::Crc,
            7 => Errn::Custom,
            _ => unreachable!(),
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Errn::NoError
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == Errn::Stuff
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == Errn::Form
    }
    #[doc = "Acknowledgment Error"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Errn::Ack
    }
    #[doc = "Bit recessive Error"]
    #[inline(always)]
    pub fn is_bit_recessive(&self) -> bool {
        *self == Errn::BitRecessive
    }
    #[doc = "Bit dominant Error"]
    #[inline(always)]
    pub fn is_bit_dominant(&self) -> bool {
        *self == Errn::BitDominant
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == Errn::Crc
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == Errn::Custom
    }
}
#[doc = "Field `ERRN` writer - Error number"]
pub type ErrnW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Errn>;
impl<'a, REG> ErrnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Errn::NoError)
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn stuff(self) -> &'a mut crate::W<REG> {
        self.variant(Errn::Stuff)
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn form(self) -> &'a mut crate::W<REG> {
        self.variant(Errn::Form)
    }
    #[doc = "Acknowledgment Error"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Errn::Ack)
    }
    #[doc = "Bit recessive Error"]
    #[inline(always)]
    pub fn bit_recessive(self) -> &'a mut crate::W<REG> {
        self.variant(Errn::BitRecessive)
    }
    #[doc = "Bit dominant Error"]
    #[inline(always)]
    pub fn bit_dominant(self) -> &'a mut crate::W<REG> {
        self.variant(Errn::BitDominant)
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(Errn::Crc)
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn custom(self) -> &'a mut crate::W<REG> {
        self.variant(Errn::Custom)
    }
}
#[doc = "Field `TECNT` reader - Transmit Error Count"]
pub type TecntR = crate::FieldReader;
#[doc = "Field `RECNT` reader - Receive Error Count"]
pub type RecntR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Warning error"]
    #[inline(always)]
    pub fn werr(&self) -> WerrR {
        WerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Passive error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-off error"]
    #[inline(always)]
    pub fn boerr(&self) -> BoerrR {
        BoerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    pub fn errn(&self) -> ErrnR {
        ErrnR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Error Count"]
    #[inline(always)]
    pub fn tecnt(&self) -> TecntR {
        TecntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Error Count"]
    #[inline(always)]
    pub fn recnt(&self) -> RecntR {
        RecntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    #[must_use]
    pub fn errn(&mut self) -> ErrnW<ErrSpec> {
        ErrnW::new(self, 4)
    }
}
#[doc = "Error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrSpec;
impl crate::RegisterSpec for ErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err::R`](R) reader structure"]
impl crate::Readable for ErrSpec {}
#[doc = "`write(|w| ..)` method takes [`err::W`](W) writer structure"]
impl crate::Writable for ErrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ErrSpec {
    const RESET_VALUE: u32 = 0;
}
