use no_mutex::Mutex;

static FOO: Mutex<Foo> = Mutex::default();

#[derive(Debug)]
struct Foo {
    i:u32
}

impl Default for Foo {
    fn default() -> Self {
        Foo { i: 42 }
    }
}

fn main() {
    let r = FOO.lock();
    println!("{:?}",r);
}