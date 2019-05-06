#![doc = "Peripheral access API for STM32L4X2 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD_PVM();
    fn RTC_TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn ADC1();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM15();
    fn TIM1_UP_TIM16();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn DFSDM1_FLT3();
    fn SDMMC1();
    fn SPI3();
    fn UART4();
    fn TIM6_DACUNDER();
    fn TIM7();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn DFSDM1();
    fn DFSDM2();
    fn DFSDM1_FLT2();
    fn COMP();
    fn LPTIM1();
    fn LPTIM2();
    fn USB();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn LPUART1();
    fn QUADSPI();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SAI1();
    fn SWPMI1();
    fn TSC();
    fn LCD();
    fn AES();
    fn RNG();
    fn FPU();
    fn CRS();
    fn I2C4_EV();
    fn I2C4_ER();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 85] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD_PVM },
    Vector {
        _handler: RTC_TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
    Vector { _handler: DMA1_CH4 },
    Vector { _handler: DMA1_CH5 },
    Vector { _handler: DMA1_CH6 },
    Vector { _handler: DMA1_CH7 },
    Vector { _handler: ADC1 },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM15,
    },
    Vector {
        _handler: TIM1_UP_TIM16,
    },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector {
        _handler: DFSDM1_FLT3,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SDMMC1 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _reserved: 0 },
    Vector {
        _handler: TIM6_DACUNDER,
    },
    Vector { _handler: TIM7 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: DFSDM1 },
    Vector { _handler: DFSDM2 },
    Vector {
        _handler: DFSDM1_FLT2,
    },
    Vector { _handler: COMP },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: USB },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector { _handler: LPUART1 },
    Vector { _handler: QUADSPI },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SAI1 },
    Vector { _reserved: 0 },
    Vector { _handler: SWPMI1 },
    Vector { _handler: TSC },
    Vector { _handler: LCD },
    Vector { _handler: AES },
    Vector { _handler: RNG },
    Vector { _handler: FPU },
    Vector { _handler: CRS },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG,
    #[doc = "1 - PVD through EXTI line detection"]
    PVD_PVM,
    #[doc = "2 - Tamper and TimeStamp interrupts"]
    RTC_TAMP_STAMP,
    #[doc = "3 - RTC Tamper or TimeStamp /CSS on LSE through EXTI line 19 interrupts"]
    RTC_WKUP,
    #[doc = "4 - Flash global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI Line 0 interrupt"]
    EXTI0,
    #[doc = "7 - EXTI Line 1 interrupt"]
    EXTI1,
    #[doc = "8 - EXTI Line 2 interrupt"]
    EXTI2,
    #[doc = "9 - EXTI Line 3 interrupt"]
    EXTI3,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4,
    #[doc = "11 - DMA1 Channel1 global interrupt"]
    DMA1_CH1,
    #[doc = "12 - DMA1 Channel2 global interrupt"]
    DMA1_CH2,
    #[doc = "13 - DMA1 Channel3 interrupt"]
    DMA1_CH3,
    #[doc = "14 - DMA1 Channel4 interrupt"]
    DMA1_CH4,
    #[doc = "15 - DMA1 Channel5 interrupt"]
    DMA1_CH5,
    #[doc = "16 - DMA1 Channel6 interrupt"]
    DMA1_CH6,
    #[doc = "17 - DMA1 Channel 7 interrupt"]
    DMA1_CH7,
    #[doc = "18 - ADC1 and ADC2 global interrupt"]
    ADC1,
    #[doc = "19 - CAN1 TX interrupts"]
    CAN1_TX,
    #[doc = "20 - CAN1 RX0 interrupts"]
    CAN1_RX0,
    #[doc = "21 - CAN1 RX1 interrupts"]
    CAN1_RX1,
    #[doc = "22 - CAN1 SCE interrupt"]
    CAN1_SCE,
    #[doc = "23 - EXTI Line5 to Line9 interrupts"]
    EXTI9_5,
    #[doc = "24 - Timer 15 global interrupt"]
    TIM1_BRK_TIM15,
    #[doc = "25 - Timer 16 global interrupt"]
    TIM1_UP_TIM16,
    #[doc = "26 - TIM1 trigger and commutation interrupt"]
    TIM1_TRG_COM,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2,
    #[doc = "37 - USART1 global interrupt"]
    USART1,
    #[doc = "38 - USART2 global interrupt"]
    USART2,
    #[doc = "39 - USART3 global interrupt"]
    USART3,
    #[doc = "40 - EXTI Lines 10 to 15 interrupts"]
    EXTI15_10,
    #[doc = "41 - RTC alarms through EXTI line 18 interrupts"]
    RTC_ALARM,
    #[doc = "42 - DFSDM1_FLT3 global interrupt"]
    DFSDM1_FLT3,
    #[doc = "49 - SDMMC global Interrupt"]
    SDMMC1,
    #[doc = "51 - SPI3 global Interrupt"]
    SPI3,
    #[doc = "52 - UART4 global Interrupt"]
    UART4,
    #[doc = "54 - TIM6 global and DAC1 and 2 underrun error interrupts"]
    TIM6_DACUNDER,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7,
    #[doc = "56 - DMA2 Channel 1 global Interrupt"]
    DMA2_CH1,
    #[doc = "57 - DMA2 Channel 2 global Interrupt"]
    DMA2_CH2,
    #[doc = "58 - DMA2 Channel 3 global Interrupt"]
    DMA2_CH3,
    #[doc = "59 - DMA2 Channel 4 global Interrupt"]
    DMA2_CH4,
    #[doc = "60 - DMA2 Channel 5 global Interrupt"]
    DMA2_CH5,
    #[doc = "61 - DFSDM1_FLT0 global interrupt"]
    DFSDM1,
    #[doc = "62 - DFSDM1_FLT1 global interrupt"]
    DFSDM2,
    #[doc = "63 - DFSDM1_FLT2 global interrupt"]
    DFSDM1_FLT2,
    #[doc = "64 - COMP1 and COMP2 interrupts"]
    COMP,
    #[doc = "65 - LP TIM1 interrupt"]
    LPTIM1,
    #[doc = "66 - LP TIM2 interrupt"]
    LPTIM2,
    #[doc = "67 - USB event interrupt through EXTI"]
    USB,
    #[doc = "68 - DMA2 Channel 6 global Interrupt"]
    DMA2_CH6,
    #[doc = "69 - DMA2 Channel 7 global Interrupt"]
    DMA2_CH7,
    #[doc = "70 - LPUART1 global interrupt"]
    LPUART1,
    #[doc = "71 - Quad SPI global interrupt"]
    QUADSPI,
    #[doc = "72 - I2C3 event interrupt"]
    I2C3_EV,
    #[doc = "73 - I2C3 error interrupt"]
    I2C3_ER,
    #[doc = "74 - SAI1 global interrupt"]
    SAI1,
    #[doc = "76 - SWPMI1 global interrupt"]
    SWPMI1,
    #[doc = "77 - TSC global interrupt"]
    TSC,
    #[doc = "78 - LCD global interrupt"]
    LCD,
    #[doc = "79 - AES global interrupt"]
    AES,
    #[doc = "80 - RNG global interrupt"]
    RNG,
    #[doc = "81 - Floating point interrupt"]
    FPU,
    #[doc = "82 - CRS interrupt"]
    CRS,
    #[doc = "83 - I2C4 event interrupt, wakeup through EXTI"]
    I2C4_EV,
    #[doc = "84 - I2C4 error interrupt"]
    I2C4_ER,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD_PVM => 1,
            Interrupt::RTC_TAMP_STAMP => 2,
            Interrupt::RTC_WKUP => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2 => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::DMA1_CH1 => 11,
            Interrupt::DMA1_CH2 => 12,
            Interrupt::DMA1_CH3 => 13,
            Interrupt::DMA1_CH4 => 14,
            Interrupt::DMA1_CH5 => 15,
            Interrupt::DMA1_CH6 => 16,
            Interrupt::DMA1_CH7 => 17,
            Interrupt::ADC1 => 18,
            Interrupt::CAN1_TX => 19,
            Interrupt::CAN1_RX0 => 20,
            Interrupt::CAN1_RX1 => 21,
            Interrupt::CAN1_SCE => 22,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM1_BRK_TIM15 => 24,
            Interrupt::TIM1_UP_TIM16 => 25,
            Interrupt::TIM1_TRG_COM => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::I2C1_EV => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::I2C2_EV => 33,
            Interrupt::I2C2_ER => 34,
            Interrupt::SPI1 => 35,
            Interrupt::SPI2 => 36,
            Interrupt::USART1 => 37,
            Interrupt::USART2 => 38,
            Interrupt::USART3 => 39,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTC_ALARM => 41,
            Interrupt::DFSDM1_FLT3 => 42,
            Interrupt::SDMMC1 => 49,
            Interrupt::SPI3 => 51,
            Interrupt::UART4 => 52,
            Interrupt::TIM6_DACUNDER => 54,
            Interrupt::TIM7 => 55,
            Interrupt::DMA2_CH1 => 56,
            Interrupt::DMA2_CH2 => 57,
            Interrupt::DMA2_CH3 => 58,
            Interrupt::DMA2_CH4 => 59,
            Interrupt::DMA2_CH5 => 60,
            Interrupt::DFSDM1 => 61,
            Interrupt::DFSDM2 => 62,
            Interrupt::DFSDM1_FLT2 => 63,
            Interrupt::COMP => 64,
            Interrupt::LPTIM1 => 65,
            Interrupt::LPTIM2 => 66,
            Interrupt::USB => 67,
            Interrupt::DMA2_CH6 => 68,
            Interrupt::DMA2_CH7 => 69,
            Interrupt::LPUART1 => 70,
            Interrupt::QUADSPI => 71,
            Interrupt::I2C3_EV => 72,
            Interrupt::I2C3_ER => 73,
            Interrupt::SAI1 => 74,
            Interrupt::SWPMI1 => 76,
            Interrupt::TSC => 77,
            Interrupt::LCD => 78,
            Interrupt::AES => 79,
            Interrupt::RNG => 80,
            Interrupt::FPU => 81,
            Interrupt::CRS => 82,
            Interrupt::I2C4_EV => 83,
            Interrupt::I2C4_ER => 84,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Digital-to-analog converter"]
pub struct DAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC1 {}
impl DAC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac1::RegisterBlock {
        1073771520 as *const _
    }
}
impl Deref for DAC1 {
    type Target = dac1::RegisterBlock;
    fn deref(&self) -> &dac1::RegisterBlock {
        unsafe { &*DAC1::ptr() }
    }
}
#[doc = "Digital-to-analog converter"]
pub mod dac1;
#[doc = "Direct memory access controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "Direct memory access controller"]
pub mod dma1;
#[doc = "DMA2"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073873920 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "Liquid crystal display controller"]
pub struct LCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD {}
impl LCD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lcd::RegisterBlock {
        1073751040 as *const _
    }
}
impl Deref for LCD {
    type Target = lcd::RegisterBlock;
    fn deref(&self) -> &lcd::RegisterBlock {
        unsafe { &*LCD::ptr() }
    }
}
#[doc = "Liquid crystal display controller"]
pub mod lcd;
#[doc = "Touch sensing controller"]
pub struct TSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSC {}
impl TSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tsc::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for TSC {
    type Target = tsc::RegisterBlock;
    fn deref(&self) -> &tsc::RegisterBlock {
        unsafe { &*TSC::ptr() }
    }
}
#[doc = "Touch sensing controller"]
pub mod tsc;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "System window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "System window watchdog"]
pub mod wwdg;
#[doc = "Comparator"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const comp::RegisterBlock {
        1073807872 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    fn deref(&self) -> &comp::RegisterBlock {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator"]
pub mod comp;
#[doc = "Firewall"]
pub struct FIREWALL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FIREWALL {}
impl FIREWALL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const firewall::RegisterBlock {
        1073814528 as *const _
    }
}
impl Deref for FIREWALL {
    type Target = firewall::RegisterBlock;
    fn deref(&self) -> &firewall::RegisterBlock {
        unsafe { &*FIREWALL::ptr() }
    }
}
#[doc = "Firewall"]
pub mod firewall;
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073763328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "I2C3"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073765376 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073764352 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C4"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073775616 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscfg::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    fn deref(&self) -> &syscfg::RegisterBlock {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Random number generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rng::RegisterBlock {
        1342572544 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random number generator"]
pub mod rng;
#[doc = "Advanced encryption standard hardware accelerator"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aes::RegisterBlock {
        1342570496 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    fn deref(&self) -> &aes::RegisterBlock {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "Advanced encryption standard hardware accelerator"]
pub mod aes;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1342439424 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1207959552 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1207960576 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207961600 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioc;
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207962624 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOE"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207963648 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "GPIOH"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207966720 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sai1::RegisterBlock {
        1073828864 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    fn deref(&self) -> &sai1::RegisterBlock {
        unsafe { &*SAI1::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub mod sai1;
#[doc = "General-purpose-timers"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim2;
#[doc = "TIM3"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "General purpose timers"]
pub struct TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM15 {}
impl TIM15 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim15::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    fn deref(&self) -> &tim15::RegisterBlock {
        unsafe { &*TIM15::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim15;
#[doc = "General purpose timers"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim16::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    fn deref(&self) -> &tim16::RegisterBlock {
        unsafe { &*TIM16::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim16;
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073818624 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "Basic-timers"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "Basic-timers"]
pub mod tim6;
#[doc = "TIM7"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073746944 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "Low power timer"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptim1::RegisterBlock {
        1073773568 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    fn deref(&self) -> &lptim1::RegisterBlock {
        unsafe { &*LPTIM1::ptr() }
    }
}
#[doc = "Low power timer"]
pub mod lptim1;
#[doc = "LPTIM2"]
pub struct LPTIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM2 {}
impl LPTIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptim1::RegisterBlock {
        1073779712 as *const _
    }
}
impl Deref for LPTIM2 {
    type Target = lptim1::RegisterBlock;
    fn deref(&self) -> &lptim1::RegisterBlock {
        unsafe { &*LPTIM2::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073821696 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073759232 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "UART4"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073761280 as *const _
    }
}
impl Deref for UART4 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart3::RegisterBlock {
        1073760256 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart3::RegisterBlock;
    fn deref(&self) -> &usart3::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart3;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpuart1::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &lpuart1::RegisterBlock {
        unsafe { &*LPUART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod lpuart1;
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub mod spi1;
#[doc = "SPI3"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073757184 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "SPI2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073756160 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub struct SDMMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC {}
impl SDMMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdmmc::RegisterBlock {
        1073817600 as *const _
    }
}
impl Deref for SDMMC {
    type Target = sdmmc::RegisterBlock;
    fn deref(&self) -> &sdmmc::RegisterBlock {
        unsafe { &*SDMMC::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub mod sdmmc;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "Voltage reference buffer"]
pub struct VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREFBUF {}
impl VREFBUF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vrefbuf::RegisterBlock {
        1073807408 as *const _
    }
}
impl Deref for VREFBUF {
    type Target = vrefbuf::RegisterBlock;
    fn deref(&self) -> &vrefbuf::RegisterBlock {
        unsafe { &*VREFBUF::ptr() }
    }
}
#[doc = "Voltage reference buffer"]
pub mod vrefbuf;
#[doc = "Controller area network"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1073767424 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Controller area network"]
pub mod can1;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Single Wire Protocol Master Interface"]
pub struct SWPMI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWPMI1 {}
impl SWPMI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swpmi1::RegisterBlock {
        1073776640 as *const _
    }
}
impl Deref for SWPMI1 {
    type Target = swpmi1::RegisterBlock;
    fn deref(&self) -> &swpmi1::RegisterBlock {
        unsafe { &*SWPMI1::ptr() }
    }
}
#[doc = "Single Wire Protocol Master Interface"]
pub mod swpmi1;
#[doc = "Operational amplifiers"]
pub struct OPAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPAMP {}
impl OPAMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const opamp::RegisterBlock {
        1073772544 as *const _
    }
}
impl Deref for OPAMP {
    type Target = opamp::RegisterBlock;
    fn deref(&self) -> &opamp::RegisterBlock {
        unsafe { &*OPAMP::ptr() }
    }
}
#[doc = "Operational amplifiers"]
pub mod opamp;
#[doc = "Clock recovery system"]
pub struct CRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRS {}
impl CRS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crs::RegisterBlock {
        1073766400 as *const _
    }
}
impl Deref for CRS {
    type Target = crs::RegisterBlock;
    fn deref(&self) -> &crs::RegisterBlock {
        unsafe { &*CRS::ptr() }
    }
}
#[doc = "Clock recovery system"]
pub mod crs;
#[doc = "Universal serial bus full-speed device interface"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1073768448 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Universal serial bus full-speed device interface"]
pub mod usb;
#[doc = "Digital filter for sigma delta modulators"]
pub struct DFSDM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DFSDM {}
impl DFSDM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dfsdm::RegisterBlock {
        1073831936 as *const _
    }
}
impl Deref for DFSDM {
    type Target = dfsdm::RegisterBlock;
    fn deref(&self) -> &dfsdm::RegisterBlock {
        unsafe { &*DFSDM::ptr() }
    }
}
#[doc = "Digital filter for sigma delta modulators"]
pub mod dfsdm;
#[doc = "QuadSPI interface"]
pub struct QUADSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QUADSPI {}
impl QUADSPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const quadspi::RegisterBlock {
        2684358656 as *const _
    }
}
impl Deref for QUADSPI {
    type Target = quadspi::RegisterBlock;
    fn deref(&self) -> &quadspi::RegisterBlock {
        unsafe { &*QUADSPI::ptr() }
    }
}
#[doc = "QuadSPI interface"]
pub mod quadspi;
#[doc = "MCU debug component"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbgmcu::RegisterBlock {
        3758366720 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    fn deref(&self) -> &dbgmcu::RegisterBlock {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "MCU debug component"]
pub mod dbgmcu;
#[doc = "Floting point unit"]
pub struct FPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU {}
impl FPU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpu::RegisterBlock {
        3758157620 as *const _
    }
}
impl Deref for FPU {
    type Target = fpu::RegisterBlock;
    fn deref(&self) -> &fpu::RegisterBlock {
        unsafe { &*FPU::ptr() }
    }
}
#[doc = "Floting point unit"]
pub mod fpu;
#[doc = "SysTick timer"]
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const stk::RegisterBlock {
        3758153744 as *const _
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    fn deref(&self) -> &stk::RegisterBlock {
        unsafe { &*STK::ptr() }
    }
}
#[doc = "SysTick timer"]
pub mod stk;
#[doc = "Nested vectored interrupt controller"]
pub struct NVIC_STIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVIC_STIR {}
impl NVIC_STIR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvic_stir::RegisterBlock {
        3758157568 as *const _
    }
}
impl Deref for NVIC_STIR {
    type Target = nvic_stir::RegisterBlock;
    fn deref(&self) -> &nvic_stir::RegisterBlock {
        unsafe { &*NVIC_STIR::ptr() }
    }
}
#[doc = "Nested vectored interrupt controller"]
pub mod nvic_stir;
#[doc = "Floating point unit CPACR"]
pub struct FPU_CPACR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_CPACR {}
impl FPU_CPACR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpu_cpacr::RegisterBlock {
        3758157192 as *const _
    }
}
impl Deref for FPU_CPACR {
    type Target = fpu_cpacr::RegisterBlock;
    fn deref(&self) -> &fpu_cpacr::RegisterBlock {
        unsafe { &*FPU_CPACR::ptr() }
    }
}
#[doc = "Floating point unit CPACR"]
pub mod fpu_cpacr;
#[doc = "System control block ACTLR"]
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB_ACTRL {}
impl SCB_ACTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scb_actrl::RegisterBlock {
        3758153736 as *const _
    }
}
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    fn deref(&self) -> &scb_actrl::RegisterBlock {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
#[doc = "System control block ACTLR"]
pub mod scb_actrl;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "DAC1"]
    pub DAC1: DAC1,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "LCD"]
    pub LCD: LCD,
    #[doc = "TSC"]
    pub TSC: TSC,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "FIREWALL"]
    pub FIREWALL: FIREWALL,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM15"]
    pub TIM15: TIM15,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "LPTIM2"]
    pub LPTIM2: LPTIM2,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SDMMC"]
    pub SDMMC: SDMMC,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "VREFBUF"]
    pub VREFBUF: VREFBUF,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SWPMI1"]
    pub SWPMI1: SWPMI1,
    #[doc = "OPAMP"]
    pub OPAMP: OPAMP,
    #[doc = "CRS"]
    pub CRS: CRS,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "DFSDM"]
    pub DFSDM: DFSDM,
    #[doc = "QUADSPI"]
    pub QUADSPI: QUADSPI,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "FPU"]
    pub FPU: FPU,
    #[doc = "STK"]
    pub STK: STK,
    #[doc = "NVIC_STIR"]
    pub NVIC_STIR: NVIC_STIR,
    #[doc = "FPU_CPACR"]
    pub FPU_CPACR: FPU_CPACR,
    #[doc = "SCB_ACTRL"]
    pub SCB_ACTRL: SCB_ACTRL,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            DAC1: DAC1 {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            LCD: LCD {
                _marker: PhantomData,
            },
            TSC: TSC {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            FIREWALL: FIREWALL {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            LPTIM2: LPTIM2 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SDMMC: SDMMC {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            VREFBUF: VREFBUF {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SWPMI1: SWPMI1 {
                _marker: PhantomData,
            },
            OPAMP: OPAMP {
                _marker: PhantomData,
            },
            CRS: CRS {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            DFSDM: DFSDM {
                _marker: PhantomData,
            },
            QUADSPI: QUADSPI {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            FPU: FPU {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            FPU_CPACR: FPU_CPACR {
                _marker: PhantomData,
            },
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
        }
    }
}
