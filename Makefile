CARGO=xargo
TARGET=thumbv7em-none-eabihf
PROJECT=stm32OnRust

.PHONY: build clean flash
build:
	$(CARGO) $@
clean:
	$(CARGO) $@
flash:
	echo target/$(TARGET)/debug/$(PROJECT) | xargs -I {} openocd -f interface/cmsis-dap.cfg -f target/stm32f3x.cfg -c 'init;program {};reset;exit'
