use restaurant::back_of_house::Breakfast;
use restaurant::{eat_at_restaurant, front_of_house::hosting};
fn main() {
    eat_at_restaurant();
    let _b = Breakfast {
        toast: String::from("Wheat"),
        seasonal_fruit: String::from("Blueberries"),
    };
    let order1 = restaurant::back_of_house::Appetizer::Soup;
    let order2 = restaurant::back_of_house::Appetizer::Salad;
    println!("order1: {:?}, order2: {:?}", order1, order2);

    hosting::add_to_waitlist();
}
