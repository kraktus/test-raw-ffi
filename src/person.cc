#include "test-raw-ffi/include/person.h"
#include "test-raw-ffi/src/lib.rs.h"
#include <algorithm> // what do they do
#include <functional> // what do they do
#include <iostream>


uint32_t foo() {
	std::cout << "Hello world";
	0;
}