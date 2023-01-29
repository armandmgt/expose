import { Controller } from "@hotwired/stimulus"
import { RandomToken } from "../RandomToken"

export default class extends Controller {
  static targets = ["inputField"]

  generate() {
    this.inputFieldTarget.value = RandomToken.generateId()
  }
}
