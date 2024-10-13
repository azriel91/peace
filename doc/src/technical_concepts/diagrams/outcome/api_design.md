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
  file_download: ! FileDownloadParams
    src: "https://example.com/file.zip"
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

#### With values

From these item params:

```yaml
items:
  app_file_download: !FileDownloadParams
    src: "https://github.com/my_work_org/app/app_v1.zip"
    dest: "./target/customer/app_v1/app_v1.zip"

  config_file_download: !FileDownloadParams
    src: "https://my_work_org.internal/customer/app_v1/config.yaml"
    dest: "./target/customer/app_v1/config.yaml"

  app_file_upload: !S3ObjectParams
    file_path: "./target/customer/app_v1/app_v1.zip"
    bucket_name: "my-work-org-12345-ap-southeast-2"
    object_key: "/customer/solution/app/app_v1.zip"

  config_file_upload: !S3ObjectParams
    file_path: "./target/customer/app_v1/config.yaml"
    bucket_name: "my-work-org-12345-ap-southeast-2"
    object_key: "/customer/solution/app/config.yaml"
```

<object type="image/svg+xml" data="api_design/multiple_items.svg" /></object>

<small>[source](https://azriel.im/dot_ix/#src=LQhQBMEsCcFMGMAukD2A7AXAAgG62svAIYA2oAFpPkdPOQJ4ahZZEDuAzkyy+xwPocAzNgDEWAOYkUAI1JY0RALawOAByLxYAGiwyAroiwlIeDlkhpWWDpamwscCajTMeLcX0FD+RNYJRDclgiDkR+ACZud1ZOb34DeABrWHClen42FGgk-myJfgBGCKEAFgBWX38OQMRg0PCotxjYgWEE-WTU-nTM7Nz8opKKqoCgkLDI-nh9MJQVaACSQxcq-z9-HEL+AC9INWwAbwBfZpivdsSUtIysnLzoAuKyyr8xuonG6dnEefwllboNbTdAAM0gBXoyhIR1ObmcdX0MhBSmiHiwAGVyIESOAsGwHOQiHgLGhEPgVFAiOSFChwKosKDslgPlh8qw0Hi4GoUAB+M7iAAqwSwAAMAGKQEiwAAiKDYaGkRHAoos5KUNmx+lxaAA5EY4KQSPQbGoEJB5MQOLAAHRnBHkJEonq3foPAobYFbXb7WGgNy9O4DR78Szk6CKGFnQNuwZh-CR75zBZe7bwMEQ-hQpQwrAnf0saTEEjYsJo4woYul8KIGgSbozZP-T3elvbPYHPOndxF0jV-i1x4Nn5-RZtkFocGQ6F+0BoOmwfiKFRceGQRHI9OorAAIgdSJtW53a43zpj90GnvHHewO7bNo7x5Y5+DBXjEdIt5f7pt78jT6wb84zJBNSCTX4U3HdNJ0zbNcx3aCpxtOCAN7EsUDLXc0OrVDKz7DCazrYcmzHNRNm2a9fV3e9HzcbCCIHIjwkbCDmzI-hvUQ2CZ13LiJGQ6EAL4W8AEElCIHZ0CwAB1WAZExfAcEgLQOCEuJhFvDEhDUtofCubogJDZ4RjeGpxgaSIv3oYAg2AfJgGM8pgD8YAzI+BpgAiHT4n0m4+gvIzhleapanqSYInA0cAWQIEr3Y70b2o+LCgffZvMuTprhdfzXyGF5RjcsKvhYqKamWGK0GBPisx4hCM34lD-XEed6X4ekOHgVdxBYfdN3mbKg3dYFKM7HdyEQRA1C4AB6abesPeZpsMiRpo2VbktStRj26wDXQCt8QI-EhIsg5KJynGqc1vcbJpmpa9tfX9DsjaaSoWdbyNe+qBJzbaeHoyZB3rZiR1O8jUx9UabWmoHUle0H8A+jjCiRrZNr+ws8PQwGmJOtjwequDb2h2HEHhkjUZRvifpIDHWh8zKDIeobHIK0LPimN7-jKwFKri8HEp3cnWOgaaeYqj7KfR0AdouPTGb8wbBlZ0z2YsiKucWcXVk9QnauF0cxZQcqXEl6nGrnBd+FgJQUAAK0gVcevXR0+u3QAeDcABH2T1ds9mcvdiRuwQBeDcAQZ2A39kM-0-LAvYjnKhuj47NYhvWrqwYPABGduiserEPAG6dnOqwY0m8dI8Gg4z8PMeLnGhxBkjU-qy7cyztw5Y6LpFdjQL8tV8zwuwQA+DcAYr32-U+Wu4GnuniCtmB+KhGteN3nhrOxKw-H3TO6y5a8pMkKF85pfop19i09b7PQFgcB6ydita-Cfher9hOA-Wdf9n4fg4AAR30VQiByzAEkC7J0W5p77TXgLX0HA2BnBAQDIBaB4E13wnXYGZcIYjW-i-CBe9+bI0ht-OA6h0DWmAaA08+DI4ekDp-A41oEEP3QYReuWDxwjSLqw7+e8k4cLOhfEhsB-6AMoXw56YEU5QWbkTOBzCkEYBQdw7GbDMHSMEbI6EvDaGhkkcnE+MiYLThzMIshaAKHMIkeGRMGiCZaKukw9wiDc4l1xnYohF8VH9lLh41sDDv4d18pA3KKtD7uXClg7WsV6EwIOMwoJCsQksznv3CJi9G7RL5rEohHYFGuIwcREW2CGHeLcewvxaYHHHV8BPHeTM369wPu8Iqx9MkrwqlVaplDElTz3mElpHMNYnyyV04xLd8mP0YhUwxmjxlwX9BIaAfhyBKR2L4Ca0B77qBMGgVQ2AZjQDwOANwN96wTjCMssM980GqNwWAt2yT34Qw7MI0RmFQSkCcf9ApT9rGgQMY3IxF04JvIAR8r5sBTm30XFALZ5YkH3Oof1AhOSEpfx-iI8FQC9CaCSGcRFTyo76IEfY+Z2jMXvJxXIZIBLfnTPUbMiuATanb2Cf01J4TWnDPaSbGJH84nYBpfins9LfFMs8dUwJdT2W6IGYVIZUSOln38BfIVeL-QfBUNEMI9BpQ3KwOIM5sLYCfO1IgA1O0lK2BkFKdcjBSTWsgDIaUBKITjWwAAMhMBIcaZwWDghIMdDgRJWrzmgOJXMupCgAAYY26n9YyKUwbQ2LmxHgaA2BdTlHjYmwNKblSLiZI2LNsbc0tHzYIVNvgkCmFgKWuNCaWiXJQCkKthalzZEjVm0ojbE0trbSGjt6b8BZqEH25tiBoCtsXEO1qmhkB4CzTm8tMQB2zsQHq+tWBwAoAmjfRNGhwBQDQBIMdTaeB4PmOWFgAAeW92AABUPq-XNqJGaEE0hM1YDUPoaAahXXuH+UdG9WB71PpfYgft77Fzpi-dgJZRB6B0sfqB8DWBn3uqg2+vwsHjbZCFcsKF7hhKJvQ5h312G10wc-QR1gSgZD4DOHLNDD6MOQbzcm9tYau0x11BECd1HcO0e-coRj0BmMyqSRyvuXKhmsYg1h6Dwm4N0cQ8hoAA)</small>


#### Without values

TODO


## Learnings

### State (Values) For Non-Existent Items

Options:

1. 🟡 **A:** Always get item implementors to return a value, but tagged as unknown or generated.

    Pros:

    1. 🟢 Can always generate a diagram and show what might be, even if we don't actually have values to do so.
    2. 🟢 Whether using `dot` or `FlexDiag`, the diagram layout will more consistent from the clean state to the final state if nodes are styled to be invisible, rather than not present.

    Cons:

    1. 🔴 What *is* generated can depend on input values, and if we have fake input values, we may generate an inaccurate diagram.

    2. 🟡 Choosing a default example value of "application version X" may cause a subsequent item to fail, because the application version doesn't exist.

        Item implementors would be constrained to not make any external calls.

        1. If we made every parameter value tagged with `!Example` vs `!Real`, then it can avoid this problem, but it is high maintenance on the item implementor.
        2. Maybe we pass in an additional parameter in `Item::apply_dry` so it isn't missed.
        3. Any `!Example` value used in a calculation can only produce `!Example` values.
        4. 🔵 If a dry run is intended to detect issues that *would* happen from an actual run, then we *do* want external systems to be called with the parameters passed in. e.g. detect credentials that are incorrect.
        5. 🔵 If a dry run is **NOT** intended to detect issues, then we will have simpler code implementation -- never make any external system calls (network, file IO).

    3. 🟡 Choosing a default cloud provider in one item, could make a nonsensical diagram if another item uses a different cloud provider.

        Note that an `Item` may still generate sensible example state based on example parameter values.

    4. 🔴 Code has complexity of either another derive macro generated type with `RealOrExample<T>` for each state field, or a wrapper for `RealOrExample<Item::State>`.

2. 🟡 **B:** Add `Item::state_example`, which provide fake state.

    Pros:

    1. 🟢 Can always generate a diagram and show what might be, even if we don't actually have values to do so.
    2. 🟢 Whether using `dot` or `FlexDiag`, the diagram layout will more consistent from the clean state to the final state if nodes are styled to be invisible, rather than not present.

    Cons:

    1. 🔴 Even more functions in the `Item` trait, creating more burden on item implementors.
    2. 🟡 Choosing a default cloud provider in one item, could make a nonsensical diagram if another item uses a different cloud provider.

        Note that an `Item` may still generate sensible example state based on example parameter values.


3. 🟡 **C:** Return `Option<T>` for each value that is unknown.

    Pros:

    1. 🟢 Never have false information in diagrams.
    2. 🟢 Code can put `None` for unknown values.

    Cons:

    1. 🔴 Unable to generate useful diagram when starting from a clean state. i.e. cannot visualize the fully deployed state before deploying anything.
    2. 🔴 Code has complexity of another derive macro generated type with `Option<T>` for each state field.

Let's go with **B**.


### Notes From Designing Diagrams

1. The node ID must be fully derivable from the `ItemLocation` / parameter / state, i.e. we cannot randomly insert a middle group's name in the middle of the ID.
2. The node ID must be namespaced as much as possible, so two same paths on different hosts / groups don't collide.
3. The node ID must NOT use the flow's graph edges (sequence / nesting) in its construction. This is because flows may evolve -- more items inserted before/after, and the node ID should be unaffected by those changes.
