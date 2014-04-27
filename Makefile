# rust compiler command
RUSTC=rustc

# name of file with main()
MAINFILE=main.rs

# location of source files
SRCLOC=./src

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
	$(RUSTC) $(RUSTFLAGS) $(SRCLOC)/$(MAINFILE)

# run it
run:
	make && $(BINLOC)/$(EXEC)

# clear out binaries
clean:
	rm -rf $(BINLOC)/*
