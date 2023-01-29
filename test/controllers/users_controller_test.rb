# frozen_string_literal: true

require 'test_helper'

class UsersControllerTest < ActionDispatch::IntegrationTest
  include Devise::Test::IntegrationHelpers

  setup do
    @user = users(:one)
    sign_in @user
  end

  teardown do
    sign_out @user
  end

  test 'should get edit' do
    get edit_user_url(@user)

    assert_response :success
  end

  test 'should patch update' do
    patch user_url(users(:one)), params: { user: { api_token: 'new_token' } }

    assert_response :found
  end
end
