#include "Helper.h"
#include <iostream>

void Helper::SayHello()
{
    std::cout << HelloString() << std::endl;
}

std::string Helper::HelloString()
{
    return "Hello, World!";
}