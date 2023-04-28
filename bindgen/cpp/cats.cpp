#include "cats.hpp"

cat::cat(const char* name) : name_(name) {}
    
const char* cat::name() const { return name_; }
void cat::meow() { std::cout << "meow\n"; }