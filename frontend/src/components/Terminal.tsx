import { useEffect, useRef } from 'react'
import { init, Terminal as GhosttyTerminal, FitAddon } from 'ghostty-web'

function Terminal() {
  const containerRef = useRef<HTMLDivElement>(null)
  const terminalRef = useRef<GhosttyTerminal | null>(null)
  const wsRef = useRef<WebSocket | null>(null)

  useEffect(() => {
    let mounted = true

    async function setup() {
      if (!containerRef.current || terminalRef.current) return

      await init()

      if (!mounted) return

      const term = new GhosttyTerminal({
        fontSize: 14,
        fontFamily: 'Menlo, Monaco, "Courier New", monospace',
        cursorBlink: true,
        cursorStyle: 'block',
        theme: {
          background: '#1a1a1a',
          foreground: '#ffffff',
          cursor: '#ffffff',
        },
      })

      term.open(containerRef.current)
      terminalRef.current = term

      // Load FitAddon for auto-resizing
      const fitAddon = new FitAddon()
      term.loadAddon(fitAddon)
      fitAddon.fit()
      fitAddon.observeResize()

      const ws = new WebSocket('ws://localhost:3001/ws')
      wsRef.current = ws

      ws.binaryType = 'arraybuffer'

      ws.onopen = () => {
        console.log('WebSocket connected')
        term.focus()
      }

      ws.onmessage = (event) => {
        if (event.data instanceof ArrayBuffer) {
          const text = new TextDecoder().decode(event.data)
          term.write(text)
        } else {
          term.write(event.data)
        }
      }

      ws.onerror = (error) => {
        console.error('WebSocket error:', error)
      }

      ws.onclose = () => {
        console.log('WebSocket disconnected')
      }

      // Send user input to WebSocket
      term.onData((data: string) => {
        if (ws.readyState === WebSocket.OPEN) {
          ws.send(data)
        }
      })

      // Handle terminal resize
      term.onResize(({ cols, rows }) => {
        console.log(`Terminal resized to ${cols}x${rows}`)
        // TODO: Send resize event to backend
      })
    }

    setup()

    return () => {
      mounted = false
      if (wsRef.current) {
        wsRef.current.close()
      }
      if (terminalRef.current) {
        terminalRef.current.dispose()
      }
    }
  }, [])

  return <div ref={containerRef} className="terminal-container" />
}

export default Terminal
