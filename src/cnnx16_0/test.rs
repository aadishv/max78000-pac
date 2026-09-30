#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `sbistrun` reader - Run the data SRAM BIST."]
pub type SbistrunR = crate::BitReader;
#[doc = "Field `sbistrun` writer - Run the data SRAM BIST."]
pub type SbistrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sramz` reader - Zeroize the data SRAM."]
pub type SramzR = crate::BitReader;
#[doc = "Field `sramz` writer - Zeroize the data SRAM."]
pub type SramzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbistrun` reader - Run the MRAM BIST."]
pub type MbistrunR = crate::BitReader;
#[doc = "Field `mbistrun` writer - Run the MRAM BIST."]
pub type MbistrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mramz` reader - Zeroize the MRAM."]
pub type MramzR = crate::BitReader;
#[doc = "Field `mramz` writer - Zeroize the MRAM."]
pub type MramzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tbistrun` reader - Run the TRAM BIST."]
pub type TbistrunR = crate::BitReader;
#[doc = "Field `tbistrun` writer - Run the TRAM BIST."]
pub type TbistrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tramz` reader - Zeroize the TRAM."]
pub type TramzR = crate::BitReader;
#[doc = "Field `tramz` writer - Zeroize the TRAM."]
pub type TramzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bbistrun` reader - Run the bias memory BIST."]
pub type BbistrunR = crate::BitReader;
#[doc = "Field `bbistrun` writer - Run the bias memory BIST."]
pub type BbistrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bramz` reader - Zeroize the bias memory."]
pub type BramzR = crate::BitReader;
#[doc = "Field `bramz` writer - Zeroize the bias memory."]
pub type BramzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bistsel` reader - BIST controller status selection."]
pub type BistselR = crate::FieldReader;
#[doc = "Field `bistsel` writer - BIST controller status selection."]
pub type BistselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `sallbfail` reader - Data SRAM BIST failed."]
pub type SallbfailR = crate::BitReader;
#[doc = "Field `mallbfail` reader - MRAM BIST failed."]
pub type MallbfailR = crate::BitReader;
#[doc = "Field `tallbfail` reader - TRAM BIST failed."]
pub type TallbfailR = crate::BitReader;
#[doc = "Field `ballbfail` reader - Bias memory BIST failed."]
pub type BallbfailR = crate::BitReader;
#[doc = "Field `sallbdone` reader - Data SRAM BIST complete."]
pub type SallbdoneR = crate::BitReader;
#[doc = "Field `mallbdone` reader - MRAM BIST complete."]
pub type MallbdoneR = crate::BitReader;
#[doc = "Field `tallbdone` reader - TRAM BIST complete."]
pub type TallbdoneR = crate::BitReader;
#[doc = "Field `ballbdone` reader - Bias memory BIST complete."]
pub type BallbdoneR = crate::BitReader;
#[doc = "Field `sallzdone` reader - Data SRAM zeroization complete."]
pub type SallzdoneR = crate::BitReader;
#[doc = "Field `mallzdone` reader - MRAM zeroization complete."]
pub type MallzdoneR = crate::BitReader;
#[doc = "Field `tallzdone` reader - TRAM zeroization complete."]
pub type TallzdoneR = crate::BitReader;
#[doc = "Field `ballzdone` reader - Bias memory zeroization complete."]
pub type BallzdoneR = crate::BitReader;
#[doc = "Field `bistfail` reader - A BIST run detected a failure."]
pub type BistfailR = crate::BitReader;
#[doc = "Field `bistdone` reader - BIST run complete."]
pub type BistdoneR = crate::BitReader;
#[doc = "Field `zero_done` reader - Zeroization complete."]
pub type ZeroDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Run the data SRAM BIST."]
    #[inline(always)]
    pub fn sbistrun(&self) -> SbistrunR {
        SbistrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Zeroize the data SRAM."]
    #[inline(always)]
    pub fn sramz(&self) -> SramzR {
        SramzR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Run the MRAM BIST."]
    #[inline(always)]
    pub fn mbistrun(&self) -> MbistrunR {
        MbistrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Zeroize the MRAM."]
    #[inline(always)]
    pub fn mramz(&self) -> MramzR {
        MramzR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Run the TRAM BIST."]
    #[inline(always)]
    pub fn tbistrun(&self) -> TbistrunR {
        TbistrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Zeroize the TRAM."]
    #[inline(always)]
    pub fn tramz(&self) -> TramzR {
        TramzR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run the bias memory BIST."]
    #[inline(always)]
    pub fn bbistrun(&self) -> BbistrunR {
        BbistrunR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Zeroize the bias memory."]
    #[inline(always)]
    pub fn bramz(&self) -> BramzR {
        BramzR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - BIST controller status selection."]
    #[inline(always)]
    pub fn bistsel(&self) -> BistselR {
        BistselR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Data SRAM BIST failed."]
    #[inline(always)]
    pub fn sallbfail(&self) -> SallbfailR {
        SallbfailR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MRAM BIST failed."]
    #[inline(always)]
    pub fn mallbfail(&self) -> MallbfailR {
        MallbfailR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TRAM BIST failed."]
    #[inline(always)]
    pub fn tallbfail(&self) -> TallbfailR {
        TallbfailR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bias memory BIST failed."]
    #[inline(always)]
    pub fn ballbfail(&self) -> BallbfailR {
        BallbfailR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data SRAM BIST complete."]
    #[inline(always)]
    pub fn sallbdone(&self) -> SallbdoneR {
        SallbdoneR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MRAM BIST complete."]
    #[inline(always)]
    pub fn mallbdone(&self) -> MallbdoneR {
        MallbdoneR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TRAM BIST complete."]
    #[inline(always)]
    pub fn tallbdone(&self) -> TallbdoneR {
        TallbdoneR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bias memory BIST complete."]
    #[inline(always)]
    pub fn ballbdone(&self) -> BallbdoneR {
        BallbdoneR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data SRAM zeroization complete."]
    #[inline(always)]
    pub fn sallzdone(&self) -> SallzdoneR {
        SallzdoneR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MRAM zeroization complete."]
    #[inline(always)]
    pub fn mallzdone(&self) -> MallzdoneR {
        MallzdoneR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TRAM zeroization complete."]
    #[inline(always)]
    pub fn tallzdone(&self) -> TallzdoneR {
        TallzdoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bias memory zeroization complete."]
    #[inline(always)]
    pub fn ballzdone(&self) -> BallzdoneR {
        BallzdoneR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - A BIST run detected a failure."]
    #[inline(always)]
    pub fn bistfail(&self) -> BistfailR {
        BistfailR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BIST run complete."]
    #[inline(always)]
    pub fn bistdone(&self) -> BistdoneR {
        BistdoneR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Zeroization complete."]
    #[inline(always)]
    pub fn zero_done(&self) -> ZeroDoneR {
        ZeroDoneR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run the data SRAM BIST."]
    #[inline(always)]
    pub fn sbistrun(&mut self) -> SbistrunW<TestSpec> {
        SbistrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Zeroize the data SRAM."]
    #[inline(always)]
    pub fn sramz(&mut self) -> SramzW<TestSpec> {
        SramzW::new(self, 1)
    }
    #[doc = "Bit 2 - Run the MRAM BIST."]
    #[inline(always)]
    pub fn mbistrun(&mut self) -> MbistrunW<TestSpec> {
        MbistrunW::new(self, 2)
    }
    #[doc = "Bit 3 - Zeroize the MRAM."]
    #[inline(always)]
    pub fn mramz(&mut self) -> MramzW<TestSpec> {
        MramzW::new(self, 3)
    }
    #[doc = "Bit 4 - Run the TRAM BIST."]
    #[inline(always)]
    pub fn tbistrun(&mut self) -> TbistrunW<TestSpec> {
        TbistrunW::new(self, 4)
    }
    #[doc = "Bit 5 - Zeroize the TRAM."]
    #[inline(always)]
    pub fn tramz(&mut self) -> TramzW<TestSpec> {
        TramzW::new(self, 5)
    }
    #[doc = "Bit 6 - Run the bias memory BIST."]
    #[inline(always)]
    pub fn bbistrun(&mut self) -> BbistrunW<TestSpec> {
        BbistrunW::new(self, 6)
    }
    #[doc = "Bit 7 - Zeroize the bias memory."]
    #[inline(always)]
    pub fn bramz(&mut self) -> BramzW<TestSpec> {
        BramzW::new(self, 7)
    }
    #[doc = "Bits 8:13 - BIST controller status selection."]
    #[inline(always)]
    pub fn bistsel(&mut self) -> BistselW<TestSpec> {
        BistselW::new(self, 8)
    }
}
#[doc = "SRAM test: memory BIST and zeroization.\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {
    const RESET_VALUE: u32 = 0;
}
