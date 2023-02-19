# frozen_string_literal: true

class ProxiedRequestsChannel < ApplicationCable::Channel
  before_subscribe :set_cli_connection

  def subscribed
    stream_for @cli_connection
  end

  def unsubscribe
    stop_stream_for @cli_connection
    connection.disconnect
  end

  private

  def set_cli_connection
    @cli_connection = CliConnection.where(user: current_user).find_by(id: params[:id])
    reject unless @cli_connection
  end
end
