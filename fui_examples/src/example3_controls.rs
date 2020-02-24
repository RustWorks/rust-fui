#![windows_subsystem = "windows"]

use fui::*;
use fui_app::*;
use fui_controls::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::Rc;

use typemap::TypeMap;
use winit::window::WindowBuilder;

struct MainViewModel {
    pub text: Property<String>,
    pub counter: Property<i32>,
    pub counter2: Property<i32>,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(MainViewModel {
            text: Property::new("My text"),
            counter: Property::new(10),
            counter2: Property::new(0),
        }))
    }

    pub fn increase(&mut self) {
        self.counter.change(|c| c + 1);
    }

    pub fn decrease(&mut self) {
        self.counter.change(|c| c - 1);
    }
}

impl RcView for MainViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();

        vm.counter2.bind(&mut vm.counter);
        vm.counter.bind(&mut vm.counter2);

        ui!(
            Grid {
                columns: 2,
                heights: vec![(0, Length::Auto)],

                TextBox {
                    text: &mut vm.text,
                },
                Text {
                    text: &mut vm.text,
                },

                Text { text: (&vm.counter, |counter| format!("Counter {}", counter)) },
                Button {
                    clicked: Callback::new(view_model, |vm, _| vm.decrease()),
                    Text { text: "Decrease" },
                },
                Button {
                    clicked: Callback::new(view_model, |vm, _| vm.increase()),
                    Text { text: "Increase" },
                },
                Text { text: (&vm.counter2, |counter| format!("Counter2 {}", counter)) },
            }
        )
    }
}

fn main() -> Result<()> {
    let mut app = Application::new("Example: layout").unwrap();

    app.add_window(
        WindowBuilder::new().with_title("Example: layout"),
        MainViewModel::new(),
    )?;

    app.run();

    Ok(())
}