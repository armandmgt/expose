# frozen_string_literal: true

class ApplicationController < ActionController::Base
  include Authenticate

  skip_before_action :verify_authenticity_token, if: :json_request?
  before_action :authenticate_user_from_token!, if: :json_request?
  before_action :authenticate_user!

  rescue_from UnauthenticatedError do
    redirect_to :new_user_session
  end

  private

  def authenticate_user_from_token!
    raise UnauthenticatedError unless (user = find_user_from_token)

    sign_in user, store: false
  end

  def json_request?
    request.format.json?
  end
end
