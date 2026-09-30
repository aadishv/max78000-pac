#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `full` reader - Per FIFO full status."]
pub type FullR = crate::FieldReader;
#[doc = "Field `empty` reader - Per FIFO empty status."]
pub type EmptyR = crate::FieldReader;
#[doc = "Field `almost_full` reader - Per FIFO almost full status."]
pub type AlmostFullR = crate::FieldReader;
#[doc = "Field `almost_empty` reader - Per FIFO almost empty status."]
pub type AlmostEmptyR = crate::FieldReader;
#[doc = "Field `fifos_full` reader - Logical AND of the individual FIFO full statuses."]
pub type FifosFullR = crate::BitReader;
#[doc = "Field `fifos_empty` reader - Logical OR of the individual FIFO empty statuses."]
pub type FifosEmptyR = crate::BitReader;
#[doc = "Field `fifos_almost_full` reader - Logical AND of the individual FIFO almost full statuses."]
pub type FifosAlmostFullR = crate::BitReader;
#[doc = "Field `fifos_almost_empty` reader - Logical OR of the individual FIFO almost empty statuses."]
pub type FifosAlmostEmptyR = crate::BitReader;
#[doc = "Field `wptr_eq` reader - All active FIFO write pointers are equal."]
pub type WptrEqR = crate::BitReader;
#[doc = "Field `rptr_eq` reader - All active FIFO read pointers are equal."]
pub type RptrEqR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Per FIFO full status."]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Per FIFO empty status."]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Per FIFO almost full status."]
    #[inline(always)]
    pub fn almost_full(&self) -> AlmostFullR {
        AlmostFullR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Per FIFO almost empty status."]
    #[inline(always)]
    pub fn almost_empty(&self) -> AlmostEmptyR {
        AlmostEmptyR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Logical AND of the individual FIFO full statuses."]
    #[inline(always)]
    pub fn fifos_full(&self) -> FifosFullR {
        FifosFullR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Logical OR of the individual FIFO empty statuses."]
    #[inline(always)]
    pub fn fifos_empty(&self) -> FifosEmptyR {
        FifosEmptyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Logical AND of the individual FIFO almost full statuses."]
    #[inline(always)]
    pub fn fifos_almost_full(&self) -> FifosAlmostFullR {
        FifosAlmostFullR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Logical OR of the individual FIFO almost empty statuses."]
    #[inline(always)]
    pub fn fifos_almost_empty(&self) -> FifosAlmostEmptyR {
        FifosAlmostEmptyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - All active FIFO write pointers are equal."]
    #[inline(always)]
    pub fn wptr_eq(&self) -> WptrEqR {
        WptrEqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - All active FIFO read pointers are equal."]
    #[inline(always)]
    pub fn rptr_eq(&self) -> RptrEqR {
        RptrEqR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "FIFO status.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
