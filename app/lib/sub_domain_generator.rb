# frozen_string_literal: true

class SubDomainGenerator
  WORDS = %w[excellent shrill cheerful obey creator mean fascinated concentrate
             crawl voiceless probable incompetent].freeze

  class << self
    def generate(unique_by: -> {})
      MAX_RETRIES.times do
        v = Array.new(3) { WORDS[rand(WORDS.length)] }.join('-')
        return v unless unique_by[v]
      end
      raise "failed to generate a unique domain after #{MAX_RETRIES} retries"
    end
  end

  MAX_RETRIES = 3
  private_constant :MAX_RETRIES
end
