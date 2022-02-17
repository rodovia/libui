cmake_minimum_required(VERSION 3.19)
project(libui LANGUAGES C CXX)

# Should I add OLD_table.c to the archive? Don't think so.
set(common_SOURCES 
	common/areaevents.c
	common/attribute.c
	common/attrlist.c
	common/attrstr.c
	common/control.c
	common/debug.c
	common/matrix.c
	common/opentype.c
	common/shouldquit.c
	common/tablemodel.c
	common/tablevalue.c
	common/userbugs.c
	common/utf.c
)

add_library(libuicomm STATIC ${common_SOURCES})
if (DEFINED CMAKE_STATIC_LIBRARY_PREFIX)
set_target_properties(libuicomm PROPERTIES OUTPUT_NAME uicomm)
endif()
set_target_properties(libuicomm PROPERTIES INCLUDE_DIRECTORIES ${CMAKE_CURRENT_LIST_DIR}/common)

if(WIN32)
	add_subdirectory(windows)
elseif(APPLE)
	message(WARNING "APPLE is not supported by libui-modif. No new features will be made for it.")
	add_subdirectory(darwin)
elseif(UNIX AND NOT APPLE)
	add_subdirectory(unix)
endif()