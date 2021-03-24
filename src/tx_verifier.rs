use ckb_error::{impl_error_conversion_with_kind, prelude::*, Error, ErrorKind};
use ckb_types::{
    core::{TransactionView, Capacity, Version},
    packed::{Byte32, OutPoint},
};
use derive_more::Display;

#[derive(Clone, Debug, Display, Eq, PartialEq)]
pub enum TransactionErrorSource {
    CellDeps,
    HeaderDeps,
    Inputs,
    Outputs,
    OutputsData,
    Witnesses,
}

/// TODO(doc): @keroro520
#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum TransactionError {
    /// output.occupied_capacity() > output.capacity()
    #[error("InsufficientCellCapacity({inner}[{index}]): expected occupied capacity ({occupied_capacity:#x}) <= capacity ({capacity:#x})")]
    InsufficientCellCapacity {
        /// TODO(doc): @keroro520
        inner: TransactionErrorSource,
        /// TODO(doc): @keroro520
        index: usize,
        /// TODO(doc): @keroro520
        occupied_capacity: Capacity,
        /// TODO(doc): @keroro520
        capacity: Capacity,
    },

    /// SUM([o.capacity for o in outputs]) > SUM([i.capacity for i in inputs])
    #[error("OutputsSumOverflow: expected outputs capacity ({outputs_sum:#x}) <= inputs capacity ({inputs_sum:#x})")]
    OutputsSumOverflow {
        /// TODO(doc): @keroro520
        inputs_sum: Capacity,
        /// TODO(doc): @keroro520
        outputs_sum: Capacity,
    },

    /// inputs.is_empty() || outputs.is_empty()
    #[error("Empty({inner})")]
    Empty {
        /// TODO(doc): @keroro520
        inner: TransactionErrorSource,
    },

    /// Duplicated dep-out-points within the same transaction
    #[error("DuplicateCellDeps({out_point})")]
    DuplicateCellDeps {
        /// TODO(doc): @keroro520
        out_point: OutPoint,
    },

    /// Duplicated headers deps without within the same transaction
    #[error("DuplicateHeaderDeps({hash})")]
    DuplicateHeaderDeps {
        /// TODO(doc): @keroro520
        hash: Byte32,
    },

    /// outputs.len() != outputs_data.len()
    #[error("OutputsDataLengthMismatch: expected outputs data length ({outputs_data_len}) = outputs length ({outputs_len})")]
    OutputsDataLengthMismatch {
        /// TODO(doc): @keroro520
        outputs_len: usize,
        /// TODO(doc): @keroro520
        outputs_data_len: usize,
    },

    /// The format of `transaction.since` is invalid
    #[error("InvalidSince(Inputs[{index}]): the field since is invalid")]
    InvalidSince {
        /// TODO(doc): @keroro520
        index: usize,
    },

    /// The transaction is not mature which is required by `transaction.since`
    #[error(
    "Immature(Inputs[{index}]): the transaction is immature because of the since requirement"
    )]
    Immature {
        /// TODO(doc): @keroro520
        index: usize,
    },

    /// The transaction is not mature which is required by cellbase maturity rule
    #[error("CellbaseImmaturity({inner}[{index}])")]
    CellbaseImmaturity {
        /// TODO(doc): @keroro520
        inner: TransactionErrorSource,
        /// TODO(doc): @keroro520
        index: usize,
    },

    /// The transaction version is mismatched with the system can hold
    #[error("MismatchedVersion: expected {}, got {}", expected, actual)]
    MismatchedVersion {
        /// TODO(doc): @keroro520
        expected: Version,
        /// TODO(doc): @keroro520
        actual: Version,
    },

    /// The transaction size is too large
    #[error("ExceededMaximumBlockBytes: expected transaction serialized size ({actual}) < block size limit ({limit})")]
    ExceededMaximumBlockBytes {
        /// TODO(doc): @keroro520
        limit: u64,
        /// TODO(doc): @keroro520
        actual: u64,
    },
}

impl TransactionError {
    /// TODO(doc): @keroro520
    pub fn is_malformed_tx(&self) -> bool {
        match self {
            TransactionError::OutputsSumOverflow { .. }
            | TransactionError::DuplicateCellDeps { .. }
            | TransactionError::DuplicateHeaderDeps { .. }
            | TransactionError::Empty { .. }
            | TransactionError::InsufficientCellCapacity { .. }
            | TransactionError::InvalidSince { .. }
            | TransactionError::ExceededMaximumBlockBytes { .. }
            | TransactionError::OutputsDataLengthMismatch { .. } => true,

            TransactionError::Immature { .. }
            | TransactionError::CellbaseImmaturity { .. }
            | TransactionError::MismatchedVersion { .. } => false,
        }
    }
}

impl_error_conversion_with_kind!(TransactionError, ErrorKind::Transaction, Error);

pub struct OutputsDataVerifier<'a> {
    transaction: &'a TransactionView,
}

impl<'a> OutputsDataVerifier<'a> {
    pub fn new(transaction: &'a TransactionView) -> Self {
        Self { transaction }
    }

    pub fn verify(&self) -> Result<(), TransactionError> {
        let outputs_len = self.transaction.outputs().len();
        let outputs_data_len = self.transaction.outputs_data().len();

        if outputs_len != outputs_data_len {
            return Err(TransactionError::OutputsDataLengthMismatch {
                outputs_data_len,
                outputs_len,
            });
        }
        Ok(())
    }
}
