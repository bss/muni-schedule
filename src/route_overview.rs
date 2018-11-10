extern crate image;
extern crate conrod;
extern crate glium;

use std::cmp;
use conrod::{color, Sizeable, Widget, Positionable, Colorable, Borderable};
use conrod::widget::{self, CommonBuilder};

pub struct RouteData {
    pub name: String,
    pub background_color: conrod::Color,
    pub inbounds: Vec<String>,
    pub outbounds: Vec<String>,
}

#[derive(WidgetCommon)]
pub struct RouteOverview<'a> {
    #[conrod(common_builder)]
    pub common: CommonBuilder,

    pub data: &'a RouteData,
}

widget_ids!(
    struct Ids {
        master_container,
        name_container,
        name_text,
        timing_cols,

        timing_list_left,
        timing_list_left_text,
        timing_list_right,
        timing_list_right_text,

        test,
    }
);

/// The `State` of the `Map` widget that will be cached within the `Ui`.
pub struct State {
    ids: Ids,
}

impl<'a> RouteOverview<'a> {
    pub fn new(data: &'a RouteData) -> Self {
        RouteOverview {
            common: widget::CommonBuilder::default(),
            data: data,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
}

const PAD: conrod::Scalar = 20.0;

impl<'a> Widget for RouteOverview<'a> {
    type State = State;
    type Style = Style;
    type Event = ();

    fn init_state(&self, id_generator: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_generator),
        }
    }

    fn style(&self) -> Self::Style {
        Style::default()
    }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { state, ui, .. } = args;
        let RouteOverview { data, .. } = self;
        let RouteData { name, background_color, inbounds, outbounds, .. } = data;

        widget::Canvas::new().border(0.0).flow_right(&[
            (state.ids.name_container, widget::Canvas::new().border(0.0).length(100.0).color(background_color.with_alpha(0.4))),
            (state.ids.timing_cols, widget::Canvas::new().border(0.0).flow_right(&[
                (state.ids.timing_list_left, widget::Canvas::new().border(0.0).color(background_color.with_alpha(0.4))),
                (state.ids.timing_list_right, widget::Canvas::new().border(0.0).color(background_color.with_alpha(0.4))),
            ])),
        ]).set(state.ids.master_container, ui);

        widget::Text::new(name)
            .color(color::WHITE)
            .font_size(64)
            .w(64.0)
            .h(64.0*1.3)
            .center_justify()
            .line_spacing(0.0)
            .no_line_wrap()
            .middle_of(state.ids.name_container)
            .set(state.ids.name_text, ui);

        if inbounds.len() > 0 {
            widget::Text::new(&format!("Inbound\n{}", inbounds.get(0..cmp::min(3, inbounds.len())).unwrap_or(&Vec::new()).join("\n")))
                    .color(color::WHITE)
                    .padded_w_of(state.ids.timing_list_left, PAD)
                    .mid_top_with_margin_on(state.ids.timing_list_left, 5.0)
                    .left_justify()
                    .line_spacing(4.0)
                    .set(state.ids.timing_list_left_text, ui);
        }

        if outbounds.len() > 0 {
            widget::Text::new(&format!("Outbound\n{}", outbounds.get(0..cmp::min(3, outbounds.len())).unwrap_or(&Vec::new()).join("\n")))
                    .color(color::WHITE)
                    .padded_w_of(state.ids.timing_list_right, PAD)
                    .mid_top_with_margin_on(state.ids.timing_list_right, 5.0)
                    .left_justify()
                    .line_spacing(4.0)
                    .set(state.ids.timing_list_right_text, ui);
        }

    }
}
