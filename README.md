# mip
> Get IPv4 address in rust - with 0 dependencies!

Crate `mip` gets the local IP address by requesting `http://httpbin.org/ip` and parsing the returned output. 

It has 0 dependencies and only relies on the Rust std lib. 

<!-- BEGIN mktoc -->
- [Usage](#usage)
- [Custom Endpoint](#custom-endpoint)
- [Todo](#todo)
- [License](#license)
<!-- END mktoc -->

## Usage

Add the following to `Cargo.toml`.
```toml
[dependencies]
mip = "0.4.0
```

```rust
use mip::IP;

fn main() {
    println!("My IP is {}", IP::is());
}
```

Or without `use`

```rust

fn main() {
    println!("My IP is {}", mip::IP::is());
}
```

## Custom Endpoint

Instead of using httpbin.org a custom endpoint can be used. Custom endpoints may not parse properly so test before using in production!

```rust
use mip::{IP, Endpoint};

fn main() {
    let ip = IP::with_endpoint(Endpoint {
        path: Some("/"),
        host: "checkip.dyndns.com",
        port: 80,
    });
    println!("{}", ip)
}
```

## Todo

Some things that need to be done

- [x] Clean-up code
- [ ] Write proper docs
- [ ] Use multiple providers randomly / with fallback like http://checkip.dyndns.com and http://checkip.dyndns.org
- [ ] Support IPv6? 
 
## License
 
 See LICENSE file. 