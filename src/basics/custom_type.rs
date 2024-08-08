

#[derive(Debug)]
struct Name {
    first_name: String,
    last_name: String,
}

enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}


fn the_struct(){
    let first_name = String::from("John");
    let last_name = String::from("Doe");

    let full_name = Name {
        first_name,
        last_name,
    };
    println!("{:?}", full_name);
    let full_name_two = Name {first_name: "Jane".to_string(), ..full_name};
    println!("{:?}", full_name_two.first_name);
    println!("{:?}", full_name_two.last_name);

    // Destructuring
    let Name {first_name: _new_first, last_name: _new_last} = full_name_two;
}

fn enum_inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn the_enum() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    enum_inspect(pressed);
    enum_inspect(pasted);
    enum_inspect(click);
    enum_inspect(load);
    enum_inspect(unload);
}


enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
pub fn main() {
    // the_struct();
    the_enum()
}
