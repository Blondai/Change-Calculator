use std::io::{self, Write};
use regex::Regex;

pub struct Money {
    received: u32,
    cost: u32,
    denominations: [u16; 15],
    amounts: [u8; 15],
}

// New
impl Money {
    fn new(received: u32, cost: u32) -> Money {
        let denominations: [u16; 15] = Money::_initialize_denominations();
        let amounts: [u8; 15] = Money::_initialize_amount();
        Money {
            received,
            cost,
            denominations,
            amounts,
        }
    }
}

// Auxiliary
impl Money {
    const fn _initialize_denominations() -> [u16; 15] {
        const DENOMINATIONS: [u16; 15] = [
            50000, 20000, 10000, 5000, 2000, 1000, 500, 200, 100, 50, 20, 10, 5, 2, 1
        ];
        DENOMINATIONS
    }

    const fn _initialize_amount() -> [u8; 15] {
        const AMOUNTS: [u8; 15] = [0u8; 15];
        AMOUNTS
    }

    fn _short_denominations() -> Vec<&'static str> {
        let denomination_strings: Vec<&'static str> = vec![
            "500.-", "200.-", "100.-", "50.-", "20.-", "10.-", "5.-", "2.-", "1.-",
            "-.50", "-.20", "-.10", "-.5", "-.2", "-.1",
        ];
        denomination_strings
    }
}

// Change calculation
impl Money {
    fn change(&mut self) {
        assert!(self.cost < self.received, "Cost of is higher than received money.");
        let mut difference: u32 = self.received - self.cost;
        for index in 0usize..15usize {
            while difference >= self.denominations[index] as u32 {
                difference -= self.denominations[index] as u32;
                self.amounts[index] += 1;
            }
        }
    }
}

// Print
impl Money {
    fn print(self) {
        let denomination_strings: Vec<&'static str> = Money::_short_denominations();
        for index in 0usize..15usize {
            if self.amounts[index] > 0 {
                println!("{} times {} €", self.amounts[index], denomination_strings[index])
            }
        }
    }
}

// Input
impl Money {
    pub fn full() {
        let mut money: Money = Money::_setup();
        println!();
        money.change();
        money.print();
        println!();
    }
    fn _setup() -> Money {
        let received: u32 = Money::_get_received_money();
        let cost: u32 = Money::_get_cost();
        Money::new(received, cost)
    }

    fn _get_received_money() -> u32 {
        let format = Regex::new(r"^\d+(\.\d{1,2})?$").unwrap();

        loop {
            print!("Received amount: ");
            io::stdout().flush().unwrap();
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: &str = input.trim();

            if format.is_match(input) {
                let amount: u32 = (input.parse::<f32>().unwrap() * 100f32).round() as u32;
                return amount;
            } else {
                println!("Wrong Format. Please enter a valid amount.");
            }
        }
    }

    fn _get_cost() -> u32 {
        let format = Regex::new(r"^\d+(\.\d{1,2})?$").unwrap();

        loop {
            print!("Cost: ");
            io::stdout().flush().unwrap();
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: &str = input.trim();

            if format.is_match(input) {
                let cost: u32 = (input.parse::<f32>().unwrap() * 100f32).round() as u32;
                return cost;
            } else {
                println!("Wrong Format. Please enter a valid amount.");
            }
        }
    }
}

