#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Address of the USART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Addr {
    #[doc = "0: 4-bit address detection"]
    Bit4 = 0,
    #[doc = "1: Full-bit address detection"]
    Full = 1,
}
impl From<Addr> for u8 {
    #[inline(always)]
    fn from(variant: Addr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Addr {
    type Ux = u8;
}
#[doc = "Field `ADDR` reader - Address of the USART"]
pub type AddrR = crate::FieldReader<Addr>;
impl AddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Addr> {
        match self.bits {
            0 => Some(Addr::Bit4),
            1 => Some(Addr::Full),
            _ => None,
        }
    }
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == Addr::Bit4
    }
    #[doc = "Full-bit address detection"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Addr::Full
    }
}
#[doc = "Field `ADDR` writer - Address of the USART"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 4, Addr>;
impl<'a, REG> AddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn bit4(self) -> &'a mut crate::W<REG> {
        self.variant(Addr::Bit4)
    }
    #[doc = "Full-bit address detection"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Addr::Full)
    }
}
#[doc = "LIN break frame length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lblen {
    #[doc = "0: 10-bit break detection"]
    Bit10 = 0,
    #[doc = "1: 11-bit break detection"]
    Bit11 = 1,
}
impl From<Lblen> for bool {
    #[inline(always)]
    fn from(variant: Lblen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBLEN` reader - LIN break frame length"]
pub type LblenR = crate::BitReader<Lblen>;
impl LblenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lblen {
        match self.bits {
            false => Lblen::Bit10,
            true => Lblen::Bit11,
        }
    }
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == Lblen::Bit10
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == Lblen::Bit11
    }
}
#[doc = "Field `LBLEN` writer - LIN break frame length"]
pub type LblenW<'a, REG> = crate::BitWriter<'a, REG, Lblen>;
impl<'a, REG> LblenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(Lblen::Bit10)
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut crate::W<REG> {
        self.variant(Lblen::Bit11)
    }
}
#[doc = "LIN break detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbdie {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated whenever LBDF=1 in the STAT register"]
    Enabled = 1,
}
impl From<Lbdie> for bool {
    #[inline(always)]
    fn from(variant: Lbdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LbdieR = crate::BitReader<Lbdie>;
impl LbdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbdie {
        match self.bits {
            false => Lbdie::Disabled,
            true => Lbdie::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lbdie::Disabled
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the STAT register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lbdie::Enabled
    }
}
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG, Lbdie>;
impl<'a, REG> LbdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdie::Disabled)
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the STAT register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdie::Enabled)
    }
}
#[doc = "CK Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clen {
    #[doc = "0: The clock pulse of the last data bit is not output to the CK pin"]
    NotOutput = 0,
    #[doc = "1: The clock pulse of the last data bit is output to the CK pin"]
    Output = 1,
}
impl From<Clen> for bool {
    #[inline(always)]
    fn from(variant: Clen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEN` reader - CK Length"]
pub type ClenR = crate::BitReader<Clen>;
impl ClenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clen {
        match self.bits {
            false => Clen::NotOutput,
            true => Clen::Output,
        }
    }
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == Clen::NotOutput
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Clen::Output
    }
}
#[doc = "Field `CLEN` writer - CK Length"]
pub type ClenW<'a, REG> = crate::BitWriter<'a, REG, Clen>;
impl<'a, REG> ClenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn not_output(self) -> &'a mut crate::W<REG> {
        self.variant(Clen::NotOutput)
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Clen::Output)
    }
}
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cph {
    #[doc = "0: The first clock transition is the first data capture edge"]
    First = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    Second = 1,
}
impl From<Cph> for bool {
    #[inline(always)]
    fn from(variant: Cph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPH` reader - Clock phase"]
pub type CphR = crate::BitReader<Cph>;
impl CphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cph {
        match self.bits {
            false => Cph::First,
            true => Cph::Second,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == Cph::First
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == Cph::Second
    }
}
#[doc = "Field `CPH` writer - Clock phase"]
pub type CphW<'a, REG> = crate::BitWriter<'a, REG, Cph>;
impl<'a, REG> CphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(Cph::First)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(Cph::Second)
    }
}
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpl {
    #[doc = "0: Steady low value on CK pin outside tranmission window"]
    NotInverted = 0,
    #[doc = "1: Steady high value on CK pin outside tranmission window"]
    Inverted = 1,
}
impl From<Cpl> for bool {
    #[inline(always)]
    fn from(variant: Cpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPL` reader - Clock polarity"]
pub type CplR = crate::BitReader<Cpl>;
impl CplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpl {
        match self.bits {
            false => Cpl::NotInverted,
            true => Cpl::Inverted,
        }
    }
    #[doc = "Steady low value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == Cpl::NotInverted
    }
    #[doc = "Steady high value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Cpl::Inverted
    }
}
#[doc = "Field `CPL` writer - Clock polarity"]
pub type CplW<'a, REG> = crate::BitWriter<'a, REG, Cpl>;
impl<'a, REG> CplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Steady low value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Cpl::NotInverted)
    }
    #[doc = "Steady high value on CK pin outside tranmission window"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Cpl::Inverted)
    }
}
#[doc = "CK pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cken {
    #[doc = "0: CK pin disabled"]
    Disabled = 0,
    #[doc = "1: CK pin enabled"]
    Enabled = 1,
}
impl From<Cken> for bool {
    #[inline(always)]
    fn from(variant: Cken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKEN` reader - CK pin enable"]
pub type CkenR = crate::BitReader<Cken>;
impl CkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cken {
        match self.bits {
            false => Cken::Disabled,
            true => Cken::Enabled,
        }
    }
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cken::Disabled
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cken::Enabled
    }
}
#[doc = "Field `CKEN` writer - CK pin enable"]
pub type CkenW<'a, REG> = crate::BitWriter<'a, REG, Cken>;
impl<'a, REG> CkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cken::Disabled)
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cken::Enabled)
    }
}
#[doc = "STOP bits length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stb {
    #[doc = "0: 1 stop bit"]
    Stop1 = 0,
    #[doc = "1: 0.5 stop bit"]
    Stop0p5 = 1,
    #[doc = "2: 2 stop bit"]
    Stop2 = 2,
    #[doc = "3: 1.5 stop bit"]
    Stop1p5 = 3,
}
impl From<Stb> for u8 {
    #[inline(always)]
    fn from(variant: Stb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stb {
    type Ux = u8;
}
#[doc = "Field `STB` reader - STOP bits length"]
pub type StbR = crate::FieldReader<Stb>;
impl StbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stb {
        match self.bits {
            0 => Stb::Stop1,
            1 => Stb::Stop0p5,
            2 => Stb::Stop2,
            3 => Stb::Stop1p5,
            _ => unreachable!(),
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == Stb::Stop1
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == Stb::Stop0p5
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == Stb::Stop2
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == Stb::Stop1p5
    }
}
#[doc = "Field `STB` writer - STOP bits length"]
pub type StbW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Stb>;
impl<'a, REG> StbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(Stb::Stop1)
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut crate::W<REG> {
        self.variant(Stb::Stop0p5)
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(Stb::Stop2)
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut crate::W<REG> {
        self.variant(Stb::Stop1p5)
    }
}
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lmen {
    #[doc = "0: LIN mode disabled"]
    Disabled = 0,
    #[doc = "1: LIN mode enabled"]
    Enabled = 1,
}
impl From<Lmen> for bool {
    #[inline(always)]
    fn from(variant: Lmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LMEN` reader - LIN mode enable"]
pub type LmenR = crate::BitReader<Lmen>;
impl LmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lmen {
        match self.bits {
            false => Lmen::Disabled,
            true => Lmen::Enabled,
        }
    }
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lmen::Disabled
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lmen::Enabled
    }
}
#[doc = "Field `LMEN` writer - LIN mode enable"]
pub type LmenW<'a, REG> = crate::BitWriter<'a, REG, Lmen>;
impl<'a, REG> LmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lmen::Disabled)
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lmen::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Address of the USART"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    pub fn lblen(&self) -> LblenR {
        LblenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CK Length"]
    #[inline(always)]
    pub fn clen(&self) -> ClenR {
        ClenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cph(&self) -> CphR {
        CphR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpl(&self) -> CplR {
        CplR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    pub fn cken(&self) -> CkenR {
        CkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    pub fn stb(&self) -> StbR {
        StbR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn lmen(&self) -> LmenR {
        LmenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address of the USART"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Ctl1Spec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 5 - LIN break frame length"]
    #[inline(always)]
    #[must_use]
    pub fn lblen(&mut self) -> LblenW<Ctl1Spec> {
        LblenW::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LbdieW<Ctl1Spec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bit 8 - CK Length"]
    #[inline(always)]
    #[must_use]
    pub fn clen(&mut self) -> ClenW<Ctl1Spec> {
        ClenW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cph(&mut self) -> CphW<Ctl1Spec> {
        CphW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CplW<Ctl1Spec> {
        CplW::new(self, 10)
    }
    #[doc = "Bit 11 - CK pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CkenW<Ctl1Spec> {
        CkenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bits length"]
    #[inline(always)]
    #[must_use]
    pub fn stb(&mut self) -> StbW<Ctl1Spec> {
        StbW::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lmen(&mut self) -> LmenW<Ctl1Spec> {
        LmenW::new(self, 14)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}
