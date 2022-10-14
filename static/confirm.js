// @ts-check

/** @type { HTMLFormElement } form */
const passwordForm = document.getElementById("password-form")
/** @type { HTMLMetaElement } tokenEl*/
const tokenEl = document.getElementById('csrf-token')
const idEl = document.getElementById('user_id')
const idReadEl = document.getElementById('user_id_show')

const stateId = new URL(location.href).searchParams.get('sid')
const relyingPartyId = decodeURIComponent(new URL(location.href).searchParams.get('cid'))
let hash = location.hash.slice(1)

fetch('/api/signin-session', {
    method: 'POST',
    credentials: 'include',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        csrf_token: tokenEl.content
    })
}).then(res => res.json()).then(json => {
    if (json.user_id) {
        idReadEl.textContent = json.user_id
        document.body.classList.remove('hidden')
    } else {
        location.href = `/signin?sid=${stateId}&cid=${encodeURIComponent(relyingPartyId)}`
    }
})

document.getElementById('other-account').addEventListener('click', (e) => {
    e.preventDefault()
    const continueUrl = encodeURIComponent(`/signin?sid=${stateId}&cid=${encodeURIComponent(relyingPartyId)}`)
    location.href = `/signout?continue=${continueUrl}`
})

document.getElementById('continue-button').addEventListener('click', () => {
    continueSignin('consent')
})

function continueSignin(auth_method) {
    /** @type {HTMLFormElement} */
    const form = document.getElementById('continue')
    form.csrf_token.value = tokenEl.content
    form.login_type.value = auth_method
    form.state_id.value = stateId
    form.relying_party_id.value = relyingPartyId

    form.submit()
}
