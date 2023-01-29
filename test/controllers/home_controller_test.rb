# frozen_string_literal: true

require 'test_helper'

class HomeControllerTest < ActionDispatch::IntegrationTest
  include Devise::Test::IntegrationHelpers

  setup do
    @user = users(:one)
    sign_in @user
  end

  teardown do
    sign_out @user
  end

  test 'should get show' do
    get root_url

    assert_response :success
  end
end
