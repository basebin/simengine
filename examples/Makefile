# Build C++ examples
# Requires: cargo build --features cxx

CXX = g++
CXXFLAGS = -std=c++17 -I../target/cxxbridge/simulation-engine-cpp/src

all: cpp_example cpp_physics

cpp_example: cpp_example.cpp
	$(CXX) $(CXXFLAGS) -o $@ $< -L../target/debug -lsimulation_engine

cpp_physics: cpp_physics.cpp
	$(CXX) $(CXXFLAGS) -o $@ $< -L../target/debug -lsimulation_engine

clean:
	rm -f cpp_example cpp_physics