# frozen_string_literal: true

class CliConnection < ApplicationRecord
  belongs_to :user

  validates :name, :subdomain, :proxied_port, presence: true

  after_initialize :set_default_name, :set_default_subdomain

  private

  def set_default_name
    self.name ||= "CLI Connection #{SecureRandom.uuid}"
  end

  def set_default_subdomain
    self.subdomain ||= SubDomainGenerator.generate(unique_by: lambda { |v|
      CliConnection.exists?(subdomain: v)
    })
  end
end
