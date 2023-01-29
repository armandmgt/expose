# frozen_string_literal: true

class HomeController < ApplicationController
  def show
    @cli_connections = current_user.cli_connections
  end
end
