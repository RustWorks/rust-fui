#![windows_subsystem = "windows"]

use fui::*;
use fui_app::*;
use fui_controls::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use typemap::TypeMap;

struct ItemViewModel {
    pub parent: Weak<RefCell<MainViewModel>>,
    pub name: Property<String>,
    pub number: Property<i32>,
}

impl ItemViewModel {
    pub fn new(
        parent: Weak<RefCell<MainViewModel>>,
        name: Property<String>,
        number: Property<i32>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ItemViewModel {
            parent,
            name,
            number,
        }))
    }
}

impl RcView for ItemViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Horizontal {
                Text { text: &vm.name },
                Text { text: (&vm.number, |n| format!(" - {}", n)) },
                Button {
                    clicked: Callback::new_rc(view_model, |vm, _| {
                        let parent = vm.borrow().parent.clone();
                        if let Some(parent) = parent.upgrade() {
                            parent.delete(vm);
                        }
                    }),
                    Text { text: "Delete" },
                }
            }
        )
    }
}

struct MainViewModel {
    pub items: ObservableVec<Rc<RefCell<ItemViewModel>>>,
    counter: i32,
}

impl MainViewModel {
    pub fn new() -> Rc<RefCell<Self>> {
        let main_vm = Rc::new(RefCell::new(MainViewModel {
            items: ObservableVec::new(),
            counter: 0,
        }));

        main_vm.add();
        main_vm.add();
        main_vm.add();
        main_vm.add();

        main_vm
    }
}

trait MainViewModelMethods {
    fn add(&self);
    fn add_100(&self);
    fn remove_all(&self);
    fn delete(&self, item: Rc<RefCell<ItemViewModel>>);
}

impl MainViewModelMethods for Rc<RefCell<MainViewModel>> {
    fn add(&self) {
        let new_item = ItemViewModel::new(
            Rc::downgrade(self),
            Property::new(format!("Element {}", self.borrow().counter)),
            Property::new(self.borrow().counter + 10),
        );
        self.borrow_mut().counter += 1;

        println!("Add {}!", new_item.borrow().name.get());
        self.borrow_mut().items.push(new_item);
    }

    fn add_100(&self) {
        for _ in 0..100 {
            self.add();
        }
    }

    fn remove_all(&self) {
        println!("Remove all!");
        self.borrow_mut().items.remove_filter(|_i| true);
    }

    fn delete(&self, item: Rc<RefCell<ItemViewModel>>) {
        println!("Delete {}!", item.borrow().name.get());
        self.borrow_mut()
            .items
            .remove_filter(|i| std::ptr::eq(i.as_ref(), item.as_ref()));
    }
}

impl RcView for MainViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        ui!(
            Grid {
                columns: 1,
                heights: vec![(0, Length::Auto)],

                Vertical {
                    Button {
                        clicked: Callback::new_rc(view_model, |vm, _| vm.add()),
                        Text { text: "Add" },
                    },
                    Button {
                        clicked: Callback::new_rc(view_model, |vm, _| vm.add_100()),
                        Text { text: "Add 100" },
                    },
                    Button {
                        clicked: Callback::new_rc(view_model, |vm, _| vm.remove_all()),
                        Text { text: "Remove all" },
                    },
                },

                ScrollViewer {
                    Vertical {
                        &vm.items,
                    },
                }
            }
        )
    }
}

fn main() {
    let mut app = Application::new("Example: list").unwrap();

    let main_view_model = MainViewModel::new();

    {
        let mut window_manager = app.get_window_manager().borrow_mut();
        let window_builder = winit::window::WindowBuilder::new().with_title("Example: list");
        window_manager
            .add_window_view_model(
                window_builder,
                app.get_event_loop().unwrap(),
                &main_view_model,
            )
            .unwrap();
    }

    app.run();
}
