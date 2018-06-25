use std::cell::RefCell;
use std::rc::{ Rc, Weak };

use control::*;
use common::*;
use drawing_context::DrawingContext;
use drawing::primitive::Primitive;
use drawing::units::{ UserPixelRect, UserPixelPoint, UserPixelThickness, UserPixelSize };
use callback::*;
use events::*;

pub struct ButtonProperties {
    pub content: Rc<RefCell<ControlObject>>,
}

pub struct ButtonEvents {
    pub clicked: Callback<()>,
}

pub struct ButtonState {
    pub is_hover: bool,
    pub is_pressed: bool,
}

pub struct ButtonData {
    pub properties: ButtonProperties,
    pub events: ButtonEvents,
    pub state: ButtonState,
    pub parent: Option<Weak<RefCell<ControlObject>>>,
    is_dirty: bool,
}

pub struct Button {
    pub data: ButtonData,
    style: Box<Style<ButtonData>>,
}

impl Button {
    pub fn new(content: Rc<RefCell<ControlObject>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Button {
            data: ButtonData {
                properties: ButtonProperties { content: content },
                events: ButtonEvents { clicked: Callback::new() },
                state: ButtonState { is_hover: false, is_pressed: false },
                parent: None,
                is_dirty: true,
            },
            style: Box::new(ButtonDefaultStyle {
                rect: Rect { x: 0f32, y: 0f32, width: 0f32, height: 0f32 },
            }),
        }))
    }
}

impl Control for Button {
    type Data = ButtonData;

    fn get_data(&self) -> &Self::Data {
        &self.data
    }

    fn get_style(&self) -> &Box<Style<Self::Data>> {
        &self.style
    }

    fn is_dirty(&self) -> bool {
        self.data.is_dirty
    }
    
    fn set_is_dirty(&mut self, is_dirty: bool) {
        self.data.is_dirty = is_dirty;
        if is_dirty {
            if let Some(ref parent) = (self as &mut Control<Data = ButtonData>).get_parent() {
                parent.borrow_mut().set_is_dirty(is_dirty)
            }
        }
    }

    fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>> {
        if let Some(ref test) = self.data.parent {
            test.upgrade()
        } else {
            None
        }
    }

    fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>) {
        self.data.parent = Some(parent);
    }

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        Vec::new()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        match event {
            ControlEvent::TapDown{ .. } => {
                self.data.state.is_pressed = true; true
            },

            ControlEvent::TapUp{ ref position } => {
                if let HitTestResult::Current = self.style.hit_test(&self.data, *position) {
                    self.data.events.clicked.emit(&());
                }
                self.data.state.is_pressed = false;
                true
            },

            ControlEvent::TapMove{ ref position } => {
                if let HitTestResult::Current = self.style.hit_test(&self.data, *position) {
                    self.data.state.is_pressed = true;
                } else {
                    self.data.state.is_pressed = false;
                }
                true
            },

            ControlEvent::HoverEnter => {
                self.data.state.is_hover = true; true
            },

            ControlEvent::HoverLeave => {
                self.data.state.is_hover = false; true
            },

            _ => false
        }
    }
}


//
// Button Default Style
//

pub struct ButtonDefaultStyle {
    rect: Rect,
}

impl Style<ButtonData> for ButtonDefaultStyle {

    fn get_preferred_size(&self, data: &ButtonData,
        drawing_context: &mut DrawingContext, size: Size) -> Size {
        let content_size = data.properties.content.borrow().get_preferred_size(drawing_context, size);
        Size::new(content_size.width + 20.0f32, content_size.height + 20.0f32)
    }

    fn set_rect(&mut self, data: &mut ButtonData, rect: Rect) {
        self.rect = rect;

        let content_rect = Rect::new(rect.x + 10.0f32, rect.y + 10.0f32, rect.width - 20.0f32, rect.height - 20.0f32);
        data.properties.content.borrow_mut().set_rect(content_rect);
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn hit_test(&self, _data: &ButtonData, point: Point) -> HitTestResult {
        if point.is_inside(&self.rect) { HitTestResult::Current } else { HitTestResult::Nothing }
    }

    fn to_primitives(&self, data: &ButtonData,
        drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        let mut vec = Vec::new();

        let x = self.rect.x;
        let y = self.rect.y;
        let width = self.rect.width;
        let height = self.rect.height;

        let background = if data.state.is_pressed { [0.1, 0.5, 0.0, 0.2] }
            else { if data.state.is_hover { [0.1, 1.0, 0.0, 0.4] } else { [0.1, 1.0, 0.0, 0.2] } };
        let line_color1 = if !data.state.is_pressed { [1.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] };
        let line_color2 = if !data.state.is_pressed { [0.0, 0.0, 0.0, 1.0] } else { [1.0, 1.0, 1.0, 1.0] };

        vec.push(Primitive::Rectangle {
            color: background,
            rect: UserPixelRect::new(UserPixelPoint::new(x + 1.0, y + 1.0),
                UserPixelSize::new(width - 2.0, height - 2.0))
        });

        vec.push(Primitive::Line {
            color: line_color1,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: line_color1,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
        });
        vec.push(Primitive::Line {
            color: line_color2,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + 0.5),
            end_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
        });
        vec.push(Primitive::Line {
            color: line_color2,
            thickness: UserPixelThickness::new(1.0f32),
            start_point: UserPixelPoint::new(x + width - 1.0 + 0.5, y + height - 1.0 + 0.5),
            end_point: UserPixelPoint::new(x + 0.5, y + height - 1.0 + 0.5),
        });

        let mut vec2 = data.properties.content.borrow_mut().to_primitives(drawing_context);
        vec.append(&mut vec2);

        vec
    }

}


//
// object safe trait
//

impl ControlObject for Button {
    fn is_dirty(&self) -> bool {
        (self as &Control<Data = ButtonData>).is_dirty()
    }

    fn set_is_dirty(&mut self, is_dirty: bool) {
        (self as &mut Control<Data = ButtonData>).set_is_dirty(is_dirty);
    }

    fn get_parent(&self) -> Option<Rc<RefCell<ControlObject>>> {
        (self as &Control<Data = ButtonData>).get_parent()
    }

    fn set_parent(&mut self, parent: Weak<RefCell<ControlObject>>) {
        (self as &mut Control<Data = ButtonData>).set_parent(parent);
    }

    fn get_children(&mut self) -> Vec<Rc<RefCell<ControlObject>>> {
        (self as &mut Control<Data = ButtonData>).get_children()
    }

    fn handle_event(&mut self, event: ControlEvent) -> bool {
        (self as &mut Control<Data = ButtonData>).handle_event(event)
    }

    fn get_preferred_size(&self, drawing_context: &mut DrawingContext, size: Size) -> Size {
        self.get_style().get_preferred_size(self.get_data(), drawing_context, size)
    }

    fn set_rect(&mut self, rect: Rect) {
        let style = &mut self.style;
        let data = &mut self.data;
        style.set_rect(data, rect);
    }

    fn get_rect(&self) -> Rect {
        self.get_style().get_rect()
    }

    fn hit_test(&self, point: Point) -> HitTestResult {
        self.get_style().hit_test(self.get_data(), point)
    }

    fn to_primitives(&self, drawing_context: &mut DrawingContext) -> Vec<Primitive> {
        self.get_style().to_primitives(self.get_data(),
            drawing_context)
    }

}
