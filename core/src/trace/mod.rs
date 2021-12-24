// Copyright 2020 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

use crate::{
    executive::ExecutiveResult,
    trace::trace::{
        Action, Call, CallResult, Create, CreateResult, ExecTrace,
        InternalTransferAction,
    },
    vm::{ActionParams, Result as VmResult},
};
pub use cfx_state::tracer::{AddressPocket, InternalTransferTracer};
use cfx_types::U256;

pub mod error_unwind;
pub mod trace;
pub mod trace_filter;

pub use error_unwind::ErrorUnwind;

/// This trait is used by executive to build traces.
pub trait Tracer: InternalTransferTracer {
    /// Prepares call trace for given params.
    fn prepare_trace_call(&mut self, params: &ActionParams);

    /// Prepares call result trace
    fn prepare_trace_call_result(&mut self, result: &VmResult<ExecutiveResult>);

    /// Prepares create trace for given params.
    fn prepare_trace_create(&mut self, params: &ActionParams);

    /// Prepares create result trace
    fn prepare_trace_create_result(
        &mut self, result: &VmResult<ExecutiveResult>,
    );

    /// Consumes self and returns all traces.
    fn drain(self) -> Vec<ExecTrace>;
}

/// Nonoperative tracer. Does not trace anything.
pub struct NoopTracer;

impl InternalTransferTracer for NoopTracer {
    fn prepare_internal_transfer_action(
        &mut self, _: AddressPocket, _: AddressPocket, _: U256,
    ) {
    }
}

impl Tracer for NoopTracer {
    fn prepare_trace_call(&mut self, _: &ActionParams) {}

    fn prepare_trace_call_result(&mut self, _: &VmResult<ExecutiveResult>) {}

    fn prepare_trace_create(&mut self, _: &ActionParams) {}

    fn prepare_trace_create_result(&mut self, _: &VmResult<ExecutiveResult>) {}

    fn drain(self) -> Vec<ExecTrace> { vec![] }
}

/// Simple executive tracer. Traces all calls and creates.
#[derive(Default)]
pub struct ExecutiveTracer {
    traces: Vec<ExecTrace>,
}

impl InternalTransferTracer for ExecutiveTracer {
    fn prepare_internal_transfer_action(
        &mut self, from: AddressPocket, to: AddressPocket, value: U256,
    ) {
        let trace =
            ExecTrace {
                action: Action::InternalTransferAction(
                    InternalTransferAction { from, to, value },
                ),
            };
        self.traces.push(trace);
    }
}

impl Tracer for ExecutiveTracer {
    fn prepare_trace_call(&mut self, params: &ActionParams) {
        let trace = ExecTrace {
            action: Action::Call(Call::from(params.clone())),
        };
        self.traces.push(trace);
    }

    fn prepare_trace_call_result(
        &mut self, result: &VmResult<ExecutiveResult>,
    ) {
        let trace = ExecTrace {
            action: Action::CallResult(CallResult::from(result)),
        };
        self.traces.push(trace);
    }

    fn prepare_trace_create(&mut self, params: &ActionParams) {
        let trace = ExecTrace {
            action: Action::Create(Create::from(params.clone())),
        };
        self.traces.push(trace);
    }

    fn prepare_trace_create_result(
        &mut self, result: &VmResult<ExecutiveResult>,
    ) {
        let trace = ExecTrace {
            action: Action::CreateResult(CreateResult::from(result)),
        };
        self.traces.push(trace);
    }

    fn drain(self) -> Vec<ExecTrace> { self.traces }
}
