extern crate image;
extern crate conrod;
extern crate glium;

use conrod::{color, Sizeable, Widget, Positionable, Colorable, Borderable};
use conrod::widget::{self, CommonBuilder};
use super::OverlayMarker;

#[derive(WidgetCommon)]
pub struct VehicleIcon<'a> {
    #[conrod(common_builder)]
    pub common: CommonBuilder,

    pub data: &'a OverlayMarker,
}

widget_ids!(
    struct Ids {
        master_container,
        background_circle,

        direction_container,
        direction_text,

        icon_image_container,
        icon_image,

        last_update_container,
        last_update_text,
        last_update_background,
    }
);

/// The `State` of the `Map` widget that will be cached within the `Ui`.
pub struct State {
    ids: Ids,
}

impl<'a> VehicleIcon<'a> {
    pub fn new(data: &'a OverlayMarker) -> Self {
        VehicleIcon {
            common: widget::CommonBuilder::default(),
            data: data,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
}

impl<'a> Widget for VehicleIcon<'a> {
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
        let widget::UpdateArgs { id, rect, state, ui, .. } = args;
        let VehicleIcon { data, .. } = self;
        let OverlayMarker { icon, color, secs_since_report, direction, .. } = data;

        let width = rect.w();
        let height = rect.h();
        
        let circle_diameter = height * 0.70;
        let image_diameter = circle_diameter * 0.7;

        let text_box_height = height * 0.25;
        let text_box_width = width * 0.6;
        let font_size = (text_box_height * 0.8) as u32;

        widget::Oval::fill([circle_diameter, circle_diameter])
                .color(*color)
                .mid_top_of(id)
                .graphics_for(id)
                .set(state.ids.background_circle, ui);

        if let Some(icon_image) = icon {
            widget::Image::new(*icon_image)
                .w_h(image_diameter, image_diameter)
                .middle_of(state.ids.background_circle)
                .graphics_for(id)
                .set(state.ids.icon_image, ui);
        }

        match direction {
            super::Direction::Inbound => {
                widget::Text::new("►")
                    .font_size(14)
                    .color(color::WHITE)
                    .right_justify()
                    .mid_right_with_margin_on(state.ids.background_circle, 1.0)
                    .set(state.ids.direction_text, ui);
            },
            super::Direction::Outbound => {
                widget::Text::new("◄")
                    .font_size(14)
                    .color(color::WHITE)
                    .right_justify()
                    .mid_left_with_margin_on(state.ids.background_circle, 1.0)
                    .set(state.ids.direction_text, ui);
            },
            super::Direction::Unknown => {},
        };
        
        widget::RoundedRectangle::fill([text_box_width, text_box_height], 5.0)
            .color(conrod::color::CHARCOAL)
            .mid_bottom_of(id)
            .set(state.ids.last_update_background, ui);

        widget::Text::new(&format!("{}s", secs_since_report))
            .font_size(font_size)
            .color(color::WHITE)
            .right_justify()
            .middle_of(state.ids.last_update_background)
            .set(state.ids.last_update_text, ui);
    }
}
