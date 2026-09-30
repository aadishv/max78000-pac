#[doc = "Register `AON` reader"]
pub type R = crate::R<AonSpec>;
#[doc = "Register `AON` writer"]
pub type W = crate::W<AonSpec>;
#[doc = "Field `rdy_sel` reader - APB wait state selection."]
pub type RdySelR = crate::FieldReader;
#[doc = "Field `rdy_sel` writer - APB wait state selection."]
pub type RdySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dsleep` reader - MRAM deep sleep, one bit per quadrant. Contents are retained."]
pub type DsleepR = crate::FieldReader;
#[doc = "Field `dsleep` writer - MRAM deep sleep, one bit per quadrant. Contents are retained."]
pub type DsleepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pd` reader - MRAM power down, one bit per quadrant. Contents are lost."]
pub type PdR = crate::FieldReader;
#[doc = "Field `pd` writer - MRAM power down, one bit per quadrant. Contents are lost."]
pub type PdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rm` reader - MRAM read margin MSB, one bit per quadrant."]
pub type RmR = crate::FieldReader;
#[doc = "Field `rm` writer - MRAM read margin MSB, one bit per quadrant."]
pub type RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - APB wait state selection."]
    #[inline(always)]
    pub fn rdy_sel(&self) -> RdySelR {
        RdySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - MRAM deep sleep, one bit per quadrant. Contents are retained."]
    #[inline(always)]
    pub fn dsleep(&self) -> DsleepR {
        DsleepR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - MRAM power down, one bit per quadrant. Contents are lost."]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MRAM read margin MSB, one bit per quadrant."]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - APB wait state selection."]
    #[inline(always)]
    pub fn rdy_sel(&mut self) -> RdySelW<AonSpec> {
        RdySelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - MRAM deep sleep, one bit per quadrant. Contents are retained."]
    #[inline(always)]
    pub fn dsleep(&mut self) -> DsleepW<AonSpec> {
        DsleepW::new(self, 8)
    }
    #[doc = "Bits 12:15 - MRAM power down, one bit per quadrant. Contents are lost."]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<AonSpec> {
        PdW::new(self, 12)
    }
    #[doc = "Bits 16:19 - MRAM read margin MSB, one bit per quadrant."]
    #[inline(always)]
    pub fn rm(&mut self) -> RmW<AonSpec> {
        RmW::new(self, 16)
    }
}
#[doc = "Always-on domain control.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSpec;
impl crate::RegisterSpec for AonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon::R`](R) reader structure"]
impl crate::Readable for AonSpec {}
#[doc = "`write(|w| ..)` method takes [`aon::W`](W) writer structure"]
impl crate::Writable for AonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AON to value 0x03"]
impl crate::Resettable for AonSpec {
    const RESET_VALUE: u32 = 0x03;
}
