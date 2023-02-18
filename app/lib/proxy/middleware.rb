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
      [200, {}, ['Success!']]
    end
  end
end
