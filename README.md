My solutions to the https://adventofcode.com/ puzzles.

## Setup
Get session cookie for when you request a puzzle input, and put it in `puzzle-input/cookie` (see cookie.example for the expected format)

## Running it
Use the following command to get the solution for a specific day.
`Cargo run -- year day`

## Testing
`auto-test.sh` will automatically rerun all tests when it detects any `.rs` files have been changed, and display a timer showing how long ago it was it last ran the tests.

The timer require a script from my config repo. It probably works without if you remove the "display-timer" and "timer_pid" lines.
