#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "CK_HXTAL divider previous PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hxtalpredv {
    #[doc = "0: HXTAL input to PLL not divided"]
    Div1 = 0,
    #[doc = "1: HXTAL input to PLL divided by 2"]
    Div2 = 1,
    #[doc = "2: HXTAL input to PLL divided by 3"]
    Div3 = 2,
    #[doc = "3: HXTAL input to PLL divided by 4"]
    Div4 = 3,
    #[doc = "4: HXTAL input to PLL divided by 5"]
    Div5 = 4,
    #[doc = "5: HXTAL input to PLL divided by 6"]
    Div6 = 5,
    #[doc = "6: HXTAL input to PLL divided by 7"]
    Div7 = 6,
    #[doc = "7: HXTAL input to PLL divided by 8"]
    Div8 = 7,
    #[doc = "8: HXTAL input to PLL divided by 9"]
    Div9 = 8,
    #[doc = "9: HXTAL input to PLL divided by 10"]
    Div10 = 9,
    #[doc = "10: HXTAL input to PLL divided by 11"]
    Div11 = 10,
    #[doc = "11: HXTAL input to PLL divided by 12"]
    Div12 = 11,
    #[doc = "12: HXTAL input to PLL divided by 13"]
    Div13 = 12,
    #[doc = "13: HXTAL input to PLL divided by 14"]
    Div14 = 13,
    #[doc = "14: HXTAL input to PLL divided by 15"]
    Div15 = 14,
    #[doc = "15: HXTAL input to PLL divided by 16"]
    Div16 = 15,
}
impl From<Hxtalpredv> for u8 {
    #[inline(always)]
    fn from(variant: Hxtalpredv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hxtalpredv {
    type Ux = u8;
}
#[doc = "Field `HXTALPREDV` reader - CK_HXTAL divider previous PLL"]
pub type HxtalpredvR = crate::FieldReader<Hxtalpredv>;
impl HxtalpredvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalpredv {
        match self.bits {
            0 => Hxtalpredv::Div1,
            1 => Hxtalpredv::Div2,
            2 => Hxtalpredv::Div3,
            3 => Hxtalpredv::Div4,
            4 => Hxtalpredv::Div5,
            5 => Hxtalpredv::Div6,
            6 => Hxtalpredv::Div7,
            7 => Hxtalpredv::Div8,
            8 => Hxtalpredv::Div9,
            9 => Hxtalpredv::Div10,
            10 => Hxtalpredv::Div11,
            11 => Hxtalpredv::Div12,
            12 => Hxtalpredv::Div13,
            13 => Hxtalpredv::Div14,
            14 => Hxtalpredv::Div15,
            15 => Hxtalpredv::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "HXTAL input to PLL not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Hxtalpredv::Div1
    }
    #[doc = "HXTAL input to PLL divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Hxtalpredv::Div2
    }
    #[doc = "HXTAL input to PLL divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Hxtalpredv::Div3
    }
    #[doc = "HXTAL input to PLL divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Hxtalpredv::Div4
    }
    #[doc = "HXTAL input to PLL divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Hxtalpredv::Div5
    }
    #[doc = "HXTAL input to PLL divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Hxtalpredv::Div6
    }
    #[doc = "HXTAL input to PLL divided by 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Hxtalpredv::Div7
    }
    #[doc = "HXTAL input to PLL divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Hxtalpredv::Div8
    }
    #[doc = "HXTAL input to PLL divided by 9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Hxtalpredv::Div9
    }
    #[doc = "HXTAL input to PLL divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Hxtalpredv::Div10
    }
    #[doc = "HXTAL input to PLL divided by 11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Hxtalpredv::Div11
    }
    #[doc = "HXTAL input to PLL divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Hxtalpredv::Div12
    }
    #[doc = "HXTAL input to PLL divided by 13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Hxtalpredv::Div13
    }
    #[doc = "HXTAL input to PLL divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Hxtalpredv::Div14
    }
    #[doc = "HXTAL input to PLL divided by 15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Hxtalpredv::Div15
    }
    #[doc = "HXTAL input to PLL divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Hxtalpredv::Div16
    }
}
#[doc = "Field `HXTALPREDV` writer - CK_HXTAL divider previous PLL"]
pub type HxtalpredvW<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, Hxtalpredv>;
impl<'a, REG> HxtalpredvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HXTAL input to PLL not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div1)
    }
    #[doc = "HXTAL input to PLL divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div2)
    }
    #[doc = "HXTAL input to PLL divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div3)
    }
    #[doc = "HXTAL input to PLL divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div4)
    }
    #[doc = "HXTAL input to PLL divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div5)
    }
    #[doc = "HXTAL input to PLL divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div6)
    }
    #[doc = "HXTAL input to PLL divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div7)
    }
    #[doc = "HXTAL input to PLL divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div8)
    }
    #[doc = "HXTAL input to PLL divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div9)
    }
    #[doc = "HXTAL input to PLL divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div10)
    }
    #[doc = "HXTAL input to PLL divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div11)
    }
    #[doc = "HXTAL input to PLL divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div12)
    }
    #[doc = "HXTAL input to PLL divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div13)
    }
    #[doc = "HXTAL input to PLL divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div14)
    }
    #[doc = "HXTAL input to PLL divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div15)
    }
    #[doc = "HXTAL input to PLL divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalpredv::Div16)
    }
}
impl R {
    #[doc = "Bits 0:3 - CK_HXTAL divider previous PLL"]
    #[inline(always)]
    pub fn hxtalpredv(&self) -> HxtalpredvR {
        HxtalpredvR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CK_HXTAL divider previous PLL"]
    #[inline(always)]
    #[must_use]
    pub fn hxtalpredv(&mut self) -> HxtalpredvW<Cfg1Spec> {
        HxtalpredvW::new(self, 0)
    }
}
#[doc = "Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
