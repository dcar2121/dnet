// pallets/dnet/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::ensure_signed;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as DnetModule {
        // Static seed per validator
        StaticSeed get(fn static_seed): map hasher(blake2_128_concat) T::AccountId => Vec<u8>;

        // Coherence signal (e.g., heartbeat)
        CoherenceSignal get(fn coherence_signal): u64;

        // Transmission signal (e.g., timestamp)
        TransmissionSignal get(fn transmission_signal): u64;
    }
}

decl_event! {
    pub enum Event<T> {
        ValidatorIDGenerated(Vec<u8>),
    }
}

decl_error! {
    pub enum Error for Module<T> {
        // Define errors
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn set_static_seed(origin, seed: Vec<u8>) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            StaticSeed::<T>::insert(&who, seed);
            Ok(())
        }

        #[weight = 10_000]
        pub fn update_signals(origin, coherence: u64, transmission: u64) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            CoherenceSignal::put(coherence);
            TransmissionSignal::put(transmission);
            Ok(())
        }

        // Function to generate validator ID
        pub fn generate_validator_id(who: T::AccountId) -> Vec<u8> {
            let seed = Self::static_seed(&who);
            let coherence = Self::coherence_signal();
            let transmission = Self::transmission_signal();

            // Concatenate data
            let mut data = seed.clone();
            data.extend_from_slice(&coherence.to_le_bytes());
            data.extend_from_slice(&transmission.to_le_bytes());

            // Hash data
            let validator_id = sp_io::hashing::sha2_256(&data);
            Self::deposit_event(RawEvent::ValidatorIDGenerated(validator_id.to_vec()));
            validator_id.to_vec()
        }
    }
}
