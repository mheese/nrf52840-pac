#[doc = "Reader of register IFCONFIG1"]
pub type R = crate::R<u32, super::IFCONFIG1>;
#[doc = "Writer for register IFCONFIG1"]
pub type W = crate::W<u32, super::IFCONFIG1>;
#[doc = "Register IFCONFIG1 `reset()`'s with value 0x0004_0480"]
impl crate::ResetValue for super::IFCONFIG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0480
    }
}
#[doc = "Reader of field `SCKDELAY`"]
pub type SCKDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCKDELAY`"]
pub struct SCKDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Enter/exit deep power-down mode (DPM) for external flash memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPMEN_A {
    #[doc = "0: Exit DPM."]
    EXIT,
    #[doc = "1: Enter DPM."]
    ENTER,
}
impl From<DPMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DPMEN_A) -> Self {
        match variant {
            DPMEN_A::EXIT => false,
            DPMEN_A::ENTER => true,
        }
    }
}
#[doc = "Reader of field `DPMEN`"]
pub type DPMEN_R = crate::R<bool, DPMEN_A>;
impl DPMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPMEN_A {
        match self.bits {
            false => DPMEN_A::EXIT,
            true => DPMEN_A::ENTER,
        }
    }
    #[doc = "Checks if the value of the field is `EXIT`"]
    #[inline(always)]
    pub fn is_exit(&self) -> bool {
        *self == DPMEN_A::EXIT
    }
    #[doc = "Checks if the value of the field is `ENTER`"]
    #[inline(always)]
    pub fn is_enter(&self) -> bool {
        *self == DPMEN_A::ENTER
    }
}
#[doc = "Write proxy for field `DPMEN`"]
pub struct DPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exit DPM."]
    #[inline(always)]
    pub fn exit(self) -> &'a mut W {
        self.variant(DPMEN_A::EXIT)
    }
    #[doc = "Enter DPM."]
    #[inline(always)]
    pub fn enter(self) -> &'a mut W {
        self.variant(DPMEN_A::ENTER)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Select SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMODE_A {
    #[doc = "0: Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    MODE0,
    #[doc = "1: Mode 3: Data are captured on the clock falling edge and data is output on a rising edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    MODE3,
}
impl From<SPIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIMODE_A) -> Self {
        match variant {
            SPIMODE_A::MODE0 => false,
            SPIMODE_A::MODE3 => true,
        }
    }
}
#[doc = "Reader of field `SPIMODE`"]
pub type SPIMODE_R = crate::R<bool, SPIMODE_A>;
impl SPIMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIMODE_A {
        match self.bits {
            false => SPIMODE_A::MODE0,
            true => SPIMODE_A::MODE3,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == SPIMODE_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == SPIMODE_A::MODE3
    }
}
#[doc = "Write proxy for field `SPIMODE`"]
pub struct SPIMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(SPIMODE_A::MODE0)
    }
    #[doc = "Mode 3: Data are captured on the clock falling edge and data is output on a rising edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(SPIMODE_A::MODE3)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SCKFREQ`"]
pub type SCKFREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCKFREQ`"]
pub struct SCKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline(always)]
    pub fn sckdelay(&self) -> SCKDELAY_R {
        SCKDELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline(always)]
    pub fn dpmen(&self) -> DPMEN_R {
        DPMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Select SPI mode."]
    #[inline(always)]
    pub fn spimode(&self) -> SPIMODE_R {
        SPIMODE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
    #[inline(always)]
    pub fn sckfreq(&self) -> SCKFREQ_R {
        SCKFREQ_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline(always)]
    pub fn sckdelay(&mut self) -> SCKDELAY_W {
        SCKDELAY_W { w: self }
    }
    #[doc = "Bit 24 - Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline(always)]
    pub fn dpmen(&mut self) -> DPMEN_W {
        DPMEN_W { w: self }
    }
    #[doc = "Bit 25 - Select SPI mode."]
    #[inline(always)]
    pub fn spimode(&mut self) -> SPIMODE_W {
        SPIMODE_W { w: self }
    }
    #[doc = "Bits 28:31 - SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
    #[inline(always)]
    pub fn sckfreq(&mut self) -> SCKFREQ_W {
        SCKFREQ_W { w: self }
    }
}
