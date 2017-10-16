test ?= $(error test target not set)
kernel ?= $(error kernel not set)
version ?= release
name := $(test)
librinit := target/$(ARCH)/$(version)/examples/lib$(name).a

include ../rux/userspace.mk

cargo:
ifeq ($(version),release)
	@RUSTFLAGS="-L $(LIBCORE) -L $(LIBALLOC) -L $(LIBSTD_UNICODE)" cargo build --release --target $(TARGET_SPEC) --example $(name)
else
	@RUSTFLAGS="-L $(LIBCORE) -L $(LIBALLOC) -L $(LIBSTD_UNICODE)" cargo build --target $(TARGET_SPEC) --example $(name)
endif

test: build
	./run.sh qemu-system-$(ARCH) -no-reboot -vnc :1 -device isa-debug-exit -kernel $(kernel) -initrd $(rinit) -serial stdio
