#include <iostream>
#include "snl.h"

int main() {
    std::cout << "[C++] Starting Game Client..." << std::endl;

    const char* ver = net_get_version();
    std::cout << "[C++] Library Version: " << ver << std::endl;

    return 0;
}