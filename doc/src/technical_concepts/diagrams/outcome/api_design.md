# API Design

> Designed around 2024-07-13


## Requirements

1. Be able to render a diagram, with/without the item existing.
2. Framework should be able to determine if an `ItemLocation` from B is:
    1. The same as an `ItemLocation` from A.
    2. Nested within an `ItemLocation` from A.
    3. Completely different.

## Information Sources

Known information before / during deployment:

1. Flow edges -- `Edge::Contains` / `Edge::Logic`. Though maybe we want to reduce this to just `Edge::Logic`.
2. Flow item params that are specified.

Missing information before / during deployment:

1. State generated / produced / discovered through execution.
2. Parameters calculated from state.


## Desired Outcome

> What it should look like.

1. `Item` returns a `Vec<ItemInteraction>`, where an `ItemInteraction` denotes a push/pull/within for a given source/dest.
1. We need `Item` implementors to render a diagram with `ParamsPartial`, or some generated.
2. However, partial params means `Item` implementors may not have the information to instantiate the `ItemLocation::{group, host, path}`s for the `Item`.

### Single Item Example

#### With values

From this item:

```yaml
items:
  file_download:
    src: "https://example.com/file.zip",
    dest: "./target/abc/file.zip"
```

One of these diagrams is generated:

<div class="column_half">
<object type="image/svg+xml" data="api_design/file_download_with_spaces.svg" /></object>

<small>[source](https://azriel.im/dot_ix/#src=OYJwhgDgFg+gzgFwJ4BsCmAuABAIwPYAeWAxLoQDRYDGAliFegFAAmdaVCNeAdtgG5oQnKmBSMoNQWHpQkGRlixoCYALYR0MKnlXzFi5Wo1otOmADMamgF40I2AN4BfBVhR4RKKHkR797z29EGARpYDQEGDAcKgsrE1t7LGdGRm48ZhNuNTQ4P0N1TW1dLAAiAuMAOmLS1wqis0sbO2xSprRKxNrFANEghFberx8EbrcPPpGQsIiomLjmpNLQkHCEAHpoqnX2zrtatIyTTLgqPLqVQpNihYSWsqgEBAg89fX6juKd+L2IUpIsABXEBiHoTYbBFZrOaxdowRKtHaAlAodYQMAIKDrBB4bEzDZbb7oX4HdKZGBoVR4ABWNHOBkuxlMJX0rP0gB4NwAI+xcjA1VLd4fcsIBeDcAgzuuIb9bBsmUiwDdOxLwf1pqtZlsBQiReLGGhmOF6UpGXyNXYYDBJVMoWr5nDEmaQGgAI6A3IDLAAbQ+zJNEEoFsh+JhPoAujyrt7babzUrLYH1ZGIPbchAeHBMB6vTcE36YwHVZF4-FBRBQ4xQJAoHwaNYok8QAbdeEYKoaNx0NwDQzeddGkW7dHArH80GE0nna7sAAmVxd8NZvtR-2RK0Fm0LxMwB1wFMd9PTgx6450TuG7sR9dmpcq6GFxZjl2+XBgKgAa1SjeOuTOflIqjAL9yLAHXHRAsDAbhmCA5NUzQJRDzgLB0gQLA8AEEFIDDJl5zvAdJjzG81xw4CHzdAByRRSNSTFKUwVxEFQXI-EUMlP3MMBkQQE9FEQEA8AAmAAHcaGYTFsFIgBGSiZWUBBwDEqhgTgPAQAAWhTVsEEEIC8EBCDdRUlBgCk-QPxgTI2I4rjDVksBsAkZhMm4GdcBQF0om4Gg-005hsAAMhwVyTHAzyMV1ZzuKgSAe3cEBsACl1wqwHi+JMej0GwZgwDgKBdUUUhsuUzFwMg8xlI9XyAD50UxYMMHdZKAJUzLsukcA5AAZlDGUGpMISRKgMT3UnCACGDYzWR6+BIvJdIQD-FAxIANgABmW8b9CaFAprAGblPmsSAFZVvWhkbOwCqVMSxR3Uq6qoFq4KvLQFT6tklKmqy7xzHMNMEBUqkBBgcS4HNVs0GkGBW0sDzNK62UFPrZS1LwDTBGczNexwq8VxHC9NydEimP0AAeYnsAAKnioKPKe5hEpkuSsAuq7yqqjF7owR7QpenqPoK77fv+1DnodNC0xgdqQZQMGIah1saFhlmEaU1T1O4TSQHRo0e35UdcIhZc40Iu4Ny3Hc0yJxRSYpqn3JC7zEsmvrRKwUj3XEkaxvpggzqZy7ZVZu6Hpp7nXt4xrmq+n6IiFwHJdB7hwZASHuGhhW0DhmUbrZmq6t5yPWrAORJ0ztllaRtWNec6XgEePya8eRLNu23a5tEMTxOOpurC2lryW8NDDrW7uURbkxSoUvJXc74eZWbvugo4GgBA7rvure-iF5gWb9tdgAWNe2UmreB8EMT2sPiaN9S6bF84FfXaO2ej+v+BkHSrBmDwJ4wpldEHNbMAc+60Ma6EStbLA5MG4IEdpFCA0VlLYDUDgNGModIIGlonUwMVsBIDQCiPAAlq65gGOAsmkDoGwKitgxBLkEqMDOFPZmigAACAEkDmHAKoQC+dPp4AFjHAGsEHDOWWgAUn0A4JK18+ZR1+tgPeAAOAA3FgFwrIZ4SKkbw-m0c3TLVUS4Vw6isBsLQBwrhPCZGR34Xo2OIs0Bi2EaIiRihtHWL4QI-RhjnKaOSNI8Oz0bFeIUSotRxjGBAA)</small>
</div><div class="column_half">
<object type="image/svg+xml" data="api_design/file_download_with_ports.svg" /></object>

<small>[source](https://azriel.im/dot_ix/#src=OYJwhgDgFg+gzgFwJ4BsCmAuABAIwPYAeWAxLoQDRYDGAliFegFAAmdaVCNeAdtgG5oQnKmBSMoNQWHpQkGRlixoCYALYR0MKnlXzFi5Wo1otOmADMamgF40I2AN4BfBVhR4RKKHkR797z29EGARpYDQEGDAcKgsrE1t7LGdGRm48ZhNuNTQ4P0N1TW1dLAAiAuMAOmLS1wqis0sbO2xSprRKxNrFANEghFberx8EbrcPPpGQsIiomLjmpNLQkHCEAHpoqnX2zrtatIyTTLgqPLqVQpNihYSWsqgEBAg89fX6juKd+L2IUpIsABXEBiHoTYbBFZrOaxdowRKtHaAlAodYQMAIKDrBB4bEzDZbb7oX4HdKZGBoVR4ABWNHOBkuxlMJX0rP0gB4NwAI+xcjA1VLd4fcsIBeDcAgzuuIb9bBsmUiwDdOxLwf1pqtZlsBQiReLGGhmOF6UpGXyNXYYDBJVMoWr5nDEmaQGgAI6A3IDLAAbQ+zJN9jgAHdKBbEABdHlXb2203mpWW-Ewn323IQHhwTAer03SO+tCBmOQuPqrMYbhoUOMUCQKB8GjWKJPEAG3XhUzcRDgGjcBAGhm866NeKCiBmoORK2RQsDu0wB3O13YcyiVN1PXHOjdw29iOTqMjlXQieLROz3y4MBUADWqUxlMwrkQqFyfkUZOOaAXyK7T-0bbw55MfpoZhMWwAByABGEDXFZZQEHAUCqGBOA8BAABaZMOwQQQsBAPBAW4TJmBQlBgEg6CVxgTJ3xQT8oP0GC4KwCRmEybhaJwFAXSibgaFUDFdWwAAydjOLAbjeMw5haMUOAoEgPt3BAbBhLQKSAR-P94GQdBsGYMAZN1RRSBk5DMVE5gsHMZCPQEgA+dFMWDDB3XUtAUN0mTpHAOQAGZQxlFyYAAoCoFA90ACYIAIYNSLZAKPPJdIQF4lBQIANgABnSmLWSaFB4FkhLkOS0CAFZMuyuiCFgsBsBslDVMUd1bPsqBHNEni+JQ5zYN-Vz3O8cxzFTBAUKpAQYDCuBzQ7NBpBgDtLG4zC-NlJq7IxVqnJcty9NkkAvIwMDyDC47ToAFlO1LyDA9KVplBCG2QtC8AwwRaIzftD2jQJY1VccbW3IdpydF1fFUgAecHsAAKmUriOok1SAqC4CsHAiqGWq2r6tlaz1ocjB2vE1zupwv8duMwbhtGvABBQh0BAbExvKmlAZrmhaOxoZaGrxlrHNJ3qKb2g6rpOi6Tsl67rtu3mHqQ1D0M7N7oKNPt+SzYc81HAsAa+h04GTVtbxlSGYbhom+Mk1k2eAR5BNtx5VNy-KwEKpLRFAm6sudqw8vikxvEZ0qfZlF2A4sDxATyNHvYxiy-dd8kz04AQvfKpGeo0iPEuKtGzoz-ys5MCOg8EUDvML2Li6TkwU5oNO0bK0Pq7JkutLTZg8CeXVVPRZiO2ACuKo+3QIahrBocdhAkdkiB5OQ7A1BwFW2VwhA2ZLUwFOwJA0BRPA-Vokcv0UM3J+n2e5O3xfcA4lTGDOGO6tcAABP8kHMcBVFyLBtv6vAVMIg0wEMkWi6UACk+gHB-2LsLQBQ0IjYDAmBVKABuLALhWTeygTA-+u0EHDWwOlDBWDMFvw-l-HIcBYFt3gUAkaY1XIM0EKmMBrJIHQNoULABDDiGkNojg5I3Dya8MQW6FB6DyGKBcEAA)</small>
</div>

Which one is generated depends on how easy/difficult it is to figure out which node ports to use -- we don't necessarily know the node positions.

Also, we may not necessarily use `dot`, but a `div_diag`.


#### Without  values

```yaml
items:
  file_download: { src: "??", dest: "??" }
```


### Multiple Items

TODO

