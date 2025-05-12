This is small project demonstrating how to process event stream in combination of Kafka server 
and Kafka Rust Clients.
Environment:
HOST - Ubuntu 24.04 with installed Multipass from Canonnical. 
This project was tested on Multipass Ubuntu 24.04 instance.
Note. Create an Instance with non-default Memory, Hard Drive Size, CPUs number.
Kafka server was running in Docker container.
File docker-compose.yml provided
Rust environment was installed on Instance.
Command line Kafka Utility kafkacat.
To enter Instance Instance Shell, pleasse use  
"multipass shell <InstanceName>"
To upload local file to Instance use 
"multipass transfer <filename> <InstanceName>:/home/ubuntu"
To upload local directory use
"multipass transfer -r <directoryname> <InstanceName>"
There are two application/kafka-Rust-Clients in the project:
    1) First consumes event stream data from wikimedia, process it by filtering using Event type, 
	    and publishes it to another topic with lesser number of fields.
     2) Pure consumer that verifies new customized event stream.
To Do:
   Redirect it to MinIO Object Storage.
   
To start to run container use
"docker compose up" 
   
To enter container shell run from Instance shell command ( Teminal 1/ T1)
"docker exec -it broker sh"

To create Kafka Topic from Container Shell:
"/usr/bin/kafka-topics --create --topic wikimedia --bootstrap-server broker:29092"
also create "short_wikimedia" topic
To list /verify topics:
"/usr/bin/kafka-topics --list --bootstrap-server broker:29092"
To reset topics use "delete"/"create" commands

To build binaries on Instance Go To appropriate directory and use 
"cargo build"

To have convinient binary names:
copy binaries from its target/debug directory to Home Directory:
with name app1, app2 (For examples)

   
To start load event stream from Instance Shell use command  (From Teerminal 2/T2)
"curl -N https://stream.wikimedia.org/v2/stream/recentchange  | awk '/^data: /{gsub(/^data: /, ""); print}' | kcat -P -b 0.0.0.0:9092 -t wikimedia"

To  verify Kafka server is working, using Kafka command line utility open Container Shell Terminal/ ( T3 ) and run command:
"/usr/bin/kafka-console-consumer --topic wikimedia --from-beginning --bootstrap-server broker:29092"

Then stop data loading, reset channel/topic
Set T3 to Instance Shell, Open Terminal T4 (also to Instance Shell)
Start loading data,
Run ./app1 in T3
Run ./app2 in T4