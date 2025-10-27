#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::ensure_signed;
use sp_std::vec::Vec;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as Dnet {
        // Static seed per validator (could be private key or nonce)
        StaticSeed get(fn static_seed): map hasher(blake2_128_concat) T::AccountId => Vec<u8>;

        // Coherence signal (e.g., heartbeat)
        CoherenceSignal get(fn coherence_signal): u64;

        // Transmission signal (e.g., timestamp)
        TransmissionSignal get(fn transmission_signal): u64;
    }
}

decl_event! {
    pub enum Event<T> {
        StaticSeedSet(T::AccountId),
        SignalsUpdated(u64, u64),
        ValidatorIDGenerated(Vec<u8>),
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        SeedNotSet,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // Set static seed for validator
        #[weight = 10_000]
        pub fn set_static_seed(origin, seed: Vec<u8>) {
            let who = ensure_signed(origin)?;
            StaticSeed::<T>::insert(&who, seed);
            Self::deposit_event(RawEvent::StaticSeedSet(who));
        }

        // Update signals
        #[weight = 10_000]
        pub fn update_signals(origin, coherence: u64, transmission: u64) {
            let _who = ensure_signed(origin)?;
            CoherenceSignal::put(coherence);
            TransmissionSignal::put(transmission);
            Self::deposit_event(RawEvent::SignalsUpdated(coherence, transmission));
        }

        // Generate validator ID based on signals and static seed
        #[weight = 10_000]
        pub fn generate_validator_id(origin) {
            let who = ensure_signed(origin)?;
            let seed = Self::static_seed(&who).ok_or(Error::<T>::SeedNotSet)?;
            let coherence = Self::coherence_signal();
            let transmission = Self::transmission_signal();

            let mut data = seed.clone();
            data.extend_from_slice(&coherence.to_le_bytes());
            data.extend_from_slice(&transmission.to_le_bytes());

            // Hash data using SHA-256
            let validator_id = sp_io::hashing::sha2_256(&data);
            Self::deposit_event(RawEvent::ValidatorIDGenerated(validator_id.to_vec()));
        }
    }
}
