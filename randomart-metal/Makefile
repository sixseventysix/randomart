# -------- config --------
STRING ?= "spiderman 2"
DEPTH  ?= 30
WIDTH  ?= 512
HEIGHT ?= 512
OUTFILE ?= $(STRING)

generate:
	cargo run --release -- generate "$(STRING)" $(DEPTH) $(WIDTH) $(HEIGHT)

read:
	cargo run --release -- read $(OUTFILE) $(WIDTH) $(HEIGHT)

.PHONY: generate read
