#include <iostream>

#include "playground/include/demo.h"

// void MyStruct::test() const { std::cout << "test" << std::endl; }

std::unique_ptr<MyStruct> create() { return std::make_unique<MyStruct>(); }
