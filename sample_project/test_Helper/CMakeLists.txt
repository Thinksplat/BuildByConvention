FILE(GLOB SOURCES *.cpp)

include_directories(${PROJECT_SOURCE_DIR}/lib_Helper)

add_executable(
  test_Helper
    ${SOURCES}
)
target_link_libraries(
  test_Helper
  GTest::gtest_main
  Helper
)

include(GoogleTest)
gtest_discover_tests(test_Helper)
