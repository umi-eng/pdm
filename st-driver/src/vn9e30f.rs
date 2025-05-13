//! VN9E30F
//!
//! 6-channel high-side driver.
//!
//! [Datasheet](https://www.st.com/resource/en/datasheet/vn9e30f.pdf)

device_driver::create_device!(
    device_name: Vn9e30f,
    dsl: {
        config {
            type RegisterAddressType = u8;
        }

        /// Output control register
        register OutCtrCr {
            type Access = RW;
            type ByteOrder = BE;
            const ADDRESS = 0x00;
            const SIZE_BITS = 16;
            const REPEAT = {
              count: 6,
              stride: 1,
            };

            /// WDTB: watchdog toggle bit
            wdtb: bool = 1,
            /// OLOFFCR: enables an internal pull-up current generator to
            /// distinguish between the two faults: open-load OFF-state vs the
            /// output shorted to VCC fault.
            oloffcr: bool = 2,
            /// DUTY_CR[9:0]: set the duty cycle value. Bit 9 (MSB) - Bit 0 (LSB)
            duty: uint = 4..=13,
        },

        /// Output configuration register
        register OutCfgR {
            type Access = RW;
            type ByteOrder = BE;
            const ADDRESS = 0x08;
            const SIZE_BITS = 16;
            const REPEAT = {
                count: 6,
                stride: 1,
            };

            /// VDSMASK: VDS detection at turn-off masking bit
            vdsmask: bool = 1,
            /// DIENCR: Direct input signal enable in normal mode (according to OTP allocation)
            diencr: bool = 2,
            /// CCR: Set the channel configuration (BULB-LED)
            ccr: uint as enum ChannelKind {
                Bulb = 0,
                Led = 1,
            } = 3..=3,
            /// PWMFCY[1:0]: PWM frequency selection
            pwmfcy: uint as enum PwmFreq {
                Ratio1024 = 0b00,
                Ratio2048 = 0b01,
                Ratio4096 = 0b10,
                Ratio512 = 0b11,
            } = 4..=5,
            /// SPCR[1:0]: Current sampling point
            spcr: uint as enum CurrentSamplePoint {
                Stop = 0b00,
                Start = 0b01,
                Continuous = 0b10,
                Filtered = 0b11,
            } = 6..=7,
            /// CHPHA[4:0]: Set the Channel phase value
            chpha: uint = 8..=12,
            /// SLOPECR[1:0]: Switching slope control bit 1 (MSB) and 0 (LSB)
            slopecr: uint as enum SlopeControl {
                Standard = 0b00,
                Fast = 0b01,
                Faster = 0b10,
                Fastest = 0b11,
            } = 14..=15,
        },

        /// Channel latch OFF timer control register
        register ChlOffTcr0 {
            type Access = RW;
            type ByteOrder = BE;
            const ADDRESS = 0x10;
            const SIZE_BITS = 16;

            /// CHLOFFTCR: Configure the output behavior in case of power
            /// limitation for the corresponding channel 0.
            chlofftcr0: uint = 4..=7,
            /// CHLOFFTCR: Configure the output behavior in case of power
            /// limitation for the corresponding channel 1.
            chlofftcr1: uint = 8..=11,
            /// CHLOFFTCR: Configure the output behavior in case of power
            /// limitation for the corresponding channel 2.
            chlofftcr2: uint = 12..=15,
        },

        /// Channel latch OFF timer control register
        register ChlOffTcr1 {
            type Access = RW;
            type ByteOrder = BE;
            const ADDRESS = 0x11;
            const SIZE_BITS = 16;

            /// CHLOFFTCR: Configure the output behavior in case of power
            /// limitation for the corresponding channel 3.
            chlofftcr3: uint = 4..=7,
            /// CHLOFFTCR: Configure the output behavior in case of power
            /// limitation for the corresponding channel 4.
            chlofftcr4: uint = 8..=11,
            /// CHLOFFTCR: Configure the output behavior in case of power
            /// limitation for the corresponding channel 4.
            chlofftcr5: uint = 12..=15,
        },

        /// Channel control register
        register Socr {
            type Access = RW;
            type ByteOrder = BE;
            const ADDRESS = 0x13;
            const SIZE_BITS = 16;

            /// Watchdog toggle bit
            wdtb: bool = 1,
            /// SOCR0: bit controls output state of channel 0
            socr0: bool = 8,
            /// SOCR1: bit controls output state of channel 1
            socr1: bool = 9,
            /// SOCR2: bit controls output state of channel 2
            socr2: bool = 10,
            /// SOCR3: bit controls output state of channel 3
            socr3: bool = 11,
            /// SOCR4: bit controls output state of channel 4
            socr4: bool = 12,
            /// SOCR5: bit controls output state of channel 5
            socr5: bool = 13,
        },

        /// Control register
        register Ctrl {
            type Access = RW;
            type ByteOrder = BE;
            const ADDRESS = 0x14;
            const SIZE_BITS = 16;

            /// PWMSYNC: PWM clock synchronisation
            pwmsync: bool = 1,
            /// LOCKEN: Protected transaction mode
            locken: uint = 2..=6,
            /// PWM_TRIG: PWM triggering mode
            pwm_trig: uint as enum PwmTrigger {
                RisingEdge = 0,
                FallingEdge = 1,
            } = 10..=10,
            /// EN: Enter normal mode
            en: bool = 11,
            /// CTDTH: Case thermal detection threshold
            ctdth: uint as enum ThermalThreshold {
                Temp120C = 0b00,
                Temp130C = 0b01,
                Temp140C = 0b10,
                Temp140C2 = 0b11,
            } = 12..=13,
            /// UNLOCK: Allows protected SPI transactions
            unlock: bool = 14,
            /// GOSTBY: Go to standby
            gostby: bool = 15,
        },

        /// Output status register
        register OutSr {
            type Access = RW;
            type ByteOrder = BE;
            const ADDRESS = 0x20;
            const SIZE_BITS = 16;
            const REPEAT = {
                count: 6,
                stride: 1,
            };

            /// VCCUV: Vcc undervoltage
            vccuv: bool = 4,
            /// PWMCLOCKLOW: PWM clock frequency too low
            pwmclocklow: bool = 5,
            /// SPIE: SPI error
            spie: bool = 6,
            /// RST: Chip reset
            rst: bool = 7,
            /// CHLOFFSRx: Channel latch-off status
            chloffsr: bool = 8,
            /// OLPUSRx: Output pull-up generator status
            olpusr: bool = 9,
            /// STKFLTRx: Output stuck to Vcc/open-load off state status
            stkfltr: bool = 10,
            /// VDSFSRx: VDS feedback status
            vdsfsr: bool = 11,
            /// CHFBSTx: Channnel feedback status
            chfbst: bool = 12,
            /// DIOTP0: Associated DIx input description bit 0
            diotp0: bool = 13,
            /// DIOTP1: Associated DIx input description bit 1
            diotp1: bool = 14,
            /// DIENSR: Direct input status, image of associated DI logic level according to OTP allocation
            diensr: bool = 15,
        },

        /// Digital current sense register
        register AdcSr {
            type Access = RW;
            type ByteOrder = BE;
            const ADDRESS = 0x28;
            const SIZE_BITS = 16;
            const REPEAT = {
                count: 6,
                stride: 1,
            };

            /// UPDTSR: updated status bit
            updtsr: bool = 1,
            /// SOCRx: output state of channel
            socr: bool = 2,
            /// ADCxSR: the 10-bit digital value of output current
            adcsr: uint = 4..=13,
        },

        /// Digital case thermal sensor voltage register
        register Adc9Sr {
            type Access = RW;
            type ByteOrder = BE;
            const ADDRESS = 0x31;
            const SIZE_BITS = 16;

            /// UPDTSR: updated status bit
            updtsr: bool = 1,
            /// ADC9SR: the 10-bit digital value of the temperature sensor voltage
            tcase: int = 4..=13,
        },
    }
);
