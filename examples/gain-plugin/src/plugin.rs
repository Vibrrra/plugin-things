use std::collections::HashMap;
use std::io::{Read, Write};
use std::rc::Rc;

use plinth_plugin::clap::ClapPlugin;
use plinth_plugin::error::Error;
use plinth_plugin::vst3::Vst3Plugin;
use plinth_plugin::{
    export_clap, export_vst3, Event, Host, HostInfo, Parameters, Plugin, ProcessorConfig,
};

use crate::editor::DelayPluginEditor;
use crate::{parameters::DelayParameters, processor::DelayPluginProcessor};

#[derive(Default)]
struct DelayPlugin {
    parameters: Rc<DelayParameters>,
}

impl Plugin for DelayPlugin {
    const NAME: &'static str = "Gain Example";
    const VENDOR: &'static str = "Viiri Audio";
    const VERSION: &'static str = "0.1";

    type Processor = DelayPluginProcessor;
    type Editor = DelayPluginEditor;
    type Parameters = DelayParameters;

    fn new(_host_info: HostInfo) -> Self {
        Self::default()
    }

    fn with_parameters<T>(&self, mut f: impl FnMut(&Self::Parameters) -> T) -> T {
        f(&self.parameters)
    }

    fn process_event(&mut self, event: &Event) {
        self.parameters.process_event(event);
    }

    fn create_processor(&mut self, _config: ProcessorConfig) -> Self::Processor {
        DelayPluginProcessor::new((*self.parameters).clone())
    }

    fn create_editor(&mut self, host: Rc<dyn Host>) -> Self::Editor {
        DelayPluginEditor::new(host, self.parameters.clone())
    }

    fn save_state(&self, writer: &mut impl Write) -> Result<(), Error> {
        let serialized_parameters: HashMap<_, _> = self.parameters.serialize().collect();
        let parameters_json =
            serde_json::to_string(&serialized_parameters).map_err(|_| Error::SerializationError)?;
        write!(writer, "{parameters_json}")?;

        Ok(())
    }

    fn load_state(&mut self, reader: &mut impl Read) -> Result<(), Error> {
        let mut parameters_json = String::new();
        reader.read_to_string(&mut parameters_json)?;

        let serialized_parameters: HashMap<_, _> =
            serde_json::from_str(&parameters_json).map_err(|_| Error::SerializationError)?;
        self.parameters.deserialize(serialized_parameters)?;

        Ok(())
    }
}

// impl ClapPlugin for GainPlugin {
//     const CLAP_ID: &'static str = "viiri-audio.gain-example";
//     const FEATURES: &'static [plinth_plugin::clap::Feature] = &[
//         plinth_plugin::clap::Feature::AudioEffect,
//         plinth_plugin::clap::Feature::Stereo,
//     ];
// }

impl Vst3Plugin for DelayPlugin {
    const CLASS_ID: u128 = 0xE84410DB1788DC81;
    const SUBCATEGORIES: &'static [plinth_plugin::vst3::Subcategory] = &[
        plinth_plugin::vst3::Subcategory::Fx,
        plinth_plugin::vst3::Subcategory::Stereo,
    ];
}

// export_clap!(GainPlugin);
export_vst3!(DelayPlugin);
