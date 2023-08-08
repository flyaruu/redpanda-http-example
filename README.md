!! Note: This is a personal project, to learn Rust and its ecosystem, combining it with my love for streaming systems.

## Example for using the RedPanda HTTP crate

CI:
[![CircleCI](https://circleci.com/gh/flyaruu/redpanda-http-example.svg?style=svg)](https://circleci.com/gh/flyaruu/redpanda-http-example)

Make sure you have docker installed, and the usual Rust stuff.

Run:
```bash
docker-compose up
```
... and switch to a new shell, keep this one open for the logs.

It will start two containers:
- Red Panda broker (= Kafka compatible broker)
- Red Panda Console (= Web UI for Red Panda)

Check the latter by opening a browser to:
http://localhost:8084

This is a good place to see what's going on in your local cluster.

Let's create a topic. You can use the UI above, but we can also exec into one of the containers and create a topic:

```bash
docker-compose exec redpanda rpk topic create color -p 2
```
This creates topic called 'color' with two partitions. Check the UI if the topic was indeed created.

Let's run our publisher! It will infinitely keep publishing messages to the 'color' topic.
```bash
cargo run --bin publish
```
While it is running, check the UI, refresh and see the number of messages increase. It isn't super fast, (+- 200msg /s) as it submits and confirms each message in a separate HTTP call.

Press CTRL+C to stop it.

Let's subscribe to those messages. Run:
```bash
cargo run --bin subscribe
```
It will print all the messages it is reading, along with some debug data.

You can check the consumer group called 'my_group' to see how much it is lagging behind the head of the log.

There is some wonkiness in the partition assignment (sometimes partitions seem to hang - red panda isn't delivering any messages from certain pertitions to our client, but after a minute or so it seems to recover).

It also commits its offsets after each message (which slows everything down quite a bit). The Red Panda Console shows those commits, but at times it seems to lose track when restarting.


