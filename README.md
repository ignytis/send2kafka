# send2kafka

Status: WIP

## Configuration

### General

Config could be provided in two ways:
- Environment variables e.g. `SEND2KAFKA__HTTP__HOST=localhost`
- YAML file, see the example in the next section
  - By default: a `config.yaml` file in current working directory
  - Could be overridden using `SEND2KAFKA__CONFIG` environment variable

_NB: for Kafka attributes you need to use single `_` instead of `.`: `SEND2KAFKA__KAFKA__BOOTSTRAP_SERVERS=localhost:1234`.
Single underscores will be replaced with dots automatically.
The configuration library uses dots as separators which conflicts with Kafka property format.
In addition, environment variable names are not supposed to have dots._


### Parameters

There are default values provided in the snippet below:

```yaml
http:
  host: 0.0.0.0
  port: 8080
kafka:
  bootstrap_servers: localhost:9092
  # Here you can provide additional Kafka parameters, for example:
  # sasl_mechanism: SCRAM-SHA-256
  # security_protocol: SASL_PLAINTEXT
  # sasl_username: me
  # sasl_password: mypassword
```

## API

### POST /topics/<topic_name>

Sends a message to Kafka topic. Message payload should be provided as a message body.
The message key __MUST__ be specified using _X-Key_ HTTP header