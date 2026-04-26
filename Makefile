# Brownie Renderer — Cross-platform Makefile wrapper around CMake
#
# Targets:
#   make          — configure + build (debug, build/debug/)
#   make release  — configure + build (release, build/release/)
#   make run      — build release + run
#   make clean    — remove all build directories
#   make help     — show this message

BUILDDIR  := build/debug
TARGET    := $(BUILDDIR)/RayTracing

# ---------------------------------------------------------------------------
# Platform detection & CMake flags
# ---------------------------------------------------------------------------
UNAME_S   := $(shell uname -s)
UNAME_M   := $(shell uname -m)
CMAKE     := cmake
CMAKE_OPT :=

# macOS: Apple Clang needs -Xpreprocessor -fopenmp and libomp paths
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

release: BUILDDIR := build/release
release: CMAKE_OPT += -DCMAKE_BUILD_TYPE=Release
release: $(BUILDDIR)/Makefile
	$(CMAKE) --build $(BUILDDIR)

$(BUILDDIR)/Makefile:
	$(CMAKE) -B $(BUILDDIR) $(CMAKE_OPT)

run: release
	./$(BUILDDIR)/RayTracing

clean:
	rm -rf build

help:
	@echo "Brownie Renderer — Cross-platform Makefile"
	@echo ""
	@echo "  make          — configure + build (debug,   build/debug/)"
	@echo "  make release  — configure + build (release, build/release/)"
	@echo "  make run      — build release + run"
	@echo "  make clean    — remove build directory"
	@echo ""
	@echo "Detected: $(UNAME_S) / $(UNAME_M)"
	@echo "CMake options: $(CMAKE_OPT)"
