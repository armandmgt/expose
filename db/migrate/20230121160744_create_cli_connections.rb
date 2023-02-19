# frozen_string_literal: true

class CreateCliConnections < ActiveRecord::Migration[7.0]
  def change
    create_table :cli_connections do |t|
      t.belongs_to :user

      t.text :name, null: false, index: { unique: true }
      t.text :subdomain, null: false, index: { unique: true }
      t.text :proxied_port, null: false
      t.text :proxy_connection_port
      t.text :upstream_connection_port
      t.datetime :alive_at, null: false

      t.timestamps
    end
  end
end
