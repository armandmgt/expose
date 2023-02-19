# frozen_string_literal: true

module DomainMatcher
  class DynamicDomain
    def self.matches?(request)
      Rails.configuration.proxy['hostnames'].exclude?(request.host).tap do |value|
        Rails.logger.info({ host: request.host, dynamic?: value })
      end
    end

    def self.regex
      /\.(#{Rails.configuration.proxy['hostnames'].compact_blank.join('|')})\z/.tap do |value|
        Rails.logger.info({ dynamic?: value })
      end
    end

    def self.subdomain(request)
      Rails.configuration.proxy['hostnames'].compact.filter_map do |hostname|
        request.host.split(hostname).first if request.host.include? hostname
      end.detect(&:itself).chomp('.')
    end
  end
end
