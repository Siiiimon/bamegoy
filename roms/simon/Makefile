RGBASM  = rgbasm
RGBLINK = rgblink
RGBFIX  = rgbfix

ROM_NAME = incdecloop
ASM_FILE = $(ROM_NAME).asm
OBJ_FILE = $(ROM_NAME).obj
GB_FILE  = $(ROM_NAME).gb

all: $(GB_FILE)

$(GB_FILE): $(OBJ_FILE)
	$(RGBLINK) -o $@ $<
	$(RGBFIX) -v -p 0 $@

$(OBJ_FILE): $(ASM_FILE)
	$(RGBASM) -o $@ $<

clean:
	rm -f *.obj *.gb

