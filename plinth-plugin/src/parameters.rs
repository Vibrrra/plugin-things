pub mod bool;
pub mod enums;
pub mod float;
pub mod formatter;
pub mod group;
pub mod info;
pub mod int;
pub mod kind;
pub mod map;
pub mod parameter;
pub mod range;

pub type ParameterId = u32;
pub type ParameterValue = f64;

pub type ModulationChangedCallback = Arc<dyn Fn(ParameterId, ParameterValue) + Send + Sync>;

use std::any::Any;
use std::{collections::HashSet, sync::Arc};

use parameter::{Parameter, ParameterPlain};

use crate::error::Error;
use crate::Event;

pub fn has_duplicates(ids: &[ParameterId]) -> bool {
    let mut set = HashSet::new();
    ids.iter().any(|id| !set.insert(id))
}

pub trait Parameters {
    /// The list of parameter IDs is cached and shouldn't change at runtime
    fn ids(&self) -> &[ParameterId];

    fn get(&self, id: impl Into<ParameterId>) -> Option<&dyn Parameter>;

    fn typed<T: Parameter>(&self, id: impl Into<ParameterId>) -> Option<&T> {
        self.get(id)
            .and_then(|parameter| {
                let any_parameter = parameter as &dyn Any;
                any_parameter.downcast_ref::<T>()
            })
    }

    fn value<T: ParameterPlain>(&self, id: impl Into<ParameterId>) -> T::Plain {
        self.typed::<T>(id).unwrap().plain()
    }

    fn modulated_value<T: ParameterPlain>(&self, id: impl Into<ParameterId>) -> T::Plain {
        self.typed::<T>(id).unwrap().modulated_plain()
    }

    fn process_event(&self, event: &Event) {
        match event {
            Event::ParameterValue { id, value, .. } => {
                let parameter = self.get(*id).unwrap_or_else(|| panic!("Tried to get parameter with id {id} but it doesn't exist"));
                parameter.set_normalized_value(*value).unwrap();
            },

            Event::ParameterModulation { id, amount, .. } => {
                let parameter = self.get(*id).unwrap_or_else(|| panic!("Tried to get parameter with id {id} but it doesn't exist"));
                parameter.set_normalized_modulation(*amount);
            },

            _ => {},
        }
    }

    fn reset(&self) {
        for id in self.ids().iter().copied() {
            let parameter = self.get(id).unwrap();
            parameter.set_normalized_value(parameter.info().default_normalized_value()).unwrap();
        }
    }

    fn serialize(&self) -> impl Iterator<Item = (ParameterId, ParameterValue)> {
        self.ids().iter()
            .map(|&id| {
                let parameter = self.get(id);
                (id, parameter.unwrap().serialize_value())
            })
    }

    fn deserialize(&self, parameters: impl IntoIterator<Item = (ParameterId, ParameterValue)>) -> Result<(), Error> {
        self.reset();

        for (id, value) in parameters.into_iter() {
            let parameter = match self.get(id) {
                Some(id) => id,
                None => {
                    return Err(Error::ParameterIdError);
                },
            };

            parameter.deserialize_value(value)?;
        }

        Ok(())
    }
}
