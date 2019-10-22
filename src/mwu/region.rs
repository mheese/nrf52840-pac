#[doc = "Description cluster: Start address for region n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [start](start) module"]
pub type START = crate::Reg<u32, _START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _START;
#[doc = "`read()` method returns [start::R](start::R) reader structure"]
impl crate::Readable for START {}
#[doc = "`write(|w| ..)` method takes [start::W](start::W) writer structure"]
impl crate::Writable for START {}
#[doc = "Description cluster: Start address for region n"]
pub mod start;
#[doc = "Description cluster: End address of region n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [end](end) module"]
pub type END = crate::Reg<u32, _END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _END;
#[doc = "`read()` method returns [end::R](end::R) reader structure"]
impl crate::Readable for END {}
#[doc = "`write(|w| ..)` method takes [end::W](end::W) writer structure"]
impl crate::Writable for END {}
#[doc = "Description cluster: End address of region n"]
pub mod end;
