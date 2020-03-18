#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet certifybook with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, StorageMap,
	weights::{SimpleDispatchInfo},
	ensure
};
use system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as certifybookModule {

		AllCertificatesArray get(certificate_by_index): map hasher(blake2_256) u64 => T::Hash;
		AllCertificatesCount get(all_certificates_count): u64;
		
		OrgCertificatesArray get(certificate_of_org_by_index): map hasher(blake2_256) (T::AccountId, u64) => T::Hash;
		// The count of certificates issued by organizations
		OrgCertificatesCount get(certificates_count_of_org): map hasher(blake2_256) T::AccountId => u64;
		//OrgCertificatesIndex: map T::Hash => u64;

		CertificatesIssuer get(issuer_of_certificates): map hasher(blake2_256) T::Hash => T::AccountId;
	
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, 
	Hash = <T as system::Trait>::Hash {
		/// Just a dummy event.
		/// To emit this event, we call the deposit function, from our runtime functions
		CertificateStored(Hash, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = SimpleDispatchInfo::FixedNormal(0)]
		pub fn new_certificate(origin, certificate: T::Hash) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!<CertificatesIssuer<T>>::contains_key(certificate), "Certificate already exists");
			
			let all_certificates_count = Self::all_certificates_count();
			let new_all_certificates_count = all_certificates_count.checked_add(1).ok_or("Overflow add a new certificate")?;
			AllCertificatesCount::put(new_all_certificates_count);
			<AllCertificatesArray<T>>::insert(all_certificates_count, certificate);

			let certificates_count_of_org = Self::certificates_count_of_org(&who);
			let new_certificates_count_of_org = certificates_count_of_org.checked_add(1).ok_or("Overflow add a new certificate")?;
			<OrgCertificatesCount<T>>::insert(who.clone(), new_certificates_count_of_org);
			<OrgCertificatesArray<T>>::insert((who.clone(), certificates_count_of_org), certificate);
			<CertificatesIssuer<T>>::insert(certificate, who.clone());

			Self::deposit_event(RawEvent::CertificateStored(certificate, who));
			Ok(())
		}

	}
}
