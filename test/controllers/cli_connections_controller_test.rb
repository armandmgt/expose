# frozen_string_literal: true

require 'test_helper'

class CliConnectionsControllerTest < ActionDispatch::IntegrationTest
  include Devise::Test::IntegrationHelpers

  setup do
    @cli_connection = cli_connections(:one)
    sign_in @cli_connection.user
  end

  teardown do
    sign_out @cli_connection.user
  end

  test 'should get new' do
    get new_cli_connection_url

    assert_response :success
  end

  test 'should create cli_connection' do
    assert_difference('CliConnection.count') do
      post cli_connections_url, params: { proxied_port: '4000' }.to_json, headers: json_header
    end

    assert_response :created
  end

  test 'should show cli_connection' do
    get cli_connection_url(@cli_connection)

    assert_response :success
  end

  test 'should update cli_connection' do
    patch cli_connection_url(@cli_connection), params: { cli_connection: { alive_at: Time.current } }.to_json,
      headers: json_header

    assert_response :success
  end

  test 'should destroy cli_connection' do
    assert_difference('CliConnection.count', -1) do
      delete cli_connection_url(@cli_connection)
    end

    assert_redirected_to root_url
  end
end
