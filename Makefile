# -------- config --------
STRING ?= "spiderman 2"
DEPTH ?= 30
WIDTH ?= 512
HEIGHT ?= 512
OUTFILE ?= $(STRING)

INPUT_TXT = $(OUTFILE)
METAL_SRC = data/metal/randomart_shader.metal
METAL_LIB = bin/randomart.metallib
SWIFT_SRC = src/main.swift
SWIFT_BIN = bin/run_art
OUT_PNG = data/images/$(OUTFILE).png

# Full flow: generate tree, compile metal, run and save image
generate: 
	$(MAKE) clean
	cargo run --release -- generate "$(STRING)" $(DEPTH)
	$(MAKE) render

# Read from formula instead of generating
read:
	$(MAKE) clean
	cargo run --release -- read $(OUTFILE)
	$(MAKE) render

# Render assumes metal + swift + final PNG
render: $(METAL_LIB) $(SWIFT_BIN)
	$(SWIFT_BIN) "$(OUTFILE).png" $(WIDTH) $(HEIGHT)

# Metal compilation
$(METAL_LIB): $(METAL_SRC)
	xcrun -sdk macosx metal $(METAL_SRC) -o $(METAL_LIB)

# Swift compilation
$(SWIFT_BIN): $(SWIFT_SRC)
	swiftc $(SWIFT_SRC) -o $(SWIFT_BIN)

# Generate metal code from random string
$(METAL_SRC): 
	cargo run --release -- generate "$(STRING)" $(DEPTH)

# Clean all outputs
clean:
	rm -f $(METAL_LIB) $(SWIFT_BIN) $(METAL_SRC)

.PHONY: generate read render clean