#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `SPI_DATA` reader - Data transfer register"]
pub type SpiDataR = crate::FieldReader<u16>;
#[doc = "Field `SPI_DATA` writer - Data transfer register"]
pub type SpiDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data transfer register"]
    #[inline(always)]
    pub fn spi_data(&self) -> SpiDataR {
        SpiDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data transfer register"]
    #[inline(always)]
    #[must_use]
    pub fn spi_data(&mut self) -> SpiDataW<DataSpec> {
        SpiDataW::new(self, 0)
    }
}
#[doc = "data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0;
}
