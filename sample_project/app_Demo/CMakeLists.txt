include_directories(${PROJECT_SOURCE_DIR}/lib_Helper)

FILE(GLOB SOURCES *.cpp)

# Add executable called "helloDemo" that is built from the source files
# "demo.cxx" and "demo_b.cxx". The extensions are automatically found.
add_executable (app_Demo ${SOURCES})

target_link_libraries (app_Demo LINK_PUBLIC Helper)
