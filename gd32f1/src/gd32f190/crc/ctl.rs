#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstw {
    #[doc = "1: Resets the DATA register to IDATA, with no effect on FDATA"]
    Reset = 1,
}
impl From<Rstw> for bool {
    #[inline(always)]
    fn from(variant: Rstw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - reset bit"]
pub type RstR = crate::BitReader<Rstw>;
impl RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rstw> {
        match self.bits {
            true => Some(Rstw::Reset),
            _ => None,
        }
    }
    #[doc = "Resets the DATA register to IDATA, with no effect on FDATA"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Rstw::Reset
    }
}
#[doc = "Field `RST` writer - reset bit"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG, Rstw>;
impl<'a, REG> RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the DATA register to IDATA, with no effect on FDATA"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rstw::Reset)
    }
}
#[doc = "Input Data Reverse Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RevI {
    #[doc = "0: Bit order not affected"]
    Normal = 0,
    #[doc = "1: Bit reversal done by byte"]
    Byte = 1,
    #[doc = "2: Bit reversal done by half-word"]
    HalfWord = 2,
    #[doc = "3: Bit reversal done by word"]
    Word = 3,
}
impl From<RevI> for u8 {
    #[inline(always)]
    fn from(variant: RevI) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RevI {
    type Ux = u8;
}
#[doc = "Field `REV_I` reader - Input Data Reverse Function"]
pub type RevIR = crate::FieldReader<RevI>;
impl RevIR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RevI {
        match self.bits {
            0 => RevI::Normal,
            1 => RevI::Byte,
            2 => RevI::HalfWord,
            3 => RevI::Word,
            _ => unreachable!(),
        }
    }
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RevI::Normal
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == RevI::Byte
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == RevI::HalfWord
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == RevI::Word
    }
}
#[doc = "Field `REV_I` writer - Input Data Reverse Function"]
pub type RevIW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RevI>;
impl<'a, REG> RevIW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(RevI::Normal)
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(RevI::Byte)
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(RevI::HalfWord)
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(RevI::Word)
    }
}
#[doc = "Output Data Reverse Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RevO {
    #[doc = "0: Bit order not affected"]
    Normal = 0,
    #[doc = "1: Bit reversed output"]
    Reversed = 1,
}
impl From<RevO> for bool {
    #[inline(always)]
    fn from(variant: RevO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV_O` reader - Output Data Reverse Function"]
pub type RevOR = crate::BitReader<RevO>;
impl RevOR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RevO {
        match self.bits {
            false => RevO::Normal,
            true => RevO::Reversed,
        }
    }
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RevO::Normal
    }
    #[doc = "Bit reversed output"]
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == RevO::Reversed
    }
}
#[doc = "Field `REV_O` writer - Output Data Reverse Function"]
pub type RevOW<'a, REG> = crate::BitWriter<'a, REG, RevO>;
impl<'a, REG> RevOW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(RevO::Normal)
    }
    #[doc = "Bit reversed output"]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut crate::W<REG> {
        self.variant(RevO::Reversed)
    }
}
impl R {
    #[doc = "Bit 0 - reset bit"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 5:6 - Input Data Reverse Function"]
    #[inline(always)]
    pub fn rev_i(&self) -> RevIR {
        RevIR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Output Data Reverse Function"]
    #[inline(always)]
    pub fn rev_o(&self) -> RevOR {
        RevOR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<CtlSpec> {
        RstW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Input Data Reverse Function"]
    #[inline(always)]
    #[must_use]
    pub fn rev_i(&mut self) -> RevIW<CtlSpec> {
        RevIW::new(self, 5)
    }
    #[doc = "Bit 7 - Output Data Reverse Function"]
    #[inline(always)]
    #[must_use]
    pub fn rev_o(&mut self) -> RevOW<CtlSpec> {
        RevOW::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
