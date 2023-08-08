use std::{thread::sleep, time::{Duration, Instant}};

use simplehttp::simplehttp_reqwest::SimpleHttpClientReqwest;
use redpanda_http::publisher::{Publisher, PublishRecordList};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Deserialize, Serialize, Copy, Clone)]
pub struct Rotate {
    pub pulses: u32,
    pub direction: Direction,
}

pub struct Color(String);
fn main() {

    // a rainbow:
    let colors = [
        "9900dd",
        "440088",
        "0000ff",
        "00ff00",
        "ffff00",
        "ff7700",
        "ff0000",
    ];

    let mut redpanda = Publisher::new(
        SimpleHttpClientReqwest::new_reqwest().unwrap(),
        "http://localhost:8082/",
    );

    // publish color messages forever
    colors.iter()
        .map(|f| Color((*f).to_owned()))
        .cycle()
        .for_each(|f| {
            let _now = Instant::now();
            redpanda.publish(
                "color".to_owned(),
                PublishRecordList::from_string(f.0.to_owned()),
            )
            .unwrap();
            // sleep(Duration::from_millis(1000));
        });
    
}
