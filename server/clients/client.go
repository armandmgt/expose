package clients

import (
	"errors"
	"github.com/armandmgt/expose/server/tunnel"
	"github.com/google/uuid"
	"time"
)

type Client struct {
	expiryTime time.Time
	UUID       string
	Tunnel     tunnel.Tunnel
}

const ClientTimeout = 5 * time.Second

func (c *Client) IsExpired() bool {
	return c.expiryTime.Before(time.Now())
}

func (c *Client) ExtendLife() error {
	if c.IsExpired() {
		return errors.New("client is already expired")
	}
	c.expiryTime = time.Now().Add(ClientTimeout)
	return nil
}

func NewClient() *Client {
	return &Client{expiryTime: time.Now().Add(ClientTimeout), UUID: uuid.NewString()}
}
