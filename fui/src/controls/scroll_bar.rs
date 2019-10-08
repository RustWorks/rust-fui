use std::cell::RefCell;
use std::rc::Rc;

use children_source::*;
use common::*;
use control::*;
use control_object::*;
use drawing::primitive::Primitive;
use drawing::units::{UserPixelPoint, UserPixelRect, UserPixelSize, UserPixelThickness};
use drawing_context::DrawingContext;
use events::*;
use observable::*;
use style::*;
use typed_builder::TypedBuilder;
use view::*;

#[derive(TypedBuilder)]
pub struct ScrollBar {
    #[builder(default = Orientation::Vertical)]
    pub orientation: Orientation,

    #[builder(default = Property::new(0.0f32))]
    pub min_value: Property<f32>,

    #[builder(default = Property::new(1.0f32))]
    pub max_value: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub value: Property<f32>,

    #[builder(default = Property::new(0.0f32))]
    pub viewport_size: Property<f32>,
}

impl View for ScrollBar {
    fn to_view(self, context: ViewContext) -> Rc<RefCell<ControlObject>> {
        Control::new(self, ScrollBarDefaultStyle::new(), context)
    }
}

//
// ScrollBar Default Style
//

pub struct ScrollBarDefaultStyle {
    rect: Rect,
    thumb_pos_px: f32,
    thumb_size_px: f32,

    is_thumb_hover: Property<bool>,
    is_thumb_pressed: Property<bool>,
    pressed_position: Point,

    event_subscriptions: Vec<EventSubscription>,
}

impl ScrollBarDefaultStyle {
    pub fn new() -> Self {
        ScrollBarDefaultStyle {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                width: 0f32,
                height: 0f32,
            },
            thumb_pos_px: 0f32,
            thumb_size_px: 0f32,
            is_thumb_hover: Property::new(false),
            is_thumb_pressed: Property::new(false),
            pressed_position: Point::new(0.0f32, 0.0f32),
            event_subscriptions: Vec::new(),
        }
    }

    fn calc_sizes(&mut self, data: &ScrollBar) {
        let scroll_bar_size_px = match data.orientation {
            Orientation::Horizontal => self.rect.width - self.rect.x,
            Orientation::Vertical => self.rect.height - self.rect.y,
        };
        let scroll_bar_size_f32 =
            data.max_value.get() - data.min_value.get() + data.viewport_size.get();

        self.thumb_size_px =
            ((data.viewport_size.get() * scroll_bar_size_px) / scroll_bar_size_f32).max(20.0f32);

        self.thumb_pos_px = (scroll_bar_size_px - self.thumb_size_px)
            * (data.value.get() - data.min_value.get())
            / (data.max_value.get() - data.min_value.get());
    }
}

impl Style<ScrollBar> for ScrollBarDefaultStyle {
    fn setup_dirty_watching(
        &mut self,
        data: &mut ScrollBar,
        control: &Rc<RefCell<Control<ScrollBar>>>,
    ) {
        self.event_subscriptions
            .push(self.is_thumb_hover.dirty_watching(control));
        self.event_subscriptions
            .push(self.is_thumb_pressed.dirty_watching(control));

        self.event_subscriptions
            .push(data.min_value.dirty_watching(control));
        self.event_subscriptions
            .push(data.max_value.dirty_watching(control));
        self.event_subscriptions
            .push(data.value.dirty_watching(control));
        self.event_subscriptions
            .push(data.viewport_size.dirty_watching(control));
    }

    fn handle_event(
        &mut self,
        data: &mut ScrollBar,
        children: &Box<dyn ChildrenSource>,
        event: ControlEvent,
    ) {
        match event {
            ControlEvent::TapDown { position } => {
                if position.x >= self.thumb_pos_px
                    && position.x < self.thumb_pos_px + self.thumb_size_px
                {
                    self.is_thumb_pressed.set(true);
                    self.pressed_position = position;
                }
            }

            ControlEvent::TapUp { ref position } => {
                if let HitTestResult::Current = self.hit_test(&data, &children, *position) {
                    //data.clicked.emit(());
                }
                self.is_thumb_pressed.set(false);
            }

            ControlEvent::TapMove { ref position } => {
                if self.is_thumb_pressed.get() {
                    let scroll_bar_size_px = match data.orientation {
                        Orientation::Horizontal => self.rect.width - self.rect.x,
                        Orientation::Vertical => self.rect.height - self.rect.y,
                    };
                    let offset_x = position.x - self.pressed_position.x;
                    let current_value = data.value.get();
                    let new_value = (data.min_value.get()
                        + (self.thumb_pos_px + offset_x)
                            * (data.max_value.get() - data.min_value.get())
                            / (scroll_bar_size_px - self.thumb_size_px))
                        .max(data.min_value.get())
                        .min(data.max_value.get());

                    if new_value != current_value {
                        self.pressed_position = *position;
                        data.value.set(new_value);
                    }
                }
            }

            ControlEvent::HoverEnter => {
                self.is_thumb_hover.set(true);
            }

            ControlEvent::HoverLeave => {
                self.is_thumb_hover.set(false);
            }

            _ => (),
        }
    }

    fn measure(
        &mut self,
        data: &ScrollBar,
        _children: &Box<dyn ChildrenSource>,
        _drawing_context: &mut DrawingContext,
        _size: Size,
    ) {
        match data.orientation {
            Orientation::Horizontal => {
                self.rect = Rect::new(0.0f32, 0.0f32, 150.0f32, 20.0f32);
            }
            Orientation::Vertical => {
                self.rect = Rect::new(0.0f32, 0.0f32, 20.0f32, 150.0f32);
            }
        }
    }

    fn set_rect(&mut self, data: &ScrollBar, _children: &Box<dyn ChildrenSource>, rect: Rect) {
        self.rect = rect;
        self.calc_sizes(data);
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(
        &self,
        _data: &ScrollBar,
        _children: &Box<dyn ChildrenSource>,
        point: Point,
    ) -> HitTestResult {
        if point.is_inside(&self.rect) {
            HitTestResult::Current
        } else {
            HitTestResult::Nothing
        }
    }

    fn to_primitives(
        &self,
        data: &ScrollBar,
        _children: &Box<dyn ChildrenSource>,
        _drawing_context: &mut DrawingContext,
    ) -> Vec<Primitive> {
        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let scroll_bar_size_px = match data.orientation {
            Orientation::Horizontal => width - x,
            Orientation::Vertical => height - y,
        };

        let background = [0.1, 0.5, 0.0, 0.2];

        let mut vec = Vec::new();
        if self.thumb_pos_px > 0.0f32 {
            vec.push(Primitive::Rectangle {
                color: background,
                rect: UserPixelRect::new(
                    UserPixelPoint::new(x, y),
                    UserPixelSize::new(self.thumb_pos_px, height),
                ),
            });
        }

        default_theme::button(
            &mut vec,
            x + self.thumb_pos_px,
            y + 1.0f32,
            self.thumb_size_px,
            height - 2.0f32,
            self.is_thumb_pressed.get(),
            self.is_thumb_hover.get(),
        );

        if self.thumb_pos_px + self.thumb_size_px < scroll_bar_size_px {
            vec.push(Primitive::Rectangle {
                color: background,
                rect: UserPixelRect::new(
                    UserPixelPoint::new(x + self.thumb_pos_px + self.thumb_size_px, y),
                    UserPixelSize::new(
                        scroll_bar_size_px - self.thumb_pos_px - self.thumb_size_px,
                        height,
                    ),
                ),
            });
        }

        default_theme::border_3d(&mut vec, x, y, width, height, true);

        vec
    }
}