# frozen_string_literal: true

module Proxy
  class Middleware
    NOT_FOUND_RESPONSE = [404, {}, []].freeze
    def initialize(app)
      @app = app
    end

    def call(env)
      @request = ActionDispatch::Request.new(env)

      return @app.call(env) unless proxy_call?
      return NOT_FOUND_RESPONSE unless CliConnection.exists?(subdomain: subdomain)

      proxy
    end

    private

    def proxy_call?
      Rails.configuration.proxy['hostnames'].exclude?(@request.host)
    end

    def subdomain
      @subdomain ||= Rails.configuration.proxy['hostnames'].lazy.map do |hostname|
        @request.host.split(hostname).first
      end.detect(&:itself).chomp('.')
    end

    def proxy
      cli_connection = CliConnection.find_by_subdomain(subdomain)
      proxy_conn = TCPSocket.new("0.0.0.0", cli_connection.proxy_connection_port.to_i)
      IO.copy_stream(@request.body_stream, proxy_conn)
      proxy_conn.close
      @request.body_stream.rewind
      [200, {}, ["Success! #{cli_connection.name}"]]
    end
  end
end
