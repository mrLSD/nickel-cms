# Nickel CMS
[![Build Status](https://travis-ci.org/irony-rust/nickel-cms.svg?branch=master)](https://travis-ci.org/irony-rust/nickel-cms) [![License](http://img.shields.io/badge/license-mit-blue.svg?style=flat-square)](https://raw.githubusercontent.com/irony-rust/nickel-cms/master/LICENSE)

Inspired by Rust, CMS based on Iron CMS & Nickel Framework

### Our aims and basic features

* We love **Rust** and we want create amazing tool and infrastructure for scaffolding and smart development. Fast development, efficient code, safe environment. Big and bold goal, but it is worth it.
* We investigate several Rust framework, our base for that - performance and smart development with chosen tools and framework infrastructure.
* We **not guaranteeing** use Iron as basic framework before ending investigation.
* We developed middlewares for Form Request parsing and complex form data valdation
* We use **only stable** version of Rust and their libraries
* We tests cover our code as far as it possible
* We trying develop full-featured tool, including: templates, for fetching and validation, auth, cookies, session, database, http-security tools,cloud integration, admin/backend tools, caching, migrations, CI-orientation, flexible integration new features.
* We want to attract interest in Rust and to the fact that web development with Rust is funny and effective.


#### Current status:
Active development

#### How to build and run:
```
$ git clone http://github.com/irony-rust/nickel-cms
$ cargo build --release
$ target/release/nickel-cms
```

#### Requirements:
* Rust 1.15.0+
  
#### Some useful commands:
* **install Rust:** `make install`
* **run:** `cargo run` or `make`
* **build:** `cargo build` or `make build`
* **release build:** `cargo build --release` or `make release`
* **test:** `cargo test` or `make test`

#### License: MIT
