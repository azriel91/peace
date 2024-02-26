# Outcome

An outcome diagram should:

* Show the "physical" items that exist / yet-to-exist / to-be-cleaned.
* Show which steps are linked to it, e.g. clicking on a file shows the steps that write to / read from the file.


## Determining Information for Rendering

To render the outcome diagram, we need to deduce the physical things from `Item`s, and determine:

1. **Source:** where data comes from, whether completely from parameters, or whether parameters are a reference to the data.
2. **Destination:** where data moves to or the system work is done to.
3. Whether the source or destination are declared in parameters.

As of 2024-02-18, `Item::Params` is a single type, which would take in both **source** and **destination** parameters, so we cannot (realistically) determine the source/destination/host/cloud from the `Item::Params` type.


## What Rendering Makes Sense

Conceptually, `Item`s can be thought of either an edge or a node:

* **Edge:** The item represents an action / work to between the source(s) and the destination(s).
* **Node:** The item represents the destination thing.


### Naive

Consider the following diagram, which is the first attempt at rendering an outcome diagram on 2024-02-18 -- this uses edges / hierarchy to draw nodes and edges:

<object type="image/svg+xml" data="outcome/2024-02-18_outcome_diagram.svg" /></object>

#### Notes

1. It is not clear where the `app_download` `Item` transfers the file from, or to.
2. It is not clear that the `iam_policy`, `iam_role`, and `instance_profile` are defined in AWS.
3. The links between `iam_policy`, `iam_role`, and `instance_profile` should probably be reversed, to indicate what references what.
4. The `s3_object` item transfers the downloaded application file, to the S3 bucket created by `s3_bucket`, but it isn't clear whether we should be highlighting the link, or the nested node.
5. If we highlight the link, then note that it is a forward link (data is pushed), compared to point 3 which are backward links (references).


### Hosts, Realms, and Edges

If we could show:

1. The hosts of all network communication
2. The "realm" where resources live in, which may be a cloud provider or a region within
3. A node for each resource that is created/modified/deleted
4. For each item, the edges and nodes that interact

then what is happening becomes slightly clearer:

[dot_ix](https://azriel.im/dot_ix/?src=LQhQBMEsCcFMGMAukD2A7AXAAgG62svAIYA2oAFpPkdPOQJ4ahZZEDuAzkyy5EQLYB9AA4oSkeIywBvAL7MefIdDGxschbzQdERNPFgiVAM0gk1M%2BTywcAzIIBGAV3gBrWIm7Wb9lA4BWCJ6WoAokKMQk5Cg6XixEwsKC4ChsaOFE4OpWPAlJsAAeiNBESNmhLADmkIjkTg5xWNW19YJ5ggBekMLloGgo4IaQaMYxXuGR0bHePNJYsPwo-pDYgLwbgN07ADRYaAIWAEQTpFOI%2B9szWIMc8Nj7AJooTtAAPA7QWAD0AHzwKPzCTkQ%2BH2WBy7C4FwucwWSxWWEAgGSAeD-trt%2BAdwWdITMrjcsPsAIL8IgddBYADqsAcr2g3wAyvgcBJYBwQTlmnUGliWNDFss1oBMHZRe1u7PqmK5ONuAHEahzWQpRQ42olOt1sNz5ry4atAIM7QrRtzyADousJ5fEVSk0hkst4ebC1oBTnf16MtqXSKEy4p4krxxtN5tYKsKxVKwTtmodWFWgCGdl2G4NFEpIb0%2B5m4-YfFDCRAfPKBpQiMQSKQze186OAXZ343jC6JxJJU2nrrcAKIAYQATBhqZ8vviSOE2L26bYsAAlWBe0EKQsqczqqGRiurQAMuzX9nPVE3rL79h3O1gAJL4gCyI6%2B9ZLF6IiF0dDRaFOM60Oj0BiMKFMC5ky7hgB4NwB3-eRHZhVrbRdH0QxhBMMxYExPcDywYY3ygi951gVg71KchH2fHI7EcFx3HDMs-2wQA%2BDcAYr2N0I5w3A8HcWD3QZFmAGhkGMMMWRfHxBD8QIykhcttT1UCDTxQiBKCJjLnTBNhBNbp5VAWBwEqZkvHaK0PUyQRlXyJMw2wABtbT3RtbZ2hDZNEAAXVnAQiwbeh9K3H8TLrYtJG2dzYAc3gnIwtyIPfaDYI8vzfNCqDP2-fyFHM61PXAfSpICIJTKS3TwG2dLBPsxyhCvSQ0t8DKhM8pySvoPLyoKgKmllVp2lNfTrKMyrFQM1VhCsxNQyQBzQF0Mw2GGVL4BIIgOA4TSFDUjTklgLinBIRAIUocBBjQRK3WSvTBHyzKsC%2BYBNCwEyADJL1vcg7IwHQVHcYBOwu67btqB6npQF6JsgSoUGAAA2AAGUH3pu%2Bt6EBtBvuKX7YGAf7AZB8HIc%2B%2B6MBMn6XvAGbyBoEpGFsRqWA%2B4Q7oevRICJIFgBxhG8YJr9jDmxBgEWPBBAAFg4QRxDQKdoEEYZTDQGoEusCmxBh9AHuiPBoAwb8SGRtAoFRnn0elqGqYwRX8EepmkZRoHtYh3XL1l2GHtGeAnC4VX1c1oGAFYdZ4Cn9ftx2MEeRBBaRt6rcpr6VYiP2A6Dl2AaB2xPfJvXw99rho%2BGJH8Y4cg1IxsOsdTjAVCcDW1OAAoyGsB3oA4FBoGAURhiBaAKmQ6rvNco76pOradsSzhsDOvOfbMNWBAcfBgAARkTy7k6x3Gkan4fw8X9j%2BAn%2BuLZXrHGeezOCaJohGE7Mm58xhWUCVlXR-XzfgA9nfL%2Bvtfx8nj3La9%2Be7cjp3b-oWAg5UivVnt7FOv9jb7zvu-UB38I4OzToCGOIcv4X3gVHJBGdgAAKAWwYACdP5JzQYXdOQtgBZxzuAJ%2B6CuDF1LuAculceDV1rvXRuT58CtyOFEGIwQh6hxHoOYADgSBOCXrAtBa9l4CNXibYRoikbbxkbvNeFCj4nzPmArGhtlbOxEWIh%2BhDz752fkbNe%2BikYf2oYXZ2HBXD0BAUYrRP8EGQMRvIgxVjlEuIwYHLBKCiEmJof7TBZC7EOIIdYiBpCD7Z1zt44JdDBgMIrhdFhdcG4oCbpwhUzVOT8NQUE2x016YzycXAqR1C14cBKYoiRQS97uLUdAYmGBT7UJ0TfIRNTbyWPKWgzp1TamGP6UUiBztKjE0cVE1xQzekjJmb45BizEF%2BLIZM4%2B%2BD6k%2B2iaE2JlCVlF0ePQxhaSnisMydkluoB4CzUHudFgAABdw9BjAlDRBwGwciKGs3ZpzK%2BmFpAXVBgAUlmF8qBPzjBsw8NgHmAAOAA3LxFgZSwVzFUSzaF7NsCg2RTkWQQA&css=AIawpgngZgTghgWzAZwATIC4wPbgLQAmcyAFtlFMmBngtgG5ioDeAUKqgAwCkHL6WXGELEyFKhgBcqACwAOANyoAvu1QBGTj36Yc%2BIqXKVq0zktXKgA)

<object type="image/svg+xml" data="outcome/2024-02-18_outcome_diagram_2.svg" /></object>

#### Notes

1. There is only one level of detail.
2. It is useful to have expandable detail, e.g. hide a full URL, and allow the user to expand it if they need to.
3. It is useful to show and hide animated edges while that step is in progress.
