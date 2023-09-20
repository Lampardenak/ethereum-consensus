// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::SyncAggregateTestCase;
use ethereum_consensus::{altair::minimal as spec, ssz::prelude::*};

#[test]
fn test_case_0() {
    let test_case = SyncAggregateTestCase::from(
        "../consensus-spec-tests/tests/minimal/altair/ssz_static/SyncAggregate/ssz_zero/case_0",
    );

    test_case.execute(|encoding| {
        let mut data: spec::SyncAggregate =
            ethereum_consensus::ssz::prelude::deserialize(encoding).unwrap();
        let serialized = ethereum_consensus::ssz::prelude::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}
