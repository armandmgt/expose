# frozen_string_literal: true

class ApplicationController < ActionController::Base
  skip_before_action :verify_authenticity_token, if: :json_request?
  before_action :authenticate_user_from_token!
  before_action :authenticate_user!

  private

  def authenticate_user_from_token!
    user_email, api_token = request.authorization&.split(':')
    user = user_email && User.find_by(email: user_email)

    return unless user && Devise.secure_compare(user.api_token, api_token)

    sign_in user, store: false
  end

  def json_request?
    request.format.json?
  end
end
