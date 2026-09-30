#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `rdy_sel` reader - APB wait state selection."]
pub type RdySelR = crate::FieldReader;
#[doc = "Field `rdy_sel` writer - APB wait state selection."]
pub type RdySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `full_thresh` reader - FIFO almost full threshold."]
pub type FullThreshR = crate::FieldReader;
#[doc = "Field `full_thresh` writer - FIFO almost full threshold."]
pub type FullThreshW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `empty_thresh` reader - FIFO almost empty threshold."]
pub type EmptyThreshR = crate::FieldReader;
#[doc = "Field `empty_thresh` writer - FIFO almost empty threshold."]
pub type EmptyThreshW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `fifo_cpl` reader - FIFO coupling. Forces the FIFOs to operate in lockstep."]
pub type FifoCplR = crate::BitReader;
#[doc = "Field `fifo_cpl` writer - FIFO coupling. Forces the FIFOs to operate in lockstep."]
pub type FifoCplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo_en` reader - Per FIFO enable. Bit n enables the FIFO for quadrant n."]
pub type FifoEnR = crate::FieldReader;
#[doc = "Field `fifo_en` writer - Per FIFO enable. Bit n enables the FIFO for quadrant n."]
pub type FifoEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `full_int_en` reader - FIFO full interrupt enable, one bit per quadrant."]
pub type FullIntEnR = crate::FieldReader;
#[doc = "Field `full_int_en` writer - FIFO full interrupt enable, one bit per quadrant."]
pub type FullIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `empty_int_en` reader - FIFO empty interrupt enable, one bit per quadrant."]
pub type EmptyIntEnR = crate::FieldReader;
#[doc = "Field `empty_int_en` writer - FIFO empty interrupt enable, one bit per quadrant."]
pub type EmptyIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `almost_full_int_en` reader - FIFO almost full interrupt enable, one bit per quadrant."]
pub type AlmostFullIntEnR = crate::FieldReader;
#[doc = "Field `almost_full_int_en` writer - FIFO almost full interrupt enable, one bit per quadrant."]
pub type AlmostFullIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `almost_empty_int_en` reader - FIFO almost empty interrupt enable, one bit per quadrant."]
pub type AlmostEmptyIntEnR = crate::FieldReader;
#[doc = "Field `almost_empty_int_en` writer - FIFO almost empty interrupt enable, one bit per quadrant."]
pub type AlmostEmptyIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - APB wait state selection."]
    #[inline(always)]
    pub fn rdy_sel(&self) -> RdySelR {
        RdySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - FIFO almost full threshold."]
    #[inline(always)]
    pub fn full_thresh(&self) -> FullThreshR {
        FullThreshR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 7:9 - FIFO almost empty threshold."]
    #[inline(always)]
    pub fn empty_thresh(&self) -> EmptyThreshR {
        EmptyThreshR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 11 - FIFO coupling. Forces the FIFOs to operate in lockstep."]
    #[inline(always)]
    pub fn fifo_cpl(&self) -> FifoCplR {
        FifoCplR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Per FIFO enable. Bit n enables the FIFO for quadrant n."]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - FIFO full interrupt enable, one bit per quadrant."]
    #[inline(always)]
    pub fn full_int_en(&self) -> FullIntEnR {
        FullIntEnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - FIFO empty interrupt enable, one bit per quadrant."]
    #[inline(always)]
    pub fn empty_int_en(&self) -> EmptyIntEnR {
        EmptyIntEnR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - FIFO almost full interrupt enable, one bit per quadrant."]
    #[inline(always)]
    pub fn almost_full_int_en(&self) -> AlmostFullIntEnR {
        AlmostFullIntEnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - FIFO almost empty interrupt enable, one bit per quadrant."]
    #[inline(always)]
    pub fn almost_empty_int_en(&self) -> AlmostEmptyIntEnR {
        AlmostEmptyIntEnR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - APB wait state selection."]
    #[inline(always)]
    pub fn rdy_sel(&mut self) -> RdySelW<CtlSpec> {
        RdySelW::new(self, 0)
    }
    #[doc = "Bits 2:4 - FIFO almost full threshold."]
    #[inline(always)]
    pub fn full_thresh(&mut self) -> FullThreshW<CtlSpec> {
        FullThreshW::new(self, 2)
    }
    #[doc = "Bits 7:9 - FIFO almost empty threshold."]
    #[inline(always)]
    pub fn empty_thresh(&mut self) -> EmptyThreshW<CtlSpec> {
        EmptyThreshW::new(self, 7)
    }
    #[doc = "Bit 11 - FIFO coupling. Forces the FIFOs to operate in lockstep."]
    #[inline(always)]
    pub fn fifo_cpl(&mut self) -> FifoCplW<CtlSpec> {
        FifoCplW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Per FIFO enable. Bit n enables the FIFO for quadrant n."]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<CtlSpec> {
        FifoEnW::new(self, 12)
    }
    #[doc = "Bits 16:19 - FIFO full interrupt enable, one bit per quadrant."]
    #[inline(always)]
    pub fn full_int_en(&mut self) -> FullIntEnW<CtlSpec> {
        FullIntEnW::new(self, 16)
    }
    #[doc = "Bits 20:23 - FIFO empty interrupt enable, one bit per quadrant."]
    #[inline(always)]
    pub fn empty_int_en(&mut self) -> EmptyIntEnW<CtlSpec> {
        EmptyIntEnW::new(self, 20)
    }
    #[doc = "Bits 24:27 - FIFO almost full interrupt enable, one bit per quadrant."]
    #[inline(always)]
    pub fn almost_full_int_en(&mut self) -> AlmostFullIntEnW<CtlSpec> {
        AlmostFullIntEnW::new(self, 24)
    }
    #[doc = "Bits 28:31 - FIFO almost empty interrupt enable, one bit per quadrant."]
    #[inline(always)]
    pub fn almost_empty_int_en(&mut self) -> AlmostEmptyIntEnW<CtlSpec> {
        AlmostEmptyIntEnW::new(self, 28)
    }
}
#[doc = "FIFO control.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x03"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x03;
}
