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

    def forward(request); end

    def shutdown
      @proxy_connections.each do |r|
        r.send(:quit)
        r.take
      end
    end

    private

    def proxy_ractor(connection)
      initial_msg = {
        connection: Ractor.make_shareable(connection, copy: true),
        time_zone: Ractor.make_shareable(Time.zone, copy: true),
        logger: Ractor.make_shareable(Rails.logger, copy: true),
      }
      Ractor.new(initial_msg, name: "proxy-conn-#{connection.name}") do |msg|
        c, tz, logger = msg.values_at(:connection, :time_zone, :logger)
        proxy_server = TCPServer.new('0.0.0.0', 0)
        upstream_server = TCPServer.new('0.0.0.0', 0)
        Ractor.yield({
          proxy_port: proxy_server.addr[1],
          upstream_port: upstream_server.addr[1],
        })
        logger.info('after Ractor.yield')
        loop do
          tcp_conn, = upstream_server.accept
          c.update(alive_at: tz.now)
          tcp_conn.puts 'HELLO'
          tcp_conn.close
        end
      end
    end
  end
end
