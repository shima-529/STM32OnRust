use crate::startup::Reset_Handler;
extern "C" {
#[no_mangle]
    fn _estack();
}


#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn NMI_Handler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn HardFault_Handler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn MemManage_Handler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn BusFault_Handler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn UsageFault_Handler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn SVC_Handler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn DebugMon_Handler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn PendSV_Handler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn SysTick_Handler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn WWDG_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn PVD_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn TAMP_STAMP_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn RTC_WKUP_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn FLASH_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn RCC_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn EXTI0_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn EXTI1_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn EXTI2_TSC_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn EXTI3_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn EXTI4_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn DMA1_Channel1_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn DMA1_Channel2_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn DMA1_Channel3_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn DMA1_Channel4_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn DMA1_Channel5_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn DMA1_Channel6_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn DMA1_Channel7_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn ADC1_2_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn CAN_TX_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn CAN_RX0_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn CAN_RX1_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn CAN_SCE_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn EXTI9_5_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn TIM1_BRK_TIM15_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn TIM1_UP_TIM16_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn TIM1_TRG_COM_TIM17_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn TIM1_CC_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn TIM2_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn TIM3_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn I2C1_EV_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn I2C1_ER_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn SPI1_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn USART1_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn USART2_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn USART3_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn EXTI15_10_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn RTC_Alarm_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn TIM6_DAC1_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn TIM7_DAC2_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn COMP2_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn COMP4_6_IRQHandler() {}
#[cfg_attr(all(feature="weak"), linkage="weak")]
    pub extern "C" fn FPU_IRQHandler() {}


pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[link_section=".isr_vector"]
#[no_mangle]
pub static isr_vectors: [Vector; 98] = [
	Vector{ handler:_estack},
	Vector{ handler: Reset_Handler},
	Vector{ handler: NMI_Handler},
	Vector{ handler: HardFault_Handler},
	Vector{ handler: MemManage_Handler},
	Vector{ handler: BusFault_Handler},
	Vector{ handler: UsageFault_Handler},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ handler: SVC_Handler},
	Vector{ handler: DebugMon_Handler},
	Vector{ reserved: 0},
	Vector{ handler: PendSV_Handler},
	Vector{ handler: SysTick_Handler},
	Vector{ handler: WWDG_IRQHandler},
	Vector{ handler: PVD_IRQHandler},
	Vector{ handler: TAMP_STAMP_IRQHandler},
	Vector{ handler: RTC_WKUP_IRQHandler},
	Vector{ handler: FLASH_IRQHandler},
	Vector{ handler: RCC_IRQHandler},
	Vector{ handler: EXTI0_IRQHandler},
	Vector{ handler: EXTI1_IRQHandler},
	Vector{ handler: EXTI2_TSC_IRQHandler},
	Vector{ handler: EXTI3_IRQHandler},
	Vector{ handler: EXTI4_IRQHandler},
	Vector{ handler: DMA1_Channel1_IRQHandler},
	Vector{ handler: DMA1_Channel2_IRQHandler},
	Vector{ handler: DMA1_Channel3_IRQHandler},
	Vector{ handler: DMA1_Channel4_IRQHandler},
	Vector{ handler: DMA1_Channel5_IRQHandler},
	Vector{ handler: DMA1_Channel6_IRQHandler},
	Vector{ handler: DMA1_Channel7_IRQHandler},
	Vector{ handler: ADC1_2_IRQHandler},
	Vector{ handler: CAN_TX_IRQHandler},
	Vector{ handler: CAN_RX0_IRQHandler},
	Vector{ handler: CAN_RX1_IRQHandler},
	Vector{ handler: CAN_SCE_IRQHandler},
	Vector{ handler: EXTI9_5_IRQHandler},
	Vector{ handler: TIM1_BRK_TIM15_IRQHandler},
	Vector{ handler: TIM1_UP_TIM16_IRQHandler},
	Vector{ handler: TIM1_TRG_COM_TIM17_IRQHandler},
	Vector{ handler: TIM1_CC_IRQHandler},
	Vector{ handler: TIM2_IRQHandler},
	Vector{ handler: TIM3_IRQHandler},
	Vector{ reserved: 0},
	Vector{ handler: I2C1_EV_IRQHandler},
	Vector{ handler: I2C1_ER_IRQHandler},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ handler: SPI1_IRQHandler},
	Vector{ reserved: 0},
	Vector{ handler: USART1_IRQHandler},
	Vector{ handler: USART2_IRQHandler},
	Vector{ handler: USART3_IRQHandler},
	Vector{ handler: EXTI15_10_IRQHandler},
	Vector{ handler: RTC_Alarm_IRQHandler},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ handler: TIM6_DAC1_IRQHandler},
	Vector{ handler: TIM7_DAC2_IRQHandler},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ handler: COMP2_IRQHandler},
	Vector{ handler: COMP4_6_IRQHandler},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ reserved: 0},
	Vector{ handler: FPU_IRQHandler},
];
