#include "../target/cxxbridge/cats/src/main.rs.h"
#include <iostream>
void test() {
    Cat marshmallow = make("Marshmallow");
    marshmallow.meow();
    std::cout << std::string(marshmallow.name());
}