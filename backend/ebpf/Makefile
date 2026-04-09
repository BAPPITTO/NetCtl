# eBPF/XDP Build Makefile (pro edition)

CLANG ?= clang
LLVM_STRIP ?= llvm-strip
BPFTOOL ?= bpftool
VMLINUX_BTF ?= /sys/kernel/btf/vmlinux

OUTPUT := output
LIBBPF_DIR ?= /usr/include/bpf

CFLAGS := -O2 -g -target bpf -D __TARGET_ARCH_x86 -D __BPF_CORE__ \
          -I$(LIBBPF_DIR)

# Source files
SRCS := src/xdp.c src/xdp_shaper.c
OBJS := $(patsubst src/%.c,$(OUTPUT)/%.o,$(SRCS))
SKELS := $(patsubst $(OUTPUT)/%.o,$(OUTPUT)/%.skel.h,$(OBJS))

# Default target
all: $(OUTPUT) $(OBJS) $(SKELS)

# Ensure output directory exists
$(OUTPUT):
 mkdir -p $@

# Compile eBPF object
$(OUTPUT)/%.o: src/%.c | $(OUTPUT)
 $(CLANG) $(CFLAGS) -c $< -o $@
 $(LLVM_STRIP) -g $@

# Generate skeleton headers for libbpf
$(OUTPUT)/%.skel.h: $(OUTPUT)/%.o
 $(BPFTOOL) gen skeleton $< > $@

# Clean everything
clean:
 rm -rf $(OUTPUT)

.PHONY: all clean
