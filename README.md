# GateWASI

## Usage

> Create a route

```
curl -XPOST \
  -d'{"method": "GET", "path": "/add_one", "handler": {"code": "AGFzbQEAAAABBgFgAX8BfwMCAQAHCwEHYWRkX29uZQAACgkBBwAgAEEBagsAGgRuYW1lAQoBAAdhZGRfb25lAgcBAAEAAnAw" } }' \
  localhost:8900/routes
```

> Test the created route

```
curl localhost:8700/add_one
```

> List all routes

```
curl localhost:8900/routes
```
