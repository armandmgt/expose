# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# This file is the source Rails uses to define your schema when running `bin/rails
# db:schema:load`. When creating a new database, `bin/rails db:schema:load` tends to
# be faster and is potentially less error prone than running all of your
# migrations from scratch. Old migrations may fail to apply correctly if those
# migrations use external dependencies or application code.
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema[7.0].define(version: 2023_01_21_160744) do
  # These are extensions that must be enabled in order to support this database
  enable_extension "plpgsql"

  create_table "cli_connections", force: :cascade do |t|
    t.bigint "user_id"
    t.text "name", null: false
    t.text "subdomain", null: false
    t.text "proxied_port", null: false
    t.text "proxy_connection_port"
    t.text "upstream_connection_port"
    t.datetime "alive_at", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["name"], name: "index_cli_connections_on_name", unique: true
    t.index ["subdomain"], name: "index_cli_connections_on_subdomain", unique: true
    t.index ["user_id"], name: "index_cli_connections_on_user_id"
  end

  create_table "users", force: :cascade do |t|
    t.string "email", null: false
    t.string "encrypted_password", null: false
    t.string "api_token", null: false
    t.integer "sign_in_count", default: 0, null: false
    t.datetime "current_sign_in_at"
    t.datetime "last_sign_in_at"
    t.string "current_sign_in_ip"
    t.string "last_sign_in_ip"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["api_token"], name: "index_users_on_api_token", unique: true
    t.index ["email"], name: "index_users_on_email", unique: true
    t.index ["encrypted_password"], name: "index_users_on_encrypted_password", unique: true
  end

end
