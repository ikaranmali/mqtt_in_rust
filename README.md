# MQTT in Rust #

### Functionality ###
- Pub utility just publishes a default message with utc time to the topic which Sub utility consumes and print on console,
the pub script can be modified to send more specific detail messages, similarly sub script can be modified for further processing of messages.
- both binaries are compressed using gzip, extract them to use binaries directly.

## Compilation ##
- To modify & run utility on other OS like windows, do compile it and use the bins. 
>cargo build --release

~ copy compiled binary from release directory ~
- Use Pub and Sub linux binary files by passing config file as argument
> ./pub pub-conf.ini
> ./sub sub-conf.ini


## Configuration ##
- Input details for pub-conf.ini config file.
>[Parameters]
>host = tcp://localhost:1883
>interval_in_ms = 1000
>client_id = test-pub
>topic = test
>qos = 2

- Input details for sub-conf.ini config file.
>[Parameters]
>host = tcp://localhost:1883
>clean_session = true
>client_id = test-sub
>topic = test
>qos = 2

* Note: For persistant session keep clean_session parameter as false.

## To be added features ##
 - Username & Password Authentication
 - TLS/SSL 