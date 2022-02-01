use crate::{
    prehash::PreHashed, settings::EVENT_ID_SIZE_BYTES, Address, BlockId, ModelsError, Slot,
};
use massa_hash::hash::Hash;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// By product of a byte code execution
pub struct SCOutputEvent {
    /// event id computed from if it is read only, the slot, the index in the slot
    pub id: SCOutputEventId,
    /// context generated by the execution context
    pub context: EventExecutionContext,
    /// json data string
    pub data: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SCOutputEventId(pub Hash);

impl FromStr for SCOutputEventId {
    type Err = ModelsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SCOutputEventId(Hash::from_str(s)?))
    }
}

impl PreHashed for SCOutputEventId {}

impl SCOutputEventId {
    /// ## Example
    /// ```rust
    /// # use serde::{Deserialize, Serialize};
    /// # use massa_models::output_event::SCOutputEventId;
    /// # use massa_hash::hash::Hash;
    /// # use std::str::FromStr;
    /// # let hash = Hash::compute_from(&"hello world".as_bytes());
    /// let event = SCOutputEventId(hash);
    /// let bytes = event.to_bytes();
    /// # let res_event = SCOutputEventId::from_bytes(&bytes).unwrap();
    /// # assert_eq!(event, res_event);
    /// ```
    pub fn to_bytes(&self) -> [u8; EVENT_ID_SIZE_BYTES] {
        self.0.to_bytes()
    }

    /// ## Example
    /// ```rust
    /// # use serde::{Deserialize, Serialize};
    /// # use massa_models::output_event::SCOutputEventId;
    /// # use massa_hash::hash::Hash;
    /// # use std::str::FromStr;
    /// # let hash = Hash::compute_from(&"hello world".as_bytes());
    /// let event = SCOutputEventId(hash);
    /// let bytes = event.clone().into_bytes();
    /// # let res_event = SCOutputEventId::from_bytes(&bytes).unwrap();
    /// # assert_eq!(event, res_event);
    /// ```
    pub fn into_bytes(self) -> [u8; EVENT_ID_SIZE_BYTES] {
        self.0.into_bytes()
    }

    /// ## Example
    /// ```rust
    /// # use serde::{Deserialize, Serialize};
    /// # use massa_models::output_event::SCOutputEventId;
    /// # use massa_hash::hash::Hash;
    /// # use std::str::FromStr;
    /// # let hash = Hash::compute_from(&"hello world".as_bytes());
    /// let event = SCOutputEventId(hash);
    /// let bytes = event.to_bytes();
    /// let res_event = SCOutputEventId::from_bytes(&bytes).unwrap();
    /// # assert_eq!(event, res_event);
    /// ```
    pub fn from_bytes(data: &[u8; EVENT_ID_SIZE_BYTES]) -> Result<SCOutputEventId, ModelsError> {
        Ok(SCOutputEventId(
            Hash::from_bytes(data).map_err(|_| ModelsError::HashError)?,
        ))
    }

    /// ## Example
    /// ```rust
    /// # use serde::{Deserialize, Serialize};
    /// # use massa_models::output_event::SCOutputEventId;
    /// # use massa_hash::hash::Hash;
    /// # use std::str::FromStr;
    /// # let hash = Hash::compute_from(&"hello world".as_bytes());
    /// let event = SCOutputEventId(hash);
    /// let ser = event.to_bs58_check();
    /// # let res_event = SCOutputEventId::from_bs58_check(&ser).unwrap();
    /// # assert_eq!(event, res_event);
    /// ```
    pub fn from_bs58_check(data: &str) -> Result<SCOutputEventId, ModelsError> {
        Ok(SCOutputEventId(
            Hash::from_bs58_check(data).map_err(|_| ModelsError::HashError)?,
        ))
    }

    /// ## Example
    /// ```rust
    /// # use serde::{Deserialize, Serialize};
    /// # use massa_models::output_event::SCOutputEventId;
    /// # use massa_hash::hash::Hash;
    /// # use std::str::FromStr;
    /// # let hash = Hash::compute_from(&"hello world".as_bytes());
    /// let event = SCOutputEventId(hash);
    /// let ser = event.to_bs58_check();
    /// # let res_event = SCOutputEventId::from_bs58_check(&ser).unwrap();
    /// # assert_eq!(event, res_event);
    /// ```
    pub fn to_bs58_check(&self) -> String {
        self.0.to_bs58_check()
    }
}

/// Context of the event (not generated by the user)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventExecutionContext {
    /// when was it generated
    pub slot: Slot,
    /// block id if there was a block at that slot
    pub block: Option<BlockId>,
    /// if the event was generated during a read only execution
    pub read_only: bool,
    /// index of the event in the slot
    pub index_in_slot: u64,
    /// most recent at the end
    pub call_stack: VecDeque<Address>,
}
