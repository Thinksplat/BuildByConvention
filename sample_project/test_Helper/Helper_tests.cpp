#include <gtest/gtest.h>
#include "Helper.h"

TEST(Helper, HelloString_ReturnsHelloWorld) {
    EXPECT_STREQ(Hello::HelloString().c_str(), "Hello, World!");
}