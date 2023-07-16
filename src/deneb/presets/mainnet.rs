pub use crate::altair::mainnet::SYNC_COMMITTEE_SIZE;
pub use crate::altair::mainnet::{
    AggregateAndProof, Attestation, AttesterSlashing, ContributionAndProof, HistoricalBatch,
    IndexedAttestation, LightClientUpdate, PendingAttestation, SignedAggregateAndProof,
    SignedContributionAndProof, SyncAggregate, SyncCommittee, SyncCommitteeContribution,
    SyncCommitteeMessage,
};
pub use crate::bellatrix::mainnet::Transaction;
pub use crate::bellatrix::mainnet::{
    BYTES_PER_LOGS_BLOOM, MAX_BYTES_PER_TRANSACTION, MAX_EXTRA_DATA_BYTES,
    MAX_TRANSACTIONS_PER_PAYLOAD, PROPORTIONAL_SLASHING_MULTIPLIER_BELLATRIX,
};
pub use crate::capella::mainnet::{MAX_BLS_TO_EXECUTION_CHANGES, MAX_WITHDRAWALS_PER_PAYLOAD};
use crate::deneb;
use crate::deneb::presets::Preset;
pub use crate::phase0::mainnet::{
    EPOCHS_PER_HISTORICAL_VECTOR, EPOCHS_PER_SLASHINGS_VECTOR, ETH1_DATA_VOTES_BOUND,
    HISTORICAL_ROOTS_LIMIT, MAX_ATTESTATIONS, MAX_ATTESTER_SLASHINGS, MAX_DEPOSITS,
    MAX_PROPOSER_SLASHINGS, MAX_VALIDATORS_PER_COMMITTEE, MAX_VOLUNTARY_EXITS,
    SLOTS_PER_HISTORICAL_ROOT, VALIDATOR_REGISTRY_LIMIT,
};

pub use deneb::*;

pub const FIELD_ELEMENTS_PER_BLOB: usize = 4096;
pub const MAX_BLOB_COMMITMENTS_PER_BLOCK: usize = 4096;
pub const MAX_BLOBS_PER_BLOCK: usize = 6;
pub const BYTES_PER_BLOB: usize = BYTES_PER_FIELD_ELEMENT * FIELD_ELEMENTS_PER_BLOB;
pub const MAX_REQUEST_BLOB_SIDECARS: usize = MAX_REQUEST_BLOCKS_DENEB * MAX_BLOBS_PER_BLOCK;

pub const PRESET: Preset = Preset {
    field_elements_per_blob: FIELD_ELEMENTS_PER_BLOB,
    max_blob_commitments_per_block: MAX_BLOB_COMMITMENTS_PER_BLOCK,
    max_blobs_per_block: MAX_BLOBS_PER_BLOCK,
};

pub type ExecutionPayload = deneb::ExecutionPayload<
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
>;

pub type ExecutionPayloadHeader =
    deneb::ExecutionPayloadHeader<BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES>;

pub type BlindedBeaconBlock = deneb::BlindedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOBS_PER_BLOCK,
>;

pub type BlindedBeaconBlockBody = deneb::BlindedBeaconBlockBody<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOBS_PER_BLOCK,
>;

pub type SignedBlindedBeaconBlock = deneb::SignedBlindedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOBS_PER_BLOCK,
>;

pub type BeaconState = deneb::BeaconState<
    SLOTS_PER_HISTORICAL_ROOT,
    HISTORICAL_ROOTS_LIMIT,
    ETH1_DATA_VOTES_BOUND,
    VALIDATOR_REGISTRY_LIMIT,
    EPOCHS_PER_HISTORICAL_VECTOR,
    EPOCHS_PER_SLASHINGS_VECTOR,
    MAX_VALIDATORS_PER_COMMITTEE,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
>;

pub type BeaconBlockBody = deneb::BeaconBlockBody<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOBS_PER_BLOCK,
>;

pub type BeaconBlock = deneb::BeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOBS_PER_BLOCK,
>;

pub type SignedBeaconBlock = deneb::SignedBeaconBlock<
    MAX_PROPOSER_SLASHINGS,
    MAX_VALIDATORS_PER_COMMITTEE,
    MAX_ATTESTER_SLASHINGS,
    MAX_ATTESTATIONS,
    MAX_DEPOSITS,
    MAX_VOLUNTARY_EXITS,
    SYNC_COMMITTEE_SIZE,
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    MAX_BLS_TO_EXECUTION_CHANGES,
    MAX_BLOBS_PER_BLOCK,
>;

pub type Blob = deneb::Blob<BYTES_PER_BLOB>;
pub type BlobSidecar = deneb::BlobSidecar<BYTES_PER_BLOB>;
pub type SignedBlobSidecar = deneb::SignedBlobSidecar<MAX_BLOBS_PER_BLOCK, BYTES_PER_BLOB>;
pub type BlindedBlobSidecar = deneb::BlindedBlobSidecar<BYTES_PER_BLOB>;
pub type SignedBlindedBlobSidecar =
    deneb::SignedBlindedBlobSidecar<MAX_BLOBS_PER_BLOCK, BYTES_PER_BLOB>;

pub type NoOpExecutionEngine = deneb::NoOpExecutionEngine<
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
>;

pub type MockExecutionEngine<F> = deneb::MockExecutionEngine<
    BYTES_PER_LOGS_BLOOM,
    MAX_EXTRA_DATA_BYTES,
    MAX_BYTES_PER_TRANSACTION,
    MAX_TRANSACTIONS_PER_PAYLOAD,
    MAX_WITHDRAWALS_PER_PAYLOAD,
    F,
>;
