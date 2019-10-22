#[doc = "Reader of register SHORTS"]
pub type R = crate::R<u32, super::SHORTS>;
#[doc = "Writer for register SHORTS"]
pub type W = crate::W<u32, super::SHORTS>;
#[doc = "Register SHORTS `reset()`'s with value 0"]
impl crate::ResetValue for super::SHORTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shortcut between event FIELDDETECTED and task ACTIVATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDDETECTED_ACTIVATE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED,
    #[doc = "1: Enable shortcut"]
    ENABLED,
}
impl From<FIELDDETECTED_ACTIVATE_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDDETECTED_ACTIVATE_A) -> Self {
        match variant {
            FIELDDETECTED_ACTIVATE_A::DISABLED => false,
            FIELDDETECTED_ACTIVATE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `FIELDDETECTED_ACTIVATE`"]
pub type FIELDDETECTED_ACTIVATE_R = crate::R<bool, FIELDDETECTED_ACTIVATE_A>;
impl FIELDDETECTED_ACTIVATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDDETECTED_ACTIVATE_A {
        match self.bits {
            false => FIELDDETECTED_ACTIVATE_A::DISABLED,
            true => FIELDDETECTED_ACTIVATE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDDETECTED_ACTIVATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDDETECTED_ACTIVATE_A::ENABLED
    }
}
#[doc = "Write proxy for field `FIELDDETECTED_ACTIVATE`"]
pub struct FIELDDETECTED_ACTIVATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDDETECTED_ACTIVATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDDETECTED_ACTIVATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_ACTIVATE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_ACTIVATE_A::ENABLED)
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
#[doc = "Shortcut between event FIELDLOST and task SENSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDLOST_SENSE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED,
    #[doc = "1: Enable shortcut"]
    ENABLED,
}
impl From<FIELDLOST_SENSE_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDLOST_SENSE_A) -> Self {
        match variant {
            FIELDLOST_SENSE_A::DISABLED => false,
            FIELDLOST_SENSE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `FIELDLOST_SENSE`"]
pub type FIELDLOST_SENSE_R = crate::R<bool, FIELDLOST_SENSE_A>;
impl FIELDLOST_SENSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDLOST_SENSE_A {
        match self.bits {
            false => FIELDLOST_SENSE_A::DISABLED,
            true => FIELDLOST_SENSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDLOST_SENSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDLOST_SENSE_A::ENABLED
    }
}
#[doc = "Write proxy for field `FIELDLOST_SENSE`"]
pub struct FIELDLOST_SENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDLOST_SENSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDLOST_SENSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDLOST_SENSE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDLOST_SENSE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Shortcut between event TXFRAMEEND and task ENABLERXDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMEEND_ENABLERXDATA_A {
    #[doc = "0: Disable shortcut"]
    DISABLED,
    #[doc = "1: Enable shortcut"]
    ENABLED,
}
impl From<TXFRAMEEND_ENABLERXDATA_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMEEND_ENABLERXDATA_A) -> Self {
        match variant {
            TXFRAMEEND_ENABLERXDATA_A::DISABLED => false,
            TXFRAMEEND_ENABLERXDATA_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TXFRAMEEND_ENABLERXDATA`"]
pub type TXFRAMEEND_ENABLERXDATA_R = crate::R<bool, TXFRAMEEND_ENABLERXDATA_A>;
impl TXFRAMEEND_ENABLERXDATA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFRAMEEND_ENABLERXDATA_A {
        match self.bits {
            false => TXFRAMEEND_ENABLERXDATA_A::DISABLED,
            true => TXFRAMEEND_ENABLERXDATA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFRAMEEND_ENABLERXDATA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFRAMEEND_ENABLERXDATA_A::ENABLED
    }
}
#[doc = "Write proxy for field `TXFRAMEEND_ENABLERXDATA`"]
pub struct TXFRAMEEND_ENABLERXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRAMEEND_ENABLERXDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFRAMEEND_ENABLERXDATA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFRAMEEND_ENABLERXDATA_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFRAMEEND_ENABLERXDATA_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn fielddetected_activate(&self) -> FIELDDETECTED_ACTIVATE_R {
        FIELDDETECTED_ACTIVATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn fieldlost_sense(&self) -> FIELDLOST_SENSE_R {
        FIELDLOST_SENSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub fn txframeend_enablerxdata(&self) -> TXFRAMEEND_ENABLERXDATA_R {
        TXFRAMEEND_ENABLERXDATA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn fielddetected_activate(&mut self) -> FIELDDETECTED_ACTIVATE_W {
        FIELDDETECTED_ACTIVATE_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn fieldlost_sense(&mut self) -> FIELDLOST_SENSE_W {
        FIELDLOST_SENSE_W { w: self }
    }
    #[doc = "Bit 5 - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub fn txframeend_enablerxdata(&mut self) -> TXFRAMEEND_ENABLERXDATA_W {
        TXFRAMEEND_ENABLERXDATA_W { w: self }
    }
}
