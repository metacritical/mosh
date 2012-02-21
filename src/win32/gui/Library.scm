(win32-gui
  c-function-table
  *internal*
  (libname: win32-gui)
  (header: "aio_win32.h")
  #(ret name args)
  (int win32_messagebox (void* void* int int))
  (void win32_window_move (void* int int int int))
  (void win32_window_show (void* int))
  (void win32_window_hide (void*))
  (void win32_window_settitle (void* void*))
  (void win32_window_close (void*))
  (void win32_window_destroy (void*))
  (void win32_registerwindowclass)
  (void* win32_window_alloc)
  (void win32_window_fitbuffer (void*))
  (void win32_window_create (void* void*))
  (void win32_window_updaterects (void* void* void* int))
  (void* win32_window_createbitmap (void* int int))
  (int win32_window_getwindowrect (void* void* void* void* void*))
  (int win32_window_getclientrect (void* void* void* void* void*))
  (int win32_window_getclientrect_x (void*))
  (int win32_window_getclientrect_y (void*))
  (int win32_window_clienttoscreen (void* int int void* void*))
  (void* win32_dc_create)
  (void win32_dc_dispose (void*))
  (void win32_dc_selectobject (void* void*))
  (void win32_dc_transform (void* void*))
  (void win32_dc_settransform (void* void*))
  (void win32_gdi_deleteobject (void*))
  (void* win32_pen_create (int int int int))
  (void* win32_brush_create (int int int))
  (void* win32_font_create (int int int void*))
  (void* win32_dc_draw (void* void* void* int))
  (void* win32_dc_measure_text (void* void* int void* void*))
  (void win32_getmonitorinfo (int int void* void* void* void* void*))
  (void win32_cursor_hide (void*))
  )
