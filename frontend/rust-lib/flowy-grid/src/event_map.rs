use crate::event_handler::*;
use crate::manager::GridManager;
use flowy_derive::{Flowy_Event, ProtoBuf_Enum};
use lib_dispatch::prelude::*;
use std::sync::Arc;
use strum_macros::Display;

pub fn create(grid_manager: Arc<GridManager>) -> Module {
    let mut module = Module::new().name(env!("CARGO_PKG_NAME")).data(grid_manager);
    module = module
        .event(GridEvent::GetGridData, get_grid_data_handler)
        .event(GridEvent::GetRows, get_rows_handler)
        .event(GridEvent::GetFields, get_fields_handler)
        .event(GridEvent::CreateRow, create_row_handler);

    module
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, Hash, ProtoBuf_Enum, Flowy_Event)]
#[event_err = "FlowyError"]
pub enum GridEvent {
    #[event(input = "GridId", output = "Grid")]
    GetGridData = 0,

    #[event(input = "QueryRowPayload", output = "RepeatedRow")]
    GetRows = 1,

    #[event(input = "QueryFieldPayload", output = "RepeatedField")]
    GetFields = 2,

    #[event(input = "GridId")]
    CreateRow = 3,
}
