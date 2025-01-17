use async_trait::async_trait;
use infusion::c;

use crate::dht::DhtInterface;
use crate::infu_collection::Collection;
use crate::types::{ImmutablePointer, ResolvedImmutablePointerRecord};
use crate::{
    Blake3Hash,
    ConfigConsumer,
    ConfigProviderInterface,
    DhtSocket,
    SignerInterface,
    WithStartAndShutdown,
};

/// The resolver is responsible to resolve an FNIP (Fleek Network Immutable Pointer),
/// into a Blake3 hash of the content.
#[async_trait]
#[infusion::service]
pub trait ResolverInterface<C: Collection>: Sized + ConfigConsumer + WithStartAndShutdown {
    fn _init(config: ::ConfigProviderInterface, dht: ::DhtInterface, signer: ::SignerInterface) {
        Self::init(config.get::<Self>(), dht.get_socket(), signer)
    }

    type OriginFinder: OriginFinderAsyncIter;

    /// Initialize and return the resolver service.
    fn init(
        config: Self::Config,
        dht: DhtSocket,
        signer: &c!(C::SignerInterface),
    ) -> anyhow::Result<Self>;

    /// Publish new records into the resolver global hash table about us witnessing
    /// the given blake3 hash from resolving the following pointers.
    fn publish(&self, hash: Blake3Hash, pointers: &[ImmutablePointer]);

    /// Tries to find the blake3 hash of an immutable pointer by performing a global lookup.
    ///
    /// This can return [`None`] based on an implementation specific timeout.
    async fn get_blake3_hash_globally(
        &self,
        pointer: ImmutablePointer,
    ) -> Option<ResolvedImmutablePointerRecord>;

    /// Tries to find the blake3 hash of an immutable pointer by only relying on locally cached
    /// records and without performing any contact with other nodes.
    ///
    /// This can return [`None`] if no local record is found.
    async fn get_blake3_hash_locally(
        &self,
        pointer: ImmutablePointer,
    ) -> Option<ResolvedImmutablePointerRecord>;

    /// Returns an origin finder that can yield origins for the provided blake3 hash.
    fn get_origin_finder(&self, hash: Blake3Hash) -> Self::OriginFinder;
}

/// An `async-iterator`-like interface that tries to find the immutable pointers of
#[async_trait]
#[infusion::blank]
pub trait OriginFinderAsyncIter: Sized + Send + Sync {
    /// Returns the hash of requested content.
    fn hash(&self) -> &Blake3Hash;

    /// Find and return the next origin for the requested hash. Returns `None`
    /// after the implementation defined timeout has passed.
    async fn next(&mut self) -> Option<ImmutablePointer>;

    /// The sync version of `next`. This returns `None` if there are no further
    /// items in our outgoing queue. So this function can return `None` while
    /// the subsequent call to `next` can return `Some`.
    ///
    /// This is only a way to access the internal state of the iterator when several
    /// items are already found.
    fn next_sync(&mut self) -> Option<ImmutablePointer>;
}
