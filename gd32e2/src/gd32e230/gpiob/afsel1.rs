#[doc = "Register `AFSEL1` reader"]
pub type R = crate::R<Afsel1Spec>;
#[doc = "Register `AFSEL1` writer"]
pub type W = crate::W<Afsel1Spec>;
#[doc = "Alternate function selection for port x bit y (y = 8..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel8 {
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
impl From<Sel8> for u8 {
    #[inline(always)]
    fn from(variant: Sel8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel8 {
    type Ux = u8;
}
#[doc = "Field `SEL8` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type Sel8R = crate::FieldReader<Sel8>;
impl Sel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel8> {
        match self.bits {
            0 => Some(Sel8::Af0),
            1 => Some(Sel8::Af1),
            2 => Some(Sel8::Af2),
            3 => Some(Sel8::Af3),
            4 => Some(Sel8::Af4),
            5 => Some(Sel8::Af5),
            6 => Some(Sel8::Af6),
            7 => Some(Sel8::Af7),
            9 => Some(Sel8::Af9),
            11 => Some(Sel8::Af11),
            _ => None,
        }
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == Sel8::Af0
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == Sel8::Af1
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == Sel8::Af2
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == Sel8::Af3
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == Sel8::Af4
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == Sel8::Af5
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == Sel8::Af6
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == Sel8::Af7
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == Sel8::Af9
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == Sel8::Af11
    }
}
#[doc = "Field `SEL8` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type Sel8W<'a, REG> = crate::FieldWriter<'a, REG, 4, Sel8>;
impl<'a, REG> Sel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af7)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af9)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut crate::W<REG> {
        self.variant(Sel8::Af11)
    }
}
#[doc = "Field `SEL9` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8R as Sel9R;
#[doc = "Field `SEL10` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8R as Sel10R;
#[doc = "Field `SEL11` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8R as Sel11R;
#[doc = "Field `SEL12` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8R as Sel12R;
#[doc = "Field `SEL13` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8R as Sel13R;
#[doc = "Field `SEL14` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8R as Sel14R;
#[doc = "Field `SEL15` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8R as Sel15R;
#[doc = "Field `SEL9` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8W as Sel9W;
#[doc = "Field `SEL10` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8W as Sel10W;
#[doc = "Field `SEL11` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8W as Sel11W;
#[doc = "Field `SEL12` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8W as Sel12W;
#[doc = "Field `SEL13` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8W as Sel13W;
#[doc = "Field `SEL14` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8W as Sel14W;
#[doc = "Field `SEL15` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub use Sel8W as Sel15W;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel8(&self) -> Sel8R {
        Sel8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel9(&self) -> Sel9R {
        Sel9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel10(&self) -> Sel10R {
        Sel10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel11(&self) -> Sel11R {
        Sel11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel12(&self) -> Sel12R {
        Sel12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel13(&self) -> Sel13R {
        Sel13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel14(&self) -> Sel14R {
        Sel14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn sel15(&self) -> Sel15R {
        Sel15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> Sel8W<Afsel1Spec> {
        Sel8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn sel9(&mut self) -> Sel9W<Afsel1Spec> {
        Sel9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn sel10(&mut self) -> Sel10W<Afsel1Spec> {
        Sel10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn sel11(&mut self) -> Sel11W<Afsel1Spec> {
        Sel11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> Sel12W<Afsel1Spec> {
        Sel12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn sel13(&mut self) -> Sel13W<Afsel1Spec> {
        Sel13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn sel14(&mut self) -> Sel14W<Afsel1Spec> {
        Sel14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn sel15(&mut self) -> Sel15W<Afsel1Spec> {
        Sel15W::new(self, 28)
    }
}
#[doc = "GPIO alternate function register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afsel1Spec;
impl crate::RegisterSpec for Afsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsel1::R`](R) reader structure"]
impl crate::Readable for Afsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`afsel1::W`](W) writer structure"]
impl crate::Writable for Afsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFSEL1 to value 0"]
impl crate::Resettable for Afsel1Spec {
    const RESET_VALUE: u32 = 0;
}
