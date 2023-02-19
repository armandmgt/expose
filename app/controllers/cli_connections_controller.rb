# frozen_string_literal: true

class CliConnectionsController < ApplicationController
  before_action :set_cli_connection, only: %i[show update destroy]

  # GET /cli_connections/1 or /cli_connections/1.json
  def show; end

  # GET /cli_connections/new
  def new
    @cli_connection = current_user.cli_connections.new
  end

  # POST /cli_connections.json
  def create
    @cli_connection = current_user.cli_connections.new(cli_connection_params.merge(alive_at: Time.current))
    if @cli_connection.save
      render :show, status: :created, location: @cli_connection
    else
      render json: @cli_connection.errors, status: :unprocessable_entity
    end
  end

  # PATCH/PUT /cli_connections/1 or /cli_connections/1.json
  def update
    respond_to do |format|
      if @cli_connection.update(cli_connection_params)
        format.json { render :show, status: :ok, location: @cli_connection }
      else
        format.json { render json: @cli_connection.errors, status: :unprocessable_entity }
      end
    end
  end

  # DELETE /cli_connections/1 or /cli_connections/1.json
  def destroy
    @cli_connection.destroy

    respond_to do |format|
      format.html { redirect_to root_path, notice: I18n.t('cli_connection.destroy.success_notice') }
      format.json { head :no_content }
    end
  end

  private

  # Use callbacks to share common setup or constraints between actions.
  def set_cli_connection
    @cli_connection = CliConnection.find(params[:id])
  end

  # Only allow a list of trusted parameters through.
  def cli_connection_params
    params.require(:cli_connection).permit(:name, :subdomain, :proxied_port)
  end
end
