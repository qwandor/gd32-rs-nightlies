#[doc = "Register `QCTL` reader"]
pub type R = crate::R<QctlSpec>;
#[doc = "Register `QCTL` writer"]
pub type W = crate::W<QctlSpec>;
#[doc = "Quad-SPI mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qmod {
    #[doc = "0: Single wire mode"]
    Single = 0,
    #[doc = "1: Quad-SPI mode"]
    Quad = 1,
}
impl From<Qmod> for bool {
    #[inline(always)]
    fn from(variant: Qmod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QMOD` reader - Quad-SPI mode enable"]
pub type QmodR = crate::BitReader<Qmod>;
impl QmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qmod {
        match self.bits {
            false => Qmod::Single,
            true => Qmod::Quad,
        }
    }
    #[doc = "Single wire mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Qmod::Single
    }
    #[doc = "Quad-SPI mode"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == Qmod::Quad
    }
}
#[doc = "Field `QMOD` writer - Quad-SPI mode enable"]
pub type QmodW<'a, REG> = crate::BitWriter<'a, REG, Qmod>;
impl<'a, REG> QmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single wire mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Qmod::Single)
    }
    #[doc = "Quad-SPI mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(Qmod::Quad)
    }
}
#[doc = "Quad-SPI mode read select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qrd {
    #[doc = "0: Quad wire write mode"]
    Write = 0,
    #[doc = "1: Quad wire read mode"]
    Read = 1,
}
impl From<Qrd> for bool {
    #[inline(always)]
    fn from(variant: Qrd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QRD` reader - Quad-SPI mode read select"]
pub type QrdR = crate::BitReader<Qrd>;
impl QrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qrd {
        match self.bits {
            false => Qrd::Write,
            true => Qrd::Read,
        }
    }
    #[doc = "Quad wire write mode"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Qrd::Write
    }
    #[doc = "Quad wire read mode"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Qrd::Read
    }
}
#[doc = "Field `QRD` writer - Quad-SPI mode read select"]
pub type QrdW<'a, REG> = crate::BitWriter<'a, REG, Qrd>;
impl<'a, REG> QrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quad wire write mode"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Qrd::Write)
    }
    #[doc = "Quad wire read mode"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Qrd::Read)
    }
}
#[doc = "Drive IO2 and IO3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Io23Drv {
    #[doc = "0: IO2 and IO3 are not driven in single wire mode"]
    Disabled = 0,
    #[doc = "1: IO2 and IO3 are driven high in single wire mode"]
    High = 1,
}
impl From<Io23Drv> for bool {
    #[inline(always)]
    fn from(variant: Io23Drv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO23_DRV` reader - Drive IO2 and IO3 enable"]
pub type Io23DrvR = crate::BitReader<Io23Drv>;
impl Io23DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io23Drv {
        match self.bits {
            false => Io23Drv::Disabled,
            true => Io23Drv::High,
        }
    }
    #[doc = "IO2 and IO3 are not driven in single wire mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Io23Drv::Disabled
    }
    #[doc = "IO2 and IO3 are driven high in single wire mode"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Io23Drv::High
    }
}
#[doc = "Field `IO23_DRV` writer - Drive IO2 and IO3 enable"]
pub type Io23DrvW<'a, REG> = crate::BitWriter<'a, REG, Io23Drv>;
impl<'a, REG> Io23DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO2 and IO3 are not driven in single wire mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Io23Drv::Disabled)
    }
    #[doc = "IO2 and IO3 are driven high in single wire mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Io23Drv::High)
    }
}
impl R {
    #[doc = "Bit 0 - Quad-SPI mode enable"]
    #[inline(always)]
    pub fn qmod(&self) -> QmodR {
        QmodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Quad-SPI mode read select"]
    #[inline(always)]
    pub fn qrd(&self) -> QrdR {
        QrdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive IO2 and IO3 enable"]
    #[inline(always)]
    pub fn io23_drv(&self) -> Io23DrvR {
        Io23DrvR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quad-SPI mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn qmod(&mut self) -> QmodW<QctlSpec> {
        QmodW::new(self, 0)
    }
    #[doc = "Bit 1 - Quad-SPI mode read select"]
    #[inline(always)]
    #[must_use]
    pub fn qrd(&mut self) -> QrdW<QctlSpec> {
        QrdW::new(self, 1)
    }
    #[doc = "Bit 2 - Drive IO2 and IO3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn io23_drv(&mut self) -> Io23DrvW<QctlSpec> {
        Io23DrvW::new(self, 2)
    }
}
#[doc = "Quad-SPI mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QctlSpec;
impl crate::RegisterSpec for QctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qctl::R`](R) reader structure"]
impl crate::Readable for QctlSpec {}
#[doc = "`write(|w| ..)` method takes [`qctl::W`](W) writer structure"]
impl crate::Writable for QctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCTL to value 0"]
impl crate::Resettable for QctlSpec {
    const RESET_VALUE: u32 = 0;
}
