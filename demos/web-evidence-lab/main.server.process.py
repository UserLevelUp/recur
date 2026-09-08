"""Browser-test adapter: launch the Julia server; implements no HTTP service."""
from pathlib import Path
import os
import queue
import shutil
import subprocess
import threading

ROOT = Path(__file__).resolve().parent


class JuliaServer:
    def __init__(self):
        alias = Path.home() / 'AppData/Local/Microsoft/WindowsApps/julia.exe'
        launcher = os.environ.get('RECUR_JULIA') or (str(alias) if alias.exists() else shutil.which('julia'))
        if not launcher:
            raise RuntimeError('Julia is required. Set RECUR_JULIA to its executable.')
        flags = subprocess.CREATE_NO_WINDOW if os.name == 'nt' else 0
        self.executable = subprocess.check_output([launcher, '--startup-file=no', '-e',
            'print(joinpath(Sys.BINDIR, Base.julia_exename()))'], text=True, timeout=30, creationflags=flags).strip()
        self.argv = [self.executable, '--startup-file=no', f'--project={ROOT}', str(ROOT / 'main.server.jl'), '--port', '0']
        self.process = subprocess.Popen(self.argv, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                        text=True, encoding='utf-8', creationflags=flags)
        self.output = []
        ready = queue.Queue()
        def read():
            for line in self.process.stdout:
                self.output.append(line.rstrip())
                if line.startswith('MAIN_SERVER_READY '):
                    ready.put(line.strip().split())
            ready.put(None)
        threading.Thread(target=read, daemon=True).start()
        try:
            line = ready.get(timeout=60)
            if line is None:
                raise RuntimeError('\n'.join(self.output))
            self.url = line[1]
            self.server_port = int(self.url.rsplit(':', 1)[1])
            self.version = line[2]
        except Exception:
            self.shutdown()
            raise

    def shutdown(self):
        if self.process.poll() is None:
            self.process.terminate()
            try:
                self.process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=10)

    def server_close(self):
        self.process.stdout.close()


def make_server():
    return JuliaServer()
