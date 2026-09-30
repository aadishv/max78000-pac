#[doc = "Register `LCNT` reader"]
pub type R = crate::R<LcntSpec>;
#[doc = "Register `LCNT` writer"]
pub type W = crate::W<LcntSpec>;
#[doc = "Field `last` reader - Index of the last layer to execute."]
pub type LastR = crate::FieldReader;
#[doc = "Field `last` writer - Index of the last layer to execute."]
pub type LastW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Index of the last layer to execute."]
    #[inline(always)]
    pub fn last(&self) -> LastR {
        LastR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Index of the last layer to execute."]
    #[inline(always)]
    pub fn last(&mut self) -> LastW<LcntSpec> {
        LastW::new(self, 0)
    }
}
#[doc = "Layer count maximum. Processing always starts at layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcntSpec;
impl crate::RegisterSpec for LcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcnt::R`](R) reader structure"]
impl crate::Readable for LcntSpec {}
#[doc = "`write(|w| ..)` method takes [`lcnt::W`](W) writer structure"]
impl crate::Writable for LcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCNT to value 0"]
impl crate::Resettable for LcntSpec {
    const RESET_VALUE: u32 = 0;
}
