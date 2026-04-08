# eBPF/XDP Build Makefile

CLANG ?= clang
LLVM_STRIP ?= llvm-strip
BPFTOOL ?= bpftool
VMLINUX_BTF ?= /sys/kernel/btf/vmlinux

# Output directory
OUTPUT := output
LIBBPF_DIR ?= /usr/include/bpf

# Source files
XDP_SRC := src/xdp.c
XDP_OBJ := $(OUTPUT)/xdp.o

all: $(XDP_OBJ)

$(OUTPUT):
	mkdir -p $@

$(XDP_OBJ): $(XDP_SRC) | $(OUTPUT)
	$(CLANG) -O2 -target bpf -D __TARGET_ARCH_x86 -D __BPF_CORE__ \
		-I/usr/include/bpf \
		-c $< -o $@
	$(LLVM_STRIP) -g $@

clean:
	rm -rf $(OUTPUT)

.PHONY: all clean
