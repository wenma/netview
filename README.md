### Way to show Net Device informations

### build
```sh
cargo build --release
```

### Run
```sh
> sudo ./netview

Net Namespace name: 30c336360de4
+-------------+----------+---------+-----------------+
| Device Name | If-Index | If-Type | Veth-Peer-Index |
| lo          | 1        |         |                 |
| eth0        | 62       | Veth    | 63              |
+-------------+----------+---------+-----------------+

Net Namespace name: default
+-------------+----------+---------+-----------------+
| Device Name | If-Index | If-Type | Veth-Peer-Index |
| lo          | 1        |         |                 |
| eth0        | 2        |         |                 |
| docker0     | 3        | Bridge  |                 |
| virbr0      | 60       | Bridge  |                 |
| virbr0-nic  | 61       | Tun     |                 |
| vethfd854c6 | 63       | Veth    | 62              |
+-------------+----------+---------+-----------------+

```
