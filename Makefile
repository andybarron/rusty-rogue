# rust compiler command
RUSTC=rustc

# location of file with main()
MAINFILE=./src/main.rs

# location of libraries
LIBLOC=./lib/*

# location of compiled binary
BINLOC=./bin

# name of compiled binary
EXEC=dungeon

# flags for rustc
RUSTFLAGS=-L $(LIBLOC) -o $(BINLOC)/$(EXEC)

# compile it
all:
	$(RUSTC) $(RUSTFLAGS) $(MAINFILE)

# run it
run:
	make && $(BINLOC)/$(EXEC)

# clear out binaries
clean:
	rm -rf $(BINLOC)/*
