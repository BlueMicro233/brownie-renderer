# Brownie Renderer — Cross-platform Makefile
#
# Targets:
#   make          — build the renderer
#   make run      — build and run
#   make clean    — remove build artifacts
#   make help     — show this message

CXX       ?= c++
CXX_STD   := -std=c++17
CXXFLAGS  := $(CXX_STD) -O3 -Wall -Wextra -Wno-unused-parameter
LDFLAGS   :=
LDLIBS    :=

SRCDIR    := src
BUILDDIR  := build
TARGET    := $(BUILDDIR)/RayTracing

# Gather source files
CPP_FILES := $(wildcard $(SRCDIR)/*.cpp)
OBJ_FILES := $(CPP_FILES:$(SRCDIR)/%.cpp=$(BUILDDIR)/%.o)

# ---------------------------------------------------------------------------
# Platform detection
# ---------------------------------------------------------------------------
UNAME_S   := $(shell uname -s)
UNAME_M   := $(shell uname -m)

# ---------------------------------------------------------------------------
# macOS: Apple Clang + libomp
# ---------------------------------------------------------------------------
ifeq ($(UNAME_S),Darwin)
    CXXFLAGS += -Xpreprocessor -fopenmp
    LDFLAGS  += -L/opt/homebrew/opt/libomp/lib
    LDLIBS   += -lomp

    # Homebrew include path
    LDFLAGS  += -I/opt/homebrew/opt/libomp/include
    CXXFLAGS += -I/opt/homebrew/opt/libomp/include

# ---------------------------------------------------------------------------
# Linux: GCC / Clang with native OpenMP
# ---------------------------------------------------------------------------
else ifeq ($(UNAME_S),Linux)
    CXXFLAGS += -fopenmp
    LDLIBS   += -fopenmp
endif

# ---------------------------------------------------------------------------
# Architecture-specific flags
# ---------------------------------------------------------------------------
ifneq (,$(filter $(UNAME_M),x86_64 amd64))
    CXXFLAGS += -mavx -mavx2
else ifneq (,$(filter $(UNAME_M),arm64 aarch64))
    CXXFLAGS += -D__ARM_NEON
endif

# ---------------------------------------------------------------------------
# Rules
# ---------------------------------------------------------------------------
.PHONY: all run clean help

all: $(TARGET)

$(BUILDDIR)/%.o: $(SRCDIR)/%.cpp $(SRCDIR)/%.hpp
	@mkdir -p $(BUILDDIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Object files whose headers don't follow the naming convention
$(BUILDDIR)/main.o: $(SRCDIR)/main.cpp
	@mkdir -p $(BUILDDIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(BUILDDIR)/Hardware_Detector.o: $(SRCDIR)/Hardware_Detector.cpp $(SRCDIR)/Hardware_Detector.hpp
	@mkdir -p $(BUILDDIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(TARGET): $(OBJ_FILES)
	@mkdir -p $(BUILDDIR)
	$(CXX) $(CXXFLAGS) $^ $(LDFLAGS) $(LDLIBS) -o $@

run: all
	$(TARGET)

clean:
	rm -rf $(BUILDDIR)

help:
	@echo "Brownie Renderer — Cross-platform Makefile"
	@echo ""
	@echo "  make        — build the renderer"
	@echo "  make run    — build and run"
	@echo "  make clean  — remove build artifacts"
	@echo ""
	@echo "Detected platform: $(UNAME_S) / $(UNAME_M)"
	@echo "Compiler: $(CXX)"
	@echo "Flags: $(CXXFLAGS)"
