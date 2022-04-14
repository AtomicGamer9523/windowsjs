use node_bindgen::derive::node_bindgen;

struct MyClass {
    val: f64,
}


#[node_bindgen]
impl MyClass {

    #[node_bindgen(constructor)]
    fn new(val: f64) -> Self {
        Self { val }
    }

    #[node_bindgen(name = "pl1")]
    fn plus_one(&mut self) {
        self.val += 1.0;
    }

    #[node_bindgen(getter, name = "v")]
    fn v(&self) -> f64 {
        self.val
    }

    #[node_bindgen(name = "setVal")]
    fn set_val(&mut self, newval:f64){
        self.val = newval
    }
}