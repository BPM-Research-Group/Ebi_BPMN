# Ebi_BPMN
A BPMN library for Rust

Contains a parser, a data structure and a writer.
For now, this crate focuses on the behaviour of BPMN models; not on the data or resource perspectives.

# Supported elements

* Start, end and intermediate none events
* Start, end and intermediate message events
* Start and intermediate timer events
* Exclusive, inclusive, parallel and event-based gateways
* Expanded and collapsed pools
* Message flows
* Sequence flows
* Tasks
* Expanded and collapsed sub-processes

Other elements are gracefully ignored, as long as they do not have in- or outgoing message or sequence flows.

# Process instance intitation

In accordance with the BPMN standard, a process instance can start as follows:
* If the model contains a single start event, then that event is fired to start a process instance.
* If the model contains multiple start events, one of them can be chosen to start a process instance.
* If the model contains no start events, every eligible element receives a token, and they thus start in parallel.

# Deviations from the BPMN 2.0.2 standard

The interpretation of BPMN of this crate differs from the BPMN 2.0.2 standard on the following aspects:
* There is no difference made between a deadlock and proper termination. That is, a trace is considered finished if and as soon as it reaches a deadlock.
* The inclusive (OR) gateway uses a slightly different semantics.

For more information on these elements, see [this Youtube playlist](https://youtu.be/k0XAej_0In8?si=37Bd6jOFPwqAURlV).