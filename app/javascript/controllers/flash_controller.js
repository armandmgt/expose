import { Controller } from "@hotwired/stimulus"
import { RandomToken } from "../RandomToken"

export default class extends Controller {
  dismiss() {
    this.element.classList.add("hidden")
  }
}
