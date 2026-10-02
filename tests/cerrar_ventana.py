"""Pide el cierre de una ventana X11 como lo hace un gestor de ventanas: un ClientMessage
WM_PROTOCOLS con WM_DELETE_WINDOW. Uso: python3 tests/cerrar_ventana.py ID_DE_VENTANA"""

import ctypes
import sys

x11 = ctypes.CDLL("libX11.so.6")
x11.XOpenDisplay.restype = ctypes.c_void_p
x11.XOpenDisplay.argtypes = [ctypes.c_char_p]
x11.XInternAtom.restype = ctypes.c_ulong
x11.XInternAtom.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_int]
x11.XSendEvent.argtypes = [
    ctypes.c_void_p, ctypes.c_ulong, ctypes.c_int, ctypes.c_long, ctypes.c_void_p
]
x11.XFlush.argtypes = [ctypes.c_void_p]
x11.XCloseDisplay.argtypes = [ctypes.c_void_p]


class MensajeDeCliente(ctypes.Structure):
    """XClientMessageEvent con format 32, rellenado hasta el tamaño de XEvent."""

    _fields_ = [
        ("type", ctypes.c_int),
        ("serial", ctypes.c_ulong),
        ("send_event", ctypes.c_int),
        ("display", ctypes.c_void_p),
        ("window", ctypes.c_ulong),
        ("message_type", ctypes.c_ulong),
        ("format", ctypes.c_int),
        ("data", ctypes.c_long * 5),
        ("relleno", ctypes.c_long * 24),
    ]


CLIENT_MESSAGE = 33
CURRENT_TIME = 0

pantalla = x11.XOpenDisplay(None)
if not pantalla:
    sys.exit("no hay pantalla X")
ventana = int(sys.argv[1])
evento = MensajeDeCliente()
evento.type = CLIENT_MESSAGE
evento.display = pantalla
evento.window = ventana
evento.message_type = x11.XInternAtom(pantalla, b"WM_PROTOCOLS", 0)
evento.format = 32
evento.data[0] = x11.XInternAtom(pantalla, b"WM_DELETE_WINDOW", 0)
evento.data[1] = CURRENT_TIME
if not x11.XSendEvent(pantalla, ventana, 0, 0, ctypes.byref(evento)):
    sys.exit("XSendEvent falló")
x11.XFlush(pantalla)
x11.XCloseDisplay(pantalla)
