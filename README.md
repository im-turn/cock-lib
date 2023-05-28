# Turn's Cock Tier Evaluator :chicken: :trophy:

Library for Turn's Cock Tier Evaluator.

## The Anatomy of the Evaluator :eggplant: :stethoscope:

I've dissected the cock-rating task into neat, testable modules. Here's a quick breakdown:

- **bin_modules (mod bin_modules):** Our little helpers, the `bin_modules` are exclusively used within the operations of the executables and won't be found outside of this module and the `bin` files. It's where things such as the `cock_handler_build` function is located, which creates a `CockHandler` based on your personal input during the execution of the default binary `cock-tier`/`main.rs`.
- **User (user.rs):** You, the user, the cock-owner.
- **CockStruct (cock_struct.rs):** Your magnificent (or not so magnificient) monument's blueprint, combining all its noteworthy aspects into a single entity.
- **CockHandler (cock_handler.rs):** The gentle hands wrapping up the `User` and `CockStruct` to provide easy methods to calculate and print all the juicy details about your member.
- **Traits (traits.rs):** These aren't your cock traits, they're various Rust traits used throughout the application.
- **Tier (tier.rs):** Enum for letter grades.
- **cock_parts (mod cock_parts):** Each of these modules, like `size.rs`, `aesthetic.rs`, `balls.rs` (and others), represents a particular feature of your shlong, providing a score for several of them (a few features have no impact on ratings as of right now).

## Getting Rated and Library Usage :open_book: :male_detective:

Want to see where you stand in the land of peen, but you're kinda a dummy?! Luckily I'm here to guide you through the process.

### The Binary Packages

- [`cock-cli`](https://crates.io/crates/cock-cli)
- [`cock-tui`](https://crates.io/crates/cock-tui)
- [`cock-web`](https://crates.io/crates/cock-web): COMING SOON

### The API

You're a developer who sees the inherent value in this? Strange. However, it is fairly simple to get started with things. To begin, add the following to your `Cargo.toml` file:

```toml
[dependencies]
cock-tier = "x.x.x"
```

Below is an example of one way you could go about using API.

```rust
use cock_tier::{
    Abnormalities, Aesthetic, Balls, Circumcision, CockHandler, CockStruct, Curvature,
    Shape, Size, SizeType::Inches, Tier, ID, Veininess,
};

let user = ID::Anonymous;

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

let handler = CockHandler { id: user, cock };

// Now you can access the CockHandler's methods:
let grade = handler.grade(); // Returns Tier::C
let score = handler.total_score().score; // Returns 48.0
let percentage = handler.total_score().percentage; // Returns 68.571434
```

## Dependencies

The `cock-tui` relies on and is rendered using [Cursive](https://crates.io/crates/cursive).

[Cursive](https://crates.io/crates/cursive) also requires the `ncurses` lib to be installed on the machine.

## Testing :petri_dish: :test_tube:

This library comes with a built-in test suite that verifies the functionality of the code. To run the tests, use the command `cargo test` in the project's root directory.

## License :clipboard: :briefcase:

MIT License
