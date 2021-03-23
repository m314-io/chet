use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct Chet;

#[gdnative::methods]
impl Chet {
    fn new(_owner: &Node) -> Self {
        Chet
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, Potatoe.")
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Chet>();
}

godot_init!(init);
