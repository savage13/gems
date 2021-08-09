# gems
Gems Calculator for BotW

Microservice that parses simple input describing the amount of gems and return the value in rupees.

# Examples

    Input:
       3 Amber 2 Ruby 1 Diamond verbose
    Output:
       3 amber 90 | 2 ruby 420 | 1 diamond 500 | Total 1010
       
    Input: (single letters can be used as abbrevations)
       3 a 2 r d v
    Output:
       3 amber 90 | 2 ruby 420 | 1 diamond 500 | Total 1010

    Input: (without verbose or v, only the total is returned)
       3 a 2 r d
    Output:
       1010

This is a basic microservice written in [Rust](https://www.rust-lang.org/) using [Rocket](https://rocket.rs/)

# License
BSD 2-Clause
