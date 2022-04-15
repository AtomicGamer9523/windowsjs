use druid::{AppLauncher, Data, Lens, Widget, WindowDesc};
use druid::widget::{Align, Flex, Label};
use node_bindgen::derive::node_bindgen;


struct Window {
    title: String,
    window_size: (f64, f64)
}

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}


#[node_bindgen]
impl Window {

    #[node_bindgen(constructor)]
    fn new(title: String, size: Option<(f64, f64)>) -> Self {
        Self {
            title: title,
            window_size: size.unwrap_or((100.0, 100.0))
        }
    }

    #[node_bindgen(name = "launch")]
    fn launch(&mut self){
        let main_window = WindowDesc::new(build_widget)
        .title(String::from(self.title.to_string()))
        .window_size(self.window_size);

        let initial_state = HelloState {
            name: "World".into(),
        };

        AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
    }
}



fn build_widget() -> impl Widget<HelloState> {
    let label = Label::new(String::from("Hello World"));
    let layout = Flex::column().with_child(label);
    Align::centered(layout)
}