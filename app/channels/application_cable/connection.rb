# frozen_string_literal: true

module ApplicationCable
  class Connection < ActionCable::Connection::Base
    include Authenticate

    identified_by :current_user

    def connect
      if (self.current_user = find_user_from_token)
        current_user
      else
        reject_unauthorized_connection
      end
    end
  end
end
