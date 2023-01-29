# frozen_string_literal: true

class AddApiTokenToUsers < ActiveRecord::Migration[7.0]
  def change
    add_column :users, :api_token, :text, null: false, default: ''
    add_index :users, :api_token, unique: true
  end
end
