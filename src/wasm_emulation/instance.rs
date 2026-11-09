use cosmwasm_vm::internals::compile_module;
use cosmwasm_vm::internals::instance_from_module;
use cosmwasm_vm::{Backend, BackendApi, Instance, InstanceOptions, Querier, Storage, VmResult};
use wasmer::{Engine, Module, Store};

pub fn create_module(code: &[u8]) -> VmResult<(Engine, Module)> {
    let (module, engine) = compile_module(code, None)?;
    Ok((engine, module))
}

/// This is the only Instance constructor that can be called from outside of cosmwasm-vm,
/// e.g. in test code that needs a customized variant of cosmwasm_vm::testing::mock_instance*.
pub fn instance_from_reused_module<A, S, Q>(
    (engine, module): (Engine, Module),
    backend: Backend<A, S, Q>,
    options: InstanceOptions,
) -> VmResult<Instance<A, S, Q>>
where
    A: BackendApi + 'static,
    S: Storage + 'static,
    Q: Querier + 'static,
{
    let store = Store::new(engine);
    instance_from_module(store, &module, backend, options.gas_limit, None)
}
