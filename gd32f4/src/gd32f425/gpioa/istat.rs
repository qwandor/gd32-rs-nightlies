#[doc = "Register `ISTAT` reader"]
pub type R = crate::R<IstatSpec>;
#[doc = "Field `ISTAT0` reader - Port input status (y = 0)"]
pub type Istat0R = crate::BitReader;
#[doc = "Field `ISTAT1` reader - Port input status (y = 1)"]
pub type Istat1R = crate::BitReader;
#[doc = "Field `ISTAT2` reader - Port input status (y = 2)"]
pub type Istat2R = crate::BitReader;
#[doc = "Field `ISTAT3` reader - Port input status (y = 3)"]
pub type Istat3R = crate::BitReader;
#[doc = "Field `ISTAT4` reader - Port input status (y = 4)"]
pub type Istat4R = crate::BitReader;
#[doc = "Field `ISTAT5` reader - Port input status (y = 5)"]
pub type Istat5R = crate::BitReader;
#[doc = "Field `ISTAT6` reader - Port input status (y = 6)"]
pub type Istat6R = crate::BitReader;
#[doc = "Field `ISTAT7` reader - Port input status (y = 7)"]
pub type Istat7R = crate::BitReader;
#[doc = "Field `ISTAT8` reader - Port input status (y = 8)"]
pub type Istat8R = crate::BitReader;
#[doc = "Field `ISTAT9` reader - Port input status (y = 9)"]
pub type Istat9R = crate::BitReader;
#[doc = "Field `ISTAT10` reader - Port input status (y = 10)"]
pub type Istat10R = crate::BitReader;
#[doc = "Field `ISTAT11` reader - Port input status (y = 11)"]
pub type Istat11R = crate::BitReader;
#[doc = "Field `ISTAT12` reader - Port input status (y = 12)"]
pub type Istat12R = crate::BitReader;
#[doc = "Field `ISTAT13` reader - Port input status (y = 13)"]
pub type Istat13R = crate::BitReader;
#[doc = "Field `ISTAT14` reader - Port input status (y = 14)"]
pub type Istat14R = crate::BitReader;
#[doc = "Field `ISTAT15` reader - Port input status (y = 15)"]
pub type Istat15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Port input status (y = 0)"]
    #[inline(always)]
    pub fn istat0(&self) -> Istat0R {
        Istat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input status (y = 1)"]
    #[inline(always)]
    pub fn istat1(&self) -> Istat1R {
        Istat1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input status (y = 2)"]
    #[inline(always)]
    pub fn istat2(&self) -> Istat2R {
        Istat2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input status (y = 3)"]
    #[inline(always)]
    pub fn istat3(&self) -> Istat3R {
        Istat3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input status (y = 4)"]
    #[inline(always)]
    pub fn istat4(&self) -> Istat4R {
        Istat4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input status (y = 5)"]
    #[inline(always)]
    pub fn istat5(&self) -> Istat5R {
        Istat5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input status (y = 6)"]
    #[inline(always)]
    pub fn istat6(&self) -> Istat6R {
        Istat6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input status (y = 7)"]
    #[inline(always)]
    pub fn istat7(&self) -> Istat7R {
        Istat7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input status (y = 8)"]
    #[inline(always)]
    pub fn istat8(&self) -> Istat8R {
        Istat8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input status (y = 9)"]
    #[inline(always)]
    pub fn istat9(&self) -> Istat9R {
        Istat9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input status (y = 10)"]
    #[inline(always)]
    pub fn istat10(&self) -> Istat10R {
        Istat10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input status (y = 11)"]
    #[inline(always)]
    pub fn istat11(&self) -> Istat11R {
        Istat11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input status (y = 12)"]
    #[inline(always)]
    pub fn istat12(&self) -> Istat12R {
        Istat12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input status (y = 13)"]
    #[inline(always)]
    pub fn istat13(&self) -> Istat13R {
        Istat13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input status (y = 14)"]
    #[inline(always)]
    pub fn istat14(&self) -> Istat14R {
        Istat14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input status (y = 15)"]
    #[inline(always)]
    pub fn istat15(&self) -> Istat15R {
        Istat15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "GPIO port input status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstatSpec;
impl crate::RegisterSpec for IstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istat::R`](R) reader structure"]
impl crate::Readable for IstatSpec {}
#[doc = "`reset()` method sets ISTAT to value 0"]
impl crate::Resettable for IstatSpec {
    const RESET_VALUE: u32 = 0;
}
