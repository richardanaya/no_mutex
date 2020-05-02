# no_mutex

**this only works on nightly**

A mutex like construct for single threaded applications. 

* Gives you Sync and Send container
* Lazily loads the mutex value with `Default`
* panics if you lock it twice since deadlocking a single threaded app would be pointless

```rust
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
```
