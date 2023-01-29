# frozen_string_literal: true

class CliConnection < ApplicationRecord
  belongs_to :user

  attribute :name, type: :string, default: -> { "CLI Connection #{SecureRandom.uuid}" }
  attribute :subdomain, type: :string, default: lambda {
                                                  SubDomainGenerator.generate(unique_by: lambda { |v|
                                                                                           CliConnection.exists?(subdomain: v)
                                                                                         })
                                                }

  validates :name, :subdomain, :proxied_port, presence: true
end
