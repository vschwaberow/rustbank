# Rust Banking System Demo Project

This is a simple Rust project designed to demonstrate various concepts of the Rust programming language in an easy-to-understand way. It implements a basic banking system with user accounts, transactions, and error handling.

## Features

* Create new accounts with initial balances
* Perform transactions between accounts
* Check account balances
* Error handling for insufficient funds and unknown accounts
* Command-line interface for user interaction
* Unit tests to ensure functionality

## How to Use

1. Clone the repository
2. Run ``cargo run`` to start the program
3. Follow the menu prompts to:
    * Create accounts
    * Perform transactions
    * Check balances
    * Exit the program

## Project Structure

The project consists of a single file, ``main.rs``, which contains:

* The Bank struct and its methods
* Error handling using enums and implementations
* The main function with a command-line interface
* Helper functions for user input
* Unit tests

## Concepts Demonstrated

This project showcases various Rust concepts, including:

* Structs and methods
* Error handling with enums and trait implementations
* Command-line input/output
* Unit testing
* Pattern matching
* Option handling
* Error propagation