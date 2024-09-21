pub mod socket;
pub mod ipc_proto {
    include!(concat!(env!("OUT_DIR"), "/ipc.rs"));
}
