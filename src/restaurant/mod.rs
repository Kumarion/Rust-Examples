mod pizza_order {
    pub struct Pizza{
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("wheat"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Seating customer at table");
        }
        pub fn take_order() {
            seat_at_table();
            let cust_pizza = super::Pizza::lunch("pepperoni");
            serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza: super::Pizza) {
            println!("Here is your pizza with {} dough, {} cheese, and {} toppings", cust_pizza.dough, cust_pizza.cheese, cust_pizza.topping);
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}