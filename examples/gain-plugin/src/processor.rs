use plinth_plugin::{plinth_core::signals::signal::{Signal, SignalMut}, BoolParameter, Event, FloatParameter, Parameters, ProcessState, Processor, Transport};

use crate::parameters::{DelayParameter, DelayParameters};

fn db_to_amplitude(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

// pub struct GainPluginProcessor {
//     parameters: GainParameters,
// }
pub struct DelayPluginProcessor {
    parameters: DelayParameters,
}

impl DelayPluginProcessor {
    pub fn new(parameters: DelayParameters) -> Self {
        Self {
            parameters,
        }
    }
}

impl Processor for DelayPluginProcessor {
    fn reset(&mut self) {
    }

    fn process(
        &mut self,
        buffer: &mut impl SignalMut,
        _aux: Option<&impl Signal>,
        _transport: Option<Transport>,
        events: impl Iterator<Item = Event>
    ) -> ProcessState {
        for event in events {
            self.parameters.process_event(&event);
        }
        let transport = _transport.unwrap();
        let tempo = transport.tempo();

        if self.parameters.value::<BoolParameter>(DelayParameter::Sync) == true {
            
        }  
        let mix = self.parameters.value::<FloatParameter>(DelayParameter::Mix);

        // let gain_db = self.parameters.value::<FloatParameter>(DelayParameter::Gain);
        // let gain = db_to_amplitude(mix as _);

        for channel in buffer.iter_channels_mut() {
            for sample in channel.iter_mut() {
                *sample *= mix as f32;
            }
        }        

        ProcessState::Normal
    }

    fn process_events(&mut self, events: impl Iterator<Item = Event>) {
        for event in events {
            self.parameters.process_event(&event);
            
        }
    }
}
