#include "BoxDynChop.h"
#include <algorithm>
#include <array>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <initializer_list>
#include <iterator>
#include <new>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_PANIC
#define CXXBRIDGE1_PANIC
template <typename Exception>
void panic [[noreturn]] (const char *msg);
#endif // CXXBRIDGE1_PANIC

struct unsafe_bitcopy_t;

namespace {
template <typename T>
class impl;
} // namespace

class Opaque;

template <typename T>
::std::size_t size_of();
template <typename T>
::std::size_t align_of();

#ifndef CXXBRIDGE1_RUST_STRING
#define CXXBRIDGE1_RUST_STRING
class String final {
public:
  String() noexcept;
  String(const String &) noexcept;
  String(String &&) noexcept;
  ~String() noexcept;

  String(const std::string &);
  String(const char *);
  String(const char *, std::size_t);
  String(const char16_t *);
  String(const char16_t *, std::size_t);

  String &operator=(const String &) &noexcept;
  String &operator=(String &&) &noexcept;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  const char *c_str() noexcept;

  std::size_t capacity() const noexcept;
  void reserve(size_t new_cap) noexcept;

  using iterator = char *;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const String &) const noexcept;
  bool operator!=(const String &) const noexcept;
  bool operator<(const String &) const noexcept;
  bool operator<=(const String &) const noexcept;
  bool operator>(const String &) const noexcept;
  bool operator>=(const String &) const noexcept;

  void swap(String &) noexcept;

  String(unsafe_bitcopy_t, const String &) noexcept;

private:
  friend void swap(String &lhs, String &rhs) noexcept { lhs.swap(rhs); }

  std::array<std::uintptr_t, 3> repr;
};
#endif // CXXBRIDGE1_RUST_STRING

#ifndef CXXBRIDGE1_RUST_SLICE
#define CXXBRIDGE1_RUST_SLICE
namespace detail {
template <bool>
struct copy_assignable_if {};

template <>
struct copy_assignable_if<false> {
  copy_assignable_if() noexcept = default;
  copy_assignable_if(const copy_assignable_if &) noexcept = default;
  copy_assignable_if &operator=(const copy_assignable_if &) &noexcept = delete;
  copy_assignable_if &operator=(copy_assignable_if &&) &noexcept = default;
};
} // namespace detail

template <typename T>
class Slice final
    : private detail::copy_assignable_if<std::is_const<T>::value> {
public:
  using value_type = T;

  Slice() noexcept;
  Slice(T *, std::size_t count) noexcept;

  Slice &operator=(const Slice<T> &) &noexcept = default;
  Slice &operator=(Slice<T> &&) &noexcept = default;

  T *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  T &operator[](std::size_t n) const noexcept;
  T &at(std::size_t n) const;
  T &front() const noexcept;
  T &back() const noexcept;

  Slice(const Slice<T> &) noexcept = default;
  ~Slice() noexcept = default;

  class iterator;
  iterator begin() const noexcept;
  iterator end() const noexcept;

  void swap(Slice &) noexcept;

private:
  class uninit;
  Slice(uninit) noexcept;
  friend impl<Slice>;
  friend void sliceInit(void *, const void *, std::size_t) noexcept;
  friend void *slicePtr(const void *) noexcept;
  friend std::size_t sliceLen(const void *) noexcept;

  std::array<std::uintptr_t, 2> repr;
};

template <typename T>
class Slice<T>::iterator final {
public:
  using iterator_category = std::random_access_iterator_tag;
  using value_type = T;
  using difference_type = std::ptrdiff_t;
  using pointer = typename std::add_pointer<T>::type;
  using reference = typename std::add_lvalue_reference<T>::type;

  reference operator*() const noexcept;
  pointer operator->() const noexcept;
  reference operator[](difference_type) const noexcept;

  iterator &operator++() noexcept;
  iterator operator++(int) noexcept;
  iterator &operator--() noexcept;
  iterator operator--(int) noexcept;

  iterator &operator+=(difference_type) noexcept;
  iterator &operator-=(difference_type) noexcept;
  iterator operator+(difference_type) const noexcept;
  iterator operator-(difference_type) const noexcept;
  difference_type operator-(const iterator &) const noexcept;

  bool operator==(const iterator &) const noexcept;
  bool operator!=(const iterator &) const noexcept;
  bool operator<(const iterator &) const noexcept;
  bool operator<=(const iterator &) const noexcept;
  bool operator>(const iterator &) const noexcept;
  bool operator>=(const iterator &) const noexcept;

private:
  friend class Slice;
  void *pos;
  std::size_t stride;
};

template <typename T>
Slice<T>::Slice() noexcept {
  sliceInit(this, reinterpret_cast<void *>(align_of<T>()), 0);
}

template <typename T>
Slice<T>::Slice(T *s, std::size_t count) noexcept {
  assert(s != nullptr || count == 0);
  sliceInit(this,
            s == nullptr && count == 0
                ? reinterpret_cast<void *>(align_of<T>())
                : const_cast<typename std::remove_const<T>::type *>(s),
            count);
}

template <typename T>
T *Slice<T>::data() const noexcept {
  return reinterpret_cast<T *>(slicePtr(this));
}

template <typename T>
std::size_t Slice<T>::size() const noexcept {
  return sliceLen(this);
}

template <typename T>
std::size_t Slice<T>::length() const noexcept {
  return this->size();
}

template <typename T>
bool Slice<T>::empty() const noexcept {
  return this->size() == 0;
}

template <typename T>
T &Slice<T>::operator[](std::size_t n) const noexcept {
  assert(n < this->size());
  auto ptr = static_cast<char *>(slicePtr(this)) + size_of<T>() * n;
  return *reinterpret_cast<T *>(ptr);
}

template <typename T>
T &Slice<T>::at(std::size_t n) const {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Slice index out of range");
  }
  return (*this)[n];
}

template <typename T>
T &Slice<T>::front() const noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
T &Slice<T>::back() const noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
typename Slice<T>::iterator::reference
Slice<T>::iterator::operator*() const noexcept {
  return *static_cast<T *>(this->pos);
}

template <typename T>
typename Slice<T>::iterator::pointer
Slice<T>::iterator::operator->() const noexcept {
  return static_cast<T *>(this->pos);
}

template <typename T>
typename Slice<T>::iterator::reference Slice<T>::iterator::operator[](
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ptr = static_cast<char *>(this->pos) + this->stride * n;
  return *reinterpret_cast<T *>(ptr);
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator++() noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator++(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return ret;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator--() noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator--(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return ret;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator+=(
    typename Slice<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride * n;
  return *this;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator-=(
    typename Slice<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride * n;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator+(
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) + this->stride * n;
  return ret;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator-(
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) - this->stride * n;
  return ret;
}

template <typename T>
typename Slice<T>::iterator::difference_type
Slice<T>::iterator::operator-(const iterator &other) const noexcept {
  auto diff = std::distance(static_cast<char *>(other.pos),
                            static_cast<char *>(this->pos));
  return diff / this->stride;
}

template <typename T>
bool Slice<T>::iterator::operator==(const iterator &other) const noexcept {
  return this->pos == other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator!=(const iterator &other) const noexcept {
  return this->pos != other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator<(const iterator &other) const noexcept {
  return this->pos < other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator<=(const iterator &other) const noexcept {
  return this->pos <= other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator>(const iterator &other) const noexcept {
  return this->pos > other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator>=(const iterator &other) const noexcept {
  return this->pos >= other.pos;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::begin() const noexcept {
  iterator it;
  it.pos = slicePtr(this);
  it.stride = size_of<T>();
  return it;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::end() const noexcept {
  iterator it = this->begin();
  it.pos = static_cast<char *>(it.pos) + it.stride * this->size();
  return it;
}

template <typename T>
void Slice<T>::swap(Slice &rhs) noexcept {
  std::swap(*this, rhs);
}
#endif // CXXBRIDGE1_RUST_SLICE

#ifndef CXXBRIDGE1_RUST_BITCOPY_T
#define CXXBRIDGE1_RUST_BITCOPY_T
struct unsafe_bitcopy_t final {
  explicit unsafe_bitcopy_t() = default;
};
#endif // CXXBRIDGE1_RUST_BITCOPY_T

#ifndef CXXBRIDGE1_RUST_VEC
#define CXXBRIDGE1_RUST_VEC
template <typename T>
class Vec final {
public:
  using value_type = T;

  Vec() noexcept;
  Vec(std::initializer_list<T>);
  Vec(const Vec &);
  Vec(Vec &&) noexcept;
  ~Vec() noexcept;

  Vec &operator=(Vec &&) &noexcept;
  Vec &operator=(const Vec &) &;

  std::size_t size() const noexcept;
  bool empty() const noexcept;
  const T *data() const noexcept;
  T *data() noexcept;
  std::size_t capacity() const noexcept;

  const T &operator[](std::size_t n) const noexcept;
  const T &at(std::size_t n) const;
  const T &front() const noexcept;
  const T &back() const noexcept;

  T &operator[](std::size_t n) noexcept;
  T &at(std::size_t n);
  T &front() noexcept;
  T &back() noexcept;

  void reserve(std::size_t new_cap);
  void push_back(const T &value);
  void push_back(T &&value);
  template <typename... Args>
  void emplace_back(Args &&...args);

  using iterator = typename Slice<T>::iterator;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = typename Slice<const T>::iterator;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  void swap(Vec &) noexcept;

  Vec(unsafe_bitcopy_t, const Vec &) noexcept;

private:
  void reserve_total(std::size_t new_cap) noexcept;
  void set_len(std::size_t len) noexcept;
  void drop() noexcept;

  friend void swap(Vec &lhs, Vec &rhs) noexcept { lhs.swap(rhs); }

  std::array<std::uintptr_t, 3> repr;
};

template <typename T>
Vec<T>::Vec(std::initializer_list<T> init) : Vec{} {
  this->reserve_total(init.size());
  std::move(init.begin(), init.end(), std::back_inserter(*this));
}

template <typename T>
Vec<T>::Vec(const Vec &other) : Vec() {
  this->reserve_total(other.size());
  std::copy(other.begin(), other.end(), std::back_inserter(*this));
}

template <typename T>
Vec<T>::Vec(Vec &&other) noexcept : repr(other.repr) {
  new (&other) Vec();
}

template <typename T>
Vec<T>::~Vec() noexcept {
  this->drop();
}

template <typename T>
Vec<T> &Vec<T>::operator=(Vec &&other) &noexcept {
  this->drop();
  this->repr = other.repr;
  new (&other) Vec();
  return *this;
}

template <typename T>
Vec<T> &Vec<T>::operator=(const Vec &other) & {
  if (this != &other) {
    this->drop();
    new (this) Vec(other);
  }
  return *this;
}

template <typename T>
bool Vec<T>::empty() const noexcept {
  return this->size() == 0;
}

template <typename T>
T *Vec<T>::data() noexcept {
  return const_cast<T *>(const_cast<const Vec<T> *>(this)->data());
}

template <typename T>
const T &Vec<T>::operator[](std::size_t n) const noexcept {
  assert(n < this->size());
  auto data = reinterpret_cast<const char *>(this->data());
  return *reinterpret_cast<const T *>(data + n * size_of<T>());
}

template <typename T>
const T &Vec<T>::at(std::size_t n) const {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
const T &Vec<T>::front() const noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
const T &Vec<T>::back() const noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
T &Vec<T>::operator[](std::size_t n) noexcept {
  assert(n < this->size());
  auto data = reinterpret_cast<char *>(this->data());
  return *reinterpret_cast<T *>(data + n * size_of<T>());
}

template <typename T>
T &Vec<T>::at(std::size_t n) {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
T &Vec<T>::front() noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
T &Vec<T>::back() noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
void Vec<T>::reserve(std::size_t new_cap) {
  this->reserve_total(new_cap);
}

template <typename T>
void Vec<T>::push_back(const T &value) {
  this->emplace_back(value);
}

template <typename T>
void Vec<T>::push_back(T &&value) {
  this->emplace_back(std::move(value));
}

template <typename T>
template <typename... Args>
void Vec<T>::emplace_back(Args &&...args) {
  auto size = this->size();
  this->reserve_total(size + 1);
  ::new (reinterpret_cast<T *>(reinterpret_cast<char *>(this->data()) +
                               size * size_of<T>()))
      T(std::forward<Args>(args)...);
  this->set_len(size + 1);
}

template <typename T>
typename Vec<T>::iterator Vec<T>::begin() noexcept {
  return Slice<T>(this->data(), this->size()).begin();
}

template <typename T>
typename Vec<T>::iterator Vec<T>::end() noexcept {
  return Slice<T>(this->data(), this->size()).end();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::begin() const noexcept {
  return this->cbegin();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::end() const noexcept {
  return this->cend();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::cbegin() const noexcept {
  return Slice<const T>(this->data(), this->size()).begin();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::cend() const noexcept {
  return Slice<const T>(this->data(), this->size()).end();
}

template <typename T>
void Vec<T>::swap(Vec &rhs) noexcept {
  using std::swap;
  swap(this->repr, rhs.repr);
}

template <typename T>
Vec<T>::Vec(unsafe_bitcopy_t, const Vec &bits) noexcept : repr(bits.repr) {}
#endif // CXXBRIDGE1_RUST_VEC

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

#ifndef CXXBRIDGE1_LAYOUT
#define CXXBRIDGE1_LAYOUT
class layout {
  template <typename T>
  friend std::size_t size_of();
  template <typename T>
  friend std::size_t align_of();
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return T::layout::size();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return sizeof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      size_of() {
    return do_size_of<T>();
  }
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return T::layout::align();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return alignof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      align_of() {
    return do_align_of<T>();
  }
};

template <typename T>
std::size_t size_of() {
  return layout::size_of<T>();
}

template <typename T>
std::size_t align_of() {
  return layout::align_of<T>();
}
#endif // CXXBRIDGE1_LAYOUT

#ifndef CXXBRIDGE1_RELOCATABLE
#define CXXBRIDGE1_RELOCATABLE
namespace detail {
template <typename... Ts>
struct make_void {
  using type = void;
};

template <typename... Ts>
using void_t = typename make_void<Ts...>::type;

template <typename Void, template <typename...> class, typename...>
struct detect : std::false_type {};
template <template <typename...> class T, typename... A>
struct detect<void_t<T<A...>>, T, A...> : std::true_type {};

template <template <typename...> class T, typename... A>
using is_detected = detect<void, T, A...>;

template <typename T>
using detect_IsRelocatable = typename T::IsRelocatable;

template <typename T>
struct get_IsRelocatable
    : std::is_same<typename T::IsRelocatable, std::true_type> {};
} // namespace detail

template <typename T>
struct IsRelocatable
    : std::conditional<
          detail::is_detected<detail::detect_IsRelocatable, T>::value,
          detail::get_IsRelocatable<T>,
          std::integral_constant<
              bool, std::is_trivially_move_constructible<T>::value &&
                        std::is_trivially_destructible<T>::value>>::type {};
#endif // CXXBRIDGE1_RELOCATABLE

namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union ManuallyDrop {
  T value;
  ManuallyDrop(T &&value) : value(::std::move(value)) {}
  ~ManuallyDrop() {}
};

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};
} // namespace cxxbridge1
} // namespace rust

struct NumericParameter;
struct PuleParameter;
struct StringParameter;
struct OperatorInfo;
struct ChopParams;
struct ChopChannel;
struct ParamValue;
struct ChopOperatorInputs;
struct ChopOperatorInput;
struct ChopOutputInfo;
struct ChopOutput;
struct ChopInfoChan;
struct ChopInfoDatSize;
struct ChopInfoDatEntries;
struct ChopGeneralInfo;

#ifndef CXXBRIDGE1_STRUCT_NumericParameter
#define CXXBRIDGE1_STRUCT_NumericParameter
struct NumericParameter final {
  ::rust::String name;
  ::rust::String label;
  ::rust::String page;
  ::std::array<double, 4> default_values;
  ::std::array<double, 4> min_values;
  ::std::array<double, 4> max_values;
  ::std::array<bool, 4> clamp_mins;
  ::std::array<bool, 4> clamp_maxes;
  ::std::array<double, 4> min_sliders;
  ::std::array<double, 4> max_sliders;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_NumericParameter

#ifndef CXXBRIDGE1_STRUCT_PuleParameter
#define CXXBRIDGE1_STRUCT_PuleParameter
struct PuleParameter final {
  ::rust::String name;
  ::rust::String label;
  ::rust::String page;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_PuleParameter

#ifndef CXXBRIDGE1_STRUCT_StringParameter
#define CXXBRIDGE1_STRUCT_StringParameter
struct StringParameter final {
  ::rust::String name;
  ::rust::String label;
  ::rust::String page;
  ::rust::String default_value;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_StringParameter

#ifndef CXXBRIDGE1_STRUCT_OperatorInfo
#define CXXBRIDGE1_STRUCT_OperatorInfo
struct OperatorInfo final {
  ::rust::String operator_type;
  ::rust::String operator_label;
  ::rust::String operator_icon;
  ::std::int32_t min_inputs;
  ::std::int32_t max_inputs;
  ::rust::String author_name;
  ::rust::String author_email;
  ::std::int32_t major_version;
  ::std::int32_t minor_version;
  ::rust::String python_version;
  bool cook_on_start;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_OperatorInfo

#ifndef CXXBRIDGE1_STRUCT_ChopParams
#define CXXBRIDGE1_STRUCT_ChopParams
struct ChopParams final {
  ::rust::Vec<::NumericParameter> numeric_params;
  ::rust::Vec<::StringParameter> string_params;
  ::rust::Vec<::PuleParameter> pulse_params;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopParams

#ifndef CXXBRIDGE1_STRUCT_ChopChannel
#define CXXBRIDGE1_STRUCT_ChopChannel
struct ChopChannel final {
  ::rust::Vec<float> data;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopChannel

#ifndef CXXBRIDGE1_STRUCT_ParamValue
#define CXXBRIDGE1_STRUCT_ParamValue
struct ParamValue final {
  ::rust::String name;
  ::rust::String str_value;
  double double_value;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ParamValue

#ifndef CXXBRIDGE1_STRUCT_ChopOperatorInputs
#define CXXBRIDGE1_STRUCT_ChopOperatorInputs
struct ChopOperatorInputs final {
  ::std::int32_t num_inputs;
  ::rust::Vec<::ChopOperatorInput> inputs;
  ::rust::Vec<::ParamValue> params;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopOperatorInputs

#ifndef CXXBRIDGE1_STRUCT_ChopOperatorInput
#define CXXBRIDGE1_STRUCT_ChopOperatorInput
struct ChopOperatorInput final {
  ::rust::String path;
  ::std::uint32_t id;
  ::std::uint32_t num_channels;
  ::std::uint32_t num_samples;
  double sample_rate;
  double start_index;
  ::rust::Vec<::ChopChannel> channels;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopOperatorInput

#ifndef CXXBRIDGE1_STRUCT_ChopOutputInfo
#define CXXBRIDGE1_STRUCT_ChopOutputInfo
struct ChopOutputInfo final {
  ::std::uint32_t num_channels;
  ::std::uint32_t num_samples;
  double sample_rate;
  ::std::size_t start_index;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopOutputInfo

#ifndef CXXBRIDGE1_STRUCT_ChopOutput
#define CXXBRIDGE1_STRUCT_ChopOutput
struct ChopOutput final {
  ::rust::Vec<::ChopChannel> channels;
  ::std::int32_t num_channels;
  ::std::int32_t num_samples;
  ::std::int32_t sample_rate;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopOutput

#ifndef CXXBRIDGE1_STRUCT_ChopInfoChan
#define CXXBRIDGE1_STRUCT_ChopInfoChan
struct ChopInfoChan final {
  ::rust::String name;
  float value;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopInfoChan

#ifndef CXXBRIDGE1_STRUCT_ChopInfoDatSize
#define CXXBRIDGE1_STRUCT_ChopInfoDatSize
struct ChopInfoDatSize final {
  ::std::int32_t rows;
  ::std::int32_t columns;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopInfoDatSize

#ifndef CXXBRIDGE1_STRUCT_ChopInfoDatEntries
#define CXXBRIDGE1_STRUCT_ChopInfoDatEntries
struct ChopInfoDatEntries final {
  ::rust::Vec<::rust::String> values;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopInfoDatEntries

#ifndef CXXBRIDGE1_STRUCT_ChopGeneralInfo
#define CXXBRIDGE1_STRUCT_ChopGeneralInfo
struct ChopGeneralInfo final {
  bool cook_every_frame;
  bool cook_every_frame_if_asked;
  bool timeslice;
  ::std::int32_t input_match_index;

  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_ChopGeneralInfo

static_assert(
    ::rust::IsRelocatable<::BoxDynChop>::value,
    "type BoxDynChop should be trivially move constructible and trivially destructible in C++ to be used as a return value of `chop_new` or non-pinned mutable reference in signature of `chop_get_params`, `chop_on_reset`, `chop_get_output_info` in Rust");
static_assert(
    ::rust::IsRelocatable<::PtrBoxDynChop>::value,
    "type PtrBoxDynChop should be trivially move constructible and trivially destructible in C++ to be used as an argument of `dyn_chop_drop_in_place` in Rust");

extern "C" {
void cxxbridge1$dyn_chop_drop_in_place(::PtrBoxDynChop *ptr) noexcept;

void cxxbridge1$chop_get_operator_info(::OperatorInfo *return$) noexcept;

void cxxbridge1$chop_get_params(::BoxDynChop &chop, ::ChopParams *return$) noexcept;

void cxxbridge1$chop_on_reset(::BoxDynChop &chop) noexcept;

::std::int32_t cxxbridge1$chop_get_num_info_chop_chans(const ::BoxDynChop &chop) noexcept;

void cxxbridge1$chop_get_info_chop_chan(const ::BoxDynChop &chop, ::std::int32_t index, ::ChopInfoChan *return$) noexcept;

bool cxxbridge1$chop_get_output_info(::BoxDynChop &chop, ::ChopOutputInfo &info, const ::ChopOperatorInputs &inputs) noexcept;

void cxxbridge1$chop_get_channel_name(const ::BoxDynChop &chop, ::std::int32_t index, const ::ChopOperatorInputs &inputs, ::rust::String *return$) noexcept;

bool cxxbridge1$chop_get_info_dat_size(const ::BoxDynChop &chop, ::ChopInfoDatSize &size) noexcept;

void cxxbridge1$chop_get_info_dat_entries(const ::BoxDynChop &chop, ::std::int32_t index, ::std::int32_t num_entries, ::ChopInfoDatEntries &entries) noexcept;

void cxxbridge1$chop_execute(::BoxDynChop &chop, ::ChopOutput &output, const ::ChopOperatorInputs &inputs) noexcept;

::ChopGeneralInfo cxxbridge1$chop_get_general_info(const ::BoxDynChop &chop) noexcept;

void cxxbridge1$chop_get_info(const ::BoxDynChop &chop, ::rust::String *return$) noexcept;

void cxxbridge1$chop_get_warning(const ::BoxDynChop &chop, ::rust::String *return$) noexcept;

void cxxbridge1$chop_get_error(const ::BoxDynChop &chop, ::rust::String *return$) noexcept;

void cxxbridge1$chop_new(::BoxDynChop *return$) noexcept;
} // extern "C"

void dyn_chop_drop_in_place(::PtrBoxDynChop ptr) noexcept {
  ::rust::ManuallyDrop<::PtrBoxDynChop> ptr$(::std::move(ptr));
  cxxbridge1$dyn_chop_drop_in_place(&ptr$.value);
}

::OperatorInfo chop_get_operator_info() noexcept {
  ::rust::MaybeUninit<::OperatorInfo> return$;
  cxxbridge1$chop_get_operator_info(&return$.value);
  return ::std::move(return$.value);
}

::ChopParams chop_get_params(::BoxDynChop &chop) noexcept {
  ::rust::MaybeUninit<::ChopParams> return$;
  cxxbridge1$chop_get_params(chop, &return$.value);
  return ::std::move(return$.value);
}

void chop_on_reset(::BoxDynChop &chop) noexcept {
  cxxbridge1$chop_on_reset(chop);
}

::std::int32_t chop_get_num_info_chop_chans(const ::BoxDynChop &chop) noexcept {
  return cxxbridge1$chop_get_num_info_chop_chans(chop);
}

::ChopInfoChan chop_get_info_chop_chan(const ::BoxDynChop &chop, ::std::int32_t index) noexcept {
  ::rust::MaybeUninit<::ChopInfoChan> return$;
  cxxbridge1$chop_get_info_chop_chan(chop, index, &return$.value);
  return ::std::move(return$.value);
}

bool chop_get_output_info(::BoxDynChop &chop, ::ChopOutputInfo &info, const ::ChopOperatorInputs &inputs) noexcept {
  return cxxbridge1$chop_get_output_info(chop, info, inputs);
}

::rust::String chop_get_channel_name(const ::BoxDynChop &chop, ::std::int32_t index, const ::ChopOperatorInputs &inputs) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  cxxbridge1$chop_get_channel_name(chop, index, inputs, &return$.value);
  return ::std::move(return$.value);
}

bool chop_get_info_dat_size(const ::BoxDynChop &chop, ::ChopInfoDatSize &size) noexcept {
  return cxxbridge1$chop_get_info_dat_size(chop, size);
}

void chop_get_info_dat_entries(const ::BoxDynChop &chop, ::std::int32_t index, ::std::int32_t num_entries, ::ChopInfoDatEntries &entries) noexcept {
  cxxbridge1$chop_get_info_dat_entries(chop, index, num_entries, entries);
}

void chop_execute(::BoxDynChop &chop, ::ChopOutput &output, const ::ChopOperatorInputs &inputs) noexcept {
  cxxbridge1$chop_execute(chop, output, inputs);
}

::ChopGeneralInfo chop_get_general_info(const ::BoxDynChop &chop) noexcept {
  return cxxbridge1$chop_get_general_info(chop);
}

::rust::String chop_get_info(const ::BoxDynChop &chop) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  cxxbridge1$chop_get_info(chop, &return$.value);
  return ::std::move(return$.value);
}

::rust::String chop_get_warning(const ::BoxDynChop &chop) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  cxxbridge1$chop_get_warning(chop, &return$.value);
  return ::std::move(return$.value);
}

::rust::String chop_get_error(const ::BoxDynChop &chop) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  cxxbridge1$chop_get_error(chop, &return$.value);
  return ::std::move(return$.value);
}

::BoxDynChop chop_new() noexcept {
  ::rust::MaybeUninit<::BoxDynChop> return$;
  cxxbridge1$chop_new(&return$.value);
  return ::std::move(return$.value);
}

extern "C" {
void cxxbridge1$rust_vec$NumericParameter$new(const ::rust::Vec<::NumericParameter> *ptr) noexcept;
void cxxbridge1$rust_vec$NumericParameter$drop(::rust::Vec<::NumericParameter> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$NumericParameter$len(const ::rust::Vec<::NumericParameter> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$NumericParameter$capacity(const ::rust::Vec<::NumericParameter> *ptr) noexcept;
const ::NumericParameter *cxxbridge1$rust_vec$NumericParameter$data(const ::rust::Vec<::NumericParameter> *ptr) noexcept;
void cxxbridge1$rust_vec$NumericParameter$reserve_total(::rust::Vec<::NumericParameter> *ptr, ::std::size_t new_cap) noexcept;
void cxxbridge1$rust_vec$NumericParameter$set_len(::rust::Vec<::NumericParameter> *ptr, ::std::size_t len) noexcept;

void cxxbridge1$rust_vec$StringParameter$new(const ::rust::Vec<::StringParameter> *ptr) noexcept;
void cxxbridge1$rust_vec$StringParameter$drop(::rust::Vec<::StringParameter> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$StringParameter$len(const ::rust::Vec<::StringParameter> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$StringParameter$capacity(const ::rust::Vec<::StringParameter> *ptr) noexcept;
const ::StringParameter *cxxbridge1$rust_vec$StringParameter$data(const ::rust::Vec<::StringParameter> *ptr) noexcept;
void cxxbridge1$rust_vec$StringParameter$reserve_total(::rust::Vec<::StringParameter> *ptr, ::std::size_t new_cap) noexcept;
void cxxbridge1$rust_vec$StringParameter$set_len(::rust::Vec<::StringParameter> *ptr, ::std::size_t len) noexcept;

void cxxbridge1$rust_vec$PuleParameter$new(const ::rust::Vec<::PuleParameter> *ptr) noexcept;
void cxxbridge1$rust_vec$PuleParameter$drop(::rust::Vec<::PuleParameter> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$PuleParameter$len(const ::rust::Vec<::PuleParameter> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$PuleParameter$capacity(const ::rust::Vec<::PuleParameter> *ptr) noexcept;
const ::PuleParameter *cxxbridge1$rust_vec$PuleParameter$data(const ::rust::Vec<::PuleParameter> *ptr) noexcept;
void cxxbridge1$rust_vec$PuleParameter$reserve_total(::rust::Vec<::PuleParameter> *ptr, ::std::size_t new_cap) noexcept;
void cxxbridge1$rust_vec$PuleParameter$set_len(::rust::Vec<::PuleParameter> *ptr, ::std::size_t len) noexcept;

void cxxbridge1$rust_vec$ChopOperatorInput$new(const ::rust::Vec<::ChopOperatorInput> *ptr) noexcept;
void cxxbridge1$rust_vec$ChopOperatorInput$drop(::rust::Vec<::ChopOperatorInput> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$ChopOperatorInput$len(const ::rust::Vec<::ChopOperatorInput> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$ChopOperatorInput$capacity(const ::rust::Vec<::ChopOperatorInput> *ptr) noexcept;
const ::ChopOperatorInput *cxxbridge1$rust_vec$ChopOperatorInput$data(const ::rust::Vec<::ChopOperatorInput> *ptr) noexcept;
void cxxbridge1$rust_vec$ChopOperatorInput$reserve_total(::rust::Vec<::ChopOperatorInput> *ptr, ::std::size_t new_cap) noexcept;
void cxxbridge1$rust_vec$ChopOperatorInput$set_len(::rust::Vec<::ChopOperatorInput> *ptr, ::std::size_t len) noexcept;

void cxxbridge1$rust_vec$ParamValue$new(const ::rust::Vec<::ParamValue> *ptr) noexcept;
void cxxbridge1$rust_vec$ParamValue$drop(::rust::Vec<::ParamValue> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$ParamValue$len(const ::rust::Vec<::ParamValue> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$ParamValue$capacity(const ::rust::Vec<::ParamValue> *ptr) noexcept;
const ::ParamValue *cxxbridge1$rust_vec$ParamValue$data(const ::rust::Vec<::ParamValue> *ptr) noexcept;
void cxxbridge1$rust_vec$ParamValue$reserve_total(::rust::Vec<::ParamValue> *ptr, ::std::size_t new_cap) noexcept;
void cxxbridge1$rust_vec$ParamValue$set_len(::rust::Vec<::ParamValue> *ptr, ::std::size_t len) noexcept;

void cxxbridge1$rust_vec$ChopChannel$new(const ::rust::Vec<::ChopChannel> *ptr) noexcept;
void cxxbridge1$rust_vec$ChopChannel$drop(::rust::Vec<::ChopChannel> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$ChopChannel$len(const ::rust::Vec<::ChopChannel> *ptr) noexcept;
::std::size_t cxxbridge1$rust_vec$ChopChannel$capacity(const ::rust::Vec<::ChopChannel> *ptr) noexcept;
const ::ChopChannel *cxxbridge1$rust_vec$ChopChannel$data(const ::rust::Vec<::ChopChannel> *ptr) noexcept;
void cxxbridge1$rust_vec$ChopChannel$reserve_total(::rust::Vec<::ChopChannel> *ptr, ::std::size_t new_cap) noexcept;
void cxxbridge1$rust_vec$ChopChannel$set_len(::rust::Vec<::ChopChannel> *ptr, ::std::size_t len) noexcept;
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
Vec<::NumericParameter>::Vec() noexcept {
  cxxbridge1$rust_vec$NumericParameter$new(this);
}
template <>
void Vec<::NumericParameter>::drop() noexcept {
  return cxxbridge1$rust_vec$NumericParameter$drop(this);
}
template <>
::std::size_t Vec<::NumericParameter>::size() const noexcept {
  return cxxbridge1$rust_vec$NumericParameter$len(this);
}
template <>
::std::size_t Vec<::NumericParameter>::capacity() const noexcept {
  return cxxbridge1$rust_vec$NumericParameter$capacity(this);
}
template <>
const ::NumericParameter *Vec<::NumericParameter>::data() const noexcept {
  return cxxbridge1$rust_vec$NumericParameter$data(this);
}
template <>
void Vec<::NumericParameter>::reserve_total(::std::size_t new_cap) noexcept {
  return cxxbridge1$rust_vec$NumericParameter$reserve_total(this, new_cap);
}
template <>
void Vec<::NumericParameter>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$NumericParameter$set_len(this, len);
}
template <>
Vec<::StringParameter>::Vec() noexcept {
  cxxbridge1$rust_vec$StringParameter$new(this);
}
template <>
void Vec<::StringParameter>::drop() noexcept {
  return cxxbridge1$rust_vec$StringParameter$drop(this);
}
template <>
::std::size_t Vec<::StringParameter>::size() const noexcept {
  return cxxbridge1$rust_vec$StringParameter$len(this);
}
template <>
::std::size_t Vec<::StringParameter>::capacity() const noexcept {
  return cxxbridge1$rust_vec$StringParameter$capacity(this);
}
template <>
const ::StringParameter *Vec<::StringParameter>::data() const noexcept {
  return cxxbridge1$rust_vec$StringParameter$data(this);
}
template <>
void Vec<::StringParameter>::reserve_total(::std::size_t new_cap) noexcept {
  return cxxbridge1$rust_vec$StringParameter$reserve_total(this, new_cap);
}
template <>
void Vec<::StringParameter>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$StringParameter$set_len(this, len);
}
template <>
Vec<::PuleParameter>::Vec() noexcept {
  cxxbridge1$rust_vec$PuleParameter$new(this);
}
template <>
void Vec<::PuleParameter>::drop() noexcept {
  return cxxbridge1$rust_vec$PuleParameter$drop(this);
}
template <>
::std::size_t Vec<::PuleParameter>::size() const noexcept {
  return cxxbridge1$rust_vec$PuleParameter$len(this);
}
template <>
::std::size_t Vec<::PuleParameter>::capacity() const noexcept {
  return cxxbridge1$rust_vec$PuleParameter$capacity(this);
}
template <>
const ::PuleParameter *Vec<::PuleParameter>::data() const noexcept {
  return cxxbridge1$rust_vec$PuleParameter$data(this);
}
template <>
void Vec<::PuleParameter>::reserve_total(::std::size_t new_cap) noexcept {
  return cxxbridge1$rust_vec$PuleParameter$reserve_total(this, new_cap);
}
template <>
void Vec<::PuleParameter>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$PuleParameter$set_len(this, len);
}
template <>
Vec<::ChopOperatorInput>::Vec() noexcept {
  cxxbridge1$rust_vec$ChopOperatorInput$new(this);
}
template <>
void Vec<::ChopOperatorInput>::drop() noexcept {
  return cxxbridge1$rust_vec$ChopOperatorInput$drop(this);
}
template <>
::std::size_t Vec<::ChopOperatorInput>::size() const noexcept {
  return cxxbridge1$rust_vec$ChopOperatorInput$len(this);
}
template <>
::std::size_t Vec<::ChopOperatorInput>::capacity() const noexcept {
  return cxxbridge1$rust_vec$ChopOperatorInput$capacity(this);
}
template <>
const ::ChopOperatorInput *Vec<::ChopOperatorInput>::data() const noexcept {
  return cxxbridge1$rust_vec$ChopOperatorInput$data(this);
}
template <>
void Vec<::ChopOperatorInput>::reserve_total(::std::size_t new_cap) noexcept {
  return cxxbridge1$rust_vec$ChopOperatorInput$reserve_total(this, new_cap);
}
template <>
void Vec<::ChopOperatorInput>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$ChopOperatorInput$set_len(this, len);
}
template <>
Vec<::ParamValue>::Vec() noexcept {
  cxxbridge1$rust_vec$ParamValue$new(this);
}
template <>
void Vec<::ParamValue>::drop() noexcept {
  return cxxbridge1$rust_vec$ParamValue$drop(this);
}
template <>
::std::size_t Vec<::ParamValue>::size() const noexcept {
  return cxxbridge1$rust_vec$ParamValue$len(this);
}
template <>
::std::size_t Vec<::ParamValue>::capacity() const noexcept {
  return cxxbridge1$rust_vec$ParamValue$capacity(this);
}
template <>
const ::ParamValue *Vec<::ParamValue>::data() const noexcept {
  return cxxbridge1$rust_vec$ParamValue$data(this);
}
template <>
void Vec<::ParamValue>::reserve_total(::std::size_t new_cap) noexcept {
  return cxxbridge1$rust_vec$ParamValue$reserve_total(this, new_cap);
}
template <>
void Vec<::ParamValue>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$ParamValue$set_len(this, len);
}
template <>
Vec<::ChopChannel>::Vec() noexcept {
  cxxbridge1$rust_vec$ChopChannel$new(this);
}
template <>
void Vec<::ChopChannel>::drop() noexcept {
  return cxxbridge1$rust_vec$ChopChannel$drop(this);
}
template <>
::std::size_t Vec<::ChopChannel>::size() const noexcept {
  return cxxbridge1$rust_vec$ChopChannel$len(this);
}
template <>
::std::size_t Vec<::ChopChannel>::capacity() const noexcept {
  return cxxbridge1$rust_vec$ChopChannel$capacity(this);
}
template <>
const ::ChopChannel *Vec<::ChopChannel>::data() const noexcept {
  return cxxbridge1$rust_vec$ChopChannel$data(this);
}
template <>
void Vec<::ChopChannel>::reserve_total(::std::size_t new_cap) noexcept {
  return cxxbridge1$rust_vec$ChopChannel$reserve_total(this, new_cap);
}
template <>
void Vec<::ChopChannel>::set_len(::std::size_t len) noexcept {
  return cxxbridge1$rust_vec$ChopChannel$set_len(this, len);
}
} // namespace cxxbridge1
} // namespace rust
