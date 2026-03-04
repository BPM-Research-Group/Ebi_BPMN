use crate::{
    BusinessProcessModelAndNotation,
    element::BPMNElementTrait,
    parser::parser_state::GlobalIndex,
    semantics::{BPMNRootMarking, BPMNSubMarking, TransitionIndex},
    traits::{
        objectable::{BPMNObject, EMPTY_FLOWS},
        processable::Processable,
        transitionable::Transitionable,
    },
};
use anyhow::{Result, anyhow};
use bitvec::{bitvec, vec::BitVec};
use ebi_activity_key::Activity;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct BPMNInclusiveGateway {
    pub(crate) global_index: GlobalIndex,
    pub(crate) id: String,
    pub(crate) local_index: usize,
    pub(crate) incoming_sequence_flows: Vec<usize>,
    pub(crate) outgoing_sequence_flows: Vec<usize>,
}
impl BPMNElementTrait for BPMNInclusiveGateway {
    fn add_incoming_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        self.incoming_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_outgoing_sequence_flow(&mut self, flow_index: usize) -> Result<()> {
        if self.outgoing_sequence_flows.len() == usize::BITS.try_into().unwrap() {
            return Err(anyhow!(
                "cannot add more than {} outgoing sequence flows to an inclusive gateway",
                usize::BITS
            ));
        }
        self.outgoing_sequence_flows.push(flow_index);
        Ok(())
    }

    fn add_incoming_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("gateways cannot have incoming message flows"))
    }

    fn add_outgoing_message_flow(&mut self, _flow_index: usize) -> Result<()> {
        Err(anyhow!("gateways cannot have outgoing message flows"))
    }

    fn verify_structural_correctness(
        &self,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        Ok(())
    }
}

impl BPMNObject for BPMNInclusiveGateway {
    fn local_index(&self) -> usize {
        self.local_index
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn global_index(&self) -> GlobalIndex {
        self.global_index
    }

    fn is_unconstrained_start_event(
        &self,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<bool> {
        Ok(false)
    }

    fn is_end_event(&self) -> bool {
        false
    }

    fn incoming_sequence_flows(&self) -> &[usize] {
        &self.incoming_sequence_flows
    }

    fn outgoing_sequence_flows(&self) -> &[usize] {
        &self.outgoing_sequence_flows
    }

    fn incoming_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn outgoing_message_flows(&self) -> &[usize] {
        &EMPTY_FLOWS
    }

    fn can_start_process_instance(&self, _bpmn: &BusinessProcessModelAndNotation) -> Result<bool> {
        Ok(self.incoming_sequence_flows().len() == 0)
    }

    fn outgoing_message_flows_always_have_tokens(&self) -> bool {
        false
    }

    fn outgoing_messages_cannot_be_removed(&self) -> bool {
        false
    }

    fn incoming_messages_are_ignored(&self) -> bool {
        false
    }

    fn can_have_incoming_sequence_flows(&self) -> bool {
        true
    }

    fn can_have_outgoing_sequence_flows(&self) -> bool {
        true
    }
}

impl Transitionable for BPMNInclusiveGateway {
    fn number_of_transitions(&self, _marking: &BPMNSubMarking) -> usize {
        2usize.pow(self.outgoing_sequence_flows.len() as u32) - 1
    }

    fn enabled_transitions(
        &self,
        _root_marking: &BPMNRootMarking,
        sub_marking: &BPMNSubMarking,
        parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<BitVec> {
        if self.incoming_sequence_flows.len() == 0 {
            //if there are no sequence flows, then initiation mode 2 applies.
            //that is, look in the extra virtual sequence flow
            if sub_marking.element_index_2_tokens[self.local_index] >= 1 {
                //enabled
                return Ok(bitvec![1;self.number_of_transitions(sub_marking)]);
            } else {
                //not enabled
                return Ok(bitvec![0;self.number_of_transitions(sub_marking)]);
            }
        } else {
            //gather a list of incoming sequence flows that do not have a token
            let mut empty_sequence_flows = HashSet::new();
            for sequence_flow_index in &self.incoming_sequence_flows {
                if sub_marking.sequence_flow_2_tokens[*sequence_flow_index] == 0 {
                    let sequence_flow =
                        &parent.sequence_flows_non_recursive()[*sequence_flow_index];
                    empty_sequence_flows.insert(sequence_flow.local_index);
                }
            }

            if empty_sequence_flows.len() == self.incoming_sequence_flows.len() {
                //not enabled as there is no token
                return Ok(bitvec![0;self.number_of_transitions(sub_marking)]);
            }

            //perform a backwards search to find tokens that may still come to the gateway
            let mut queue = VecDeque::new();
            queue.extend(empty_sequence_flows.clone());
            let mut seen_sequence_flows = empty_sequence_flows;
            while let Some(sequence_flow_index) = queue.pop_front() {
                let sequence_flow = &parent.sequence_flows_non_recursive()[sequence_flow_index];

                //check whether this sequence flow has a token
                if *sub_marking
                    .sequence_flow_2_tokens
                    .get(sequence_flow.local_index)
                    .ok_or_else(|| anyhow!("sequence flow not found"))?
                    >= 1
                {
                    // we encountered a token on our search
                    // not enabled, as that token may end up at the OR join
                    return Ok(bitvec![0;self.number_of_transitions(sub_marking)]);
                }

                //check whether this sequece flow comes from a sub-process that has instantiations
                if !sub_marking.element_index_2_sub_markings[sequence_flow.source_local_index]
                    .is_empty()
                {
                    //not enabled: the instantiation may finish and that token may end up at the OR join
                    return Ok(bitvec![0;self.number_of_transitions(sub_marking)]);
                }

                //check whether the source of this sequence flow is enabled by the initiation mode
                if sub_marking.element_index_2_tokens[sequence_flow.source_local_index] >= 1 {
                    //not enabled: this virtual token may end up at the OR join
                    return Ok(bitvec![0;self.number_of_transitions(sub_marking)]);
                }

                //get the source
                let source = parent
                    .elements_non_recursive()
                    .get(sequence_flow.source_local_index)
                    .ok_or_else(|| anyhow!("source not found"))?;
                for next_sequence_flow_index in source.incoming_sequence_flows() {
                    let next_sequence_flow = parent
                        .sequence_flows_non_recursive()
                        .get(*next_sequence_flow_index)
                        .ok_or_else(|| anyhow!("next sequence flow not found"))?;
                    if seen_sequence_flows.insert(next_sequence_flow.local_index) {
                        queue.push_back(*&next_sequence_flow.local_index);
                    }
                }
            }

            //enabled
            return Ok(bitvec![1;self.number_of_transitions(sub_marking)]);
        }
    }

    fn execute_transition(
        &self,
        mut transition_index: TransitionIndex,
        _root_marking: &mut BPMNRootMarking,
        sub_marking: &mut BPMNSubMarking,
        _parent: &dyn Processable,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Result<()> {
        //consume
        if self.incoming_sequence_flows.len() == 0 {
            //if there are no sequence flows, then initiation mode 2 applies.
            //that is, look in the extra virtual sequence flow
            sub_marking.element_index_2_tokens[self.local_index] -= 1;
        } else {
            //consume a token from each incoming sequence flow that has one
            for sequence_flow_index in &self.incoming_sequence_flows {
                if sub_marking.sequence_flow_2_tokens[*sequence_flow_index] > 0 {
                    sub_marking.sequence_flow_2_tokens[*sequence_flow_index] -= 1;
                }
            }
        }

        //produce
        for sequence_flow_index in &self.outgoing_sequence_flows {
            if transition_index % 2 == 0 {
                sub_marking.sequence_flow_2_tokens[*sequence_flow_index] += 1;
                transition_index <<= 1;
            }
        }
        Ok(())
    }

    fn transition_activity(
        &self,
        _transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
    ) -> Option<Activity> {
        None
    }

    fn transition_debug(
        &self,
        transition_index: TransitionIndex,
        _marking: &BPMNSubMarking,
        _bpmn: &BusinessProcessModelAndNotation,
    ) -> Option<String> {
        Some(format!(
            "inclusive gateway `{}`; internal transition {}",
            self.id, transition_index
        ))
    }
}
