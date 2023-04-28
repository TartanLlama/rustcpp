#include <string>
#include <iostream>

class cat {
public:
    cat(const char* name);
    
    const char* name();
    void meow();

private:
    const char* name_;
};
