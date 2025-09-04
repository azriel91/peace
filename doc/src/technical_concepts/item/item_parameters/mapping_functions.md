# Mapping Functions

> Github Issues: [#156][#156], [#208][#208]

Mapping functions could be passed to a `Params`'s `FieldWiseSpec`. There are implications whether or not it is implemented.


[#156]: https://github.com/azriel91/peace/issues/156
[#208]: https://github.com/azriel91/peace/issues/208


### Summary

| With it                                  | Without it                                     |
|:-----------------------------------------|:-----------------------------------------------|
| Runtime error on mismatched types        | Compilation error on mismatched types          |
| Need to define mapping function key enum | Don't need to define mapping function key enum |
| Don't need to specify params spec for "used what was set last time" for mapping functions -- always set in cmd ctx                      | May forget specifying "used what was set last time" for mapping functions, and hit runtime error |


### With It

### Developers

1. Define an enum to name the function keys:

    ```rust
    enum MappingFunctions {
        BucketNameFromBucketState,
        IamPolicyArnFromIamPolicyState,
    }
    ```

2. Define the mappings:

    ```rust
    let mapping_functions = {
        let mut mapping_functions = MappingFunctions::new();
        mapping_functions.insert(BucketNameFromBucketState, S3BucketState::bucket_name);
        // ..

        mapping_functions
    };
    ```

    **Note:**

    If we want to have a compilation error here, the `MappingFunctions::insert` function needs to have a type parameter that tells it the `Item > Params > Field` that the mapping function is for.

    However, developers use `#[derive(Params)]` to derive the `<ItemParams>FieldWiseSpec`, and requiring them to specify something like the following is arduous:

    ```rust
    MappingFunction::insert<FromType, ToType>(BucketNameFromBucketState, S3BucketState::bucket_name)
    ```


3. Pass `MappingFunctions` to `CmdCtxBuilder`, for each code instantiation (may be just one):

    ```rust
    cmd_ctx_builder.with_mapping_functions(mapping_functions);
    ```

4. Not have to call `.with_item_params::<TheItem>(..)` in subsequent calls.


#### Users

1. Get runtime error if the mapping function type doesn't match, but it should be caught by tests.


#### Framework Maintainers

1. `MappingFunctions` map will have magic logic to store the function argument types and return type.
2. Error reporting when types are mismatched.


### Without It

#### Developers

1. Define the item params spec:

    ```rust
    // First execution
    let s3_object_params_spec = S3ObjectParams::<WebApp>::field_wise_spec()
        .with_file_path(web_app_path_local)
        .with_object_key(object_key)
        .with_bucket_name_from_map(S3BucketState::bucket_name)
        .build();

    // Subsequent executions
    let s3_object_params_spec = S3ObjectParams::<WebApp>::field_wise_spec()
        .with_bucket_name_from_map(S3BucketState::bucket_name)
        .build();
    ```

2. Pass the item params spec to `CmdCtxBuilder`, for every separate code instantiation:

    ```rust
    cmd_ctx_builder
        .with_item_params::<S3ObjectItem<WebApp>>(
            item_id!("s3_object"),
            s3_object_params_spec,
        )
    ```

    This is somewhat of an inconvenience, because if this isn't done, the user / developer will have a runtime error, which looks like this:

    ```md
    peace_rt_model::params_specs_mismatch

      × Item params specs do not match with the items in the flow.
      help: The following items either have not had a params spec provided previously,
            or had contained a mapping function, which cannot be loaded from disk.

            So the params spec needs to be provided to the command context for:

            * s3_object
    ```

When the closure passed to `with_*_from_map` doesn't have the argument type specified, or mismatches, the compilation error is still unclear. [rust#119888][rust#119888] will allow us to return a useful compilation error.


[rust#119888]: https://github.com/rust-lang/rust/pull/119888


#### Users

No runtime error, because it will be caught at compile time.


#### Framework Maintainers

1. Error messages / diagnostics showing which `CmdCtx` is missing which item spec for which field, should be made clear.
