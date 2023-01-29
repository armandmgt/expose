import { Controller } from '@hotwired/stimulus'

export default class extends Controller {
  static targets = ['contentNode']

  copy () {
    try {
      this.contentNodeTarget
      if (document.body.createTextRange) {
        const range = document.body.createTextRange()
        range.moveToElementText(this.contentNodeTarget)
        range.select()
      } else if (window.getSelection) {
        const selection = window.getSelection()
        const range = document.createRange()
        range.selectNodeContents(this.contentNodeTarget)
        selection.removeAllRanges()
        selection.addRange(range)
      } else {
        this.failed()
        return
      }

      document.execCommand('copy')

      this.success()
    } catch (error) {
      console.error(error)
      this.failed()
    }
  }

  success () {
    this.element.querySelector('.copy-icon').classList.toggle('hidden')
    this.element.querySelector('.success-icon').classList.toggle('hidden')
    setTimeout(() => {
      this.element.querySelector('.copy-icon').classList.toggle('hidden')
      this.element.querySelector('.success-icon').classList.toggle('hidden')
    }, 2000)
  }

  failed () {
    this.element.querySelector('.copy-icon').classList.toggle('hidden')
    this.element.querySelector('.failure-icon').classList.toggle('hidden')
    setTimeout(() => {
      this.element.querySelector('.copy-icon').classList.toggle('hidden')
      this.element.querySelector('.failure-icon').classList.toggle('hidden')
    }, 2000)
  }
}
