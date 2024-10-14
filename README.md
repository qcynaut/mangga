# Mangga

Mangga is an open-source Rust ODM (Object-Document Mapper) for MongoDB, currently in early development.

## Overview

Mangga aims to provide a seamless and idiomatic way to work with MongoDB in Rust applications. By offering an ODM tailored for Rust, Mangga bridges the gap between Rust's strong type system and MongoDB's flexible document model.

## Features

*Note: As Mangga is in early development, many features are planned but not yet implemented.*

- [x] Basic CRUD operations
- [x] Schema definition using Rust structs
- [x] Query builder with type-safe operations
- [x] Index management
- [ ] Aggregation pipeline support
- [ ] Others

## Installation

Mangga is not yet available on crates.io. To use it in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
mangga = { git = "https://github.com/qcynaut/mangga" }
```

## Quick Start

```rust
use mangga::prelude::*;
use serde::{Serialize, Deserialize};

#[model("users")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... load environment variable
    let db_uri = std::env::var("DATABASE_URL").unwrap();
    connect_database(db_uri, "some").await?;

    // init the model
    user::doc.init().await?;

    let users = vec![
        User::new("John Doe", 25),
        User::new("Jane Doe", 30)
    ];

    user::doc.insert(users).execute().await?;

    let res = user::doc.filter(user::age.gt(20)).execute().await?;
    println!("{:?}", res);
}
```

## Documentation

Comprehensive documentation will be available as the project matures. For now, please refer to the inline documentation and examples in the source code.

## Contributing

We welcome contributions to Mangga! If you're interested in helping, please:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Rust community for their invaluable resources and support
- MongoDB for their excellent database system

## Contact