package tunnel

type Tunnel interface {
	Start()
	Stop() error
}

type Kind int

const (
	HttpTunnelKind Kind = iota
	TcpTunnelKind
)
