HOST:=$(shell rustc --version | grep host | sed -e 's/host:\s//g')
TEST_DIR:=test
LIB_DIR:=lib
BIN_DIR:=bin

SYNEXT_LIB:=$(LIB_DIR)/$(shell rustc --crate-file-name lib.rs | grep ".so")
TEST_LIB:=$(LIB_DIR)/$(shell rustc --crate-file-name testlib.rs | grep ".so")

.PHONY: lib test clean

test: lib $(TEST_DIR)/*.rs

$(TEST_DIR)/%.rs : lib
	rustc $@ --out-dir $(BIN_DIR) -L $(LIB_DIR)

lib: $(SYNEXT_LIB) $(TEST_LIB)

$(SYNEXT_LIB) : lib.rs
	rustc $< --out-dir $(LIB_DIR)

$(TEST_LIB) : testlib.rs
	rustc $< --out-dir $(LIB_DIR)

clean:
	-rm -f $(LIB_DIR)/*.so
	-rm -f $(BIN_DIR)/*.so
	-rm -f *.o
