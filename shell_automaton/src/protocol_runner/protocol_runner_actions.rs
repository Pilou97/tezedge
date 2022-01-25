// Copyright (c) SimpleStaking, Viable Systems and Tezedge Contributors
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::protocol_runner::ProtocolRunnerState;
use crate::service::protocol_runner_service::ProtocolRunnerResult;
use crate::storage::blocks::genesis::init::StorageBlocksGenesisInitState;
use crate::{EnablingCondition, State};

use super::init::ProtocolRunnerInitState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolRunnerStartAction {}

impl EnablingCondition<State> for ProtocolRunnerStartAction {
    fn is_enabled(&self, state: &State) -> bool {
        match &state.protocol_runner {
            ProtocolRunnerState::Idle => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolRunnerReadyAction {}

impl EnablingCondition<State> for ProtocolRunnerReadyAction {
    fn is_enabled(&self, state: &State) -> bool {
        match &state.protocol_runner {
            ProtocolRunnerState::Init(ProtocolRunnerInitState::Success { .. }) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolRunnerResponseAction {
    pub result: ProtocolRunnerResult,
}

impl EnablingCondition<State> for ProtocolRunnerResponseAction {
    fn is_enabled(&self, state: &State) -> bool {
        match &state.protocol_runner {
            ProtocolRunnerState::Ready(_) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolRunnerResponseUnexpectedAction {
    pub result: ProtocolRunnerResult,
}

impl EnablingCondition<State> for ProtocolRunnerResponseUnexpectedAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

/// Notify to pieces outside state machine about protocol runner's
/// or context's initialization status.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolRunnerNotifyStatusAction {}

impl EnablingCondition<State> for ProtocolRunnerNotifyStatusAction {
    fn is_enabled(&self, state: &State) -> bool {
        match &state.protocol_runner {
            ProtocolRunnerState::Ready(s) => {
                !s.genesis_commit_hash.is_some()
                    || matches!(
                        &state.storage.blocks.genesis.init,
                        StorageBlocksGenesisInitState::Success
                    )
            }
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolRunnerShutdownInitAction {}

impl EnablingCondition<State> for ProtocolRunnerShutdownInitAction {
    fn is_enabled(&self, state: &State) -> bool {
        match &state.protocol_runner {
            ProtocolRunnerState::ShutdownPending | ProtocolRunnerState::ShutdownSuccess => false,
            _ => true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolRunnerShutdownPendingAction {}

impl EnablingCondition<State> for ProtocolRunnerShutdownPendingAction {
    fn is_enabled(&self, state: &State) -> bool {
        match &state.protocol_runner {
            ProtocolRunnerState::ShutdownPending | ProtocolRunnerState::ShutdownSuccess => false,
            _ => true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolRunnerShutdownSuccessAction {}

impl EnablingCondition<State> for ProtocolRunnerShutdownSuccessAction {
    fn is_enabled(&self, state: &State) -> bool {
        match &state.protocol_runner {
            ProtocolRunnerState::ShutdownPending => true,
            _ => false,
        }
    }
}
