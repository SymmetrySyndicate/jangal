CC = gcc
CFLAGS = -Wall -Wextra -I include

# Dynamically find all modules (base names from src/*.c files)
MODULES = $(shell find src -name "*.c" -exec basename {} .c \;)

# Function to get source files for a module (including dependencies)
define get_src_files
$(shell find src -name "$(1).c" -o -name "$(1).c")
endef

# Function to get test file for a module
define get_test_file
$(shell find tests -name "test_$(1).c" 2>/dev/null || echo "")
endef

# Function to get header file for a module
define get_header_file
$(shell find include -name "$(1).h" 2>/dev/null || echo "")
endef

all: test

# Test target that runs all module tests
test: $(addprefix test_,$(MODULES))

# Dynamic test target for each module
define test_template
test_$(1): $(call get_src_files,$(1)) $(call get_test_file,$(1))
	@if [ -f "$(call get_test_file,$(1))" ]; then \
		echo "Testing module: $(1)"; \
		$(CC) $(CFLAGS) $(call get_src_files,$(1)) $(call get_test_file,$(1)) -o test_$(1).out -lm; \
		./test_$(1).out; \
	else \
		echo "No test file found for module: $(1)"; \
	fi
endef

# Generate test targets for each module
$(foreach module,$(MODULES),$(eval $(call test_template,$(module))))

clean:
	rm -f *.out

.PHONY: all test clean $(addprefix test_,$(MODULES))
