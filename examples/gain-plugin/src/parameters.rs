use std::sync::Arc;

use plinth_derive::ParameterKind;
use plinth_plugin::{parameters::bool::ValueChangedCallback, BoolParameter, FloatFormatter, FloatParameter, LinearFloatRange, LogFloatRange, Parameter, ParameterId, ParameterMap, ParameterRange, Parameters};

const MIN_GAIN: f64 = -80.0;
const MAX_GAIN: f64 = 80.0;

#[derive(ParameterKind)]
pub enum DelayParameter {
    Time,
    Sync,
    Feedback,
    Freq,
    Mix,
}

#[derive(Clone)]
pub struct DelayParameters {
    map: ParameterMap,
}

impl Default for DelayParameters {
    fn default() -> Self {
        let mut map = ParameterMap::new();
        
        map.add(
            FloatParameter::new(
                DelayParameter::Time,
                "Time",
                Arc::new(LinearFloatRange::new(MIN_GAIN, MAX_GAIN)),
            )
            .with_default_value(0.0)
            .with_formatter(Arc::new(FloatFormatter::new(1, "dB")))
        );
        map.add(BoolParameter::new(
            DelayParameter::Sync,
            "Sync",
        ));
        map.add(FloatParameter::new(
            DelayParameter::Mix,
            "Mix",
            Arc::new(LinearFloatRange::new(0.0, 1.0))
        ));

        let func = |id, x| {};
        let float_range = LogFloatRange::new(50.0, 2000.0, 2.0);
        let default = float_range.plain_to_normalized(0.5);
        map.add(FloatParameter::new(
            DelayParameter::Sync,
            "Time",
            Arc::new(float_range)
        ).with_default_value(default.unwrap()).on_value_changed(Arc::new(func)));
        Self {
            map,
        }
    }
}

impl Parameters for DelayParameters {
    fn ids(&self) -> &[ParameterId] {
        self.map.ids()
    }

    fn get(&self, id: impl Into<ParameterId>) -> Option<&dyn Parameter> {
        self.map.get(id)
    }
}
