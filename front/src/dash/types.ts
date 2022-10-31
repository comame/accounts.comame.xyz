export type apis = {
    "/dash/rp/list": [empty, listRpResponse],
    "/dash/rp/create": [clientIdRequest, createRpResponse],
    "/dash/rp/delete": [clientIdRequest, empty],
    "/dash/rp/redirect_uri/add": [relyingPartyUriRequest, empty],
    "/dash/rp/redirect_uri/remove": [relyingPartyUriRequest, empty],
    "/dash/user/list": [empty, userListResponse],
    "/dash/user/create": [userIdRequest, empty],
    "/dash/user/delete": [userIdRequest, empty],
    "/dash/user/password/change": [userIdPasswordRequest, empty],
    "/dash/user/password/remove": [userIdRequest, empty],
    "/dash/user/authentication/list": [userIdRequest, authenticationListResponse]
}

export type request<T extends keyof apis> = apis[T][0]
export type response<T extends keyof apis> = apis[T][1]

type empty = {}

export type relyingParty = {
    client_id: string,
    redirect_uris: string[],
    hashed_client_secret: string
}

type listRpResponse = {
    values: relyingParty[]
}

type clientIdRequest = {
    client_id: string
}

type relyingPartyUriRequest = {
    client_id: string,
    redirect_uri: string
}

type createRpResponse = {
    client_id: string,
    client_secret: string,
}

type userIdRequest = {
    user_id: string
}

type userIdPasswordRequest = {
    user_id: string,
    password: string,
}

type userListResponse = {
    values: {
        user_id: string,
        has_password: boolean
    }[]
}

type idTokenIssue = {
    sub: string,
    aud: string,
    iat: number
}

type authenticationListResponse = {
    values: idTokenIssue[]
}
