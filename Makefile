RUST_SRC:=$(wildcard ./src/*)
CPP_SRC:=$(wildcard ./cpp/*.cpp)
ifeq ($(OS),Windows_NT)
PLUGIN=Release\RustChop.dll
TARGET=x86_64-pc-windows-msvc
LIB_FILE=td_rs.lib
MS_BUILD='C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe'
else
PLUGIN=cpp/build/Release/RustCHOP.plugin
LIB_FILE=libtd_rs.a
TARGET=x86_64-apple-darwin
endif
CXX_BRIDGE:=target/$(TARGET)/cxxbridge/rust/ target/$(TARGET)/cxxbridge/td-rs/
RUST_TARGET:=target/$(TARGET)/release/$(LIB_FILE)

.PHONY: build
build: $(PLUGIN)

$(PLUGIN): $(RUST_TARGET) $(CXX_BRIDGE) $(CPP_SRC)
#	cp -r $(RUST_TARGET) $(CXX_BRIDGE) ./cpp

ifeq ($(OS),Windows_NT)
	# todo: cleanup hax to rename include from cxx macro
	sed -i 's/td-rs\/src\///g' target/$(TARGET)/cxxbridge/td-rs/src/lib.rs.h
	$(MS_BUILD) /p:Configuration=Release /p:Platform=x64 /p:AdditionalDependencies=.\target\$(TARGET)\release\$(LIB_FILE)
else
	# todo: cleanup hax to rename include from cxx macro
	sed -i '' 's/td-rs\/cpp\///g' ./cpp/lib.rs.h
	sed -i '' 's/td-rs\/cpp\///g' ./cpp/lib.rs.cc
	xcodebuild -project ./cpp/RustCHOP.xcodeproj/ clean build
endif

 $(RUST_TARGET) $(CXX_BRIDGE): $(RUST_SRC)
	cargo build --release --target=$(TARGET)
