#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `en` reader - CNN enable. A 0 to 1 transition starts processing."]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - CNN enable. A 0 to 1 transition starts processing."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rdy_sel` reader - APB wait state selection."]
pub type RdySelR = crate::FieldReader;
#[doc = "Field `rdy_sel` writer - APB wait state selection."]
pub type RdySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clk_en` reader - Data processing clock enable."]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `clk_en` writer - Data processing clock enable."]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `calcmax` reader - Globally enable max pooling when pool_en is set."]
pub type CalcmaxR = crate::BitReader;
#[doc = "Field `calcmax` writer - Globally enable max pooling when pool_en is set."]
pub type CalcmaxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pool_en` reader - Globally enable pooling for all layers."]
pub type PoolEnR = crate::BitReader;
#[doc = "Field `pool_en` writer - Globally enable pooling for all layers."]
pub type PoolEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bigdata` reader - Four data bytes per read for each group of four processors."]
pub type BigdataR = crate::BitReader;
#[doc = "Field `bigdata` writer - Four data bytes per read for each group of four processors."]
pub type BigdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `apbclkena` reader - Keep the APB clock always on."]
pub type ApbclkenaR = crate::BitReader;
#[doc = "Field `apbclkena` writer - Keep the APB clock always on."]
pub type ApbclkenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `oneshot` reader - One-shot layer mode."]
pub type OneshotR = crate::BitReader;
#[doc = "Field `oneshot` writer - One-shot layer mode."]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ext_sync` reader - External sync select. Bit n syncs to quadrant n."]
pub type ExtSyncR = crate::FieldReader;
#[doc = "Field `ext_sync` writer - External sync select. Bit n syncs to quadrant n."]
pub type ExtSyncW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `irq` reader - Completion flag. Write zero to acknowledge."]
pub type IrqR = crate::BitReader;
#[doc = "Field `irq` writer - Completion flag. Write zero to acknowledge."]
pub type IrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pool_rnd` reader - Average pool rounding."]
pub type PoolRndR = crate::BitReader;
#[doc = "Field `pool_rnd` writer - Average pool rounding."]
pub type PoolRndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stream_en` reader - Streaming mode enable."]
pub type StreamEnR = crate::BitReader;
#[doc = "Field `stream_en` writer - Streaming mode enable."]
pub type StreamEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo_en` reader - Take the input layer's data from the CNN FIFO."]
pub type FifoEnR = crate::BitReader;
#[doc = "Field `fifo_en` writer - Take the input layer's data from the CNN FIFO."]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mlat_ld` reader - Mlator load data."]
pub type MlatLdR = crate::BitReader;
#[doc = "Field `mlat_ld` writer - Mlator load data."]
pub type MlatLdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mlat_sel` reader - Mlator packed channel select."]
pub type MlatSelR = crate::FieldReader;
#[doc = "Field `mlat_sel` writer - Mlator packed channel select."]
pub type MlatSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lilbuf` reader - Stream mode circular buffer enable."]
pub type LilbufR = crate::BitReader;
#[doc = "Field `lilbuf` writer - Stream mode circular buffer enable."]
pub type LilbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mexpress` reader - Load the mask memories using packed data."]
pub type MexpressR = crate::BitReader;
#[doc = "Field `mexpress` writer - Load the mask memories using packed data."]
pub type MexpressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `simple1b` reader - Simple 1-bit weight mode."]
pub type Simple1bR = crate::BitReader;
#[doc = "Field `simple1b` writer - Simple 1-bit weight mode."]
pub type Simple1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fast_fifo` reader - Fast FIFO enable."]
pub type FastFifoR = crate::BitReader;
#[doc = "Field `fast_fifo` writer - Fast FIFO enable."]
pub type FastFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo_group` reader - FIFO group output."]
pub type FifoGroupR = crate::BitReader;
#[doc = "Field `fifo_group` writer - FIFO group output."]
pub type FifoGroupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fclk_dly` reader - FIFO clock delay."]
pub type FclkDlyR = crate::FieldReader;
#[doc = "Field `fclk_dly` writer - FIFO clock delay."]
pub type FclkDlyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `timeshft` reader - Add a pooling stage wait state."]
pub type TimeshftR = crate::BitReader;
#[doc = "Field `timeshft` writer - Add a pooling stage wait state."]
pub type TimeshftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qupac` reader - QuPac mode."]
pub type QupacR = crate::BitReader;
#[doc = "Field `qupac` writer - QuPac mode."]
pub type QupacW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNN enable. A 0 to 1 transition starts processing."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - APB wait state selection."]
    #[inline(always)]
    pub fn rdy_sel(&self) -> RdySelR {
        RdySelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Data processing clock enable."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Globally enable max pooling when pool_en is set."]
    #[inline(always)]
    pub fn calcmax(&self) -> CalcmaxR {
        CalcmaxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Globally enable pooling for all layers."]
    #[inline(always)]
    pub fn pool_en(&self) -> PoolEnR {
        PoolEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Four data bytes per read for each group of four processors."]
    #[inline(always)]
    pub fn bigdata(&self) -> BigdataR {
        BigdataR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Keep the APB clock always on."]
    #[inline(always)]
    pub fn apbclkena(&self) -> ApbclkenaR {
        ApbclkenaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - One-shot layer mode."]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - External sync select. Bit n syncs to quadrant n."]
    #[inline(always)]
    pub fn ext_sync(&self) -> ExtSyncR {
        ExtSyncR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Completion flag. Write zero to acknowledge."]
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Average pool rounding."]
    #[inline(always)]
    pub fn pool_rnd(&self) -> PoolRndR {
        PoolRndR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Streaming mode enable."]
    #[inline(always)]
    pub fn stream_en(&self) -> StreamEnR {
        StreamEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Take the input layer's data from the CNN FIFO."]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mlator load data."]
    #[inline(always)]
    pub fn mlat_ld(&self) -> MlatLdR {
        MlatLdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Mlator packed channel select."]
    #[inline(always)]
    pub fn mlat_sel(&self) -> MlatSelR {
        MlatSelR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Stream mode circular buffer enable."]
    #[inline(always)]
    pub fn lilbuf(&self) -> LilbufR {
        LilbufR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Load the mask memories using packed data."]
    #[inline(always)]
    pub fn mexpress(&self) -> MexpressR {
        MexpressR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Simple 1-bit weight mode."]
    #[inline(always)]
    pub fn simple1b(&self) -> Simple1bR {
        Simple1bR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Fast FIFO enable."]
    #[inline(always)]
    pub fn fast_fifo(&self) -> FastFifoR {
        FastFifoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - FIFO group output."]
    #[inline(always)]
    pub fn fifo_group(&self) -> FifoGroupR {
        FifoGroupR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - FIFO clock delay."]
    #[inline(always)]
    pub fn fclk_dly(&self) -> FclkDlyR {
        FclkDlyR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Add a pooling stage wait state."]
    #[inline(always)]
    pub fn timeshft(&self) -> TimeshftR {
        TimeshftR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - QuPac mode."]
    #[inline(always)]
    pub fn qupac(&self) -> QupacR {
        QupacR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNN enable. A 0 to 1 transition starts processing."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - APB wait state selection."]
    #[inline(always)]
    pub fn rdy_sel(&mut self) -> RdySelW<CtlSpec> {
        RdySelW::new(self, 1)
    }
    #[doc = "Bit 3 - Data processing clock enable."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<CtlSpec> {
        ClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Globally enable max pooling when pool_en is set."]
    #[inline(always)]
    pub fn calcmax(&mut self) -> CalcmaxW<CtlSpec> {
        CalcmaxW::new(self, 4)
    }
    #[doc = "Bit 5 - Globally enable pooling for all layers."]
    #[inline(always)]
    pub fn pool_en(&mut self) -> PoolEnW<CtlSpec> {
        PoolEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Four data bytes per read for each group of four processors."]
    #[inline(always)]
    pub fn bigdata(&mut self) -> BigdataW<CtlSpec> {
        BigdataW::new(self, 6)
    }
    #[doc = "Bit 7 - Keep the APB clock always on."]
    #[inline(always)]
    pub fn apbclkena(&mut self) -> ApbclkenaW<CtlSpec> {
        ApbclkenaW::new(self, 7)
    }
    #[doc = "Bit 8 - One-shot layer mode."]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<CtlSpec> {
        OneshotW::new(self, 8)
    }
    #[doc = "Bits 9:11 - External sync select. Bit n syncs to quadrant n."]
    #[inline(always)]
    pub fn ext_sync(&mut self) -> ExtSyncW<CtlSpec> {
        ExtSyncW::new(self, 9)
    }
    #[doc = "Bit 12 - Completion flag. Write zero to acknowledge."]
    #[inline(always)]
    pub fn irq(&mut self) -> IrqW<CtlSpec> {
        IrqW::new(self, 12)
    }
    #[doc = "Bit 13 - Average pool rounding."]
    #[inline(always)]
    pub fn pool_rnd(&mut self) -> PoolRndW<CtlSpec> {
        PoolRndW::new(self, 13)
    }
    #[doc = "Bit 14 - Streaming mode enable."]
    #[inline(always)]
    pub fn stream_en(&mut self) -> StreamEnW<CtlSpec> {
        StreamEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Take the input layer's data from the CNN FIFO."]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<CtlSpec> {
        FifoEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Mlator load data."]
    #[inline(always)]
    pub fn mlat_ld(&mut self) -> MlatLdW<CtlSpec> {
        MlatLdW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Mlator packed channel select."]
    #[inline(always)]
    pub fn mlat_sel(&mut self) -> MlatSelW<CtlSpec> {
        MlatSelW::new(self, 17)
    }
    #[doc = "Bit 19 - Stream mode circular buffer enable."]
    #[inline(always)]
    pub fn lilbuf(&mut self) -> LilbufW<CtlSpec> {
        LilbufW::new(self, 19)
    }
    #[doc = "Bit 20 - Load the mask memories using packed data."]
    #[inline(always)]
    pub fn mexpress(&mut self) -> MexpressW<CtlSpec> {
        MexpressW::new(self, 20)
    }
    #[doc = "Bit 21 - Simple 1-bit weight mode."]
    #[inline(always)]
    pub fn simple1b(&mut self) -> Simple1bW<CtlSpec> {
        Simple1bW::new(self, 21)
    }
    #[doc = "Bit 22 - Fast FIFO enable."]
    #[inline(always)]
    pub fn fast_fifo(&mut self) -> FastFifoW<CtlSpec> {
        FastFifoW::new(self, 22)
    }
    #[doc = "Bit 23 - FIFO group output."]
    #[inline(always)]
    pub fn fifo_group(&mut self) -> FifoGroupW<CtlSpec> {
        FifoGroupW::new(self, 23)
    }
    #[doc = "Bits 24:29 - FIFO clock delay."]
    #[inline(always)]
    pub fn fclk_dly(&mut self) -> FclkDlyW<CtlSpec> {
        FclkDlyW::new(self, 24)
    }
    #[doc = "Bit 30 - Add a pooling stage wait state."]
    #[inline(always)]
    pub fn timeshft(&mut self) -> TimeshftW<CtlSpec> {
        TimeshftW::new(self, 30)
    }
    #[doc = "Bit 31 - QuPac mode."]
    #[inline(always)]
    pub fn qupac(&mut self) -> QupacW<CtlSpec> {
        QupacW::new(self, 31)
    }
}
#[doc = "Quadrant control.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0x06"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x06;
}
