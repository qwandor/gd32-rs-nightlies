#[doc = "Register `CCHP` reader"]
pub type R = crate::R<CchpSpec>;
#[doc = "Register `CCHP` writer"]
pub type W = crate::W<CchpSpec>;
#[doc = "Field `DTCFG` reader - Dead-time generator setup"]
pub type DtcfgR = crate::FieldReader;
#[doc = "Field `DTCFG` writer - Dead-time generator setup"]
pub type DtcfgW<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Break enable"]
pub use crate::gd32e231::timer0::cchp::Brken;
#[doc = "Field `BRKEN` reader - Break enable"]
pub use crate::gd32e231::timer0::cchp::BrkenR;
#[doc = "Field `BRKEN` writer - Break enable"]
pub use crate::gd32e231::timer0::cchp::BrkenW;
#[doc = "Break polarity"]
pub use crate::gd32e231::timer0::cchp::Brkp;
#[doc = "Field `BRKP` reader - Break polarity"]
pub use crate::gd32e231::timer0::cchp::BrkpR;
#[doc = "Field `BRKP` writer - Break polarity"]
pub use crate::gd32e231::timer0::cchp::BrkpW;
#[doc = "Off-state selection for Idle mode"]
pub use crate::gd32e231::timer0::cchp::Ios;
#[doc = "Field `IOS` reader - Off-state selection for Idle mode"]
pub use crate::gd32e231::timer0::cchp::IosR;
#[doc = "Field `IOS` writer - Off-state selection for Idle mode"]
pub use crate::gd32e231::timer0::cchp::IosW;
#[doc = "Automatic output enable"]
pub use crate::gd32e231::timer0::cchp::Oaen;
#[doc = "Field `OAEN` reader - Automatic output enable"]
pub use crate::gd32e231::timer0::cchp::OaenR;
#[doc = "Field `OAEN` writer - Automatic output enable"]
pub use crate::gd32e231::timer0::cchp::OaenW;
#[doc = "Main output enable"]
pub use crate::gd32e231::timer0::cchp::Poen;
#[doc = "Field `POEN` reader - Main output enable"]
pub use crate::gd32e231::timer0::cchp::PoenR;
#[doc = "Field `POEN` writer - Main output enable"]
pub use crate::gd32e231::timer0::cchp::PoenW;
#[doc = "complementary register protect control"]
pub use crate::gd32e231::timer0::cchp::Prot;
#[doc = "Field `PROT` reader - complementary register protect control"]
pub use crate::gd32e231::timer0::cchp::ProtR;
#[doc = "Field `PROT` writer - complementary register protect control"]
pub use crate::gd32e231::timer0::cchp::ProtW;
#[doc = "Off-state selection for Run mode"]
pub use crate::gd32e231::timer0::cchp::Ros;
#[doc = "Field `ROS` reader - Off-state selection for Run mode"]
pub use crate::gd32e231::timer0::cchp::RosR;
#[doc = "Field `ROS` writer - Off-state selection for Run mode"]
pub use crate::gd32e231::timer0::cchp::RosW;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtcfg(&self) -> DtcfgR {
        DtcfgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - complementary register protect control"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ios(&self) -> IosR {
        IosR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ros(&self) -> RosR {
        RosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn brken(&self) -> BrkenR {
        BrkenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn brkp(&self) -> BrkpR {
        BrkpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn oaen(&self) -> OaenR {
        OaenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn poen(&self) -> PoenR {
        PoenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    #[must_use]
    pub fn dtcfg(&mut self) -> DtcfgW<CchpSpec> {
        DtcfgW::new(self, 0)
    }
    #[doc = "Bits 8:9 - complementary register protect control"]
    #[inline(always)]
    #[must_use]
    pub fn prot(&mut self) -> ProtW<CchpSpec> {
        ProtW::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn ios(&mut self) -> IosW<CchpSpec> {
        IosW::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    #[must_use]
    pub fn ros(&mut self) -> RosW<CchpSpec> {
        RosW::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    #[must_use]
    pub fn brken(&mut self) -> BrkenW<CchpSpec> {
        BrkenW::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    #[must_use]
    pub fn brkp(&mut self) -> BrkpW<CchpSpec> {
        BrkpW::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn oaen(&mut self) -> OaenW<CchpSpec> {
        OaenW::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen(&mut self) -> PoenW<CchpSpec> {
        PoenW::new(self, 15)
    }
}
#[doc = "break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cchp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cchp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CchpSpec;
impl crate::RegisterSpec for CchpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cchp::R`](R) reader structure"]
impl crate::Readable for CchpSpec {}
#[doc = "`write(|w| ..)` method takes [`cchp::W`](W) writer structure"]
impl crate::Writable for CchpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCHP to value 0"]
impl crate::Resettable for CchpSpec {
    const RESET_VALUE: u32 = 0;
}
