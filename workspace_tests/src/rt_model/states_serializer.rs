use peace::{
    cfg::{flow_id, item_id, FlowId, ItemId},
    resources::{
        internal::StatesMut, paths::StatesCurrentFile, states::StatesCurrentStored,
        type_reg::untagged::TypeReg,
    },
    rt_model::{Error, StatesSerializer, Storage},
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let storage = Storage;
    let states_current_file = StatesCurrentFile::new(tempdir.path().join("states_current.yaml"));

    let states = {
        let mut states = StatesMut::new();
        states.insert(item_id!("a"), 123u32);
        StatesCurrentStored::from(states)
    };
    StatesSerializer::<Error>::serialize(&storage, &states, &states_current_file).await?;

    let serialized = tokio::fs::read_to_string(states_current_file).await?;
    assert_eq!("a: 123\n", serialized);

    Ok(())
}

#[tokio::test]
async fn deserialize_stored() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let flow_id = flow_id!("test_flow");
    let storage = Storage;
    let item_id = item_id!("a");
    let mut states_type_reg = TypeReg::new_typed();
    states_type_reg.register::<u32>(item_id.clone());
    let states_current_file = StatesCurrentFile::new(tempdir.path().join("states_current.yaml"));

    let states = {
        let mut states = StatesMut::new();
        states.insert(item_id.clone(), 123u32);
        StatesCurrentStored::from(states)
    };
    StatesSerializer::<Error>::serialize(&storage, &states, &states_current_file).await?;

    let states_deserialized = StatesSerializer::<Error>::deserialize_stored(
        &flow_id,
        &storage,
        &states_type_reg,
        &states_current_file,
    )
    .await?;

    assert_eq!(
        Some(123),
        states_deserialized.get::<u32, _>(&item_id).copied()
    );

    Ok(())
}

#[tokio::test]
async fn deserialize_stored_error_maps_byte_indices() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let flow_id = flow_id!("test_flow");
    let storage = Storage;
    let item_id = item_id!("a");
    let mut states_type_reg = TypeReg::new_typed();
    states_type_reg.register::<u32>(item_id.clone());
    let states_current_file = StatesCurrentFile::new(tempdir.path().join("states_current.yaml"));

    let contents = "a: [123]\n";
    tokio::fs::write(&states_current_file, contents).await?;

    let error = StatesSerializer::<Error>::deserialize_stored(
        &flow_id,
        &storage,
        &states_type_reg,
        &states_current_file,
    )
    .await
    .unwrap_err();

    #[cfg(feature = "error_reporting")]
    {
        use peace::miette::SourceOffset;
        let error_span_expected = {
            let line = 1;
            let column = 4;
            Some(SourceOffset::from_location(contents, line, column))
        };

        if let Error::StatesDeserialize {
            flow_id: flow_id_actual,
            states_file_source: _,
            error_span,
            error_message,
            context_span,
            error: _,
        } = error
        {
            assert_eq!(flow_id, flow_id_actual);
            assert_eq!(error_span_expected, error_span);
            assert_eq!("a: invalid type: sequence, expected u32", error_message);
            assert_eq!(None, context_span);
        } else {
            panic!("Expected error to be `Error::StatesDeserialize {{ .. }}`, but was {error:?}");
        }
    }
    #[cfg(not(feature = "error_reporting"))]
    {
        assert!(matches!(
            error,
            Error::StatesDeserialize {
                flow_id: flow_id_actual,
                error: _
            }
            if flow_id == flow_id_actual
        ));
    }

    Ok(())
}
