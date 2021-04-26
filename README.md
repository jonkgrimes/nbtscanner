# nbtscanner (NetBIOS scanner)

A tiny tool to scan a set of IP's for NetBIOS name information.

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