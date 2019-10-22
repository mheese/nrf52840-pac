#[doc = "Reader of register MCKFREQ"]
pub type R = crate::R<u32, super::MCKFREQ>;
#[doc = "Writer for register MCKFREQ"]
pub type W = crate::W<u32, super::MCKFREQ>;
#[doc = "Register MCKFREQ `reset()`'s with value 0x2000_0000"]
impl crate::ResetValue for super::MCKFREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_0000
    }
}
#[doc = "Master clock generator frequency.\n\nValue on reset: 536870912"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKFREQ_A {
    #[doc = "536870912: 32 MHz / 8 = 4.0 MHz"]
    _32MDIV8,
    #[doc = "402653184: 32 MHz / 10 = 3.2 MHz"]
    _32MDIV10,
    #[doc = "369098752: 32 MHz / 11 = 2.9090909 MHz"]
    _32MDIV11,
    #[doc = "285212672: 32 MHz / 15 = 2.1333333 MHz"]
    _32MDIV15,
    #[doc = "268435456: 32 MHz / 16 = 2.0 MHz"]
    _32MDIV16,
    #[doc = "201326592: 32 MHz / 21 = 1.5238095"]
    _32MDIV21,
    #[doc = "184549376: 32 MHz / 23 = 1.3913043 MHz"]
    _32MDIV23,
    #[doc = "142606336: 32 MHz / 30 = 1.0666667 MHz"]
    _32MDIV30,
    #[doc = "138412032: 32 MHz / 31 = 1.0322581 MHz"]
    _32MDIV31,
    #[doc = "134217728: 32 MHz / 32 = 1.0 MHz"]
    _32MDIV32,
    #[doc = "100663296: 32 MHz / 42 = 0.7619048 MHz"]
    _32MDIV42,
    #[doc = "68157440: 32 MHz / 63 = 0.5079365 MHz"]
    _32MDIV63,
    #[doc = "34340864: 32 MHz / 125 = 0.256 MHz"]
    _32MDIV125,
}
impl From<MCKFREQ_A> for u32 {
    #[inline(always)]
    fn from(variant: MCKFREQ_A) -> Self {
        match variant {
            MCKFREQ_A::_32MDIV8 => 536870912,
            MCKFREQ_A::_32MDIV10 => 402653184,
            MCKFREQ_A::_32MDIV11 => 369098752,
            MCKFREQ_A::_32MDIV15 => 285212672,
            MCKFREQ_A::_32MDIV16 => 268435456,
            MCKFREQ_A::_32MDIV21 => 201326592,
            MCKFREQ_A::_32MDIV23 => 184549376,
            MCKFREQ_A::_32MDIV30 => 142606336,
            MCKFREQ_A::_32MDIV31 => 138412032,
            MCKFREQ_A::_32MDIV32 => 134217728,
            MCKFREQ_A::_32MDIV42 => 100663296,
            MCKFREQ_A::_32MDIV63 => 68157440,
            MCKFREQ_A::_32MDIV125 => 34340864,
        }
    }
}
#[doc = "Reader of field `MCKFREQ`"]
pub type MCKFREQ_R = crate::R<u32, MCKFREQ_A>;
impl MCKFREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, MCKFREQ_A> {
        use crate::Variant::*;
        match self.bits {
            536870912 => Val(MCKFREQ_A::_32MDIV8),
            402653184 => Val(MCKFREQ_A::_32MDIV10),
            369098752 => Val(MCKFREQ_A::_32MDIV11),
            285212672 => Val(MCKFREQ_A::_32MDIV15),
            268435456 => Val(MCKFREQ_A::_32MDIV16),
            201326592 => Val(MCKFREQ_A::_32MDIV21),
            184549376 => Val(MCKFREQ_A::_32MDIV23),
            142606336 => Val(MCKFREQ_A::_32MDIV30),
            138412032 => Val(MCKFREQ_A::_32MDIV31),
            134217728 => Val(MCKFREQ_A::_32MDIV32),
            100663296 => Val(MCKFREQ_A::_32MDIV42),
            68157440 => Val(MCKFREQ_A::_32MDIV63),
            34340864 => Val(MCKFREQ_A::_32MDIV125),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32MDIV8`"]
    #[inline(always)]
    pub fn is_32mdiv8(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV8
    }
    #[doc = "Checks if the value of the field is `_32MDIV10`"]
    #[inline(always)]
    pub fn is_32mdiv10(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV10
    }
    #[doc = "Checks if the value of the field is `_32MDIV11`"]
    #[inline(always)]
    pub fn is_32mdiv11(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV11
    }
    #[doc = "Checks if the value of the field is `_32MDIV15`"]
    #[inline(always)]
    pub fn is_32mdiv15(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV15
    }
    #[doc = "Checks if the value of the field is `_32MDIV16`"]
    #[inline(always)]
    pub fn is_32mdiv16(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV16
    }
    #[doc = "Checks if the value of the field is `_32MDIV21`"]
    #[inline(always)]
    pub fn is_32mdiv21(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV21
    }
    #[doc = "Checks if the value of the field is `_32MDIV23`"]
    #[inline(always)]
    pub fn is_32mdiv23(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV23
    }
    #[doc = "Checks if the value of the field is `_32MDIV30`"]
    #[inline(always)]
    pub fn is_32mdiv30(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV30
    }
    #[doc = "Checks if the value of the field is `_32MDIV31`"]
    #[inline(always)]
    pub fn is_32mdiv31(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV31
    }
    #[doc = "Checks if the value of the field is `_32MDIV32`"]
    #[inline(always)]
    pub fn is_32mdiv32(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV32
    }
    #[doc = "Checks if the value of the field is `_32MDIV42`"]
    #[inline(always)]
    pub fn is_32mdiv42(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV42
    }
    #[doc = "Checks if the value of the field is `_32MDIV63`"]
    #[inline(always)]
    pub fn is_32mdiv63(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV63
    }
    #[doc = "Checks if the value of the field is `_32MDIV125`"]
    #[inline(always)]
    pub fn is_32mdiv125(&self) -> bool {
        *self == MCKFREQ_A::_32MDIV125
    }
}
#[doc = "Write proxy for field `MCKFREQ`"]
pub struct MCKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCKFREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32 MHz / 8 = 4.0 MHz"]
    #[inline(always)]
    pub fn _32mdiv8(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV8)
    }
    #[doc = "32 MHz / 10 = 3.2 MHz"]
    #[inline(always)]
    pub fn _32mdiv10(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV10)
    }
    #[doc = "32 MHz / 11 = 2.9090909 MHz"]
    #[inline(always)]
    pub fn _32mdiv11(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV11)
    }
    #[doc = "32 MHz / 15 = 2.1333333 MHz"]
    #[inline(always)]
    pub fn _32mdiv15(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV15)
    }
    #[doc = "32 MHz / 16 = 2.0 MHz"]
    #[inline(always)]
    pub fn _32mdiv16(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV16)
    }
    #[doc = "32 MHz / 21 = 1.5238095"]
    #[inline(always)]
    pub fn _32mdiv21(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV21)
    }
    #[doc = "32 MHz / 23 = 1.3913043 MHz"]
    #[inline(always)]
    pub fn _32mdiv23(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV23)
    }
    #[doc = "32 MHz / 30 = 1.0666667 MHz"]
    #[inline(always)]
    pub fn _32mdiv30(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV30)
    }
    #[doc = "32 MHz / 31 = 1.0322581 MHz"]
    #[inline(always)]
    pub fn _32mdiv31(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV31)
    }
    #[doc = "32 MHz / 32 = 1.0 MHz"]
    #[inline(always)]
    pub fn _32mdiv32(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV32)
    }
    #[doc = "32 MHz / 42 = 0.7619048 MHz"]
    #[inline(always)]
    pub fn _32mdiv42(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV42)
    }
    #[doc = "32 MHz / 63 = 0.5079365 MHz"]
    #[inline(always)]
    pub fn _32mdiv63(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV63)
    }
    #[doc = "32 MHz / 125 = 0.256 MHz"]
    #[inline(always)]
    pub fn _32mdiv125(self) -> &'a mut W {
        self.variant(MCKFREQ_A::_32MDIV125)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Master clock generator frequency."]
    #[inline(always)]
    pub fn mckfreq(&self) -> MCKFREQ_R {
        MCKFREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Master clock generator frequency."]
    #[inline(always)]
    pub fn mckfreq(&mut self) -> MCKFREQ_W {
        MCKFREQ_W { w: self }
    }
}
