# This is a quick script for demonstrating a webapp using Rayon, it will build everything needed
# and create an output directory, containing the assets that need to be served. The script will
# then start a Python HTTP server with the appropiate COEP/COOP headers. This is absolutely not
# meant to be used for production. At a bare minimum, you would want to build in release mode,
# optimise the WebAssembly module using wasm-opt, and use something more robust than Python's HTTP
# server.
#
# If you need support with your webapp, get in touch via contact@allwright.io and tell me about
# your project.

import http.server
import os
import signal
import socketserver
import subprocess

PORT = 3000

# build project
subprocess.run([
    "cargo",
    "build"
])

# create the output directory and symlink index.html
if not os.path.isdir('output'):
    os.mkdir('output')
if not os.path.islink('output/index.html'):
    os.symlink('../index.html', 'output/index.html')

# generate bindings
subprocess.run([
    "wasm-bindgen",
    "target/wasm32-unknown-unknown/debug/webapp-mandelbrot-rayon.wasm",
    "--out-dir",
    "output",
    "--target",
    "web",
    "--no-typescript"
])

# start web server
class RequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory='output', **kwargs)
    def end_headers(self):
        # these headers are required for shared memory between workers
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        http.server.SimpleHTTPRequestHandler.end_headers(self)
server = socketserver.TCPServer(('localhost', PORT), RequestHandler)
signal.signal(signal.SIGINT,
    lambda _signal, _frame: setattr(server, '_BaseServer__shutdown_request', True))
print('serving on http://localhost:{}/'.format(PORT))
server.serve_forever()
