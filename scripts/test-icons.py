#!/usr/bin/env python3
"""Check shipping icon representations and, optionally, a native package/binary.

Uses only Python's standard library. Run with --binary on Windows or --deb on Linux.
macOS iconutil round-trip checks are in test-macos-package.py.
"""
import argparse
import ctypes
from pathlib import Path
import struct
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent
ICONS = ROOT / 'assets/icons'
WINDOWS_SIZES = [16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 128, 256]
LINUX_SIZES = [16, 24, 32, 48, 64, 128, 256, 512]


def png_size(data):
    assert data[:8] == b'\x89PNG\r\n\x1a\n', 'Invalid PNG signature'
    assert data[12:16] == b'IHDR'
    assert data[24:26] == bytes([8, 6]), 'Expected 8-bit RGBA'
    return struct.unpack('>II', data[16:24])


def check_assets():
    ico = (ICONS / 'windows/app.ico').read_bytes()
    assert struct.unpack_from('<HHH', ico) == (0, 1, len(WINDOWS_SIZES))
    end = 6 + 16 * len(WINDOWS_SIZES)
    for index, size in enumerate(WINDOWS_SIZES):
        w, h, colors, reserved, planes, bits, length, offset = struct.unpack_from('<BBBBHHII', ico, 6 + index * 16)
        assert (w or 256, h or 256, colors, reserved, planes, bits) == (size, size, 0, 0, 1, 32)
        assert offset == end
        data = ico[offset:offset + length]
        assert png_size(data) == (size, size)
        assert data == (ICONS / f'windows/{size}.png').read_bytes()
        end += length
    assert end == len(ico)
    for nominal in [16, 32, 128, 256, 512]:
        for density in [1, 2]:
            suffix = '@2x' if density == 2 else ''
            path = ICONS / f'macos.iconset/icon_{nominal}x{nominal}{suffix}.png'
            assert png_size(path.read_bytes()) == (nominal * density,) * 2
    for size in LINUX_SIZES:
        assert png_size((ICONS / f'linux/{size}.png').read_bytes()) == (size, size)
    print('Assets: 15 Windows sizes, 10 macOS representations (up to 1024), 8 Linux sizes verified.')


def check_windows(binary):
    # Read resources as data: never execute the supplied binary or installer.
    from ctypes import wintypes
    kernel = ctypes.WinDLL('kernel32', use_last_error=True)
    kernel.LoadLibraryExW.argtypes = [wintypes.LPCWSTR, wintypes.HANDLE, wintypes.DWORD]
    kernel.LoadLibraryExW.restype = wintypes.HMODULE
    kernel.FindResourceW.argtypes = [wintypes.HMODULE, ctypes.c_void_p, ctypes.c_void_p]
    kernel.FindResourceW.restype = wintypes.HANDLE
    kernel.SizeofResource.argtypes = [wintypes.HMODULE, wintypes.HANDLE]
    kernel.SizeofResource.restype = wintypes.DWORD
    kernel.LoadResource.argtypes = [wintypes.HMODULE, wintypes.HANDLE]
    kernel.LoadResource.restype = wintypes.HANDLE
    kernel.LockResource.argtypes = [wintypes.HANDLE]
    kernel.LockResource.restype = ctypes.c_void_p
    kernel.FreeLibrary.argtypes = [wintypes.HMODULE]
    callback_type = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HMODULE, ctypes.c_void_p, ctypes.c_void_p, ctypes.c_ssize_t)
    kernel.EnumResourceNamesW.argtypes = [wintypes.HMODULE, ctypes.c_void_p, callback_type, ctypes.c_ssize_t]
    kernel.EnumResourceNamesW.restype = wintypes.BOOL
    module = kernel.LoadLibraryExW(str(binary.resolve()), None, 2)
    if not module:
        raise ctypes.WinError(ctypes.get_last_error())
    try:
        def resource(name, kind):
            handle = kernel.FindResourceW(module, name, kind)
            assert handle, f'Missing resource {kind}/{name}'
            length = kernel.SizeofResource(module, handle)
            return ctypes.string_at(kernel.LockResource(kernel.LoadResource(module, handle)), length)

        groups = []
        def collect(_module, _kind, name, _parameter):
            groups.append(resource(name, 14))
            return True
        callback = callback_type(collect)
        assert kernel.EnumResourceNamesW(module, 14, callback, 0)
        matching = 0
        for group in groups:
            _, kind, count = struct.unpack_from('<HHH', group)
            assert kind == 1
            if count != len(WINDOWS_SIZES):
                continue
            for index, size in enumerate(WINDOWS_SIZES):
                w, h, _, _, _, bits, length, icon_id = struct.unpack_from('<BBBBHHIH', group, 6 + index * 14)
                assert (w or 256, h or 256, bits) == (size, size, 32)
                png = resource(icon_id, 3)
                assert len(png) == length
                assert png == (ICONS / f'windows/{size}.png').read_bytes()
            matching += 1
        assert matching, 'Executable did not contain the shipping multi-resolution icon'
    finally:
        kernel.FreeLibrary(module)
    print(f'Windows PE resources: all 15 exact PNG frames verified in {binary.name}.')


def check_deb(package):
    with tempfile.TemporaryDirectory(prefix='fmv-icons-') as directory:
        subprocess.run(['dpkg-deb', '-x', str(package), directory], check=True)
        root = Path(directory) / 'usr/share'
        desktop = (root / 'applications/FastMarkdownViewer.desktop').read_text()
        assert 'Icon=FastMarkdownViewer\n' in desktop
        assert 'StartupWMClass=FastMarkdownViewer\n' in desktop
        for size in LINUX_SIZES:
            png = root / f'icons/hicolor/{size}x{size}/apps/FastMarkdownViewer.png'
            assert png.read_bytes() == (ICONS / f'linux/{size}.png').read_bytes()
        svg = root / 'icons/hicolor/scalable/apps/FastMarkdownViewer.svg'
        assert svg.read_bytes() == (ICONS / 'linux.svg').read_bytes()
    print('Debian package: desktop identity, 8 PNG sizes, and scalable launcher icon verified.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(__doc__)
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--deb', type=Path)
    args = parser.parse_args()
    check_assets()
    if args.binary:
        check_windows(args.binary)
    if args.deb:
        check_deb(args.deb)
