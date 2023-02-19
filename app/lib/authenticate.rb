# frozen_string_literal: true

module Authenticate
  extend ActiveSupport::Concern

  class UnauthenticatedError < StandardError; end

  included do
    def find_user_from_token
      user_email, api_token = request.authorization&.split(':')
      user = user_email && User.find_by(email: user_email)

      return user if Devise.secure_compare(user&.api_token, api_token)
    end
  end
end
