use rand::{Rng, RngCore, SeedableRng};

fn call_order() {

}

mod front_house;

mod back_house {
   pub struct Breakfast {
    pub toast: String,
    pub fruit: String,
   }

   impl Breakfast {
    pub fn monday (toast: &str) -> Breakfast {
        Breakfast{
            toast: String::from(toast),
            fruit: String::from("Banana")
        }
    }
   }

   pub enum Salad {
    Soup,
    Salad,
   }
}

fn eat_at_restaurant() {
    self::front_house::hosting::add_to_waitlist();

    front_house::hosting::add_to_waitlist();

    let mut order = back_house::Breakfast::monday("Fish");

    order.toast = String::from("Chicken");

    let order2 = back_house::Breakfast{
        toast: String::from("Wheat"),
        fruit: String::from("Banana"),
    };

    let order3 = back_house::Salad::Salad;
}