package main

import (
	"flag"
)

type ProxyOpts struct {
	Port uint
}

func parseArgs() (proxyOpts ProxyOpts) {
	proxyOpts = ProxyOpts{}
	flag.UintVar(&proxyOpts.Port, "port", 8080, "Sets the port where the proxy should listen.")
	flag.Parse()
	return
}
