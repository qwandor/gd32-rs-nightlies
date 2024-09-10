#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Auto baudrate detection command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abdcmd {
    #[doc = "1: Resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    Request = 1,
}
impl From<Abdcmd> for bool {
    #[inline(always)]
    fn from(variant: Abdcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABDCMD` writer - Auto baudrate detection command"]
pub type AbdcmdW<'a, REG> = crate::BitWriter<'a, REG, Abdcmd>;
impl<'a, REG> AbdcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline(always)]
    pub fn request(self) -> &'a mut crate::W<REG> {
        self.variant(Abdcmd::Request)
    }
}
#[doc = "Send break command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbkcmd {
    #[doc = "1: Sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    Break = 1,
}
impl From<Sbkcmd> for bool {
    #[inline(always)]
    fn from(variant: Sbkcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKCMD` writer - Send break command"]
pub type SbkcmdW<'a, REG> = crate::BitWriter<'a, REG, Sbkcmd>;
impl<'a, REG> SbkcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(Sbkcmd::Break)
    }
}
#[doc = "Mute mode command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmcmd {
    #[doc = "1: Puts the USART in mute mode and sets the RWU flag"]
    Mute = 1,
}
impl From<Mmcmd> for bool {
    #[inline(always)]
    fn from(variant: Mmcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCMD` writer - Mute mode command"]
pub type MmcmdW<'a, REG> = crate::BitWriter<'a, REG, Mmcmd>;
impl<'a, REG> MmcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut crate::W<REG> {
        self.variant(Mmcmd::Mute)
    }
}
#[doc = "Receive data flush command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfcmd {
    #[doc = "1: Clears the RBNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    Discard = 1,
}
impl From<Rxfcmd> for bool {
    #[inline(always)]
    fn from(variant: Rxfcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFCMD` writer - Receive data flush command"]
pub type RxfcmdW<'a, REG> = crate::BitWriter<'a, REG, Rxfcmd>;
impl<'a, REG> RxfcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the RBNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfcmd::Discard)
    }
}
#[doc = "Transmit data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfcmd {
    #[doc = "1: Set the TBE flag. This allows to discard the transmit data"]
    Discard = 1,
}
impl From<Txfcmd> for bool {
    #[inline(always)]
    fn from(variant: Txfcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFCMD` writer - Transmit data flush request"]
pub type TxfcmdW<'a, REG> = crate::BitWriter<'a, REG, Txfcmd>;
impl<'a, REG> TxfcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set the TBE flag. This allows to discard the transmit data"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(Txfcmd::Discard)
    }
}
impl W {
    #[doc = "Bit 0 - Auto baudrate detection command"]
    #[inline(always)]
    #[must_use]
    pub fn abdcmd(&mut self) -> AbdcmdW<CmdSpec> {
        AbdcmdW::new(self, 0)
    }
    #[doc = "Bit 1 - Send break command"]
    #[inline(always)]
    #[must_use]
    pub fn sbkcmd(&mut self) -> SbkcmdW<CmdSpec> {
        SbkcmdW::new(self, 1)
    }
    #[doc = "Bit 2 - Mute mode command"]
    #[inline(always)]
    #[must_use]
    pub fn mmcmd(&mut self) -> MmcmdW<CmdSpec> {
        MmcmdW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data flush command"]
    #[inline(always)]
    #[must_use]
    pub fn rxfcmd(&mut self) -> RxfcmdW<CmdSpec> {
        RxfcmdW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    #[must_use]
    pub fn txfcmd(&mut self) -> TxfcmdW<CmdSpec> {
        TxfcmdW::new(self, 4)
    }
}
#[doc = "Request register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
