#pragma once

#include <iostream>
#include <memory>

class Base {
public:
  virtual void test() const { std::cout << "base" << std::endl; }
};

struct MyStruct : public Base {
  template <class... Args> void test_Base(Args &&...args) const {
    return Base::test(std::forward<Args>(args)...);
  }

  void test() const override { std::cout << "sub" << std::endl; }
};

std::unique_ptr<MyStruct> create();
