use std::{time::Duration, thread::{sleep, self}, collections::HashSet};

use log::{LevelFilter, warn, debug, info};
use redpanda_http::{subscriber::Subscriber, RedPandaError};
use simple_logger::SimpleLogger;
use simplehttp::simplehttp_reqwest::SimpleHttpClientReqwest;
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
    let group = "my_group";
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    consume_data("consumer_1", group).unwrap();

    // let t1 = thread::spawn(|| consume_data("consumer_1", group).unwrap());
    // sleep(Duration::from_secs(5));
    // let t2 = thread::spawn(|| consume_data("consumer_2", group).unwrap());
    
    // t1.join().unwrap();
    // t2.join().unwrap();
}

fn consume_data(id: &str, group: &str)->Result<(),RedPandaError> {
    let reqwest_client = SimpleHttpClientReqwest::new_reqwest().unwrap();
    let mut redpanda = Subscriber::new(
        reqwest_client,
        "http://localhost:8082/",
        group,
        None,
    )
    .unwrap();

    let mut found_partitions: HashSet<u16> = HashSet::new();
    redpanda.register_topic(vec![("color",2)   ])?;
    let mut total_messages = 0_u64;
    loop  {
        let messages = redpanda.poll(250)?;
        debug!("# of messages: {}",messages.len());
        for msg in messages {
            total_messages+=1;
            found_partitions.insert(msg.partition);
            // let content = msg.value.map(|f| from_utf8(&f).u/nwrap());
            debug!("process {} Message partition: {} offset: {}",id, msg.partition, msg.offset);
            info!("Total messages: {} partitions: {:?}",total_messages, found_partitions);
            redpanda.process_record(&msg);
        }
        redpanda.commit_state()?;
        sleep(Duration::from_millis(20))
    };
    Ok(())
}