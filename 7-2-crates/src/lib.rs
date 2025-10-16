// Library crate no main can only call functions or import modules
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod front_of_house {
    pub mod housing {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
        pub fn seat_at_table() {
            println!("Assigned seat");
        }
    }
    pub mod serving {
        pub fn take_order() {
            println!("Taking order");
        }
        pub fn serve_order() {
            println!("Serving order");
        }
        pub fn take_payment() {
            println!("Receiving  payment");
        }
    }
}
