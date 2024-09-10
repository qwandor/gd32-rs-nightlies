#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Field `DT0` reader - Pixel Data 0"]
pub type Dt0R = crate::FieldReader;
#[doc = "Field `DT1` reader - Pixel Data 1"]
pub type Dt1R = crate::FieldReader;
#[doc = "Field `DT2` reader - Pixel Data 2"]
pub type Dt2R = crate::FieldReader;
#[doc = "Field `DT3` reader - Pixel Data 3"]
pub type Dt3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Pixel Data 0"]
    #[inline(always)]
    pub fn dt0(&self) -> Dt0R {
        Dt0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pixel Data 1"]
    #[inline(always)]
    pub fn dt1(&self) -> Dt1R {
        Dt1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pixel Data 2"]
    #[inline(always)]
    pub fn dt2(&self) -> Dt2R {
        Dt2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Pixel Data 3"]
    #[inline(always)]
    pub fn dt3(&self) -> Dt3R {
        Dt3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DATA register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0;
}
