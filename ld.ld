ENTRY(_start);

SECTIONS {
	. = 0;
	.text : ALIGN(16) {
		KEEP(*(.init))
		*(.text);
	}
	.data : ALIGN(16) {
		*(.rodata .rodata.*)
		*(.data .data*);
		*(.got*)
	}
	/DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.note.gnu.build-id*)  }
}
