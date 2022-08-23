package main

import (
	"flag"
)

type ClientOpts struct {
	ServerName string
	ServerPort uint
}

func parseArgs() (clientOpts ClientOpts) {
	clientOpts = ClientOpts{}
	flag.StringVar(&clientOpts.ServerName, "serverName", "localhost", "Sets the port where the proxy should listen.")
	flag.UintVar(&clientOpts.ServerPort, "port", 8080, "Sets the port where the proxy should listen.")
	flag.Parse()
	return
}
