# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.10

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Remove some rules from gmake that .SUFFIXES does not remove.
SUFFIXES =

.SUFFIXES: .hpux_make_needs_suffix_list


# Suppress display of executed commands.
$(VERBOSE).SILENT:


# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E remove -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c

# Include any dependencies generated for this target.
include example/c/CMakeFiles/lib_buffer_unpack.dir/depend.make

# Include the progress variables for this target.
include example/c/CMakeFiles/lib_buffer_unpack.dir/progress.make

# Include the compile flags for this target's objects.
include example/c/CMakeFiles/lib_buffer_unpack.dir/flags.make

example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o: example/c/CMakeFiles/lib_buffer_unpack.dir/flags.make
example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o: example/c/CMakeFiles/lib_buffer_unpack.dir/includes_C.rsp
example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o: example/c/lib_buffer_unpack.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o"
	cd /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c && /home/bazk/emsdk/upstream/emscripten/emcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o   -c /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c/lib_buffer_unpack.c

example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.i"
	cd /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c && /home/bazk/emsdk/upstream/emscripten/emcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c/lib_buffer_unpack.c > CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.i

example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.s"
	cd /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c && /home/bazk/emsdk/upstream/emscripten/emcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c/lib_buffer_unpack.c -o CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.s

example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o.requires:

.PHONY : example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o.requires

example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o.provides: example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o.requires
	$(MAKE) -f example/c/CMakeFiles/lib_buffer_unpack.dir/build.make example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o.provides.build
.PHONY : example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o.provides

example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o.provides.build: example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o


# Object files for target lib_buffer_unpack
lib_buffer_unpack_OBJECTS = \
"CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o"

# External object files for target lib_buffer_unpack
lib_buffer_unpack_EXTERNAL_OBJECTS =

example/c/lib_buffer_unpack.js: example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o
example/c/lib_buffer_unpack.js: example/c/CMakeFiles/lib_buffer_unpack.dir/build.make
example/c/lib_buffer_unpack.js: libmsgpackc.a
example/c/lib_buffer_unpack.js: example/c/CMakeFiles/lib_buffer_unpack.dir/linklibs.rsp
example/c/lib_buffer_unpack.js: example/c/CMakeFiles/lib_buffer_unpack.dir/objects1.rsp
example/c/lib_buffer_unpack.js: example/c/CMakeFiles/lib_buffer_unpack.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking C executable lib_buffer_unpack.js"
	cd /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/lib_buffer_unpack.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
example/c/CMakeFiles/lib_buffer_unpack.dir/build: example/c/lib_buffer_unpack.js

.PHONY : example/c/CMakeFiles/lib_buffer_unpack.dir/build

example/c/CMakeFiles/lib_buffer_unpack.dir/requires: example/c/CMakeFiles/lib_buffer_unpack.dir/lib_buffer_unpack.c.o.requires

.PHONY : example/c/CMakeFiles/lib_buffer_unpack.dir/requires

example/c/CMakeFiles/lib_buffer_unpack.dir/clean:
	cd /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c && $(CMAKE_COMMAND) -P CMakeFiles/lib_buffer_unpack.dir/cmake_clean.cmake
.PHONY : example/c/CMakeFiles/lib_buffer_unpack.dir/clean

example/c/CMakeFiles/lib_buffer_unpack.dir/depend:
	cd /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c /home/bazk/projects/gatewasi/examples/yell-api-c/msgpack-c/example/c/CMakeFiles/lib_buffer_unpack.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : example/c/CMakeFiles/lib_buffer_unpack.dir/depend

