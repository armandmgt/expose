# frozen_string_literal: true

Rails.application.routes.draw do
  constraints DomainMatcher::NakedDomain do
    devise_for :users, controllers: { sessions: 'sessions' }
    # Define your application routes per the DSL in https://guides.rubyonrails.org/routing.html

    # Defines the root path route ("/")
    root 'home#show'

    resources :cli_connections, only: %i[show new create update destroy]
    resources :users, only: %i[edit update]
  end

  constraints DomainMatcher::DynamicDomain do
    root 'proxied_requests#proxy', via: :all, as: :proxy_root
    match '*path', to: 'proxied_requests#proxy', via: :all, as: :proxy
  end
end
