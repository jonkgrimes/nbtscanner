# NetBIOS scanner

Little tool I put together playing around with Rust.

## Usage

You can either use CIDR notation:

```bash
> nbtscanner 10.10.48.1/24
Scanning from 10.10.48.1 to 10.10.48.254 (254 total)
...
```
or simply use a dash

```bash
> nbtscanner 10.10.48.1-254
Scanning from 10.10.48.1 to 10.10.48.254 (254 total)
...
```