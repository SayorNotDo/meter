use core::panic;
use std::{future::Future, io, str::FromStr, time::Duration};

use chrono::{DateTime, Utc};
use cron::Schedule;
use std::{any::Any, time::Instant};
use toml::Table;
use uuid::Uuid;

// use super::redis::set;

#[derive(Debug)]
pub struct JobContext {
    // Job ID.
    job_id: uuid::Uuid,
    // Job name.
    job_name: Option<&'static str>,
    // Source
    source: String,
    // Start time.
    start_time: Instant,
    // Flag to indicate whether the job is disabled.
    disabled: bool,
    // Flag to indicate whether the job is executed immediately.
    immediate: bool,
    // Remaining ticks.
    remaining_ticks: Option<usize>,
    // Last time when running the job.
    last_tick: Option<DateTime<Utc>>,

    // An error occurred in the job execution.
    // execution_error: Option<Error>,
    // Optional job data.
    job_data: Option<Box<dyn Any + Send>>,
}

impl JobContext {
    pub fn new() -> Self {
        Self {
            job_id: Uuid::new_v4(),
            job_name: None,
            source: String::new(),
            start_time: Instant::now(),
            disabled: false,
            immediate: false,
            remaining_ticks: None,
            last_tick: None,
            // execution_error: None,
            job_data: None,
        }
    }

    // Start the job.
    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    // Finish the job.
    pub fn finish(&mut self) {
        if let Some(ticks) = self.remaining_ticks {
            self.remaining_ticks = Some(ticks.saturating_sub(1));
        }

        let job_id = self.job_id.to_string();
        let job_name = self.job_name;
        let execution_time = self.start_time.elapsed();
        tracing::warn!(
            job_id,
            job_name,
            remaining_ticks = self.remaining_ticks,
            last_tick = self.last_tick.map(|dt| dt.to_string()),
            execution_time_millis = execution_time.as_millis()
        );
    }

    pub fn set_source(&mut self, source: impl Into<String>) {
        self.source = source.into();
    }

    pub fn set_disabled_status(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}

pub type CronJob = fn(ctx: &mut JobContext);

pub struct Job {
    context: JobContext,
    /// Cron expression parser
    schedule: Schedule,
    exec: CronJob,
}

impl Job {
    /// Creates a new instance
    ///
    /// # Panics
    ///
    /// Panics if the cron expression is invalid
    pub fn new(cron_expr: &str, exec: CronJob) -> Self {
        let schedule = Schedule::from_str(cron_expr)
            .unwrap_or_else(|err| panic!("invalid cron expression: `{cron_expr}`: {err}"));
        let mut context = JobContext::new();
        context.set_source(cron_expr);
        Self {
            context,
            schedule,
            exec,
        }
    }
    /// Creates a new instance with configuration
    ///
    /// # Panics
    ///
    /// Panics if the configuration is invalid
    pub fn with_config(config: &Table, exec: CronJob) -> Self {
        let cron_expr = config.get_str("cron").unwrap_or_default();
        let schedule = Schedule::from_str(cron_expr)
            .unwrap_or_else(|err| panic!("invalid cron expression: `{cron_expr}`: {err}"));
        let mut context = JobContext::new();

        if let Some(disabled) = config.get_bool("disabled") {
            context.set_disabled_status(disabled);
        }
        // Create a new Job instance
        Self {
            context,
            schedule,
            exec,
        }
    }
}

pub trait Scheduler {
    // Returns `true` if the scheduler is ready to run.
    fn is_ready(&self) -> bool;

    // Returns the duration till the next job is supposed to run.
    fn time_till_next_job(&self) -> Option<Duration>;

    // Increments time for the scheduler and executes any pending jobs.
    fn tick(&mut self);
}

pub trait AsyncScheduler {
    // Returns `true` if the scheduler is ready to run.
    fn is_ready(&self) -> bool;

    // Returns `true` if the scheduler is blocking.
    fn is_blocking(&self) -> bool;

    // Returns the duration till the next job is supposed to run.
    fn time_till_next_job(&self) -> Option<Duration>;

    // Increments time for the scheduler and executes any pending jobs.
    fn tick(&mut self) -> impl Future<Output = ()> + Send;

    // Runs the scheduler and returns an `std::io::Error` if failed.
    fn run(self) -> impl Future<Output = io::Result<()>> + Send;
}
