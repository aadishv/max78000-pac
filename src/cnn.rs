#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    status: Status,
    fifo0: Fifo0,
    fifo1: Fifo1,
    fifo2: Fifo2,
    fifo3: Fifo3,
    _reserved6: [u8; 0x0fe8],
    aon: Aon,
}
impl RegisterBlock {
    #[doc = "0x00 - FIFO control."]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - FIFO status."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - FIFO 0 data port."]
    #[inline(always)]
    pub const fn fifo0(&self) -> &Fifo0 {
        &self.fifo0
    }
    #[doc = "0x0c - FIFO 1 data port."]
    #[inline(always)]
    pub const fn fifo1(&self) -> &Fifo1 {
        &self.fifo1
    }
    #[doc = "0x10 - FIFO 2 data port."]
    #[inline(always)]
    pub const fn fifo2(&self) -> &Fifo2 {
        &self.fifo2
    }
    #[doc = "0x14 - FIFO 3 data port."]
    #[inline(always)]
    pub const fn fifo3(&self) -> &Fifo3 {
        &self.fifo3
    }
    #[doc = "0x1000 - Always-on domain control."]
    #[inline(always)]
    pub const fn aon(&self) -> &Aon {
        &self.aon
    }
}
#[doc = "CTL (rw) register accessor: FIFO control.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "FIFO control."]
pub mod ctl;
#[doc = "STATUS (r) register accessor: FIFO status.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "FIFO status."]
pub mod status;
#[doc = "FIFO0 (rw) register accessor: FIFO 0 data port.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo0`]
module"]
#[doc(alias = "FIFO0")]
pub type Fifo0 = crate::Reg<fifo0::Fifo0Spec>;
#[doc = "FIFO 0 data port."]
pub mod fifo0;
#[doc = "FIFO1 (rw) register accessor: FIFO 1 data port.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo1`]
module"]
#[doc(alias = "FIFO1")]
pub type Fifo1 = crate::Reg<fifo1::Fifo1Spec>;
#[doc = "FIFO 1 data port."]
pub mod fifo1;
#[doc = "FIFO2 (rw) register accessor: FIFO 2 data port.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo2`]
module"]
#[doc(alias = "FIFO2")]
pub type Fifo2 = crate::Reg<fifo2::Fifo2Spec>;
#[doc = "FIFO 2 data port."]
pub mod fifo2;
#[doc = "FIFO3 (rw) register accessor: FIFO 3 data port.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo3`]
module"]
#[doc(alias = "FIFO3")]
pub type Fifo3 = crate::Reg<fifo3::Fifo3Spec>;
#[doc = "FIFO 3 data port."]
pub mod fifo3;
#[doc = "AON (rw) register accessor: Always-on domain control.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon`]
module"]
#[doc(alias = "AON")]
pub type Aon = crate::Reg<aon::AonSpec>;
#[doc = "Always-on domain control."]
pub mod aon;
