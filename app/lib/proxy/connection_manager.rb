# frozen_string_literal: true

module Proxy
  class ConnectionManager
    class << self
      def singleton
        @singleton ||= new
      end
    end

    def initialize
      @proxy_connections = []
    end

    def start(connection)
      proxy_conn_ractor = proxy_ractor(connection)
      proxy_port, upstream_port = proxy_conn_ractor.take.values_at(:proxy_port, :upstream_port)
      connection.update({
        proxy_connection_port: proxy_port,
        upstream_connection_port: upstream_port,
      })
      @proxy_connections << proxy_conn_ractor
    end

    def pinged; end

    def forward(request)
      ;
    end

    def shutdown
      @proxy_connections.each do |r|
        r.send(:quit)
        r.take
      end
    end

    private

    def proxy_ractor(connection)
      Ractor.new(name: "proxy-conn-#{connection.name}") do
        proxy_server = TCPServer.new('0.0.0.0', 0)
        upstream_server = TCPServer.new('0.0.0.0', 0)
        Ractor.yield({
          proxy_port: proxy_server.addr[1],
          upstream_port: upstream_server.addr[1],
        })
        msg_pipe = Ractor.new do
          loop { Ractor.yield(Ractor.recv) }
        end
        loop do
          tcp_conn = upstream_server.accept
          Ractor.new(msg_pipe, tcp_conn, name: "#{Ractor.current.name}-upstream-writer") do |msg_pipe, upstream_conn|
            loop do
              msg = msg_pipe.take
              upstream_conn.write(msg)
            end
          end
          Ractor.new(msg_pipe, proxy_server, name: "#{Ractor.current.name}-proxy-reader") do |msg_pipe, serv|
            loop do
              proxy_conn = serv.accept
              msg = proxy_conn.read
              msg_pipe.send(msg, move: true)
              proxy_conn.close
            end
          end
          tcp_conn.close
        end
      end
    end
  end
end
