use std::{marker::PhantomData, path::Path};

use peace_cfg::{FlowId, ItemSpecId};
use peace_resources::{
    paths::{StatesDesiredFile, StatesSavedFile},
    states::{
        ts::{Desired, Saved},
        States, StatesDesired, StatesSaved,
    },
    type_reg::untagged::{BoxDtDisplay, TypeReg},
};

use crate::{Error, Storage};

/// Reads [`StatesSaved`] and [`StatesDesired`] from storage.
pub struct StatesSerializer<E>(PhantomData<E>);

impl<E> StatesSerializer<E>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Returns the [`StatesSaved`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states`: States to serialize.
    /// * `states_file_path`: Path to save the serialized states to.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn serialize<TS>(
        storage: &Storage,
        states: &States<TS>,
        states_file_path: &Path,
    ) -> Result<(), E>
    where
        TS: Send + Sync,
    {
        storage
            .serialized_write(
                #[cfg(not(target_arch = "wasm32"))]
                "StatesSerializer::serialize".to_string(),
                states_file_path,
                states,
                Error::StatesSerialize,
            )
            .await?;

        Ok(())
    }

    /// Returns the [`StatesSaved`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item spec state.
    /// * `states_saved_file`: `StatesSavedFile` to deserialize.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn deserialize_saved(
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
        states_saved_file: &StatesSavedFile,
    ) -> Result<StatesSaved, E> {
        let states = Self::deserialize_internal::<Saved>(
            #[cfg(not(target_arch = "wasm32"))]
            "StatesSerializer::deserialize_saved".to_string(),
            flow_id,
            storage,
            states_type_reg,
            states_saved_file,
        )
        .await?;

        states.ok_or_else(|| E::from(Error::StatesCurrentDiscoverRequired))
    }

    /// Returns the [`StatesDesired`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item spec state.
    /// * `states_desired_file`: `StatesDesiredFile` to deserialize.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn deserialize_desired(
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
        states_desired_file: &StatesDesiredFile,
    ) -> Result<StatesDesired, E> {
        let states = Self::deserialize_internal::<Desired>(
            #[cfg(not(target_arch = "wasm32"))]
            "StatesSerializer::deserialize_desired".to_string(),
            flow_id,
            storage,
            states_type_reg,
            states_desired_file,
        )
        .await?;

        states.ok_or_else(|| E::from(Error::StatesDesiredDiscoverRequired))
    }

    /// Returns the [`StatesSaved`] of all [`ItemSpec`]s if it exists on
    /// disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item spec state.
    /// * `states_saved_file`: `StatesSavedFile` to deserialize.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub async fn deserialize_saved_opt(
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
        states_saved_file: &StatesSavedFile,
    ) -> Result<Option<StatesSaved>, E> {
        Self::deserialize_internal(
            #[cfg(not(target_arch = "wasm32"))]
            "StatesSerializer::deserialize_saved_opt".to_string(),
            flow_id,
            storage,
            states_type_reg,
            states_saved_file,
        )
        .await
    }

    /// Returns the [`States`] of all [`ItemSpec`]s if it exists on disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item spec state.
    /// * `states_saved_file`: `StatesSavedFile` to deserialize.
    ///
    /// # Type Parameters
    ///
    /// * `TS`: The states type state to use, such as [`ts::Current`] or
    ///   [`ts::Saved`].
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ts::Current`]: peace_resources::states::ts::Current
    /// [`ts::Saved`]: peace_resources::states::ts::Saved
    #[cfg(not(target_arch = "wasm32"))]
    async fn deserialize_internal<TS>(
        thread_name: String,
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
        states_file_path: &Path,
    ) -> Result<Option<States<TS>>, E>
    where
        TS: Send + Sync,
    {
        let states_opt = storage
            .serialized_typemap_read_opt(thread_name, states_type_reg, states_file_path, |error| {
                #[cfg(not(feature = "error_reporting"))]
                {
                    Error::StatesDeserialize {
                        flow_id: flow_id.clone(),
                        error,
                    }
                }
                #[cfg(feature = "error_reporting")]
                {
                    use miette::NamedSource;

                    let file_contents = std::fs::read_to_string(states_file_path).unwrap();

                    let (error_span, error_message, context_span) =
                        Self::error_and_context(&file_contents, &error);
                    let states_file_source =
                        NamedSource::new(states_file_path.to_string_lossy(), file_contents);

                    Error::StatesDeserialize {
                        flow_id: flow_id.clone(),
                        states_file_source,
                        error_span,
                        error_message,
                        context_span,
                        error,
                    }
                }
            })
            .await?;

        Ok(states_opt)
    }

    /// Returns the [`States`] of all [`ItemSpec`]s if it exists on disk.
    ///
    /// # Parameters:
    ///
    /// * `storage`: `Storage` to read from.
    /// * `states_type_reg`: Type registry with functions to deserialize each
    ///   item spec state.
    /// * `states_saved_file`: `StatesSavedFile` to deserialize.
    ///
    /// # Type Parameters
    ///
    /// * `TS`: The states type state to use, such as [`ts::Current`] or
    ///   [`ts::Saved`].
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ts::Current`]: peace_resources::states::ts::Current
    /// [`ts::Saved`]: peace_resources::states::ts::Saved
    #[cfg(target_arch = "wasm32")]
    async fn deserialize_internal<TS>(
        flow_id: &FlowId,
        storage: &Storage,
        states_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
        states_file_path: &Path,
    ) -> Result<Option<States<TS>>, E>
    where
        TS: Send + Sync,
    {
        let states_opt = storage
            .serialized_typemap_read_opt(states_type_reg, states_file_path, |error| {
                #[cfg(not(feature = "error_reporting"))]
                {
                    Error::StatesDeserialize {
                        flow_id: flow_id.clone(),
                        error,
                    }
                }
                #[cfg(feature = "error_reporting")]
                {
                    use miette::NamedSource;

                    let file_contents = std::fs::read_to_string(states_file_path).unwrap();

                    let (error_span, error_message, context_span) =
                        Self::error_and_context(&file_contents, &error);
                    let states_file_source =
                        NamedSource::new(states_file_path.to_string_lossy(), file_contents);

                    Error::StatesDeserialize {
                        flow_id: flow_id.clone(),
                        states_file_source,
                        error_span,
                        error_message,
                        context_span,
                        error,
                    }
                }
            })
            .await?;

        Ok(states_opt)
    }

    /// Returns the error location and message to pass to miette.
    ///
    /// TODO: Replace hack.
    ///
    /// The `location()` reported in the error is incorrect, due to
    /// <https://github.com/dtolnay/serde-yaml/issues/153>
    ///
    /// In certain cases, we can reverse engineer the error from the
    /// `Display` string of the error.
    #[cfg(feature = "error_reporting")]
    fn error_and_context(
        file_contents: &str,
        error: &serde_yaml::Error,
    ) -> (
        Option<miette::SourceOffset>,
        String,
        Option<miette::SourceOffset>,
    ) {
        let error_string = format!("{error}");
        let (error_span, context_span) = match error.location().map(|error_location| {
            (
                error_location.index(),
                error_location.line(),
                error_location.column(),
            )
        }) {
            // The `error_location` is not the true location. Extract it from the `Display` string.
            //
            // See:
            //
            // * <https://github.com/dtolnay/serde-yaml/blob/0.9.14/src/libyaml/error.rs#L65-L84>
            // * <https://github.com/dtolnay/serde-yaml/blob/0.9.14/src/libyaml/error.rs#L141>
            //
            // Example error strings (truncated the beginning):
            //
            // ```text
            // missing field `path` at line 2 column 12 at line 2 column 3
            // unknown variant `~`, expected one of `a`, `b` at line 2 column 11 at line 2 column 11 at line 2 column 3
            // ```
            Some((0, 1, 1)) => {
                // TODO: This may also be "at position 123", but we don't support that yet.
                let mut line_column_pairs =
                    error_string.rsplit(" at line ").filter_map(|line_column| {
                        let mut line_column_split = line_column.split(" column ");
                        let line = line_column_split
                            .next()
                            .map(str::parse::<usize>)
                            .and_then(Result::ok);
                        let column = line_column_split
                            .next()
                            .map(str::parse::<usize>)
                            .and_then(Result::ok);

                        if let (Some(line), Some(column)) = (line, column) {
                            Some((line, column))
                        } else {
                            None
                        }
                    });

                let last_mark = line_column_pairs.next().map(|(line, column)| {
                    miette::SourceOffset::from_location(file_contents, line, column)
                });
                let second_to_last_mark = line_column_pairs.next().map(|(line, column)| {
                    miette::SourceOffset::from_location(file_contents, line, column)
                });

                match (second_to_last_mark, last_mark) {
                    (error_span @ Some(_), context_span @ Some(_)) => (error_span, context_span),
                    (None, error_span @ Some(_)) => (error_span, None),
                    (Some(_), None) | (None, None) => (None, None),
                }
            }
            Some((_, line, column)) => (
                Some(miette::SourceOffset::from_location(
                    file_contents,
                    line,
                    column,
                )),
                None,
            ),
            None => (None, None),
        };

        let error_message = error_string
            .split(" at ")
            .next()
            .map(str::to_string)
            .unwrap_or(error_string);
        (error_span, error_message, context_span)
    }
}
