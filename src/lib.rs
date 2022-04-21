#[allow(unused_imports)]
#[allow(non_snake_case)]
// #[allow(dead_code)]
use druid::{AppLauncher, Data, Lens, Widget, WindowDesc, *};
use druid::widget::{Align, Flex, Label};
use node_bindgen::derive::node_bindgen;


struct Window {
    title: String,
    window_size: (f64, f64)
}

#[derive(Clone, Data, Lens)]
struct MainState {
    name: String,
}


fn build_widget() -> impl Widget<MainState> {
    let label = Label::new(String::from("Hello World"));
    let layout = Flex::column().with_child(label);
    Align::centered(layout)
}


#[node_bindgen]
impl Window {

    #[node_bindgen(constructor)]
    fn new(title: String, size: Option<(f64, f64)>) -> Self {
        Self {
            title: title,
            window_size: size.unwrap_or((100.0, 100.0)),
        }
    }

    #[node_bindgen(name = "launch")]
    fn node_launch(&mut self){
        // let main_window = WindowDesc::new(build_widget)
        // .title(String::from(self.title.to_string()))
        // .window_size(self.window_size);

        let initial_state = MainState {
            name: "World".into(),
        };

        // let mywin = druid::Window::new(id: WindowId, handle: WindowHandle, desc: WindowDesc<T>);

        let frame = druid::WindowDesc::new(build_widget)
        .title(String::from(self.title.to_string()))
        .window_size(self.window_size);

        AppLauncher::with_window(frame)
        .launch(initial_state)
        .expect("Failed to launch application");
    }

    #[node_bindgen(getter, name = "_title")]
    fn node__title(&mut self) -> String {
        String::from(&self.title)
    }
}