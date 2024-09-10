#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "PREDV0 division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Predv0 {
    #[doc = "0: Input to PLL not divided"]
    Div1 = 0,
    #[doc = "1: Input to PLL divided by 2"]
    Div2 = 1,
    #[doc = "2: Input to PLL divided by 3"]
    Div3 = 2,
    #[doc = "3: Input to PLL divided by 4"]
    Div4 = 3,
    #[doc = "4: Input to PLL divided by 5"]
    Div5 = 4,
    #[doc = "5: Input to PLL divided by 6"]
    Div6 = 5,
    #[doc = "6: Input to PLL divided by 7"]
    Div7 = 6,
    #[doc = "7: Input to PLL divided by 8"]
    Div8 = 7,
    #[doc = "8: Input to PLL divided by 9"]
    Div9 = 8,
    #[doc = "9: Input to PLL divided by 10"]
    Div10 = 9,
    #[doc = "10: Input to PLL divided by 11"]
    Div11 = 10,
    #[doc = "11: Input to PLL divided by 12"]
    Div12 = 11,
    #[doc = "12: Input to PLL divided by 13"]
    Div13 = 12,
    #[doc = "13: Input to PLL divided by 14"]
    Div14 = 13,
    #[doc = "14: Input to PLL divided by 15"]
    Div15 = 14,
    #[doc = "15: Input to PLL divided by 16"]
    Div16 = 15,
}
impl From<Predv0> for u8 {
    #[inline(always)]
    fn from(variant: Predv0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Predv0 {
    type Ux = u8;
}
#[doc = "Field `PREDV0` reader - PREDV0 division factor"]
pub type Predv0R = crate::FieldReader<Predv0>;
impl Predv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Predv0 {
        match self.bits {
            0 => Predv0::Div1,
            1 => Predv0::Div2,
            2 => Predv0::Div3,
            3 => Predv0::Div4,
            4 => Predv0::Div5,
            5 => Predv0::Div6,
            6 => Predv0::Div7,
            7 => Predv0::Div8,
            8 => Predv0::Div9,
            9 => Predv0::Div10,
            10 => Predv0::Div11,
            11 => Predv0::Div12,
            12 => Predv0::Div13,
            13 => Predv0::Div14,
            14 => Predv0::Div15,
            15 => Predv0::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Input to PLL not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Predv0::Div1
    }
    #[doc = "Input to PLL divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Predv0::Div2
    }
    #[doc = "Input to PLL divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Predv0::Div3
    }
    #[doc = "Input to PLL divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Predv0::Div4
    }
    #[doc = "Input to PLL divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Predv0::Div5
    }
    #[doc = "Input to PLL divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Predv0::Div6
    }
    #[doc = "Input to PLL divided by 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Predv0::Div7
    }
    #[doc = "Input to PLL divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Predv0::Div8
    }
    #[doc = "Input to PLL divided by 9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Predv0::Div9
    }
    #[doc = "Input to PLL divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Predv0::Div10
    }
    #[doc = "Input to PLL divided by 11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Predv0::Div11
    }
    #[doc = "Input to PLL divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Predv0::Div12
    }
    #[doc = "Input to PLL divided by 13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Predv0::Div13
    }
    #[doc = "Input to PLL divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Predv0::Div14
    }
    #[doc = "Input to PLL divided by 15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Predv0::Div15
    }
    #[doc = "Input to PLL divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Predv0::Div16
    }
}
#[doc = "Field `PREDV0` writer - PREDV0 division factor"]
pub type Predv0W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, Predv0>;
impl<'a, REG> Predv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input to PLL not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div1)
    }
    #[doc = "Input to PLL divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div2)
    }
    #[doc = "Input to PLL divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div3)
    }
    #[doc = "Input to PLL divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div4)
    }
    #[doc = "Input to PLL divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div5)
    }
    #[doc = "Input to PLL divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div6)
    }
    #[doc = "Input to PLL divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div7)
    }
    #[doc = "Input to PLL divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div8)
    }
    #[doc = "Input to PLL divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div9)
    }
    #[doc = "Input to PLL divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div10)
    }
    #[doc = "Input to PLL divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div11)
    }
    #[doc = "Input to PLL divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div12)
    }
    #[doc = "Input to PLL divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div13)
    }
    #[doc = "Input to PLL divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div14)
    }
    #[doc = "Input to PLL divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div15)
    }
    #[doc = "Input to PLL divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Predv0::Div16)
    }
}
#[doc = "Field `PREDV1` reader - PREDV1 division factor"]
pub use Predv0R as Predv1R;
#[doc = "Field `PREDV1` writer - PREDV1 division factor"]
pub use Predv0W as Predv1W;
#[doc = "Field `PLL1MF` reader - The PLL1 clock multiplication factor"]
pub type Pll1mfR = crate::FieldReader;
#[doc = "Field `PLL1MF` writer - The PLL1 clock multiplication factor"]
pub type Pll1mfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLL2MF` reader - The PLL2 clock multiplication factor"]
pub type Pll2mfR = crate::FieldReader;
#[doc = "Field `PLL2MF` writer - The PLL2 clock multiplication factor"]
pub type Pll2mfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PREDV0SEL` reader - PREDV0 input Clock Source Selection"]
pub type Predv0selR = crate::BitReader;
#[doc = "Field `PREDV0SEL` writer - PREDV0 input Clock Source Selection"]
pub type Predv0selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1SEL` reader - I2S1 Clock Source Selection"]
pub type I2s1selR = crate::BitReader;
#[doc = "Field `I2S1SEL` writer - I2S1 Clock Source Selection"]
pub type I2s1selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2SEL` reader - I2S2 Clock Source Selection"]
pub type I2s2selR = crate::BitReader;
#[doc = "Field `I2S2SEL` writer - I2S2 Clock Source Selection"]
pub type I2s2selW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - PREDV0 division factor"]
    #[inline(always)]
    pub fn predv0(&self) -> Predv0R {
        Predv0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PREDV1 division factor"]
    #[inline(always)]
    pub fn predv1(&self) -> Predv1R {
        Predv1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The PLL1 clock multiplication factor"]
    #[inline(always)]
    pub fn pll1mf(&self) -> Pll1mfR {
        Pll1mfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The PLL2 clock multiplication factor"]
    #[inline(always)]
    pub fn pll2mf(&self) -> Pll2mfR {
        Pll2mfR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - PREDV0 input Clock Source Selection"]
    #[inline(always)]
    pub fn predv0sel(&self) -> Predv0selR {
        Predv0selR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2S1 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2s1selR {
        I2s1selR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2S2 Clock Source Selection"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2s2selR {
        I2s2selR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDV0 division factor"]
    #[inline(always)]
    #[must_use]
    pub fn predv0(&mut self) -> Predv0W<Cfg1Spec> {
        Predv0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - PREDV1 division factor"]
    #[inline(always)]
    #[must_use]
    pub fn predv1(&mut self) -> Predv1W<Cfg1Spec> {
        Predv1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - The PLL1 clock multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pll1mf(&mut self) -> Pll1mfW<Cfg1Spec> {
        Pll1mfW::new(self, 8)
    }
    #[doc = "Bits 12:15 - The PLL2 clock multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pll2mf(&mut self) -> Pll2mfW<Cfg1Spec> {
        Pll2mfW::new(self, 12)
    }
    #[doc = "Bit 16 - PREDV0 input Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn predv0sel(&mut self) -> Predv0selW<Cfg1Spec> {
        Predv0selW::new(self, 16)
    }
    #[doc = "Bit 17 - I2S1 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1sel(&mut self) -> I2s1selW<Cfg1Spec> {
        I2s1selW::new(self, 17)
    }
    #[doc = "Bit 18 - I2S2 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2s2selW<Cfg1Spec> {
        I2s2selW::new(self, 18)
    }
}
#[doc = "Clock Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
