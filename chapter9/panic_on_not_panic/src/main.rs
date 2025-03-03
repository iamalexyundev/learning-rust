// // This will work because struct and main are siblings
// #[derive(Debug)]
// struct Guess {
//     value: i32,
// }
// impl Guess {
//     fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {value}.");
//         }

//         Guess { value }
//     }
//     fn value(&self) -> i32 {
//         self.value
//     }
// }
//This will not work because Guess struct is a child of a sibling(cousin? lol)
// mod guess {
//     #[derive(Debug)]
//     struct Guess {
//         value: i32,
//     }
//     impl Guess {
//         fn new(value: i32) -> Guess {
//             if value < 1 || value > 100 {
//                 panic!("Guess value must be between 1 and 100, got {value}.");
//             }

//             Guess { value }
//         }
//         fn value(&self) -> i32 {
//             self.value
//         }
//     }
// }
use panic_on_not_panic::game::Guess;
fn main() {
    // let mut guess = Guess { value: 1000 };
    // guess.value = 100;
    // println!("{:?}", guess.value());
}
