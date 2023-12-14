# Shuttle's Christmas Code Hunt

Shuttle's Christmas Code Hunt, inspired by Advent of Code, invites you to solve daily challenges using Rust in a relaxed environment. Each weekday, you will be implementing an HTTP endpoint that returns the solution to the daily challenge, and deploy it on [Shuttle](https://www.shuttle.rs/). Join the fun, solve puzzles, embrace the holiday spirit, and get rewarded! 🎄🚀

Useful links:

- [info](https://www.shuttle.rs/cch)
- [Scoreboard](https://www.shuttle.rs/cch#scoreboard)
- [console](https://console.shuttle.rs/cch)

## Test

Test with curl or [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) or `cargo test`.

Test specific day (e.g., day1):
`cargo test day1`

## Shuttle Shared DB

to use the query! macro, you may need to run:

```sh
cargo sqlx prepare # with DATABASE_URL in .env
```
