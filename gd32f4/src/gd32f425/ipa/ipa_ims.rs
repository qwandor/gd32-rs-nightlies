#[doc = "Register `IPA_IMS` reader"]
pub type R = crate::R<IpaImsSpec>;
#[doc = "Register `IPA_IMS` writer"]
pub type W = crate::W<IpaImsSpec>;
#[doc = "Field `HEIGHT` reader - Height of the image to be processed"]
pub type HeightR = crate::FieldReader<u16>;
#[doc = "Field `HEIGHT` writer - Height of the image to be processed"]
pub type HeightW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WIDTH` reader - Width of the image to be processed"]
pub type WidthR = crate::FieldReader<u16>;
#[doc = "Field `WIDTH` writer - Width of the image to be processed"]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:15 - Height of the image to be processed"]
    #[inline(always)]
    pub fn height(&self) -> HeightR {
        HeightR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Width of the image to be processed"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Height of the image to be processed"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HeightW<IpaImsSpec> {
        HeightW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Width of the image to be processed"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WidthW<IpaImsSpec> {
        WidthW::new(self, 16)
    }
}
#[doc = "Image size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipa_ims::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipa_ims::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaImsSpec;
impl crate::RegisterSpec for IpaImsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_ims::R`](R) reader structure"]
impl crate::Readable for IpaImsSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_ims::W`](W) writer structure"]
impl crate::Writable for IpaImsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPA_IMS to value 0"]
impl crate::Resettable for IpaImsSpec {
    const RESET_VALUE: u32 = 0;
}
