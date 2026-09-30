#[doc = "Register `SRAM` reader"]
pub type R = crate::R<SramSpec>;
#[doc = "Register `SRAM` writer"]
pub type W = crate::W<SramSpec>;
#[doc = "Field `extacc` reader - SRAM extended access time enable."]
pub type ExtaccR = crate::BitReader;
#[doc = "Field `extacc` writer - SRAM extended access time enable."]
pub type ExtaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rmargin_en` reader - SRAM read margin enable."]
pub type RmarginEnR = crate::BitReader;
#[doc = "Field `rmargin_en` writer - SRAM read margin enable."]
pub type RmarginEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rmargin` reader - SRAM read margin."]
pub type RmarginR = crate::FieldReader;
#[doc = "Field `rmargin` writer - SRAM read margin."]
pub type RmarginW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ra` reader - Read assist voltage."]
pub type RaR = crate::FieldReader;
#[doc = "Field `ra` writer - Read assist voltage."]
pub type RaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wneg_vol` reader - Write negative voltage."]
pub type WnegVolR = crate::FieldReader;
#[doc = "Field `wneg_vol` writer - Write negative voltage."]
pub type WnegVolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wneg_en` reader - Write negative voltage enable."]
pub type WnegEnR = crate::BitReader;
#[doc = "Field `wneg_en` writer - Write negative voltage enable."]
pub type WnegEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpulse` reader - Write pulse width."]
pub type WpulseR = crate::FieldReader;
#[doc = "Field `wpulse` writer - Write pulse width."]
pub type WpulseW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ds` reader - Memory deep sleep enable."]
pub type DsR = crate::BitReader;
#[doc = "Field `ds` writer - Memory deep sleep enable."]
pub type DsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pd` reader - Memory power down enable."]
pub type PdR = crate::BitReader;
#[doc = "Field `pd` writer - Memory power down enable."]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lsdram` reader - Data RAM light sleep."]
pub type LsdramR = crate::BitReader;
#[doc = "Field `lsdram` writer - Data RAM light sleep."]
pub type LsdramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lsmram` reader - MRAM light sleep."]
pub type LsmramR = crate::BitReader;
#[doc = "Field `lsmram` writer - MRAM light sleep."]
pub type LsmramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lstram` reader - TRAM light sleep."]
pub type LstramR = crate::BitReader;
#[doc = "Field `lstram` writer - TRAM light sleep."]
pub type LstramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lsbram` reader - Bias memory light sleep."]
pub type LsbramR = crate::BitReader;
#[doc = "Field `lsbram` writer - Bias memory light sleep."]
pub type LsbramW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM extended access time enable."]
    #[inline(always)]
    pub fn extacc(&self) -> ExtaccR {
        ExtaccR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM read margin enable."]
    #[inline(always)]
    pub fn rmargin_en(&self) -> RmarginEnR {
        RmarginEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - SRAM read margin."]
    #[inline(always)]
    pub fn rmargin(&self) -> RmarginR {
        RmarginR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Read assist voltage."]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Write negative voltage."]
    #[inline(always)]
    pub fn wneg_vol(&self) -> WnegVolR {
        WnegVolR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Write negative voltage enable."]
    #[inline(always)]
    pub fn wneg_en(&self) -> WnegEnR {
        WnegEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Write pulse width."]
    #[inline(always)]
    pub fn wpulse(&self) -> WpulseR {
        WpulseR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 15 - Memory deep sleep enable."]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Memory power down enable."]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Data RAM light sleep."]
    #[inline(always)]
    pub fn lsdram(&self) -> LsdramR {
        LsdramR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MRAM light sleep."]
    #[inline(always)]
    pub fn lsmram(&self) -> LsmramR {
        LsmramR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TRAM light sleep."]
    #[inline(always)]
    pub fn lstram(&self) -> LstramR {
        LstramR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bias memory light sleep."]
    #[inline(always)]
    pub fn lsbram(&self) -> LsbramR {
        LsbramR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM extended access time enable."]
    #[inline(always)]
    pub fn extacc(&mut self) -> ExtaccW<SramSpec> {
        ExtaccW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM read margin enable."]
    #[inline(always)]
    pub fn rmargin_en(&mut self) -> RmarginEnW<SramSpec> {
        RmarginEnW::new(self, 1)
    }
    #[doc = "Bits 2:5 - SRAM read margin."]
    #[inline(always)]
    pub fn rmargin(&mut self) -> RmarginW<SramSpec> {
        RmarginW::new(self, 2)
    }
    #[doc = "Bits 6:7 - Read assist voltage."]
    #[inline(always)]
    pub fn ra(&mut self) -> RaW<SramSpec> {
        RaW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Write negative voltage."]
    #[inline(always)]
    pub fn wneg_vol(&mut self) -> WnegVolW<SramSpec> {
        WnegVolW::new(self, 8)
    }
    #[doc = "Bit 10 - Write negative voltage enable."]
    #[inline(always)]
    pub fn wneg_en(&mut self) -> WnegEnW<SramSpec> {
        WnegEnW::new(self, 10)
    }
    #[doc = "Bits 11:13 - Write pulse width."]
    #[inline(always)]
    pub fn wpulse(&mut self) -> WpulseW<SramSpec> {
        WpulseW::new(self, 11)
    }
    #[doc = "Bit 15 - Memory deep sleep enable."]
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<SramSpec> {
        DsW::new(self, 15)
    }
    #[doc = "Bit 16 - Memory power down enable."]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<SramSpec> {
        PdW::new(self, 16)
    }
    #[doc = "Bit 19 - Data RAM light sleep."]
    #[inline(always)]
    pub fn lsdram(&mut self) -> LsdramW<SramSpec> {
        LsdramW::new(self, 19)
    }
    #[doc = "Bit 20 - MRAM light sleep."]
    #[inline(always)]
    pub fn lsmram(&mut self) -> LsmramW<SramSpec> {
        LsmramW::new(self, 20)
    }
    #[doc = "Bit 21 - TRAM light sleep."]
    #[inline(always)]
    pub fn lstram(&mut self) -> LstramW<SramSpec> {
        LstramW::new(self, 21)
    }
    #[doc = "Bit 22 - Bias memory light sleep."]
    #[inline(always)]
    pub fn lsbram(&mut self) -> LsbramW<SramSpec> {
        LsbramW::new(self, 22)
    }
}
#[doc = "SRAM control.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramSpec;
impl crate::RegisterSpec for SramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram::R`](R) reader structure"]
impl crate::Readable for SramSpec {}
#[doc = "`write(|w| ..)` method takes [`sram::W`](W) writer structure"]
impl crate::Writable for SramSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM to value 0x0e"]
impl crate::Resettable for SramSpec {
    const RESET_VALUE: u32 = 0x0e;
}
