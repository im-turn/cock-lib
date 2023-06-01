# Turn's Cock Tier Evaluator :chicken: :trophy:

Library for Turn's Cock Tier Evaluator.

## The Anatomy of the Evaluator :eggplant: :stethoscope:

I've dissected the cock-rating task into neat, testable modules. Here's a quick breakdown:

- **bin_use (mod bin_use):** Responsible for anything in the library that is only intended to be used by external binaries such as [`cock-cli`](https://crates.io/crates/cock-cli) or [`cock-tui`](https://crates.io/crates/cock-tui). This currently holds a `UserData` struct as well as a `AppState` enum.
- **User (user.rs):** You, the user, the cock-owner.
- **CockStruct (cock_struct.rs):** Your magnificent (or not so magnificient) monument's blueprint, combining all its noteworthy aspects into a single entity.
- **CockHandler (cock_handler.rs):** The gentle hands wrapping up the `User` and `CockStruct` to provide easy methods to calculate and print all the juicy details about your member.
- **Traits (traits.rs):** These aren't your cock traits, they're various Rust traits used throughout the application.
- **Tier (tier.rs):** Enum for letter grades.
- **cock_parts (mod cock_parts):** Each of these modules, like `size.rs`, `aesthetic.rs`, `balls.rs` (and others), represents a particular feature of your shlong, providing a score for several of them (a few features have no impact on ratings as of right now).

### Optional Feature Modules

- **bin_use (mod bin_use):** Responsible for anything in the library that is only intended to be used by external binaries such as [`cock-cli`](https://crates.io/crates/cock-cli) or [`cock-tui`](https://crates.io/crates/cock-tui). This currently holds a `UserData` struct as well as a `AppState` enum.

## Getting Rated and Library Usage :open_book: :male_detective:

Want to see where you stand in the land of peen, but you're kinda a dummy?! Luckily I'm here to guide you through the process.

### The Binary Packages

- [`cock-cli`](https://crates.io/crates/cock-cli): CLI App using the `cock_lib` library
- [`cock-tui`](https://crates.io/crates/cock-tui): TUI App using the `cock_lib` library
- [`cock-web`](https://crates.io/crates/cock-web): COMING SOON - WEB BACK END FOR `cock_lib`

### The API

You're a developer who sees the inherent value in this? Strange. However, it is fairly simple to get started with things. To begin, add the following to your `Cargo.toml` file:

```toml
[dependencies]
cock-lib = "x.x.x"
```

Below is an example of one way you could go about using the API to create a data structure representing a cock, otherwise known as a `CockStruct`.

```rust
use cock_lib::{
    CockStruct,
    cock_parts::{Size, Aesthetic, Balls, Shape, Curvature, Circumcision, Veininess, Abnormalities, Inches}
};

let cock = CockStruct::new(
    Size {
        length: 5.5,
        girth: 4.5,
        size_type: Inches,
    },
    Aesthetic::Normal,
    Balls::Normal,
    Shape::Cylindrical,
    Curvature::Straight,
    Circumcision::Uncircumcised,
    Veininess::Normal,
    Abnormalities::None,
);
```

## Testing :petri_dish: :test_tube:

This library comes with a built-in test suite that verifies the functionality of the code. To run the tests, use the command `cargo test` in the project's root directory.

## License :clipboard: :briefcase:

MIT License
