; testrom.asm
SECTION "Entry Point", ROM0[$0100]
    nop
    jp Start

SECTION "Code", ROM0[$0150]
Start:
    inc b
    inc c
    inc d
    inc e
    inc h
    inc l
    inc a

    dec b
    dec c
    dec d
    dec e
    dec h
    dec l
    dec a

    ld b, $69
    ld c, $69
    ld d, $69
    ld e, $69
    ld h, $69
    ld l, $69
    ld a, $69

    jp Start

