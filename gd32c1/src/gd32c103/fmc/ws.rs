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
#[doc = "Pre-fetch enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfen {
    #[doc = "0: Pre-fetch disabled"]
    Disable = 0,
    #[doc = "1: Pre-fetch enabled"]
    Enable = 1,
}
impl From<Pfen> for bool {
    #[inline(always)]
    fn from(variant: Pfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFEN` reader - Pre-fetch enable"]
pub type PfenR = crate::BitReader<Pfen>;
impl PfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfen {
        match self.bits {
            false => Pfen::Disable,
            true => Pfen::Enable,
        }
    }
    #[doc = "Pre-fetch disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pfen::Disable
    }
    #[doc = "Pre-fetch enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pfen::Enable
    }
}
#[doc = "Field `PFEN` writer - Pre-fetch enable"]
pub type PfenW<'a, REG> = crate::BitWriter<'a, REG, Pfen>;
impl<'a, REG> PfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pre-fetch disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pfen::Disable)
    }
    #[doc = "Pre-fetch enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pfen::Enable)
    }
}
#[doc = "IBUS cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icen {
    #[doc = "0: IBUS cache disabled"]
    Disable = 0,
    #[doc = "1: IBUS cache enabled"]
    Enable = 1,
}
impl From<Icen> for bool {
    #[inline(always)]
    fn from(variant: Icen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEN` reader - IBUS cache enable"]
pub type IcenR = crate::BitReader<Icen>;
impl IcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icen {
        match self.bits {
            false => Icen::Disable,
            true => Icen::Enable,
        }
    }
    #[doc = "IBUS cache disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Icen::Disable
    }
    #[doc = "IBUS cache enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Icen::Enable
    }
}
#[doc = "Field `ICEN` writer - IBUS cache enable"]
pub type IcenW<'a, REG> = crate::BitWriter<'a, REG, Icen>;
impl<'a, REG> IcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IBUS cache disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Icen::Disable)
    }
    #[doc = "IBUS cache enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Icen::Enable)
    }
}
#[doc = "DBUS cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcen {
    #[doc = "0: DBUS cache disabled"]
    Disable = 0,
    #[doc = "1: DBUS cache enabled"]
    Enable = 1,
}
impl From<Dcen> for bool {
    #[inline(always)]
    fn from(variant: Dcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN` reader - DBUS cache enable"]
pub type DcenR = crate::BitReader<Dcen>;
impl DcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcen {
        match self.bits {
            false => Dcen::Disable,
            true => Dcen::Enable,
        }
    }
    #[doc = "DBUS cache disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dcen::Disable
    }
    #[doc = "DBUS cache enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dcen::Enable
    }
}
#[doc = "Field `DCEN` writer - DBUS cache enable"]
pub type DcenW<'a, REG> = crate::BitWriter<'a, REG, Dcen>;
impl<'a, REG> DcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DBUS cache disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen::Disable)
    }
    #[doc = "DBUS cache enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dcen::Enable)
    }
}
#[doc = "IBUS cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icrst {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: IBUS cache reset"]
    Reset = 1,
}
impl From<Icrst> for bool {
    #[inline(always)]
    fn from(variant: Icrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICRST` reader - IBUS cache reset"]
pub type IcrstR = crate::BitReader<Icrst>;
impl IcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icrst {
        match self.bits {
            false => Icrst::NoEffect,
            true => Icrst::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Icrst::NoEffect
    }
    #[doc = "IBUS cache reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Icrst::Reset
    }
}
#[doc = "Field `ICRST` writer - IBUS cache reset"]
pub type IcrstW<'a, REG> = crate::BitWriter<'a, REG, Icrst>;
impl<'a, REG> IcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Icrst::NoEffect)
    }
    #[doc = "IBUS cache reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Icrst::Reset)
    }
}
#[doc = "DBUS cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcrst {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: DBUS cache reset"]
    Reset = 1,
}
impl From<Dcrst> for bool {
    #[inline(always)]
    fn from(variant: Dcrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCRST` reader - DBUS cache reset"]
pub type DcrstR = crate::BitReader<Dcrst>;
impl DcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcrst {
        match self.bits {
            false => Dcrst::NoEffect,
            true => Dcrst::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Dcrst::NoEffect
    }
    #[doc = "DBUS cache reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Dcrst::Reset
    }
}
#[doc = "Field `DCRST` writer - DBUS cache reset"]
pub type DcrstW<'a, REG> = crate::BitWriter<'a, REG, Dcrst>;
impl<'a, REG> DcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Dcrst::NoEffect)
    }
    #[doc = "DBUS cache reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dcrst::Reset)
    }
}
#[doc = "Program width to flash memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pgw {
    #[doc = "0: 32b program width to flash memory"]
    Width32 = 0,
    #[doc = "1: 64b program width to flash memory"]
    Width64 = 1,
}
impl From<Pgw> for bool {
    #[inline(always)]
    fn from(variant: Pgw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGW` reader - Program width to flash memory"]
pub type PgwR = crate::BitReader<Pgw>;
impl PgwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgw {
        match self.bits {
            false => Pgw::Width32,
            true => Pgw::Width64,
        }
    }
    #[doc = "32b program width to flash memory"]
    #[inline(always)]
    pub fn is_width32(&self) -> bool {
        *self == Pgw::Width32
    }
    #[doc = "64b program width to flash memory"]
    #[inline(always)]
    pub fn is_width64(&self) -> bool {
        *self == Pgw::Width64
    }
}
#[doc = "Field `PGW` writer - Program width to flash memory"]
pub type PgwW<'a, REG> = crate::BitWriter<'a, REG, Pgw>;
impl<'a, REG> PgwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32b program width to flash memory"]
    #[inline(always)]
    pub fn width32(self) -> &'a mut crate::W<REG> {
        self.variant(Pgw::Width32)
    }
    #[doc = "64b program width to flash memory"]
    #[inline(always)]
    pub fn width64(self) -> &'a mut crate::W<REG> {
        self.variant(Pgw::Width64)
    }
}
impl R {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WscntR {
        WscntR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Pre-fetch enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PfenR {
        PfenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - IBUS cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> IcenR {
        IcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBUS cache enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DcenR {
        DcenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IBUS cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> IcrstR {
        IcrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DBUS cache reset"]
    #[inline(always)]
    pub fn dcrst(&self) -> DcrstR {
        DcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Program width to flash memory"]
    #[inline(always)]
    pub fn pgw(&self) -> PgwR {
        PgwR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    #[must_use]
    pub fn wscnt(&mut self) -> WscntW<WsSpec> {
        WscntW::new(self, 0)
    }
    #[doc = "Bit 4 - Pre-fetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfen(&mut self) -> PfenW<WsSpec> {
        PfenW::new(self, 4)
    }
    #[doc = "Bit 9 - IBUS cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> IcenW<WsSpec> {
        IcenW::new(self, 9)
    }
    #[doc = "Bit 10 - DBUS cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcen(&mut self) -> DcenW<WsSpec> {
        DcenW::new(self, 10)
    }
    #[doc = "Bit 11 - IBUS cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> IcrstW<WsSpec> {
        IcrstW::new(self, 11)
    }
    #[doc = "Bit 12 - DBUS cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcrst(&mut self) -> DcrstW<WsSpec> {
        DcrstW::new(self, 12)
    }
    #[doc = "Bit 15 - Program width to flash memory"]
    #[inline(always)]
    #[must_use]
    pub fn pgw(&mut self) -> PgwW<WsSpec> {
        PgwW::new(self, 15)
    }
}
#[doc = "wait state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ws::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ws::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets WS to value 0x0630"]
impl crate::Resettable for WsSpec {
    const RESET_VALUE: u32 = 0x0630;
}
