// Integration test for SigmaOS-zfs-progs
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_zfs_progs as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
