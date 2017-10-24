set({{ name }}_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/modules/{{ name }})

externalproject_add({{ name }}
  URL {{ url }}
  GIT_PROGRESS true
  GIT_TAG {{ tag }}
  SOURCE_DIR ${{{ name }}_SOURCE_DIR}
  CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${GLOBAL_OUTPUT_PATH}
  UPDATE_COMMAND git -C ${{{ name }}_SOURCE_DIR} pull {{ url }})

list(APPEND GLOBAL_SHARED_LINK_LIST
  ${CMAKE_SHARED_MODULE_PREFIX}{{ name }}${CMAKE_SHARED_MODULE_SUFFIX})
