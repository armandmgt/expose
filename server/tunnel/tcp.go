package tunnel

import (
	"io"
	"log"
	"net"
	"os"
	"sync"
)

type TcpTunnel struct {
	Kind     Kind
	listener net.Listener
	Address  string
	quit     chan interface{}
	wg       sync.WaitGroup
}

func NewTCP() (tun *TcpTunnel, err error) {
	tun = &TcpTunnel{Kind: TcpTunnelKind, quit: make(chan interface{})}
	l, err := net.Listen("tcp", "localhost:")
	if err != nil {
		return
	}
	tun.Address = l.Addr().String()
	tun.listener = l
	return
}

func (tun *TcpTunnel) Start() {
	tun.wg.Add(1)
	go tun.serve()
	log.Printf("New tcp tunnel listening on address %s\n", tun.Address)
}

func (tun *TcpTunnel) Stop() (err error) {
	close(tun.quit)
	if err = tun.listener.Close(); err != nil {
		return
	}
	tun.wg.Wait()
	return
}

func (tun *TcpTunnel) serve() {
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
			go func() {
				tun.handleTcpConn(conn)
				tun.wg.Done()
			}()
		}
	}
}

func (tun *TcpTunnel) handleTcpConn(conn net.Conn) {
	defer func(conn net.Conn) {
		_ = conn.Close()
	}(conn)
	if _, err := io.Copy(os.Stdout, conn); err != nil {
		return
	}
}
