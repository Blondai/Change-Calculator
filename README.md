# Euro Change Calculator

### Description

This Rust project is a simple console application that calculates the change to be given back in euros and cents.
It takes as input the amount received and the cost of the bought product.
With this it calculates and outputs the number of each euro denomination needed to return the correct change.

### Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/Blondai/Change-Calculator.git
    cd Change-Calculator
    ```

2. Build the project:
    ```bash
    cargo build
    ```

3. Run the project:
    ```bash
    cargo run
    ```

### Usage

When you run the program, it will prompt you to enter:

1. **Received amount**: The received money from the customer.
2. **Cost**: The cost of the items.

The program will then output the number of each euro denomination that should be returned as change.

### Examples
1. Example:
   ```
   Received amount: 50
   Cost: 27.39
   
   1 times 20.- €
   1 times 2.- €
   1 times -.50 €
   1 times -.10 €
   1 times -.1 €
   ```

2. Example:
   ```
   Received amount: 10.04
   Cost: 8.24
   
   1 times 1.- €
   1 times -.50 €
   1 times -.20 €
   1 times -.10 €
   
   ```

3. Example:
   ```
   Received amount: 10,00
   Wrong Format. Please enter a valid amount.
   Received amount: 10.00
   Cost: 5.22
   
   2 times 2.- €
   1 times -.50 €
   1 times -.20 €
   1 times -.5 €
   1 times -.2 €
   ```

### Future
Probably will add comments and docstring later.
Maybe I will implement a method for outputting received amount and cost to file.
