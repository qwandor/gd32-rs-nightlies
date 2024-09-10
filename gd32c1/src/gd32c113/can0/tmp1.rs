#[doc = "Register `TMP1` reader"]
pub type R = crate::R<Tmp1Spec>;
#[doc = "Register `TMP1` writer"]
pub type W = crate::W<Tmp1Spec>;
#[doc = "Field `DLENC` reader - Data length code"]
pub type DlencR = crate::FieldReader;
#[doc = "Field `DLENC` writer - Data length code"]
pub type DlencW<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `ESI` reader - Error status indicator"]
pub type EsiR = crate::BitReader;
#[doc = "Field `ESI` writer - Error status indicator"]
pub type EsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRS` reader - Bit rate of data switch"]
pub type BrsR = crate::BitReader;
#[doc = "Field `BRS` writer - Bit rate of data switch"]
pub type BrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDF` reader - CAN FD frame flag"]
pub type FdfR = crate::BitReader;
#[doc = "Field `FDF` writer - CAN FD frame flag"]
pub type FdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Time stamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsen {
    #[doc = "0: Timestamp disabled"]
    Disabled = 0,
    #[doc = "1: Timestamp enabled"]
    Enabled = 1,
}
impl From<Tsen> for bool {
    #[inline(always)]
    fn from(variant: Tsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` reader - Time stamp enable"]
pub type TsenR = crate::BitReader<Tsen>;
impl TsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsen {
        match self.bits {
            false => Tsen::Disabled,
            true => Tsen::Enabled,
        }
    }
    #[doc = "Timestamp disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tsen::Disabled
    }
    #[doc = "Timestamp enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tsen::Enabled
    }
}
#[doc = "Field `TSEN` writer - Time stamp enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG, Tsen>;
impl<'a, REG> TsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timestamp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tsen::Disabled)
    }
    #[doc = "Timestamp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tsen::Enabled)
    }
}
#[doc = "Field `TS` reader - Time stamp"]
pub type TsR = crate::FieldReader<u16>;
#[doc = "Field `TS` writer - Time stamp"]
pub type TsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&self) -> DlencR {
        DlencR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Error status indicator"]
    #[inline(always)]
    pub fn esi(&self) -> EsiR {
        EsiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit rate of data switch"]
    #[inline(always)]
    pub fn brs(&self) -> BrsR {
        BrsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN FD frame flag"]
    #[inline(always)]
    pub fn fdf(&self) -> FdfR {
        FdfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    #[must_use]
    pub fn dlenc(&mut self) -> DlencW<Tmp1Spec> {
        DlencW::new(self, 0)
    }
    #[doc = "Bit 4 - Error status indicator"]
    #[inline(always)]
    #[must_use]
    pub fn esi(&mut self) -> EsiW<Tmp1Spec> {
        EsiW::new(self, 4)
    }
    #[doc = "Bit 5 - Bit rate of data switch"]
    #[inline(always)]
    #[must_use]
    pub fn brs(&mut self) -> BrsW<Tmp1Spec> {
        BrsW::new(self, 5)
    }
    #[doc = "Bit 7 - CAN FD frame flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdf(&mut self) -> FdfW<Tmp1Spec> {
        FdfW::new(self, 7)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<Tmp1Spec> {
        TsenW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TsW<Tmp1Spec> {
        TsW::new(self, 16)
    }
}
#[doc = "Transmit mailbox property register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmp1Spec;
impl crate::RegisterSpec for Tmp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmp1::R`](R) reader structure"]
impl crate::Readable for Tmp1Spec {}
#[doc = "`write(|w| ..)` method takes [`tmp1::W`](W) writer structure"]
impl crate::Writable for Tmp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMP1 to value 0"]
impl crate::Resettable for Tmp1Spec {
    const RESET_VALUE: u32 = 0;
}
