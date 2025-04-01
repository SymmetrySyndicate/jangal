CC = gcc
CFLAGS = -Wall -Wextra -lm -I include
SRC = $(shell find src -name "*.c")
TESTS = $(shell find tests -name "*.c")
TEST_EXEC = test_runner.out

all: test

test: $(SRC) $(TESTS)
	$(CC) $(CFLAGS) $(SRC) $(TESTS) -o $(TEST_EXEC)
	./$(TEST_EXEC)

clean:
	rm -f *.out

.PHONY: all test clean
