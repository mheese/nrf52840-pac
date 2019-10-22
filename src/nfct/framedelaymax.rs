#[doc = "Reader of register FRAMEDELAYMAX"]
pub type R = crate::R<u32, super::FRAMEDELAYMAX>;
#[doc = "Writer for register FRAMEDELAYMAX"]
pub type W = crate::W<u32, super::FRAMEDELAYMAX>;
#[doc = "Register FRAMEDELAYMAX `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::FRAMEDELAYMAX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `FRAMEDELAYMAX`"]
pub type FRAMEDELAYMAX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FRAMEDELAYMAX`"]
pub struct FRAMEDELAYMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDELAYMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Maximum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymax(&self) -> FRAMEDELAYMAX_R {
        FRAMEDELAYMAX_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Maximum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymax(&mut self) -> FRAMEDELAYMAX_W {
        FRAMEDELAYMAX_W { w: self }
    }
}
