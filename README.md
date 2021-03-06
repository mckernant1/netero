# Netero
Load testing tool written in rust for fun.

# Install

---

## Homebrew

Install via brew
```bash
brew install mckernant1/tap/netero
```

# Usage

---

## Drive Traffic


```bash
netero punch --url http://localhost:8000\
	--rps 20\
	--duration 20\
	--duration-unit second\
	--method get
```


## Aggregate

Aggregates data and prints out aggregated json

```bash
netero punch --url http://localhost:8000\
	--rps 20\
	--duration 20\
	--duration-unit second\
	--method get | 
	netero aggregate\
	-c="response_code"\
	-P=0,50,90,99,99.99:latency |
	jq
```


## Use with jq

You can use jq to merge the resulting json lines together into a single json object

```bash
netero punch --url http://localhost:8000\
        --rps 20\
        --duration 5\
        --duration-unit second\
        --method get | jq -s
```

## User Manual

```
netero punch --help                                            
netero-punch 0.0.1
Load test against an endpoint

USAGE:
    netero punch [OPTIONS] --url <URL> --rps <RPS> --duration <DURATION>

OPTIONS:
    -b, --body <BODY>
            Body of the request [default: ]

    -d, --duration <DURATION>
            How long to run the test. Used with duration_unit

    -h, --help
            Print help information

    -H, --headers <HEADERS>
            Headers on the request

    -m, --method <METHOD>
            HTTP method [default: get] [possible values: options, get, post, put, delete, head,
            trace, connect, patch]

    -n, --duration-unit <DURATION_UNIT>
            What unit to run with the test [default: minute] [possible values: second, minute, hour]

    -q, --quiet
            Less output per occurrence

    -r, --rps <RPS>
            How many RPS to drive. This can max out on certain devices

    -t, --thread-count <THREAD_COUNT>
            How many worker threads to start [default: 10]

    -u, --url <URL>
            Target URL

    -v, --verbose
            More output per occurrence

    -V, --version
            Print version information
```
