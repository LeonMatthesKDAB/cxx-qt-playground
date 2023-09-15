#include "myobject.h"

#include "target/cxxbridge/playground/src/lib.rs.h"

template <>
SignalHandler<struct test::ffi::MyClassMySignalParams *>::~SignalHandler() {
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_signal_handler_my_class_my_signal(std::move(*this));
}

template <>
template <>
void SignalHandler<struct test::ffi::MyClassMySignalParams *>::operator()<int>(
    int arg0) {
  call_signal_handler_my_class_my_signal(*this, std::move(arg0));
}

void test::ffi::connect_MyClass_mySignal(
    const test::ffi::MyClass &obj,
    test::ffi::SignalHandlerMyClassMySignal closure) {
  std::cout << "Connecting" << std::endl;
  obj.connect(std::move(closure));
  std::cout << "Connected" << std::endl;
}
