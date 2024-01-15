# cybertoken-rs [![CI](https://github.com/lucagoslar/cybertoken-rs/actions/workflows/test.yml/badge.svg)](https://github.com/lucagoslar/cybertoken-rs/actions/workflows/test.yml) [![CD](https://github.com/lucagoslar/cybertoken-rs/actions/workflows/publish.yml/badge.svg)](https://github.com/lucagoslar/cybertoken-rs/actions/workflows/publish.yml)

A Rust implementation of the [cybertoken](https://github.com/nikeee/cybertoken), a token format inspired by the GitHub API token format.

Licensed under [MIT](/LICENSE-MIT) or [APACHE 2.0](/LICENSE-APACHE) or [EUPL v 1.2](/LICENSE-EUPL).

## Install

Extend your `Cargo.toml` configuration file to include `cybertoken` as a dependency or install the package with the Cargo package manager.

```zsh
cargo add cybertoken
```

## Usage

```rs
use cybertoken::Cybertoken;

fn main() {
  let cybertoken = Cybertoken::new("zugriff");
  let token = cybertoken.generate();

  println!("token"); // zugriff_2uiWaFKqkMD9CLdUqrYZd2BWYfj2gz806DP5P

  println!("{}", cybertoken.is_token_strin("zugriff_icnocrRLDoZ3uCPosLA0277hQ58ob379X43U")
  ); // true
}
```
