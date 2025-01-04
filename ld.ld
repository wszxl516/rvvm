ENTRY(_start);

SECTIONS {
	. = 0;
	.text : ALIGN(4K) {
		*(.text);
	}
	.data : ALIGN(4K) {
		*(.rodata .rodata.*)
		*(.data .data*);
	}
	gnu : { *(.note.GNU-stack) *(.gnu_debuglink) *(.note.gnu.build-id*)  }
}
