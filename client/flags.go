package main

import (
	"flag"
	log "github.com/sirupsen/logrus"
)

type ClientOpts struct {
	ServerName string
	ServerPort uint
}

func parseArgs() (clientOpts ClientOpts) {
	clientOpts = ClientOpts{}
	flag.StringVar(&clientOpts.ServerName, "serverName", "localhost", "Sets the port where the proxy should listen.")
	flag.UintVar(&clientOpts.ServerPort, "port", 8080, "Sets the port where the proxy should listen.")
	jsonLogs := flag.Bool("json-logs", false, "Sets the log formatter as JSON.")
	logLevel := flag.String("log-level", "info", "Sets the log level of the application.")
	flag.Parse()
	if *jsonLogs {
		log.SetFormatter(&log.JSONFormatter{})
	}
	switch *logLevel {
	case "trace":
		log.SetLevel(log.TraceLevel)
	case "debug":
		log.SetLevel(log.DebugLevel)
	case "info":
		log.SetLevel(log.InfoLevel)
	case "warn":
		log.SetLevel(log.WarnLevel)
	case "fatal":
		log.SetLevel(log.FatalLevel)
	}
	log.SetReportCaller(true)
	return
}
