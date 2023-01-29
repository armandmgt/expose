class SessionsController < Devise::SessionsController
  def destroy
    super do
      # Turbo requires redirects be :see_other (303); so override Devise default (302)
      return redirect_to new_user_session_path, status: :see_other, notice: I18n.t("devise.sessions.signed_out")
    end
  end
end
