#[doc = "Reader of register DCDCEN0"]
pub type R = crate::R<u32, super::DCDCEN0>;
#[doc = "Writer for register DCDCEN0"]
pub type W = crate::W<u32, super::DCDCEN0>;
#[doc = "Register DCDCEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCDCEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable DC/DC converter for REG0 stage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDCEN_A {
    #[doc = "0: Disable"]
    DISABLED,
    #[doc = "1: Enable"]
    ENABLED,
}
impl From<DCDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCDCEN_A) -> Self {
        match variant {
            DCDCEN_A::DISABLED => false,
            DCDCEN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `DCDCEN`"]
pub type DCDCEN_R = crate::R<bool, DCDCEN_A>;
impl DCDCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDCEN_A {
        match self.bits {
            false => DCDCEN_A::DISABLED,
            true => DCDCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCDCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCDCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DCDCEN`"]
pub struct DCDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDCEN_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDCEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable DC/DC converter for REG0 stage."]
    #[inline(always)]
    pub fn dcdcen(&self) -> DCDCEN_R {
        DCDCEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DC/DC converter for REG0 stage."]
    #[inline(always)]
    pub fn dcdcen(&mut self) -> DCDCEN_W {
        DCDCEN_W { w: self }
    }
}
