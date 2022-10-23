use std::marker::PhantomData;

use peace_cfg::ItemSpecId;
use peace_resources::{
    paths::{FlowDir, StatesCurrentFile},
    resources::ts::{SetUp, WithStates},
    states::StatesCurrent,
    type_reg::untagged::{BoxDtDisplay, TypeReg},
    Resources,
};
use peace_rt_model::{CmdContext, Error, Storage};

/// Reads [`StatesCurrent`]s from storage.
#[derive(Debug)]
pub struct StatesCurrentReadCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> StatesCurrentReadCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
{
    /// Reads [`StatesCurrent`]s from storage.
    ///
    /// Either [`StatesCurrentDiscoverCmd`] or [`StatesDiscoverCmd`] must have
    /// run prior to this command to read the state.
    ///
    /// [`StatesCurrentDiscoverCmd`]: crate::StatesCurrentDiscoverCmd
    /// [`StatesDiscoverCmd`]: crate::StatesDiscoverCmd
    pub async fn exec(
        mut cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, WithStates>, E> {
        let CmdContext {
            resources,
            states_type_regs,
            ..
        } = &mut cmd_context;

        let states_current =
            Self::exec_internal(resources, states_type_regs.states_current_type_reg()).await?;

        let cmd_context = CmdContext::from((cmd_context, |resources| {
            Resources::<WithStates>::from((resources, states_current))
        }));

        Ok(cmd_context)
    }

    /// Returns the [`StatesCurrent`] of all [`ItemSpec`]s if it exists on disk.
    ///
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    pub(crate) async fn exec_internal(
        resources: &mut Resources<SetUp>,
        states_current_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
    ) -> Result<StatesCurrent, E> {
        let states = Self::deserialize_internal(resources, states_current_type_reg).await?;

        Ok(states)
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_current_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
    ) -> Result<StatesCurrent, E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        if !states_current_file.exists() {
            return Err(E::from(Error::StatesCurrentDiscoverRequired));
        }

        let states_current = storage
            .read_with_sync_api(
                "states_current_file_read".to_string(),
                &states_current_file,
                |file| {
                    let deserializer = serde_yaml::Deserializer::from_reader(file);
                    let states_current = StatesCurrent::from(
                        states_current_type_reg
                            .deserialize_map(deserializer)
                            .map_err(|error| {
                                #[cfg(not(feature = "error_reporting"))]
                                {
                                    Error::StatesCurrentDeserialize { error }
                                }
                                #[cfg(feature = "error_reporting")]
                                {
                                    use miette::NamedSource;

                                    let file_contents =
                                        std::fs::read_to_string(&states_current_file).unwrap();

                                    let (error_span, error_message, context_span) =
                                        Self::error_and_context(&file_contents, &error);
                                    let states_file_source = NamedSource::new(
                                        states_current_file.to_string_lossy(),
                                        file_contents,
                                    );

                                    Error::StatesCurrentDeserialize {
                                        states_file_source,
                                        error_span,
                                        error_message,
                                        context_span,
                                        error,
                                    }
                                }
                            })?,
                    );
                    Ok(states_current)
                },
            )
            .await?;
        drop(flow_dir);
        drop(storage);

        resources.insert(states_current_file);

        Ok(states_current)
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
            // The `error_location` is not the true
            // location. Extract it from the
            // `Display` string.
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

    #[cfg(target_arch = "wasm32")]
    async fn deserialize_internal(
        resources: &mut Resources<SetUp>,
        states_current_type_reg: &TypeReg<ItemSpecId, BoxDtDisplay>,
    ) -> Result<StatesCurrent, E> {
        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_current_file = StatesCurrentFile::from(&*flow_dir);

        let states_serialized = storage
            .get_item_opt(&states_current_file)?
            .ok_or(Error::StatesCurrentDiscoverRequired)?;
        let deserializer = serde_yaml::Deserializer::from_str(&states_serialized);
        let states_current = StatesCurrent::from(
            states_current_type_reg
                .deserialize_map(deserializer)
                .map_err(|error| {
                    #[cfg(not(feature = "error_reporting"))]
                    {
                        Error::StatesCurrentDeserialize { error }
                    }
                    #[cfg(feature = "error_reporting")]
                    {
                        use miette::NamedSource;

                        let file_contents = std::fs::read_to_string(&states_current_file).unwrap();

                        let (error_span, error_message, context_span) =
                            Self::error_and_context(&file_contents, &error);
                        let states_file_source =
                            NamedSource::new(states_current_file.to_string_lossy(), file_contents);

                        Error::StatesCurrentDeserialize {
                            states_file_source,
                            error_span,
                            error_message,
                            context_span,
                            error,
                        }
                    }
                })?,
        );

        drop(flow_dir);
        drop(storage);

        resources.insert(states_current_file);

        Ok(states_current)
    }
}

impl<E, O> Default for StatesCurrentReadCmd<E, O> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
