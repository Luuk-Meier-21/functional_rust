mod logging;
mod maybe;
mod monad;

use logging::Logging;
use maybe::Maybe;
use monad::Monad;

// static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());
fn main() {
    // Logging monad:
    let num: Logging<i32> = Logging::new(647);
    println!("{:?}", num);

    let num_double: Logging<i32> = num.double().double();
    println!("{:?}", num_double);

    // Maybe monad (Custom Option impl):
    let maybe: Maybe<i32> = Maybe::Just(20);
    println!("{:?}", maybe);

    let id: i32 = 20;

    // Binding functor
    let result: Maybe<i32> = maybe.map(|v: i32| {
        if id == v {
            return divide(v, 100);
        }

        divide(v, 3)
    });

    println!("{:?}", id);

    println!("{:?}", result);

    // Times 0 is not possible, result becomes `none`
    let none_result: Maybe<i32> = result.map(|v| divide(v, 3)).map(|v| divide(v, 0));
    println!("{:?}", none_result);
}

fn divide(value: i32, t: i32) -> Maybe<i32> {
    if t != 0 {
        Maybe::Just(value / t)
    } else {
        Maybe::None
    }
}
