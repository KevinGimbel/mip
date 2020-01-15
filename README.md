# myip
> Small crate to get local IPv4 address by making an HTTP request to httpbin.org/ip

Crate `myip` gets the local IP address by requesting `http://httpbin.org/ip` and parsing the returned output.

<!-- BEGIN mktoc -->
- [Usage](#usage)
- [Todo](#todo)
- [License](#license)
<!-- END mktoc -->

## Usage

```toml
[dependencies.myip]
 git = { "https://github.com/kevingimbel/myip", branch = "master" } 
```

```rust
use myip::IP;

fn main() {
    println!("My IP is {}", IP::is());
}
```

Or without `use`

```rust

fn main() {
    println!("My IP is {}", myip::IP::is());
}
```

## Todo

Some things that need to be done

- [ ] Clean-up code
- [ ] Write proper docs
- [ ] Use multiple providers randomly / with fallback like http://checkip.dyndns.com and http://checkip.dyndns.org
- [ ] Support IPv6? 
 
## License
 
 See LICENSE file. 