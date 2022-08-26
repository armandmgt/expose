package clients

import (
	"sync"
	"time"
)

type Handler struct {
	Clients map[string]*Client
	quit    chan interface{}
	wg      sync.WaitGroup
}

func NewHandler() *Handler {
	handler := Handler{Clients: make(map[string]*Client), quit: make(chan interface{})}
	go handler.collect()
	return &handler
}

func (h *Handler) Stop() {
	close(h.quit)
	h.wg.Wait()
}

func (h *Handler) collect() {
	h.wg.Add(1)
	for {
		select {
		case <-h.quit:
			h.clean()
			h.wg.Done()
			return
		case <-time.After(time.Second):
			h.clean()
		}
	}
}

func (h *Handler) clean() {
	for _, client := range h.Clients {
		h.wg.Add(1)
		if client.IsExpired() {
			//log.Printf("client %s is expired\n", client.UUID)
			delete(h.Clients, client.UUID)
			if client.Tunnel != nil {
				//log.Printf("cleaning tunnel %p\n", client.Tunnel)
				if err := client.Tunnel.Stop(); err != nil {
					return
				}
				//log.Printf("successfully cleaned tunnel %p\n", client.Tunnel)
			}
		}
		h.wg.Done()
	}
}
