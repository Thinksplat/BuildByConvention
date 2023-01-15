#include <gtest/gtest.h>
#include "Helper.h"

TEST(Helper, HelloString_ReturnsHelloWorld) {
    EXPECT_STREQ(Helper::HelloString().c_str(), "Hello, World!");
}