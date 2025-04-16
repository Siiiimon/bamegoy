SECTION "Serial Echo Test", ROM0

Start:
    ld a, $42        ; Byte to send
    ld [$FF01], a    ; Write to SB (serial data)
    ld a, $81        ; Enable transfer + internal clock
    ld [$FF02], a

WaitTransfer:
    ld a, [$FF02]
    and $80
    jr nz, WaitTransfer

    ld a, [$FF01]
    cp $42
    jr nz, .fail

.success:
    jr .success

.fail:
    jr .fail

