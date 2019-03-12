### Way to show Net Device informations

### Build
```sh
cargo build --release
```

### Run
```sh
> sudo ./netview

Net-Namespace Name: `30c336360de4`
+-------------+----------+---------+-----------------+------------------------------------------+
| Device Name | If-Index | If-Type | Veth-Peer-Index | Container                                |
| lo          | 1        |         |                 |                                          |
| eth0        | 62       | Veth    | 63              | ID: ef7d17263adc, Name: /priceless_yalow |
+-------------+----------+---------+-----------------+------------------------------------------+

Net-Namespace Name: `default`
+-------------+----------+---------+-----------------+------------------------------------------+
| Device Name | If-Index | If-Type | Veth-Peer-Index | Container                                |
| lo          | 1        |         |                 |                                          |
| eth0        | 2        |         |                 |                                          |
| docker0     | 3        | Bridge  |                 |                                          |
| virbr0      | 60       | Bridge  |                 |                                          |
| virbr0-nic  | 61       | Tun     |                 |                                          |
| vethfd854c6 | 63       | Veth    | 62              | ID: ef7d17263adc, Name: /priceless_yalow |
+-------------+----------+---------+-----------------+------------------------------------------+
```
