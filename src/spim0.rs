#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Start SPI transaction"]
    pub tasks_start: TASKS_START,
    #[doc = "0x14 - Stop SPI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 4usize],
    #[doc = "0x1c - Suspend SPI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume SPI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved4: [u8; 224usize],
    #[doc = "0x104 - SPI transaction has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved5: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved6: [u8; 4usize],
    #[doc = "0x118 - End of RXD buffer and TXD buffer reached"]
    pub events_end: EVENTS_END,
    _reserved7: [u8; 4usize],
    #[doc = "0x120 - End of TXD buffer reached"]
    pub events_endtx: EVENTS_ENDTX,
    _reserved8: [u8; 40usize],
    #[doc = "0x14c - Transaction started"]
    pub events_started: EVENTS_STARTED,
    _reserved9: [u8; 176usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved10: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved12: [u8; 244usize],
    #[doc = "0x400 - Stall status for EasyDMA RAM accesses. The fields in this register is set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
    pub stallstat: STALLSTAT,
    _reserved13: [u8; 252usize],
    #[doc = "0x500 - Enable SPIM"]
    pub enable: ENABLE,
    _reserved14: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved15: [u8; 12usize],
    #[doc = "0x524 - SPI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: FREQUENCY,
    _reserved16: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved19: [u8; 8usize],
    #[doc = "0x560 - Unspecified"]
    pub iftiming: IFTIMING,
    #[doc = "0x568 - Polarity of CSN output"]
    pub csnpol: CSNPOL,
    #[doc = "0x56c - Pin select for DCX signal"]
    pub pseldcx: PSELDCX,
    #[doc = "0x570 - DCX configuration"]
    pub dcxcnt: DCXCNT,
    _reserved23: [u8; 76usize],
    #[doc = "0x5c0 - Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
    pub orc: ORC,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for MOSI signal"]
    pub mosi: self::psel::MOSI,
    #[doc = "0x08 - Pin select for MISO signal"]
    pub miso: self::psel::MISO,
    #[doc = "0x0c - Pin select for CSN"]
    pub csn: self::psel::CSN,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::rxd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::rxd::LIST,
}
#[doc = r"Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Number of bytes in transmit buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::txd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::txd::LIST,
}
#[doc = r"Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = r"Register block"]
#[repr(C)]
pub struct IFTIMING {
    #[doc = "0x00 - Sample delay for input serial data on MISO"]
    pub rxdelay: self::iftiming::RXDELAY,
    #[doc = "0x04 - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
    pub csndur: self::iftiming::CSNDUR,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod iftiming;
#[doc = "Start SPI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "Stop SPI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "Suspend SPI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_suspend](tasks_suspend) module"]
pub type TASKS_SUSPEND = crate::Reg<u32, _TASKS_SUSPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SUSPEND;
#[doc = "`write(|w| ..)` method takes [tasks_suspend::W](tasks_suspend::W) writer structure"]
impl crate::Writable for TASKS_SUSPEND {}
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "Resume SPI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_resume](tasks_resume) module"]
pub type TASKS_RESUME = crate::Reg<u32, _TASKS_RESUME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RESUME;
#[doc = "`write(|w| ..)` method takes [tasks_resume::W](tasks_resume::W) writer structure"]
impl crate::Writable for TASKS_RESUME {}
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "SPI transaction has stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "End of RXD buffer reached\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endrx](events_endrx) module"]
pub type EVENTS_ENDRX = crate::Reg<u32, _EVENTS_ENDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDRX;
#[doc = "`read()` method returns [events_endrx::R](events_endrx::R) reader structure"]
impl crate::Readable for EVENTS_ENDRX {}
#[doc = "`write(|w| ..)` method takes [events_endrx::W](events_endrx::W) writer structure"]
impl crate::Writable for EVENTS_ENDRX {}
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "End of RXD buffer and TXD buffer reached\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_end](events_end) module"]
pub type EVENTS_END = crate::Reg<u32, _EVENTS_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_END;
#[doc = "`read()` method returns [events_end::R](events_end::R) reader structure"]
impl crate::Readable for EVENTS_END {}
#[doc = "`write(|w| ..)` method takes [events_end::W](events_end::W) writer structure"]
impl crate::Writable for EVENTS_END {}
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "End of TXD buffer reached\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endtx](events_endtx) module"]
pub type EVENTS_ENDTX = crate::Reg<u32, _EVENTS_ENDTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDTX;
#[doc = "`read()` method returns [events_endtx::R](events_endtx::R) reader structure"]
impl crate::Readable for EVENTS_ENDTX {}
#[doc = "`write(|w| ..)` method takes [events_endtx::W](events_endtx::W) writer structure"]
impl crate::Writable for EVENTS_ENDTX {}
#[doc = "End of TXD buffer reached"]
pub mod events_endtx;
#[doc = "Transaction started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_started](events_started) module"]
pub type EVENTS_STARTED = crate::Reg<u32, _EVENTS_STARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STARTED;
#[doc = "`read()` method returns [events_started::R](events_started::R) reader structure"]
impl crate::Readable for EVENTS_STARTED {}
#[doc = "`write(|w| ..)` method takes [events_started::W](events_started::W) writer structure"]
impl crate::Writable for EVENTS_STARTED {}
#[doc = "Transaction started"]
pub mod events_started;
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Stall status for EasyDMA RAM accesses. The fields in this register is set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stallstat](stallstat) module"]
pub type STALLSTAT = crate::Reg<u32, _STALLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STALLSTAT;
#[doc = "`read()` method returns [stallstat::R](stallstat::R) reader structure"]
impl crate::Readable for STALLSTAT {}
#[doc = "`write(|w| ..)` method takes [stallstat::W](stallstat::W) writer structure"]
impl crate::Writable for STALLSTAT {}
#[doc = "Stall status for EasyDMA RAM accesses. The fields in this register is set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
pub mod stallstat;
#[doc = "Enable SPIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frequency](frequency) module"]
pub type FREQUENCY = crate::Reg<u32, _FREQUENCY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQUENCY;
#[doc = "`read()` method returns [frequency::R](frequency::R) reader structure"]
impl crate::Readable for FREQUENCY {}
#[doc = "`write(|w| ..)` method takes [frequency::W](frequency::W) writer structure"]
impl crate::Writable for FREQUENCY {}
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Polarity of CSN output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csnpol](csnpol) module"]
pub type CSNPOL = crate::Reg<u32, _CSNPOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSNPOL;
#[doc = "`read()` method returns [csnpol::R](csnpol::R) reader structure"]
impl crate::Readable for CSNPOL {}
#[doc = "`write(|w| ..)` method takes [csnpol::W](csnpol::W) writer structure"]
impl crate::Writable for CSNPOL {}
#[doc = "Polarity of CSN output"]
pub mod csnpol;
#[doc = "Pin select for DCX signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pseldcx](pseldcx) module"]
pub type PSELDCX = crate::Reg<u32, _PSELDCX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELDCX;
#[doc = "`read()` method returns [pseldcx::R](pseldcx::R) reader structure"]
impl crate::Readable for PSELDCX {}
#[doc = "`write(|w| ..)` method takes [pseldcx::W](pseldcx::W) writer structure"]
impl crate::Writable for PSELDCX {}
#[doc = "Pin select for DCX signal"]
pub mod pseldcx;
#[doc = "DCX configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcxcnt](dcxcnt) module"]
pub type DCXCNT = crate::Reg<u32, _DCXCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCXCNT;
#[doc = "`read()` method returns [dcxcnt::R](dcxcnt::R) reader structure"]
impl crate::Readable for DCXCNT {}
#[doc = "`write(|w| ..)` method takes [dcxcnt::W](dcxcnt::W) writer structure"]
impl crate::Writable for DCXCNT {}
#[doc = "DCX configuration"]
pub mod dcxcnt;
#[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [orc](orc) module"]
pub type ORC = crate::Reg<u32, _ORC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORC;
#[doc = "`read()` method returns [orc::R](orc::R) reader structure"]
impl crate::Readable for ORC {}
#[doc = "`write(|w| ..)` method takes [orc::W](orc::W) writer structure"]
impl crate::Writable for ORC {}
#[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
pub mod orc;
