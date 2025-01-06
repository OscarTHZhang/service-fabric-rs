// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

//! This module contains components that enables unit testing for services constructed using mssf-core

use crate::runtime::{executor::Executor, stateful::StatefulServiceFactory};

pub struct TestRuntime<E: Executor, F: StatefulServiceFactory> {
    pub rt: E,
    pub stateful_service_factory: Option<F>,
}

impl<E: Executor, F: StatefulServiceFactory> TestRuntime<E, F> {
    /// Create a new TestRuntime instance for constructing unit tests
    pub fn create(rt: E) -> crate::Result<TestRuntime<E, F>> {
        Ok(TestRuntime { 
            rt,
            stateful_service_factory: None,
        })
    }

    /// Register a stateful service factory with the runtime
    pub fn register_stateful_service_factory(
        &self,
        servicetypename: &crate::WString,
        factory: impl crate::runtime::stateful::StatefulServiceFactory + 'static,
    ) -> crate::Result<()> {
        unimplemented!()
    }
    
}
