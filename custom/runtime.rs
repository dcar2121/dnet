// Import your pallet
pub use pallet_dnet;

impl pallet_dnet::Config for Runtime {
    type Event = Event;
}

// Include your pallet in the runtime
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // ... existing pallets
        Dnet: pallet_dnet::{Module, Call, Storage, Event<T>},
    }
);
