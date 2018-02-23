// Copyright 2018 OneSignal, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
extern crate zk_4lw;

static MNTR: &str = include_str!("./fixtures/mntr.response");

use zk_4lw::{Mntr, FourLetterWord};

#[test]
fn parse_mntr_response() {
    let res = <Mntr as FourLetterWord>::parse_response(MNTR).unwrap();

    assert_eq!(res.zk_avg_latency, 0);
    assert_eq!(res.zk_max_latency, 22);
    assert_eq!(res.zk_min_latency, 0);
    assert_eq!(res.zk_packets_received, 434);
    assert_eq!(res.zk_packets_sent, 436);
    assert_eq!(res.zk_outstanding_requests, 0);
    assert_eq!(res.zk_server_state, String::from("standalone"));
    assert_eq!(res.zk_znode_count, 28);
    assert_eq!(res.zk_watch_count, 16);
    assert_eq!(res.zk_ephemerals_count, 2);
    assert_eq!(res.zk_approximate_data_size, 923);
    assert_eq!(res.zk_open_file_descriptor_count, Some(28));
    assert_eq!(res.zk_max_file_descriptor_count, Some(1048576));
}

#[cfg(feature = "client-tests")]
mod client_tests {
    use zk_4lw::{Client, Mntr};
    #[test]
    fn mntr() {
        let client = Client::new("localhost:2181");
        let res = client.exec::<Mntr>().unwrap();

        println!("res: {:#?}", res);
    }
}
