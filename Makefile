all: clib lib rclib elf

lib:
	make -C ./mylib

elf:
	make -C ./myelf

clib:
	make -C ./myclib

rclib:
	make -C ./myrclib

clean:
	make -C ./mylib clean
	make -C ./myelf clean
	make -C ./myclib clean
	make -C ./myrclib clean

run:
	make -C ./myelf run


info_link:
	readelf -d myelf/target/release/myelf