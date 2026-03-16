//! A BPMN crate written in pure Rust
//!
//! Contains a parser, a data structure and a writer.
//! For now, this crate focuses on the behaviour of BPMN models; not on the data or resource perspectives.
//! The crate provides methods to parse BPMN models, methods to write BPMN models, and methods to traverse the state space of a BPMN model.
//!
//! A web-based tool to create BPMN models is [bpmn.io].
//!
//! This create is intended as a crate to be used in other software. If you are an end user, please consider using the [Ebi] crate & tool, which provides user-accessible algorithms that use BPMN.
//!
//! # Usage
//!
//! The main struct is [BusinessProcessModelAndNotation], which contains methods for parsing, writing and state-space traversal.
//! To create a BPMN model programmatically, consider the [BPMNCreator] struct.
//!
//! To parse a BPMN model:
//!  ```
//!  use std::io::prelude::*;
//!  use std::io::BufReader;
//!  use std::fs::File;
//!  use ebi_bpmn::BusinessProcessModelAndNotation;
//!  
//!  fn main() -> anyhow::Result<()> {
//!   let f = File::open("testfiles/model.bpmn")?;
//!   let mut reader = BufReader::new(f);
//!  
//!   let bpmn = BusinessProcessModelAndNotation::import_from_reader(&mut reader, true)?;
//!  
//!   let mut marking = bpmn.get_initial_marking()?.unwrap();
//!   assert_eq!(bpmn.get_enabled_transitions(&marking)?, vec![0]);
//!   bpmn.execute_transition(&mut marking, 0)?;
//!  
//!   Ok(())
//!  }
//!  ```
//!
//! [Ebi]: https://crates.io/crates/ebi
//! [bpmn.io]: https://bpmn.io

pub(crate) mod business_process_model_and_notation;
pub(crate) mod conversion;
pub(crate) mod creator;
pub mod element;
pub mod elements {
    pub mod collapsed_pool;
    pub mod collapsed_sub_process;
    pub mod end_event;
    pub mod event_based_gateway;
    pub mod exclusive_gateway;
    pub mod expanded_sub_process;
    pub mod inclusive_gateway;
    pub mod intermediate_catch_event;
    pub mod intermediate_throw_event;
    pub mod manual_task;
    pub mod message_end_event;
    pub mod message_intermediate_catch_event;
    pub mod message_intermediate_throw_event;
    pub mod message_start_event;
    pub mod parallel_gateway;
    pub mod process;
    pub mod receive_task;
    pub mod start_event;
    pub mod task;
    pub mod timer_intermediate_catch_event;
    pub mod timer_start_event;
    pub mod user_task;
}
pub(crate) mod exporter;
pub(crate) mod importer;
pub(crate) mod message_flow;
pub(crate) mod semantics;
pub(crate) mod sequence_flow;
pub(crate) mod structure_checker;
pub(crate) mod parser {
    pub mod parser;
    pub mod parser_state;
    pub mod parser_traits;
    pub mod tag_collaboration;
    pub mod tag_definitions;
    pub mod tag_end_event;
    pub mod tag_event_based_gateway;
    pub mod tag_exclusive_gateway;
    pub mod tag_inclusive_gateway;
    pub mod tag_intermediate_catch_event;
    pub mod tag_intermediate_throw_event;
    pub mod tag_manual_task;
    pub mod tag_message_event_definition;
    pub mod tag_message_flow;
    pub mod tag_parallel_gateway;
    pub mod tag_participant;
    pub mod tag_process;
    pub mod tag_receive_task;
    pub mod tag_sequence_flow;
    pub mod tag_start_event;
    pub mod tag_subprocess;
    pub mod tag_task;
    pub mod tag_timer_event_definition;
    pub mod tag_user_task;
    pub mod tag_weight;
    pub mod tags;
}
pub mod partially_ordered_run;
pub(crate) mod stochastic_business_process_model_and_notation;
pub mod traits {
    pub mod objectable;
    pub mod processable;
    pub mod searchable;
    pub mod startable;
    pub mod transitionable;
    pub mod writable;
}
pub(crate) mod writer {
    pub mod collapsed_pool;
    pub mod collapsed_sub_process;
    pub mod end_event;
    pub mod event_based_gateway;
    pub mod exclusive_gateway;
    pub mod expanded_sub_process;
    pub mod inclusive_gateway;
    pub mod intermediate_catch_event;
    pub mod intermediate_throw_event;
    pub mod manual_task;
    pub mod message_end_event;
    pub mod message_flow;
    pub mod message_intermediate_catch_event;
    pub mod message_intermediate_throw_event;
    pub mod message_start_event;
    pub mod parallel_gateway;
    pub mod process;
    pub mod receive_task;
    pub mod sequence_flow;
    pub mod start_event;
    pub mod task;
    pub mod timer_intermediate_catch_event;
    pub mod timer_start_event;
    pub mod user_task;
}

pub use business_process_model_and_notation::BusinessProcessModelAndNotation;
pub use creator::BPMNCreator;
pub use creator::Container;
pub use creator::EndEventType;
pub use creator::GatewayType;
pub use creator::IntermediateEventType;
pub use creator::StartEventType;
pub use message_flow::BPMNMessageFlow;
pub use parser::parser_state::GlobalIndex;
pub use semantics::BPMNMarking;
pub use sequence_flow::BPMNSequenceFlow;
pub use stochastic_business_process_model_and_notation::StochasticBusinessProcessModelAndNotation;
