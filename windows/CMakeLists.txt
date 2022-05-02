cmake_minimum_required(VERSION 3.19)

option(LIBUI_SHARED "Make libui-modif be built as a shared library." YES)
set(LIBUI_LIBTYPE STATIC)
if (LIBUI_SHARED)
	set(LIBUI_LIBTYPE SHARED)
endif()

set(libui32_SOURCES
	alloc.cpp
	area.cpp
	areadraw.cpp
	areaevents.cpp
	areascroll.cpp
	areautil.cpp
	attrstr.cpp
	box.cpp
	button.cpp
	checkbox.cpp
	colorbutton.cpp
	colordialog.cpp
	combobox.cpp
	container.cpp
	control.cpp
	d2dscratch.cpp
	datetimepicker.cpp
	debug.cpp
	draw.cpp
	drawmatrix.cpp
	drawpath.cpp
	drawtext.cpp
	dwrite.cpp
	editablecombo.cpp
	entry.cpp
	events.cpp
	fontbutton.cpp
	fontdialog.cpp
	fontmatch.cpp
	form.cpp
	graphemes.cpp
	grid.cpp
	group.cpp
	image.cpp
	init.cpp
	label.cpp
	main.cpp
	menu.cpp
	multilineentry.cpp
	opentype.cpp
	parent.cpp
	progressbar.cpp
	radiobuttons.cpp
	libui.manifest
	separator.cpp
	sizing.cpp
	slider.cpp
	spinbox.cpp
	status_bar.cpp
	stddialogs.cpp
	tab.cpp
	table.cpp
	tabledraw.cpp
	tabledispinfo.cpp
	tableediting.cpp
	tablemetrics.cpp
	tabpage.cpp
	text.cpp
	utf16.cpp
	utilwin.cpp
	window.cpp
	winpublic.cpp
	winutil.cpp
)

set(libui_LIBS
	user32
	kernel32
	gdi32
	comctl32
	uxtheme
	msimg32
	comdlg32
	d2d1
	dwrite
	ole32
	oleaut32
	oleacc
	uuid
	windowscodecs
)

add_library(libui ${LIBUI_LIBTYPE}
	${libui32_SOURCES})
set_target_properties(libui PROPERTIES CXX_STANDARD 20)
if(LIBUI_SHARED)
	target_compile_definitions(libui PRIVATE libui_EXPORTS=1)
	target_compile_definitions(libuicomm PRIVATE libui_EXPORTS=1)
endif()
if (DEFINED CMAKE_SHARED_LIBRARY_PREFIX)
set_target_properties(libui PROPERTIES OUTPUT_NAME ui)
endif()

target_link_libraries(libui libuicomm ${libui_LIBS})
