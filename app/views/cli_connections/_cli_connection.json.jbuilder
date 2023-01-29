# frozen_string_literal: true

json.extract! cli_connection, :id, :created_at, :updated_at
json.url cli_connection_url(cli_connection, format: :json)
