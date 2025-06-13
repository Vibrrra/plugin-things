use std::rc::Rc;

use plinth_plugin::{FloatParameter, Host, Parameter, Parameters};
use plugin_canvas_slint::{plugin_canvas::{event::EventResponse, Event}, view::PluginView};

use crate::parameters::{DelayParameter, DelayParameters};

slint::include_modules!();

pub struct DelayPluginView {
    delay_ui: DelayUI,
    parameters: Rc<DelayParameters>,
}

impl DelayPluginView {
    pub fn new(parameters: Rc<DelayParameters>, host: Rc<dyn Host>) -> Self {
        // let plugin_window = DelayUI::new().unwrap();

        let delay_ui = DelayUI::new().unwrap();
        delay_ui.global::<Util>().on_convert_time(move|x| {
            let min_log = 50.0f32.ln();
            let max_log = 2000.0f32.ln();
            let log_delay = min_log + x * (max_log - min_log);
            log_delay.exp().clamp( 50.0, 2000.0)
        });
        

        delay_ui.on_start_parameter_change({
            let host = host.clone();

            move |id| {
                host.start_parameter_change(id as _);
            }
        });

        delay_ui.on_change_parameter_value({
            let host = host.clone();

            move |id, value| {
                host.change_parameter_value(id as _, value as _);
            }
        });

        delay_ui.on_end_parameter_change({
            let host = host.clone();

            move |id| {
                host.end_parameter_change(id as _);
            }
        });

        Self {
            delay_ui,
            parameters,
        }
    }
}

impl PluginView for DelayPluginView {
    fn window(&self) -> &slint::Window {
        self.delay_ui.window()
    }

    fn on_event(&self, event: &Event) -> EventResponse {
        #[expect(clippy::single_match)]
        match event {
            Event::Draw => {
                let gain_parameter = self.parameters.typed::<FloatParameter>(DelayParameter::Time).unwrap();

                self.delay_ui.set_time(UiParameter {
                    id: gain_parameter.info().id() as _,
                    normalized_value: gain_parameter.normalized_value() as _,
                    default_normalized_value: gain_parameter.info().default_normalized_value() as _,
                    display_value: gain_parameter.to_string().into(),
                });


            }

            _ => {}
        }

        EventResponse::Ignored
    }
}
