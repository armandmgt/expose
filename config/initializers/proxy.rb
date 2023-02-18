# frozen_string_literal: true

Rails.application.config.middleware.use ReloadableMiddleware.wrap('Proxy::Middleware')

at_exit do
  Proxy::ConnectionManager.singleton.shutdown
end
