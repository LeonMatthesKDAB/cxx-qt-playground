#pragma once

#include "rust/cxx.h"
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>
#include <iostream>

template <typename CXXArguments, typename... Arguments> class SignalHandler {
  void *data[2];

public:
  SignalHandler() = delete;
  SignalHandler(const SignalHandler &) = delete;

  SignalHandler(SignalHandler &&other) {
    data[0] = other.data[0];
    data[1] = other.data[1];
    other.data[0] = nullptr;
    other.data[1] = nullptr;
  }

  ~SignalHandler() noexcept;
  void operator()(Arguments... args);
};

namespace rust {
template <typename CXXArguments, typename... Arguments>
struct IsRelocatable<SignalHandler<CXXArguments, Arguments...>>
    : std::true_type {};
} // namespace rust

namespace test::ffi {
class MyClass {
public:
  template <typename Functor> void connect(Functor functor) const {
    std::cout << "Calling" << std::endl;
    functor(5);
    std::cout << "Called" << std::endl;
  }
};

using SignalHandlerMyClassMySignal =
    SignalHandler<struct MyClassMySignalParams *, int>;

void connect_MyClass_mySignal(const MyClass &obj,
                              test::ffi::SignalHandlerMyClassMySignal closure);

MyClass *create_my_class() { return new MyClass(); }

} // namespace test::ffi
