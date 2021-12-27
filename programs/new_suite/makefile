# NOTE: anything in the "exclude" directory will not get assembled,
# so tests that will not assemble due to architecture limitations can be excluded
TEST_SRC := $(wildcard *.s)
TEST_OBJ := $(notdir $(patsubst %.s,%.o,$(TEST_SRC)))
TOOLCHAIN := riscv64-elf
BINARY := testsuite

# set the architecture to test
ARCH := rv32i

# set the abi to use
ABI := ilp32

# ABI to specify for the linker
LINK_ABI := elf32lriscv

# code starts here (will get stripped out when converted to hex file?)
TEXT_ADDR := 0x0

# uninitialized data starts here
BSS_ADDR := 0x1000

# initialized data starts here
DATA_ADDR := 0x0

# emulate the "elf32lriscv" linker by default and do not link against shared libraries
LFLAGS := -m$(LINK_ABI) -static -Ttext $(TEXT_ADDR) -Tbss $(BSS_ADDR) -Tdata $(DATA_ADDR)

.PHONY: dump dump_simple single testsuite pre

# builds the entire test suite
# compilation and linking passes are split up to give better error messages
testsuite: pre $(TEST_OBJ)
	@$(TOOLCHAIN)-ld $(LFLAGS) $(TEST_OBJ) -o $(BINARY).elf
	@$(TOOLCHAIN)-objcopy $(BINARY).elf $(BINARY).hex --output-target=binary
	@echo "build completed successfully"

# need to clear out all object files because changing macros in one file may change
# the contents of others - there's probably a better way to do this
pre:
	@rm -f *.o

%.o: %.s
	@$(TOOLCHAIN)-as -mabi=$(ABI) -march=$(ARCH) -o $@ $<

# disassemble the test suite .elf using high-level register names (a0, a1, etc), pseudoinstructions,
# and colored arrows to indicate jump directions
dump:
	@$(TOOLCHAIN)-objdump -d \
		--line-numbers \
		--visualize-jumps=extended-color \
		$(BINARY).elf | tail +8

# disassemble the test suite .elf using actual register names (x1, x2, etc) and actual instructions
dump_simple:
	@$(TOOLCHAIN)-objdump -d \
		--line-numbers \
		-Mno-aliases,numeric \
		$(BINARY).elf | tail +8

clean:
	@rm -f *.o
	@rm -f testsuite.elf testsuite.hex
