#[doc = "Register `SSTAT` reader"]
pub type R = crate::R<SstatSpec>;
#[doc = "Field `WDE0` reader - This bit equals to the WDE bit of ADC0"]
pub type Wde0R = crate::BitReader;
#[doc = "Field `EOC0` reader - This bit equals to the EOC bit of ADC0"]
pub type Eoc0R = crate::BitReader;
#[doc = "Field `EOIC0` reader - This bit equals to the EOIC bit of ADC0"]
pub type Eoic0R = crate::BitReader;
#[doc = "Field `STIC0` reader - This bit equals to the STIC bit of ADC0"]
pub type Stic0R = crate::BitReader;
#[doc = "Field `STRC0` reader - This bit equals to the STRC bit of ADC0"]
pub type Strc0R = crate::BitReader;
#[doc = "Field `ROVF0` reader - This bit equals to the ROVF bit of ADC0"]
pub type Rovf0R = crate::BitReader;
#[doc = "Field `WDE1` reader - This bit equals to the WDE bit of ADC1"]
pub type Wde1R = crate::BitReader;
#[doc = "Field `EOC1` reader - This bit equals to the EOC bit of ADC1"]
pub type Eoc1R = crate::BitReader;
#[doc = "Field `EOIC1` reader - This bit equals to the EOIC bit of ADC1"]
pub type Eoic1R = crate::BitReader;
#[doc = "Field `STIC1` reader - This bit equals to the STIC bit of ADC1"]
pub type Stic1R = crate::BitReader;
#[doc = "Field `STRC1` reader - This bit equals to the STRC bit of ADC1"]
pub type Strc1R = crate::BitReader;
#[doc = "Field `ROVF1` reader - This bit equals to the ROVF bit of ADC1"]
pub type Rovf1R = crate::BitReader;
#[doc = "Field `WDE2` reader - This bit equals to the WDE bit of ADC2"]
pub type Wde2R = crate::BitReader;
#[doc = "Field `EOC2` reader - This bit equals to the EOC bit of ADC2"]
pub type Eoc2R = crate::BitReader;
#[doc = "Field `EOIC2` reader - This bit equals to the EOIC bit of ADC2"]
pub type Eoic2R = crate::BitReader;
#[doc = "Field `STIC2` reader - This bit equals to the STIC bit of ADC2"]
pub type Stic2R = crate::BitReader;
#[doc = "Field `STRC2` reader - This bit equals to the STRC bit of ADC2"]
pub type Strc2R = crate::BitReader;
#[doc = "Field `ROVF2` reader - This bit equals to the ROVF bit of ADC2"]
pub type Rovf2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit equals to the WDE bit of ADC0"]
    #[inline(always)]
    pub fn wde0(&self) -> Wde0R {
        Wde0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit equals to the EOC bit of ADC0"]
    #[inline(always)]
    pub fn eoc0(&self) -> Eoc0R {
        Eoc0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit equals to the EOIC bit of ADC0"]
    #[inline(always)]
    pub fn eoic0(&self) -> Eoic0R {
        Eoic0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit equals to the STIC bit of ADC0"]
    #[inline(always)]
    pub fn stic0(&self) -> Stic0R {
        Stic0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit equals to the STRC bit of ADC0"]
    #[inline(always)]
    pub fn strc0(&self) -> Strc0R {
        Strc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit equals to the ROVF bit of ADC0"]
    #[inline(always)]
    pub fn rovf0(&self) -> Rovf0R {
        Rovf0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit equals to the WDE bit of ADC1"]
    #[inline(always)]
    pub fn wde1(&self) -> Wde1R {
        Wde1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit equals to the EOC bit of ADC1"]
    #[inline(always)]
    pub fn eoc1(&self) -> Eoc1R {
        Eoc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit equals to the EOIC bit of ADC1"]
    #[inline(always)]
    pub fn eoic1(&self) -> Eoic1R {
        Eoic1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit equals to the STIC bit of ADC1"]
    #[inline(always)]
    pub fn stic1(&self) -> Stic1R {
        Stic1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit equals to the STRC bit of ADC1"]
    #[inline(always)]
    pub fn strc1(&self) -> Strc1R {
        Strc1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit equals to the ROVF bit of ADC1"]
    #[inline(always)]
    pub fn rovf1(&self) -> Rovf1R {
        Rovf1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit equals to the WDE bit of ADC2"]
    #[inline(always)]
    pub fn wde2(&self) -> Wde2R {
        Wde2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit equals to the EOC bit of ADC2"]
    #[inline(always)]
    pub fn eoc2(&self) -> Eoc2R {
        Eoc2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit equals to the EOIC bit of ADC2"]
    #[inline(always)]
    pub fn eoic2(&self) -> Eoic2R {
        Eoic2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit equals to the STIC bit of ADC2"]
    #[inline(always)]
    pub fn stic2(&self) -> Stic2R {
        Stic2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit equals to the STRC bit of ADC2"]
    #[inline(always)]
    pub fn strc2(&self) -> Strc2R {
        Strc2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit equals to the ROVF bit of ADC2"]
    #[inline(always)]
    pub fn rovf2(&self) -> Rovf2R {
        Rovf2R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "summary status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstatSpec;
impl crate::RegisterSpec for SstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstat::R`](R) reader structure"]
impl crate::Readable for SstatSpec {}
#[doc = "`reset()` method sets SSTAT to value 0"]
impl crate::Resettable for SstatSpec {
    const RESET_VALUE: u32 = 0;
}
