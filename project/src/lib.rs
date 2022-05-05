// mod front_of_house{
//     pub mod hosting{
//         pub fn add_to_waitlist(){
//             println!("Please add waitlist");
//         }
//     }
// }


// // mod back_of_house{
// //     pub struct  Breakfast{
// //         pub toast: String,
// //         seasonal_fruit: String,
// //     }

// //     impl Breakfast {
// //         pub fn summer(toast: &str) -> Breakfast{
// //             Breakfast { 
// //                 toast: String::from("toast"), 
// //                 seasonal_fruit: String::from("peaches") 
// //             }
// //         }
// //     }
 
// // }

// // use crate::front_of_house::hosting;

// use self::front_of_house::hosting;


// pub fn eat_at_res(){
//     hosting::add_to_waitlist();
//     // hosting::add_to_waitlist();

//     // let mut meal = back_of_house::Breakfast::summer("Rye");

//     // meal.toast = String::from("wheat");

//     // println!("I had like {} toast please", meal.toast);
// }


use std::fmt;
use std::io::Result as IoResult;

fn function1() -> fmt::Result{
    Ok(())
}


fn function2() -> IoResult<()>{
    Ok(())
}