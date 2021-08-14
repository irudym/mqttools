# A simple MQTT tool to publish and subscribe to messages. 

#### Usage
Run with flag --help to get information about possible commands. 

#### Examples

**Publish message**
mqttools -p "test/messages" -m "this is test message payload"

**Subscribe to a topic**
mqttools -s "test/messages"