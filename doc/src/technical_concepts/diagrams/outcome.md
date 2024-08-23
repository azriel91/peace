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


### Source and Destination

An `Item` is actually a step that encapsulates:

1. Source location.
2. Destination location.

This can be further broken down into:

```rust ,ignore
/// Can be source / destination
enum ItemLocation {
    File {
        host: Host,
        path: PathBuf,
    },
    Cloud {
        cloud_provider: CloudProvider,
        global_or_region: GlobalOrRegion,
        subnet: Option<Subnet>,
        url: URL,
        name: String,
    },
}
```

Instead of using a flat enum, we may want to use a fixed set of data structures for simplicity, or a `trait` for extensible implementations.

<details><summary>Data structure option</summary>

```rust ,ignore
enum Realm {
    Host {
        address: String,
        port: Option<u16>,
    },
    Cloud {
        cloud_provider: CloudProvider,
        global_or_region: GlobalOrRegion,
        subnet: Option<Subnet>,
        url: URL,
        name: String,
    },
}

struct ItemLocation {
    /// Host or Cloud provider
    realm: Realm,
    /// Not necessarily a file path, but segments denoting hierarchy 
    path: Option<String>,
}
```

</details>

<details><summary>Trait option</summary>

```rust ,ignore
trait Name {
    fn realm(&self) -> Realm;
}
```

</details>

Note that we want to support nesting, e.g. an application server is "just a host", which may be nested within a network, or a cloud provider, or nothing.

How should we do the nesting:

1. Strictly defined levels, implementor selects:

    1. Cloud
    2. Availability Zone
    3. Network
    4. Subnet
    5. Host
    6. Path

2. Arbitrary levels, levels are not necessarily defined by network-hierarchy.

    Note: This doesn't allow consistency across item implementations, or, it requires the tool developer to pass the level into the item.

    Actually, what we can do is:

    1. Allow the item implementor to specify the list of parents that this item resides in.
    2. Each parent may be an arbitrary layer around the current item.
    3. The peace framework will have a `struct Layer(IndexMap<LayerId, Layer>);` -- this is essentially `NodeHierarchy` from `dot_ix`.
    4. When the `LayerId`s from different items match up, those are inserted into this map.
    5. When we render, we render layer by layer as the hierarchy.

### What we want

1. Tool developer passes in parameters for the item.
2. The correct realm for the item is automatically deduced.

e.g.

#### Tool Developer

```rust ,ignore
let file_download_params_spec = FileDownloadParams::new(..).into();
```

#### Item Implementor

<details><summary>File Download</summary>

```rust ,ignore
impl Item for FileDownload {
    fn location_from(
        params_partial: &<Self::Params<'_> as Params>::Partial,
        data: Self::Data<'_>,
    ) -> Vec<ItemLocation> {
        let host = params_partial
            .src()
            .map(Url::host)
            .map(Host::to_owned);
        let port = params_partial
            .src()
            .map(Url::port_or_known_default);
        let url = params_partial.src();

        let mut item_locations = Vec::new();
        if let Some(host) = host {
            item_locations.push(ItemLocation::Host { host, port });

            if let Some(url) = url {
                // May be rendered using the last segment of the URL as the node name.
                // The full URL may be used as the tooltip.
                item_locations.push(ItemLocation::Url(url));
            }
        }

        item_locations
    }

    fn location_to(params: &<Self::Params<'_> as Params>::Partial) -> Vec<ItemLocation> {
        let path = params_partial
            .dest()
            .map(Path::to_string_lossy)
            .map(Cow::to_owned);

        vec![
            ItemLocation::Localhost,
            ItemLocation::Path(path),
        ]
    }
}
```

</details>

<details><summary>S3Bucket</summary>

```rust ,ignore
impl Item for S3Bucket {
    fn location_from(
        params_partial: &<Self::Params<'_> as Params>::Partial,
        data: Self::Data<'_>,
    ) -> Vec<ItemLocation> {
        vec![ItemLocation::Localhost]
    }

    fn location_to(
        params: &<Self::Params<'_> as Params>::Partial,
        data: Self::Data<'_>,
    ) -> Vec<ItemLocation> {
        let region = data.region();
        let availability_zone = params.availability_zone();
        let name = params.name();

        let mut item_locations = Vec::with_capacity(10);
        item_locations.push(ItemLocation::CloudProvider(CloudProvider::Aws));
        item_locations.push(ItemLocation::Group(region.to_string()));

        if let Some(availability_zone) = availability_zone {
            item_locations.push(ItemLocation::Group(availability_zone.to_string()));
            if let Some(name) = name {
                item_locations.push(ItemLocation::Name(name.to_string()));
            }
        }
    }
}
```

</details>


#### Framework Maintainer

Go through all items' `Item::location_*` and add to outcome diagram -- see `flow_spec_info.rs`.


### Problems

#### A: How do we get items to be consistent with the item locations they publish?

Cloud provider name, region, availability zone, etc.

**Options:**

1. 🔴 We could provide a crate with common enums.

    This requires:

    1. The framework to have knowledge of many many types, or be inaccurate.
    2. Item implementors to always be up to date.
        <!--  -->

    It's a lot of maintenance, so probably not a good idea.

2. 🟡 Item provides default `ItemLocation`s, developer can pass in names.

    This isn't too bad, as it allows tool developers control over names of the items.

    It still requires item implementors to use the same `ItemLocation` variant.

    It could mean additional development burden.

    - If we go with sensible defaults, then we hope that item implementors use the same conventions.
    - Tool developers passing in names to the items is an escape hatch in case the conventions are different between item implementations.
    - Item implementors have to take in the names instead of only default them.
        <!--  -->

    If item implementors have consistent values for their `ItemLocation`s, then the inconsistency downside is alleviated partially.

    For example, extracting the host / port from a `Url` could be offloaded to the framework:

    ```rust ,ignore
    #[derive(Debug)]
    enum ItemLocation {
        Host(ItemLocationHost),
        Url(Url),
    }

    struct ItemLocationHost {
        host: Host<String>,
        port: Option<u16>,
    }

    impl ItemLocation {
        fn from_url(url: &Url) -> Self {
            Self::Url(url.clone())
        }
    }

    impl From<&Url> for ItemLocationHost {
        fn from(url: &Url) -> Self {
            let host = url
                .map(Url::host)
                .map(Host::to_owned)
                .expect("Expected URL to contain a host.");
            let port = url
                .map(Url::port_or_known_default);

            Self { host, port }
        }
    }
    ```

    Item implementors will implement `Item::location_from` and `Item::location_to` like so:

    ```rust ,ignore
    impl Item for FileDownload {
        fn location_from(
            params_partial: &<Self::Params<'_> as Params>::Partial,
            data: Self::Data<'_>,
        ) -> Vec<ItemLocation> {
            params_partial
                .src()
                .as_ref()
                .map(|src| {
                    vec![
                        ItemLocation::from_server(src),
                        ItemLocation::from_url(src),
                    ]
                })
                .unwrap_or_else(Vec::new)
        }

        fn location_to(
            params: &<Self::Params<'_> as Params>::Partial,
        ) -> Vec<ItemLocation> {
            let path = params_partial
                .dest()
                .map(Path::to_string_lossy)
                .map(Cow::to_owned);

            vec![
                ItemLocation::Localhost,
                ItemLocation::Path(path),
            ]
        }
    }
    ```


3. 🟡 Developer specifies `ItemLocation`s (from and to) for every item.

    Removes the item location inconsistency issue, but also means every tool developer has to specify item locations for every item, where this work could be commonized.

    This also addresses the problem B below, where the developer has control over the level of detail.

    Essentially we are placing the burden of defining the outcome diagram onto the developer.

    This isn't "nice" when the developer has to extract host names from URLs (common burden).

    Maybe the `Item` implementor provides a method like this:

    ```rust ,ignore
    impl Item for MyItem {
        fn item_location_from(
            &self,
            level: MyItemLocationLevel,
            params_partial: &<Self::Params<'_> as Params>::Partial,
        ) -> ItemLocation {

            match level {
                MyItemLocationLevel::Host => {
                    let host = params_partial
                        .src()
                        .map(Url::host)
                        .map(Host::to_owned);
                    let port = params_partial
                        .src()
                        .map(Url::port_or_known_default);
                    ItemLocation::Host { host, port }
                }
                _ => todo!(),
            }
        }
    }
    ```

    This interface is awkward -- we don't know the params_partial at the point of flow definition (for the developer).

    It could be the developer that defines that function above, combined with framework helpers:

    ```rust ,ignore
    let my_item_locations = MyItem::item_locations_spec()
        .with_locations_from(|my_item, params_partial| {
            // with the `From` implementations for 2. above:
            params_partial
                .src()
                .as_ref()
                .map(|src| {
                    vec![
                        ItemLocation::from_server(src),
                        ItemLocation::from_url(src),
                    ]
                })
                .unwrap_or_else(Vec::new)
        })
        .with_locations_to(todo!("similar to above"));
    ```

4. 🔴 Magically infer based on parameter names.

    Too inaccurate.

5. 🔴 Item implementors use a derive proc-macro.

    Too many annotations -- probably easier to write the trait directly.


#### B: How does control over whether a group is drawn get defined?

1. Users may not want so many levels of detail -- it can be overwhelming.
2. Developers may want sensible defaults / not require them to set whether a group is drawn.
3. Developers may want to set whether a group is drawn.

**Options:**

1. 🟡 They can't, everything is always shown.

    Not the best user experience -- too much detail can overwhelm.

2. 🟡 Item implementor guidelines to not include too many layers.

    May be okay? It means either we have too much / too little information sometimes.

    Perhaps this is diminishing returns, and it doesn't matter too much.

3. 🟡 Developer specifies `ItemLocation`s (from and to) for every item.

    Same as **A:3** above.

4. 🟡 Item takes in parameter whether each layer is visible.

    Probably in the `Item` constructor, so we introduce a compilation error:

    ```rust ignore
    fn new(
        /* existing params */
        #[cfg(feature = "web")]
        layer_visibility: MyItemLayerVisibility,
    ) -> Self { /* .. */ }

    struct MyItemLayerVisibility {
        cloud: bool,
        availability_zone: bool,
        subnet: bool,
    }
    ```

    In tandem with **A:2** above, maybe it's not worth the complexity.

5. 🟡 Draw everything, developer provides separate information for layers.

    1. Probably want the `Item` to export what layers it draws.
    2. Developer passes a `Map<ItemLocation, DrawEnabled>` to the framework.
        <!--  -->

    Feels weird / unintuitive as a design, not too weird as a user: "I see everything, I want to turn that one off.".


#### C: Consistent Diagram Layout

If the `ItemLocation`s changes, such as the number of `ItemLocation`s change (hierarchy level change), or the `ItemLocation` variants change (different rendering style applied), then the layout of the diagram can change:

1. Position of each box can change: pushing what used to be "here", "there".
2. Width / height of each box can expand: it doesn't fit in the screen.
3. Colours can change: different `ItemLocation` type.
4. Edges can change position.

It can be confusing to follow if these keep changing, which is counter productive to the intent of communicating clearly which physical systems are interacting.

**Options:**

1. 🟡 Don't solve it now, we don't know how much of a problem it is.

    `dot_ix`'s HTML `DivDiag` is not very mature anyway. We may end up developing this a lot more.

    This is taking a bet that we won't have to do a large refactor to the Item API later. Probably alright since there aren't any consumers of Peace just yet.

2. 🔴 Try and match nodes, and animate.

    Too magic / inaccurate?

3. 🟡 Require / request `Item` implementors to always provide the same number of `ItemLocation`s.

    May mean every `ItemLocation` that is unknown, is still populated:

    ```rust ,ignore
    let item_location_server = item_location_server.unwrap_or(ItemLocation::HostUnknown);
    let item_location_url = item_location_url.unwrap_or(ItemLocation::UrlUnknown);

    vec![
        item_location_server,
        item_location_url,
    ]
    ```

### Solution

We'll go with:

* **A:2:** Item implementors provide `ItemLocation`s
* **B:1:** Everything is shown.

    There is still the possibility to migrate to **B:4** (`Item`s take in visibility flags) or **B:5** (developers pass in visibility to framework) later.

* **C:1:** Don't solve the consistent diagram layout now.


## What Rendering Makes Sense

Conceptually, `Item`s can be thought of either an edge or a node:

* **Edge:** The item represents an action / work to between the source(s) and the destination(s).
* **Node:** The item represents the destination thing.


### Use Cases

1. Upload a file -- one source, one dest.
2. Download a file -- one source, one dest.
3. Launch servers -- one source (localhost), one dest (AWS).
4. Wait for servers to start up -- multiple within (do we need the `ItemLocationTree` for the cloud provider / subnet context? or leverage previous resource tracking to work it out?).
5. Wait for endpoints to become available -- one source, multiple dest (query each endpoint).
6. Do we want `ItemInteraction`s to be queried multiple times while `Apply` is happening? -- i.e. some servers may have started up, and so we need the `Item` to report that to us.
7. Notably, we want these `ItemInteraction`s to be queriable without the items actually existing -- so we can generate diagrams to demonstrate what *would* happen upon execution.


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
3. It is useful to show animated edges while that step is in progress, and hide them after.


<details><summary>Old idea for code</summary>

#### Item Implementor

<details><summary>File Download</summary>

```rust ,ignore
impl Item for FileDownload {
    fn location_from(&self, params_partial: &<Self::Params<'_> as Params>::Partial)
    -> ItemLocation {
        let address = params_partial
            .src()
            .map(Url::host_str)
            .unwrap_or("<to_be_determined>")
            .to_string();
        let path = params_partial
            .src()
            .map(Url::path);

        ItemLocation {
            realm: Realm::Host { address },
            path,
        }
    }

    fn location_to(&self, params: &<Self::Params<'_> as Params>::Partial) -> ItemLocation {
        let address = "localhost".to_string();
        let path = params_partial
            .dest()
            .map(Path::to_string_lossy)
            .map(Cow::to_owned);

        ItemLocation {
            realm: Realm::Host { address },
            path,
        }
    }
}
```

</details>

<details><summary>S3Bucket</summary>

```rust ,ignore
impl Item for S3Bucket {
    fn location_from(&self, params_partial: &<Self::Params<'_> as Params>::Partial)
    -> ItemLocation {
        let address = "localhost"; // or None
        ItemLocation::Host { address }
    }

    fn location_to(&self, params: &<Self::Params<'_> as Params>::Partial) -> ItemLocation {
        ItemLocation::Cloud {
            cloud_provider: CloudProvider::Aws,
            global_or_region: GlobalOrRegion::Region(Region::ApSoutheast2),
            subnet: None,
            url: URL::from(".."),
            name: params.name.unwrap_or("<to_be_determined>"),
        }
    }
}
```

</details>

</details>
