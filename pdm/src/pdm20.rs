use embedded_can::Frame;
use messages::OutputState;
use messages::pdm20::Control;
use messages::pdm20::ControlMuxM0;
use messages::pdm20::pgn;
use saelient::Id;
use socketcan::{CanFrame, tokio::CanSocket};
use std::io;

pub type Outputs = crate::Outputs<20>;

/// PDM20 interface.
pub struct Pdm20 {
    interface: CanSocket,
    address: u8,
}

impl Pdm20 {
    /// Connect to a PDM20.
    pub fn new(interface: CanSocket, address: u8) -> Self {
        Self { interface, address }
    }

    /// Set a single output on or off.
    pub async fn set_output(&self, output: usize, on: bool) -> Result<(), io::Error> {
        self.set_outputs_pwm(Outputs::new().ch(output, OutputState::from(on)), 1.0)
            .await
    }

    /// Set a single output with a PWM duty.
    pub async fn set_output_pwm(&self, output: usize, duty: f32) -> Result<(), io::Error> {
        self.set_outputs_pwm(Outputs::new().ch(output, OutputState::On), duty)
            .await
    }

    /// Set one or more outputs.
    pub async fn set_outputs(&self, outputs: Outputs) -> Result<(), io::Error> {
        self.set_outputs_pwm(outputs, 1.0).await
    }

    /// Set a number of outputs with a PWM duty.
    ///
    /// `pwm` is clamped to \[0.0, 1.0\].
    pub async fn set_outputs_pwm(&self, outputs: Outputs, pwm_duty: f32) -> Result<(), io::Error> {
        let duty = (pwm_duty.clamp(0.0, 1.0) * 255.0) as u8;

        let outputs = outputs.as_slice();
        let mut mux = ControlMuxM0::new();
        mux.set_output_1(outputs[0].into()).unwrap();
        mux.set_output_2(outputs[1].into()).unwrap();
        mux.set_output_3(outputs[2].into()).unwrap();
        mux.set_output_4(outputs[3].into()).unwrap();
        mux.set_output_5(outputs[4].into()).unwrap();
        mux.set_output_6(outputs[5].into()).unwrap();
        mux.set_output_7(outputs[6].into()).unwrap();
        mux.set_output_8(outputs[7].into()).unwrap();
        mux.set_output_9(outputs[8].into()).unwrap();
        mux.set_output_10(outputs[9].into()).unwrap();
        mux.set_output_11(outputs[10].into()).unwrap();
        mux.set_output_12(outputs[11].into()).unwrap();
        mux.set_output_13(outputs[12].into()).unwrap();
        mux.set_output_14(outputs[13].into()).unwrap();
        mux.set_output_15(outputs[14].into()).unwrap();
        mux.set_output_16(outputs[15].into()).unwrap();
        mux.set_output_17(outputs[16].into()).unwrap();
        mux.set_output_18(outputs[17].into()).unwrap();
        mux.set_output_19(outputs[18].into()).unwrap();
        mux.set_output_20(outputs[19].into()).unwrap();
        mux.set_pwm_duty(duty).unwrap();
        let mut frame = Control::new(0).unwrap();
        frame.set_m0(mux).unwrap();

        let id = Id::builder()
            .da(self.address)
            .sa(0)
            .pgn(pgn::CONTROL)
            .priority(3)
            .build()
            .unwrap();

        self.interface
            .write_frame(CanFrame::new(id, frame.data()).unwrap())
            .await?;

        Ok(())
    }
}
