#pragma once
#include <cstdint>
#include <type_traits>

struct Chop;

#ifndef CXXBRIDGE1_STRUCT_Chop
#define CXXBRIDGE1_STRUCT_Chop
struct Chop final {
  ::std::uint8_t foo;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_Chop

void exec(::Chop chop) noexcept;
