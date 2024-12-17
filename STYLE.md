# Style Guides
Self note for recommend indentation, naming conventions, file structure, etc.

## Indentation
Indentation should consist of 4-spaces width tab or spaces. If applicable, avoid unnecessary deep indentation except it seems redundant to extract it. A deep indentation that consist of small code is probably fine.

## Importing
This guide outlines the preferred order and structure for organizing imports in Rust projects. Following this convention helps maintain readability and consistency.

### Order
#### 1. Declare Modules from Other Files First
Modules (mod) should always be declared before any other imports. This makes it clear which modules belong to the current project and are defined in separate files.

```
pub mod commands;
pub mod handler;
pub mod tools;
```

Why?

- Establishes a clear distinction between module declarations and imports.
- Ensures module visibility is declared upfront.

#### 2. Import External Crates Next

External dependencies (crates) should be imported after module declarations. Group related imports together for readability.

```
use poise::serenity_prelude::{ClientBuilder, FullEvent, GatewayIntents};
use shuttle_serenity::ShuttleSerenity;
```

Why?

- External dependencies are logically separate from internal modules.
- Clearly indicates which third-party libraries the file depends on.

#### 3. Import Structs and Traits from the Current Crate

Structs and traits defined in the current crate should follow external imports. Use `crate::` to specify their origin explicitly.

```
use crate::Bot;
use crate::Data;
```

Why?

- Highlights key components or data structures used in the current module.
- Differentiates internal crate imports from third-party dependencies.

#### 4. Import Internal Files Last

Imports from other files within the same crate should be organized last. Use wildcard imports (*) sparingly and only when appropriate.

```
use crate::bot::commands::*;
use crate::bot::handler::*;
````

Why?

- Keeps the focus on higher-level dependencies first.
- Avoids mixing internal file imports with more significant dependencies.

## Structures
### Order of Code Elements

This section defines the preferred order for structuring code within a Rust file to maintain consistency and readability. Adhering to this order ensures logical progression and reduces cognitive overhead when reading or navigating the code.

#### 1. Type Declarations

Place type aliases (type) at the top of the file after imports. These provide clarity and define key abstractions used throughout the file.

```
type InteractionError = Box<dyn std::error::Error + Send + Sync>;
type InteractionContext<'a> = poise::Context<'a, Data, InteractionError>;
```

Why?

- Serves as an overview of key types used in the module.
- Allows developers to quickly understand common abstractions before diving into the details.

#### 2. Structs

Define structs after type declarations. Each struct should include a brief comment explaining its purpose.

```
// Represents the bot's main configuration and state.
pub struct Bot {
    pub name: String,
    pub version: String,
}
```

Why?

- Structs define the core data models and are central to understanding the functionality of a module.
- Placing them early in the file highlights the key components.

#### 3. Traits

Declare traits after structs. Include:

- A short comment explaining the purpose of the trait.
- Associated methods and functions that structs will implement.

```
// Defines bot-related behaviors.
pub trait BotBehavior {
    fn greet(&self) -> String;
}
```

Why?

- Traits describe the expected behavior of types.
- Placing traits here allows a natural transition from data definitions to behavior specifications.

#### 4. Implementations

Implementations (impl) should follow the struct and trait declarations.

```
impl Bot {
    // Returns the bot's full identifier.
    pub fn identifier(&self) -> String {
        format!("{} v{}", self.name, self.version)
    }
}

impl BotBehavior for Bot {
    fn greet(&self) -> String {
        format!("Hello, I am {}!", self.name)
    }
}
```

#### 5. Functions

Functions should be declared after all types, structs, traits, and implementations. Group functions logically, and document them appropriately.

```
// Initializes the bot with default values.
pub fn initialize_bot() -> Bot {
    Bot {
        name: String::from("RustBot"),
        version: String::from("1.0"),
    }
}

// Starts the bot and handles interactions.
pub fn run_bot(bot: &Bot) {
    println!("{}", bot.greet());
}

```

## Summary
```
// 1. Module Declarations
pub mod commands;
pub mod handler;
pub mod tools;

// 2. Import External Crates
use poise::serenity_prelude::{ClientBuilder, FullEvent, GatewayIntents};
use shuttle_serenity::ShuttleSerenity;

// 3. Import Structs and Traits from the Current Crate
use crate::Bot;
use crate::Data;

// 4. Import Internal Files
use crate::bot::commands::*;
use crate::bot::handler::*;

// 5. Type Declarations
type InteractionError = Box<dyn std::error::Error + Send + Sync>;
type InteractionContext<'a> = poise::Context<'a, Data, InteractionError>;

// 6. Structs
/// Represents the bot's main configuration and state.
pub struct Bot {
    pub name: String,
    pub version: String,
}

// 7. Traits
/// Defines bot-related behaviors.
pub trait BotBehavior {
    fn greet(&self) -> String;
}

// 8. Implementations
impl Bot {
    /// Returns the bot's full identifier.
    pub fn identifier(&self) -> String {
        format!("{} v{}", self.name, self.version)
    }
}

impl BotBehavior for Bot {
    fn greet(&self) -> String {
        format!("Hello, I am {}!", self.name)
    }
}

// 9. Functions
/// Initializes the bot with default values.
pub fn initialize_bot() -> Bot {
    Bot {
        name: String::from("RustBot"),
        version: String::from("1.0"),
    }
}

/// Starts the bot and handles interactions.
pub fn run_bot(bot: &Bot) {
    println!("{}", bot.greet());
}
```
