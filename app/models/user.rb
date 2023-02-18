# frozen_string_literal: true

class User < ApplicationRecord
  # Include default devise modules. Others available are:
  # :confirmable, :lockable, :timeoutable, :trackable and :omniauthable
  devise :database_authenticatable, :trackable

  has_many :cli_connections, dependent: :destroy

  before_save :ensure_api_token

  def ensure_api_token
    return if api_token.present?

    self.api_token = generate_api_token
  end

  private

  def generate_api_token
    loop do
      token = SecureRandom.hex(20)
      break token unless User.exists?(api_token: token)
    end
  end
end
