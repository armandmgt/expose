# frozen_string_literal: true

module DomainMatcher
  class NakedDomain
    def self.matches?(request)
      Rails.configuration.proxy['hostnames'].include?(request.host).tap do |value|
        Rails.logger.info({ host: request.host, naked?: value })
      end
    end

    def self.regex
      /\A(#{Rails.configuration.proxy['hostnames'].compact_blank.join('|')})\z/.tap do |value|
        Rails.logger.info({ naked?: value })
      end
    end
  end
end
