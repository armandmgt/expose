package tunnel

type HttpTunnel struct {
	Kind Kind
}

func NewHTTP() *HttpTunnel {
	return &HttpTunnel{HttpTunnelKind}
}

func (t *HttpTunnel) Start() {
}

func (t *HttpTunnel) Stop() error {
	return nil
}
