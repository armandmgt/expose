# frozen_string_literal: true

class ProxiedRequestsController < ApplicationController
  skip_before_action :verify_authenticity_token
  skip_before_action :authenticate_user_from_token!, if: :json_request?
  skip_before_action :authenticate_user!

  def proxy
    subdomain = DomainMatcher::DynamicDomain.subdomain(request)
    @cli_connection = CliConnection.find_by(subdomain: subdomain)
    render 'cli_connections/show', format: :json
  end
end
