ENDPOINT_ETH ?= mainnet.eth.streamingfast.io:443
ENDPOINT_SOL ?= mainnet.sol.streamingfast.io:443
STOP_BLOCK ?= +100

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream_eth
stream_eth: build
	substreams run -e $(ENDPOINT_ETH) substreams.yaml map_eth_stats -t $(STOP_BLOCK) $(STREAM_ARGS)

.PHONY: stream_sol
stream_sol: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_sol_stats -t $(STOP_BLOCK) $(STREAM_ARGS)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: stream
package: build
	substreams pack substreams.yaml
