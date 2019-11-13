use diesel::PgConnection;
use serde::{de::DeserializeOwned, Serialize};

use crate::db::DieselPoolObj;
use crate::errors::{EnqueueError, PerformError};
use crate::storage;

/// A background job, meant to be run asynchronously.
pub trait Job: Serialize + DeserializeOwned {
    /// The environment this job is run with. This is a struct you define,
    /// which should encapsulate things like database connection pools, any
    /// configuration, and any other static data or shared resources.
    type Environment: 'static;

    /// The key to use for storing this job, and looking it up later.
    ///
    /// Typically this is the name of your struct in `snake_case`
    const JOB_TYPE: &'static str;

    /// Enqueue this job to be run at some point in the future.
    fn enqueue(self, conn: &PgConnection) -> Result<(), EnqueueError> {
        storage::enqueue_job(conn, self)
    }

    /// The logic involved in actually performing this job.
    fn perform(self, env: &Self::Environment, pool: &dyn DieselPoolObj)
        -> Result<(), PerformError>;
}
