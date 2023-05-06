RUST_SRC:=$(wildcard ./td-rs-chop/src/*.rs)
CPP_SRC:=$(wildcard ./td-rs-chop/src/*.cpp)
ifeq ($(OS),Windows_NT)
PLUGIN=Release\RustChop.dll
TARGET=x86_64-pc-windows-msvc
LIB_FILE=td_rs_chop.lib
MS_BUILD='C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe'
else
PLUGIN=build/Release/RustCHOP.plugin
LIB_FILE=libtd_rs_chop.a
TARGET=aarch64-apple-darwin
endif
CXX_BRIDGE:=target/$(TARGET)/cxxbridge/rust/cxx.h target/$(TARGET)/cxxbridge/td-rs-chop/src/cxx.rs.h
RUST_TARGET:=target/$(TARGET)/release/$(LIB_FILE)

.PHONY: build
build: $(PLUGIN)

$(PLUGIN): $(RUST_TARGET) $(CXX_BRIDGE) $(CPP_SRC)

ifeq ($(OS),Windows_NT)
	# todo: cleanup hax to rename include from cxx macro
	sed -i 's/td-rs\/src\///g' target/$(TARGET)/cxxbridge/td-rs/src/lib.rs.h
	$(MS_BUILD) /p:Configuration=Release /p:Platform=x64 /p:AdditionalDependencies=.\target\$(TARGET)\release\$(LIB_FILE)
else
	# todo: cleanup hax to rename include from cxx macro
	sed -i '' 's/td-rs-chop\/src\///g' $(shell readlink target/$(TARGET)/cxxbridge/td-rs-chop/src/cxx.rs.h)
	sed -i '' 's/td-rs-chop\/src\///g' $(shell readlink target/$(TARGET)/cxxbridge/td-rs-chop/src/cxx.rs.cc)
	xcodebuild -project ./RustCHOP.xcodeproj/ clean build
endif

 $(RUST_TARGET) $(CXX_BRIDGE): $(RUST_SRC)
	cargo build --package=td-rs-chop --release --target=$(TARGET)
	cargo build --package=sin --release --target=$(TARGET)
