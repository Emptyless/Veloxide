# OPA Policies

This directory contains the OPA policies that are used by the OPA Sidecar.

The middleware in axum will forward the request to the OPA Sidecar, which will evaluate the request against the policies in this directory.

You can test whether a request would succeed or not by running:

```bash
opa test . -v
```


