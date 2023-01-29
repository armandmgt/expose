# frozen_string_literal: true

class UsersController < ApplicationController
  def edit; end

  def update
    if current_user.update(user_params)
      redirect_to :edit_user, notice: I18n.t('cli_connection.update.success_notice')
    else
      redirect_to :edit_user, status: :unprocessable_entity
    end
  end

  private

  # Only allow a list of trusted parameters through.
  def user_params
    params.require(:user).permit(:api_token)
  end
end
