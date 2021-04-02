# Engineering Features

## Blanket Trait Implementation

Blanket Implementation is closely related to generics.

```rust
pub trait Hello {
    fn get_name(&self) -> String;
    // zip ---
}

impl<T: Hello> IntroduceSelf for T {
    fn introduce_self(&self) {
        println!("Hello, I'm {}", self.get_name());
    }
}
```

It is much more powerful in a magical way, see the `Rayon` example:

```rust
// Rayon, a parallelization library
use rayon::prelude::*;
pub fn main(){
    let data = Vector::new()
    for i in 0..10000000000000{
        data.push(i);
    }
    // sequential mapping
    let double : Vec<i32> = data.iter().map(|x| 2*x).collect();
    // just change one line, a paralleled mapping is done
    let parallel_double : Vec<i32> = data.par_iter().map(|x| 2*x).collect();
}
```

The key recipe for `Rayon` to work is blanket implementation, which enables the developer to bind a new trait(e.g. `ParIter`) to a type that implements trait `Iter`.