CC = gcc
CFLAGS = -fPIC -O2
TARGET = libmyclib.so
OBJS = myclib.o

all: $(TARGET)

myclib.o:  myclib.c
	$(CC) $(CFLAGS) -c myclib.c -o myclib.o

$(TARGET): myclib.o
	$(CC) -shared -o $(TARGET) myclib.o

clean:
	rm -f $(OBJS) $(TARGET)
