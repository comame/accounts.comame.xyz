export type apis = {
    "/rp/list": [empty, listRpResponse]
    "/rp/create": [clientIdRequest, createRpResponse]
    "/rp/update_secret": [clientIdRequest, createRpResponse]
    "/rp/delete": [clientIdRequest, empty]
    "/rp/redirect_uri/add": [relyingPartyUriRequest, empty]
    "/rp/redirect_uri/remove": [relyingPartyUriRequest, empty]
    "/rp/binding/list": [clientIdRequest, bindingResponse]
    "/rp/binding/add": [userBinding, empty]
    "/rp/binding/remove": [userBinding, empty]
    "/user/list": [empty, userListResponse]
    "/user/create": [userIdRequest, empty]
    "/user/delete": [userIdRequest, empty]
    "/user/password/change": [userIdPasswordRequest, empty]
    "/user/password/remove": [userIdRequest, empty]
    "/user/authentication/list": [userIdRequest, authenticationListResponse]
    "/rp/federated_user_binding/list": [
        clientIdRequest,
        federatedUserBindingListResponse
    ]
    "/rp/federated_user_binding/add": [federatedUserBindingRequest, empty]
    "/rp/federated_user_binding/remove": [federatedUserBindingRequest, empty]
    "/user/role/list": [userIdRequest, rolesResponse]
    "/user/role/set": [rolesRequest, empty]
    "/rp/role/set": [setRoleAccessResponse, empty]
    "/rp/role/list": [clientIdRequest, setRoleAccessResponse]
    "/role/list": [empty, listRoleResponse]
    "/role/create": [roleNameRequest, empty]
    "/role/delete": [roleNameRequest, empty]
    "/user/userinfo/get": [userIdRequest, userinfoReponse]
}

export type request<T extends keyof apis> = apis[T][0]
export type response<T extends keyof apis> = apis[T][1]

type empty = {}

export type relyingParty = {
    client_id: string
    redirect_uris: string[]
    hashed_client_secret: string
}

type listRpResponse = {
    values: relyingParty[]
}

type clientIdRequest = {
    client_id: string
}

type relyingPartyUriRequest = {
    client_id: string
    redirect_uri: string
}

type createRpResponse = {
    client_id: string
    client_secret: string
}

type userIdRequest = {
    user_id: string
}

type userIdPasswordRequest = {
    user_id: string
    password: string
}

type userListResponse = {
    values: {
        user_id: string
        has_password: boolean
    }[]
}

type idTokenIssue = {
    sub: string
    aud: string
    iat: number
    remote_addr: string
}

type authenticationListResponse = {
    values: idTokenIssue[]
}

type userBinding = {
    client_id: string
    user_id: string
}

type bindingResponse = {
    values: userBinding[]
}

type federatedUserBindingRequest = {
    client_id: string
    issuer: string
}

type federatedUserBindingListResponse = {
    values: {
        relying_party_id: string
        issuer: string
    }[]
}

type rolesRequest = {
    roles: string[]
    user_id: string
}

type rolesResponse = {
    roles: string[]
    user_id: string
}

type setRoleAccessResponse = {
    client_id: string
    roles: string[]
}

type listRoleResponse = {
    values: string[]
}

type roleNameRequest = {
    name: string
}

type userinfoReponse = {
    value: string
}
