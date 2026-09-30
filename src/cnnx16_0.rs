#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    sram: Sram,
    lcnt: Lcnt,
    test: Test,
}
impl RegisterBlock {
    #[doc = "0x00 - Quadrant control."]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - SRAM control."]
    #[inline(always)]
    pub const fn sram(&self) -> &Sram {
        &self.sram
    }
    #[doc = "0x08 - Layer count maximum. Processing always starts at layer 0."]
    #[inline(always)]
    pub const fn lcnt(&self) -> &Lcnt {
        &self.lcnt
    }
    #[doc = "0x0c - SRAM test: memory BIST and zeroization."]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
}
#[doc = "CTL (rw) register accessor: Quadrant control.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Quadrant control."]
pub mod ctl;
#[doc = "SRAM (rw) register accessor: SRAM control.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram`]
module"]
#[doc(alias = "SRAM")]
pub type Sram = crate::Reg<sram::SramSpec>;
#[doc = "SRAM control."]
pub mod sram;
#[doc = "LCNT (rw) register accessor: Layer count maximum. Processing always starts at layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcnt`]
module"]
#[doc(alias = "LCNT")]
pub type Lcnt = crate::Reg<lcnt::LcntSpec>;
#[doc = "Layer count maximum. Processing always starts at layer 0."]
pub mod lcnt;
#[doc = "TEST (rw) register accessor: SRAM test: memory BIST and zeroization.\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "SRAM test: memory BIST and zeroization."]
pub mod test;
