#include <iostream>
#include <string>

class MyBase {
public:
  MyBase(int x, std::string y) : m_x(x), m_y(y) {}
  virtual ~MyBase() {}

  int m_x;
  std::string m_y;
};

struct MyBaseArgs {
  int x;
  std::string &&y;
};

struct MyClassArgs {
  int z;
};

struct MyArgs {
  MyBaseArgs baseArgs;
  MyClassArgs classArgs;
};

MyArgs rustArgs(int newZ, int newX, std::string &&newY) {
  return {.baseArgs = {.x = newX, .y = std::move(newY)},
          .classArgs = {.z = newZ}};
}

class MyClass : public MyBase {
public:
  MyClass(int z, int x, std::string &&y)
      : MyClass(rustArgs(std::move(z), std::move(x), std::move(y))) {}

  ~MyClass() {}

  void foo() {
    std::cout << "x: " << m_x << std::endl;
    std::cout << "y: " << m_y << std::endl;
    std::cout << "z: " << m_z << std::endl;
  }

private:
  MyClass(MyArgs &&args)
      : MyBase(std::move(args.baseArgs.x), std::move(args.baseArgs.y)),
        m_z(std::move(args.classArgs.z)) {}

  int m_z;
};

int main(int argc, char *argv[]) {
  MyClass myClass(23, 42, "Hello");

  myClass.foo();
}
