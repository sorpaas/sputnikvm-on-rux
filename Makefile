kernel := rux/kernel/build/$(ARCH)/libkernel.bin

cargo:
ifeq ($(version),release)
	@RUSTFLAGS="-L $(LIBCORE) -L $(LIBALLOC) -L $(LIBSTD_UNICODE)" cargo rustc --release --target $(TARGET_SPEC) --verbose
else
	@RUSTFLAGS="-L $(LIBCORE) -L $(LIBALLOC) -L $(LIBSTD_UNICODE)" cargo rustc --target $(TARGET_SPEC) --verbose
endif

kernel:
	@make -C rux/kernel build

kernel-release:
	@make -C rux/kernel version=release build

sinit-allocator: kernel-release
	@make -C sinits version=release kernel=$(shell realpath $(kernel)) test=allocator test
