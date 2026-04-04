"""Signal/callback management compatible with matplotlib.cbook.CallbackRegistry."""


class CallbackRegistry:
    """Manage callbacks for named signals.

    Compatible with matplotlib's CallbackRegistry interface:
    - connect(signal, func) -> cid
    - disconnect(cid)
    - process(signal, *args, **kwargs)
    """

    def __init__(self):
        self._callbacks = {}  # {signal: {cid: func}}
        self._next_cid = 0

    def connect(self, signal, func):
        """Register func to be called when signal is processed. Returns a connection id."""
        cid = self._next_cid
        self._next_cid += 1
        if signal not in self._callbacks:
            self._callbacks[signal] = {}
        self._callbacks[signal][cid] = func
        return cid

    def disconnect(self, cid):
        """Disconnect the callback with the given connection id."""
        for signal_cbs in self._callbacks.values():
            if cid in signal_cbs:
                del signal_cbs[cid]
                return

    def process(self, signal, *args, **kwargs):
        """Process signal by calling all connected callbacks."""
        if signal in self._callbacks:
            for func in list(self._callbacks[signal].values()):
                func(*args, **kwargs)
