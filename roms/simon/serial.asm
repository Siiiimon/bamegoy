SECTION "Entry", ROM0[$0100]
    jp Start

SECTION "Serial Echo Test", ROM0

Start:
    ld a, $42        ; Byte to send
    ld [$FF01], a    ; Write to SB (serial transfer data)
    ld a, $81        ; Enable transfer + use internal clock
    ld [$FF02], a    ; Start transfer

.WaitTransfer:
    ld a, [$FF02]
    and $80          ; Check if transfer is still in progress
    jr nz, .WaitTransfer

    ld a, [$FF01]    ; Read received byte
    cp $42           ; Compare with original value
    jr nz, .Fail

.Success:
    ld a, $55
    ld b, a
.loop_success:
    ld a, $AA
    nop
    nop
    nop
    ld a, b
    jp .loop_success

.Fail:
    ld a, $00
    ld b, a
.loop_fail:
    ld a, $FF
    nop
    nop
    nop
    ld a, b
    jp .loop_fail

