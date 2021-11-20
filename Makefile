RUST_SRC:=$(wildcard ./src/*)
CPP_SRC:=$(wildcard ./cpp/*.cpp)
CXX_BRIDGE:=target/x86_64-apple-darwin/cxxbridge/td-rs/src/lib.rs.h target/x86_64-apple-darwin/cxxbridge/rust/cxx.h
RUST_TARGET:=target/x86_64-apple-darwin/release/libtd_rs.a target/x86_64-apple-darwin/cxxbridge/td-rs/src/lib.rs.cc

.PHONY: build
build: cpp/build/Release/RustCHOP.plugin

cpp/build/Release/RustCHOP.plugin: $(RUST_TARGET) $(CXX_BRIDGE) $(CPP_SRC)
	cp $(RUST_TARGET) $(CXX_BRIDGE) ./cpp
	# todo: cleanup hax to rename include from cxx macro
	sed -i '' 's/td-rs\/cpp\///g' ./cpp/lib.rs.h
	sed -i '' 's/td-rs\/cpp\///g' ./cpp/lib.rs.cc
	xcodebuild -project ./cpp/RustCHOP.xcodeproj/ clean build

 $(RUST_TARGET) $(CXX_BRIDGE): $(RUST_SRC)
	cargo build --release --target=x86_64-apple-darwin
