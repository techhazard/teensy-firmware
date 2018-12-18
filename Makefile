BIN=teensy-firmware
OUTDIR=target/thumbv7em-none-eabi/release
HEX=$(OUTDIR)/$(BIN).hex
ELF=$(OUTDIR)/$(BIN)



all:: $(ELF)

.PHONY: $(ELF)

$(ELF):
	cargo build --release

$(HEX): $(ELF)
	arm-none-eabi-objcopy -O ihex $(ELF) $(HEX)

.PHONY: flash

flash: $(HEX) tools/teensy_loader_cli/teensy_loader_cli
	tools/teensy_loader_cli/teensy_loader_cli -w -mmcu=mk20dx256 $(HEX) -v

tools/teensy_loader_cli/teensy_loader_cli:
	cd tools/teensy_loader_cli && make
