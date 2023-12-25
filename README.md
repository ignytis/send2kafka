# send2kafka

Status: WIP

## Configuration

### General

Config could be provided in two ways:
- Environment variables e.g. `SEND2KAFKA__HTTP__HOST=localhost`
- YAML file, see the example in the next section
  - By default: a `config.yaml` file in current working directory
  - Could be overridden using `SEND2KAFKA__CONFIG` environment variable

### Parameters

There are default values provided in the snippet below:

```yaml
http:
  host: 0.0.0.0
  port: 8080
kafka:
  bootstrap_servers: localhost:9092
  # Here you can provide additional Kafka parameters
  # NB: as '.' is used as config section separator both here and in Kafka,
  # you will need to use underscore for Kafka config instead
  # (see the example above, originally the property name is 'bootstrap.servers').
  # The app will replace underscores with dots
```