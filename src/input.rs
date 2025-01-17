use std::any;
use std::fmt::Debug;

use crate::state_input::StagedVMState;
use libafl::inputs::Input;

use libafl::prelude::{HasMaxSize, HasRand, MutationResult, State};
use libafl::state::HasMetadata;

use crate::evm::abi::BoxedABI;

use crate::generic_vm::vm_state::VMStateT;
use crate::state::{HasCaller, HasItyState};
use primitive_types::U256;
use serde::de::DeserializeOwned;
use serde::Serialize;

// ST: Should VMInputT be the generic type for both inputs?
pub trait VMInputT<VS, Loc, Addr>:
    Input + Debug + Clone + serde_traitobject::Serialize + serde_traitobject::Deserialize
where
    VS: Default + VMStateT,
    Addr: Debug + Clone + Serialize + DeserializeOwned,
    Loc: Debug + Clone + Serialize + DeserializeOwned,
{
    fn mutate<S>(&mut self, state: &mut S) -> MutationResult
    where
        S: State
            + HasRand
            + HasMaxSize
            + HasItyState<Loc, Addr, VS>
            + HasCaller<Addr>
            + HasMetadata;
    fn get_caller_mut(&mut self) -> &mut Addr;
    fn get_caller(&self) -> Addr;
    fn set_caller(&mut self, caller: Addr);
    fn get_contract(&self) -> Addr;
    fn get_state(&self) -> &VS;
    fn get_state_mut(&mut self) -> &mut VS;
    fn set_staged_state(&mut self, state: StagedVMState<Loc, Addr, VS>, idx: usize);
    fn get_state_idx(&self) -> usize;
    fn get_staged_state(&self) -> &StagedVMState<Loc, Addr, VS>;

    // fn get_abi_cloned(&self) -> Option<BoxedABI>;
    fn set_as_post_exec(&mut self, out_size: usize);
    fn is_step(&self) -> bool;
    fn set_step(&mut self, gate: bool);
    fn pretty_txn(&self) -> Option<String>;
    fn as_any(&self) -> &dyn any::Any;

    // determine whether a input is better than another
    fn fav_factor(&self) -> f64;

    #[cfg(feature = "evm")]
    fn get_data_abi(&self) -> Option<BoxedABI>;

    #[cfg(feature = "evm")]
    fn get_data_abi_mut(&mut self) -> &mut Option<BoxedABI>;

    #[cfg(feature = "evm")]
    fn get_txn_value_temp(&self) -> Option<U256>;

    #[cfg(any(test, feature = "reexecution"))]
    fn get_direct_data(&self) -> Vec<u8>;
}
