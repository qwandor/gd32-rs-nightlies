#[doc = "Register `AFSEL0` reader"]
pub type R = crate::R<Afsel0Spec>;
#[doc = "Register `AFSEL0` writer"]
pub type W = crate::W<Afsel0Spec>;
#[doc = "Pin 0 alternate function selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel0 {
    #[doc = "0: AF0"]
    Af0 = 0,
    #[doc = "1: AF1"]
    Af1 = 1,
    #[doc = "2: AF2"]
    Af2 = 2,
    #[doc = "3: AF3"]
    Af3 = 3,
    #[doc = "4: AF4"]
    Af4 = 4,
    #[doc = "5: AF5"]
    Af5 = 5,
    #[doc = "6: AF6"]
    Af6 = 6,
    #[doc = "7: AF7"]
    Af7 = 7,
    #[doc = "9: AF9"]
    Af9 = 9,
    #[doc = "11: AF11"]
    Af11 = 11,
}
impl From<Sel0> for u8 {
    #[inline(always)]
    fn from(variant: Sel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel0 {
    type Ux = u8;
}
#[doc = "Field `SEL0` reader - Pin 0 alternate function selected"]
pub type Sel0R = crate::FieldReader<Sel0>;
impl Sel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel0> {
        match self.bits {
            0 => Some(Sel0::Af0),
            1 => Some(Sel0::Af1),
            2 => Some(Sel0::Af2),
            3 => Some(Sel0::Af3),
            4 => Some(Sel0::Af4),
            5 => Some(Sel0::Af5),
            6 => Some(Sel0::Af6),
            7 => Some(Sel0::Af7),
            9 => Some(Sel0::Af9),
            11 => Some(Sel0::Af11),
            _ => None,
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == Sel0::Af0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Sel0::Af1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Sel0::Af2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Sel0::Af3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Sel0::Af4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Sel0::Af5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == Sel0::Af6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == Sel0::Af7
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == Sel0::Af9
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == Sel0::Af11
    }
}
#[doc = "Field `SEL0` writer - Pin 0 alternate function selected"]
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Sel0>;
impl<'a, REG> Sel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af7)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af9)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(Sel0::Af11)
    }
}
#[doc = "Field `SEL1` reader - Pin 1 alternate function selected"]
pub use Sel0R as Sel1R;
#[doc = "Field `SEL2` reader - Pin 2 alternate function selected"]
pub use Sel0R as Sel2R;
#[doc = "Field `SEL3` reader - Pin 3 alternate function selected"]
pub use Sel0R as Sel3R;
#[doc = "Field `SEL4` reader - Pin 4 alternate function selected"]
pub use Sel0R as Sel4R;
#[doc = "Field `SEL5` reader - Pin 5 alternate function selected"]
pub use Sel0R as Sel5R;
#[doc = "Field `SEL6` reader - Pin 6 alternate function selected"]
pub use Sel0R as Sel6R;
#[doc = "Field `SEL7` reader - Pin 7 alternate function selected"]
pub use Sel0R as Sel7R;
#[doc = "Field `SEL1` writer - Pin 1 alternate function selected"]
pub use Sel0W as Sel1W;
#[doc = "Field `SEL2` writer - Pin 2 alternate function selected"]
pub use Sel0W as Sel2W;
#[doc = "Field `SEL3` writer - Pin 3 alternate function selected"]
pub use Sel0W as Sel3W;
#[doc = "Field `SEL4` writer - Pin 4 alternate function selected"]
pub use Sel0W as Sel4W;
#[doc = "Field `SEL5` writer - Pin 5 alternate function selected"]
pub use Sel0W as Sel5W;
#[doc = "Field `SEL6` writer - Pin 6 alternate function selected"]
pub use Sel0W as Sel6W;
#[doc = "Field `SEL7` writer - Pin 7 alternate function selected"]
pub use Sel0W as Sel7W;
impl R {
    #[doc = "Bits 0:3 - Pin 0 alternate function selected"]
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 1 alternate function selected"]
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 2 alternate function selected"]
    #[inline(always)]
    pub fn sel2(&self) -> Sel2R {
        Sel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 3 alternate function selected"]
    #[inline(always)]
    pub fn sel3(&self) -> Sel3R {
        Sel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 4 alternate function selected"]
    #[inline(always)]
    pub fn sel4(&self) -> Sel4R {
        Sel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 5 alternate function selected"]
    #[inline(always)]
    pub fn sel5(&self) -> Sel5R {
        Sel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 6 alternate function selected"]
    #[inline(always)]
    pub fn sel6(&self) -> Sel6R {
        Sel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 7 alternate function selected"]
    #[inline(always)]
    pub fn sel7(&self) -> Sel7R {
        Sel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 0 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> Sel0W<Afsel0Spec> {
        Sel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Pin 1 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> Sel1W<Afsel0Spec> {
        Sel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Pin 2 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> Sel2W<Afsel0Spec> {
        Sel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Pin 3 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> Sel3W<Afsel0Spec> {
        Sel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Pin 4 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> Sel4W<Afsel0Spec> {
        Sel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Pin 5 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel5(&mut self) -> Sel5W<Afsel0Spec> {
        Sel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Pin 6 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> Sel6W<Afsel0Spec> {
        Sel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Pin 7 alternate function selected"]
    #[inline(always)]
    #[must_use]
    pub fn sel7(&mut self) -> Sel7W<Afsel0Spec> {
        Sel7W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afsel0Spec;
impl crate::RegisterSpec for Afsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsel0::R`](R) reader structure"]
impl crate::Readable for Afsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`afsel0::W`](W) writer structure"]
impl crate::Writable for Afsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFSEL0 to value 0"]
impl crate::Resettable for Afsel0Spec {
    const RESET_VALUE: u32 = 0;
}
