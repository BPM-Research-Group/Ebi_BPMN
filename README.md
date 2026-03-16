# Ebi_BPMN
A BPMN library for Rust

Contains a parser, a data structure and a writer.
For now, this crate focuses on the behaviour of BPMN models; not on the data or resource perspectives.
The crate provides methods to parse BPMN models, methods to write BPMN models, and methods to traverse the state space of a BPMN model.

If you are an end user, please consider using the Ebi crate & tool, which provides user-accessible algorithms that use BPMN.

# Supported elements

* Start, end and intermediate none events
* Start, end and intermediate message events
* Start and intermediate timer events
* Exclusive, inclusive, parallel and event-based gateways
* Expanded and collapsed pools
* Message flows
* Sequence flows
* Tasks, and receive, user and manual tasks
* Expanded and collapsed sub-processes

Other elements are gracefully ignored, as long as they do not have in- or outgoing message or sequence flows.

# Process instance intitation

In accordance with the BPMN standard, a process instance can start as follows:
* If the model contains a single start event, then that event is fired to start a process instance.
* If the model contains multiple start events, one of them can be chosen to start a process instance.
* If the model contains no start events, every eligible element receives a token, and they thus start in parallel.

# Example

```rust
  fn main() -> anyhow::Result<()> {
   let f = File::open("testfiles/model.bpmn")?;
   let mut reader = BufReader::new(f);
  
   let bpmn = BusinessProcessModelAndNotation::import_from_reader(&mut reader, true)?;
  
   let mut marking = bpmn.get_initial_marking()?.unwrap();
   assert_eq!(bpmn.get_enabled_transitions(&marking)?, vec![0]);
   bpmn.execute_transition(&mut marking, 0)?;
  
   Ok(())
  }

```

# Deviations from the BPMN 2.0.2 standard

The interpretation of BPMN of this crate differs from the BPMN 2.0.2 standard on the following aspects:
* There is no difference made between a deadlock and proper termination. That is, a trace is considered finished if and as soon as it reaches a deadlock.
* The inclusive (OR) gateway uses a slightly different semantics.
* A completely empty model is assumed to have no traces (as opposed to the language with the empty trace).
* A task with an incoming message flow is allowed after an event-based gateway and will be treated as if it were a receive task.

For more information on these elements, see [this Youtube playlist](https://youtu.be/k0XAej_0In8?si=37Bd6jOFPwqAURlV).

# Limitations

* The crate does not currenly consider or export layouting (bpmndi) information.
* There is a maximum number of outgoing sequence flows of an inclusive gateway of 64 (on 64-bits system) or 32 (on 32-bit systems).