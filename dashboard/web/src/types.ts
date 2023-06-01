export type apis = {
    "/dash/rp/list": [empty, listRpResponse]
    "/dash/rp/create": [clientIdRequest, createRpResponse]
    "/dash/rp/update_secret": [clientIdRequest, createRpResponse]
    "/dash/rp/delete": [clientIdRequest, empty]
    "/dash/rp/redirect_uri/add": [relyingPartyUriRequest, empty]
    "/dash/rp/redirect_uri/remove": [relyingPartyUriRequest, empty]
    "/dash/rp/binding/list": [clientIdRequest, bindingResponse]
    "/dash/rp/binding/add": [userBinding, empty]
    "/dash/rp/binding/remove": [userBinding, empty]
    "/dash/user/list": [empty, userListResponse]
    "/dash/user/create": [userIdRequest, empty]
    "/dash/user/delete": [userIdRequest, empty]
    "/dash/user/password/change": [userIdPasswordRequest, empty]
    "/dash/user/password/remove": [userIdRequest, empty]
    "/dash/user/authentication/list": [
        userIdRequest,
        authenticationListResponse
    ],
    "/dash/rp/federated_user_binding/list": [clientIdRequest, federatedUserBindingListResponse],
    "/dash/rp/federated_user_binding/add": [federatedUserBindingRequest, empty],
    "/dash/rp/federated_user_binding/remove": [federatedUserBindingRequest, empty],
    "/dash/user/role/list": [userIdRequest, rolesResponse],
    "/dash/user/role/set": [rolesRequest, empty],
    "/dash/rp/role/set": [setRoleAccessResponse, empty],
    "/dash/rp/role/list": [clientIdRequest, setRoleAccessResponse],
    "/dash/role/list": [empty, listRoleResponse],
    "/dash/role/create": [roleNameRequest, empty],
    "/dash/role/delete": [roleNameRequest, empty],
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
    client_id: string,
    issuer: string,
}

type federatedUserBindingListResponse = {
    values: {
        relying_party_id: string,
        issuer: string,
    }[]
}

type rolesRequest = {
    roles: string[],
    user_id: string
}

type rolesResponse = {
    roles: string[],
    user_id: string
}

type setRoleAccessResponse = {
    client_id: string,
    roles: string[],
}

type listRoleResponse = {
    values: string[],
}

type roleNameRequest = {
    name: string,
}
