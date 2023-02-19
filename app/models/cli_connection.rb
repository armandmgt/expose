# frozen_string_literal: true

class CliConnection < ApplicationRecord
  belongs_to :user

  validates :name, :subdomain, :proxied_port, presence: true
  validates :name, :subdomain, uniqueness: true

  after_initialize :set_default_name, :set_default_subdomain
  after_destroy :notify_clients

  private

  def set_default_name
    self.name ||= "CLI Connection #{SecureRandom.uuid}"
  end

  def set_default_subdomain
    self.subdomain ||= SubDomainGenerator.generate(unique_by: lambda { |v|
      CliConnection.exists?(subdomain: v)
    })
  end

  def notify_clients
    ProxiedRequestsChannel.broadcast_to self, { type: :cli_connection_destroyed }
  end
end
