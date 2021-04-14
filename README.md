## API:
#### Get BTC balance by address and spent/unspent parameter:
```
curl 'http://0.0.0.0:8000/api/v1/addrs/{address}?unspentOnly={true/false}'
```
* Usage example:
```
curl 'http://0.0.0.0:8000/api/v1/addrs/1CL5TbB2MaR4mrFjtYQ5GyA3cP2bSmPxAn?unspentOnly=true'
```

#### Get all address records:
```
curl 'http://0.0.0.0:8000/api/v1/addrs'
```

#### Get pagination result of UTXO records:
```
curl 'http://0.0.0.0:8000/api/v1/records/offset/{offset}/limit/{limit}'
```
* Usage example:
```
curl 'http://0.0.0.0:8000/api/v1/records/offset/0/limit/20'
```