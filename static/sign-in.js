
/** @type { HTMLFormElement } form */
const passwordForm = document.getElementById("password-form")
/** @type { HTMLMetaElement } tokenEl*/
const tokenEl = document.getElementById('csrf-token')
const idEl = document.getElementById('user_id_read')
const signoutButton = document.getElementById('signout')

fetch('/signin-session', {
    method: 'POST',
    credentials: 'include',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        csrf_token: tokenEl.content
    })
}).then(res => {
    if (res.status == 200) {
        continueSignin()
    }
})

passwordForm.addEventListener("submit", e => {
    e.preventDefault()
    const formData = new FormData(passwordForm)
    const body = JSON.stringify({
        user_id: formData.get('user_id'),
        password: formData.get('password'),
        csrf_token: tokenEl.content,
    })
    fetch('/signin-password', {
        method: 'POST',
        body,
        headers: {
            'Content-Type': 'application/json'
        }
    }).then(res => {
        if (res.status == 200) {
            continueSignin()
        }
    })
})

function continueSignin() {
    /** @type {HTMLFormElement} */
    const form = document.getElementById('continue')
    form.csrf_token.value = tokenEl.content
    form.submit()
}
