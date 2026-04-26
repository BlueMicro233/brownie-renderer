# Brownie Renderer — Cross-platform Makefile wrapper around CMake
#
# Delegates the actual build to CMake, so CMakeLists.txt remains the
# single source of truth for source files, targets, and dependencies.
#
# Targets:
#   make          — configure + build the renderer (debug)
#   make release  — configure + build with -O3
#   make run      — build and run
#   make clean    — remove build directory
#   make help     — show this message

BUILDDIR  := build

# ---------------------------------------------------------------------------
# Platform detection & CMake flags
# ---------------------------------------------------------------------------
UNAME_S   := $(shell uname -s)
UNAME_M   := $(shell uname -m)
CMAKE     := cmake
CMAKE_OPT :=

# macOS: Apple Clang needs -Xpreprocessor -fopenmp and libomp paths
# We set OpenMP_* variables explicitly so CMake's FindOpenMP skips its
# broken detection test and uses our flags directly.
ifeq ($(UNAME_S),Darwin)
    OMP_PREFIX := $(shell brew --prefix libomp)
    CMAKE_OPT += -DCMAKE_CXX_FLAGS="-I$(OMP_PREFIX)/include"
    CMAKE_OPT += -DCMAKE_EXE_LINKER_FLAGS="-L$(OMP_PREFIX)/lib -lomp"
    CMAKE_OPT += -DOpenMP_CXX_FLAGS="-Xpreprocessor -fopenmp -I$(OMP_PREFIX)/include"
    CMAKE_OPT += -DOpenMP_C_FLAGS="-Xpreprocessor -fopenmp -I$(OMP_PREFIX)/include"
    CMAKE_OPT += -DOpenMP_CXX_LIB_NAMES="libomp"
    CMAKE_OPT += -DOpenMP_C_LIB_NAMES="libomp"
    CMAKE_OPT += -DOpenMP_libomp_LIBRARY="$(OMP_PREFIX)/lib/libomp.dylib"
endif

# ---------------------------------------------------------------------------
# Rules
# ---------------------------------------------------------------------------
.PHONY: all release run clean help

all: $(BUILDDIR)/Makefile
	$(CMAKE) --build $(BUILDDIR)

release: CMAKE_OPT += -DCMAKE_BUILD_TYPE=Release
release: $(BUILDDIR)/Makefile
	$(CMAKE) --build $(BUILDDIR)

$(BUILDDIR)/Makefile:
	$(CMAKE) -B $(BUILDDIR) $(CMAKE_OPT)

run: all
	./$(BUILDDIR)/RayTracing

clean:
	rm -rf $(BUILDDIR)

help:
	@echo "Brownie Renderer — Cross-platform Makefile"
	@echo ""
	@echo "  make          — configure + build (debug)"
	@echo "  make release  — configure + build (release)"
	@echo "  make run      — build and run"
	@echo "  make clean    — remove build directory"
	@echo ""
	@echo "Detected: $(UNAME_S) / $(UNAME_M)"
	@echo "CMake options: $(CMAKE_OPT)"
