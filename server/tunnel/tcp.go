package tunnel

import (
	log "github.com/sirupsen/logrus"
	"io"
	"net"
	"os"
	"sync"
)

type TCPTunnel struct {
	Kind        Kind
	listener    net.Listener
	Address     string
	quit        chan interface{}
	wg          sync.WaitGroup
	connections []net.Conn
}

func NewTCP() (tun *TCPTunnel, err error) {
	tun = &TCPTunnel{Kind: TcpTunnelKind, quit: make(chan interface{}), connections: []net.Conn{}}
	l, err := net.Listen("tcp", "localhost:")
	if err != nil {
		return
	}
	tun.Address = l.Addr().String()
	tun.listener = l
	return
}

func (tun *TCPTunnel) Start() {
	tun.wg.Add(1)
	go tun.serve()
	log.Tracef("TCPTunnel.Start: new tcp tunnel listening on address %s\n", tun.Address)
}

func (tun *TCPTunnel) Stop() (err error) {
	log.Tracef("calling tunnel %p's Stop method\n", tun)
	close(tun.quit)
	if err = tun.listener.Close(); err != nil {
		return
	}
	for _, c := range tun.connections {
		_ = c.Close()
	}
	tun.wg.Wait()
	return
}

func (tun *TCPTunnel) serve() {
	defer tun.wg.Done()

	for {
		conn, err := tun.listener.Accept()
		if err != nil {
			select {
			case <-tun.quit:
				return
			default:
				log.Println("accept error", err)
			}
		} else {
			tun.wg.Add(1)
			tun.connections = append(tun.connections, conn)
			go func() {
				tun.handleTCPConn(conn)
				tun.wg.Done()
			}()
		}
	}
}

func (tun *TCPTunnel) handleTCPConn(conn net.Conn) {
	defer func(conn net.Conn) {
		_ = conn.Close()
	}(conn)
	if _, err := io.Copy(os.Stdout, conn); err != nil {
		return
	}
}
